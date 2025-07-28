use crate::register_op::RegisterOp;
use crate::unsafes::mmap_memory;
use aep_rust_common::descender::Descender;
//
// SPDX-License-Identifier: MIT
//
// Copyright (c) 2025 Andrew Ellis Page
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
use std::io::Write;

pub struct RegisterTool {
    descender: Box<dyn Descender<dyn Write>>,
    regs: Vec<RegisterOp>,
    addr: *mut u8,
    test_mode: bool,
    device: String,
    base: u64,
    length: u64,
}

impl RegisterTool {
    pub fn dump_registers(&mut self, reg_paths: &Vec<&str>) -> Result<(), String> {
        let mut fail = false;
        let old_root = match &self.descender.get_string_field_or_parent("completion-metadata", "root") {
            Ok(r) => self.descender.set_root(r).unwrap(),
            Err(_s) => {"".to_string()}
        } ;
        let mut bad_regs: Vec<&str> = Vec::new() ;
        
        for reg in reg_paths {
            let parts: Vec<&str> = reg.split('=').collect();
            let path = parts[0];

            let offset = self.descender.get_int_field_or_parent(path, "offset");
            let rw  = self.descender.get_string_field_or_parent(path, "read-write").unwrap_or("unspecified".to_string());
            let width = self.descender.get_int_field_or_parent(path, "width").unwrap_or(32);
            let bits = self.descender.get_string_field_or_parent(path, "bits").unwrap_or("".to_string());
            let desc = self.descender.get_string_field_or_parent(path, "description").unwrap_or("not given".to_string());

            println!("{path}:") ;
            match offset {
               Ok(o) => println!("   offset: 0x{o:04X}"),
               Err(e) => { 
                   println!("  {e} offset: NOT FOUND") ;
                   fail = true ;
                   bad_regs.push(path) ;
               }
            } ;
            println!("   read-write: {}", rw) ;
            println!("   width: {}", width) ;
            println!("   bits: {}", bits) ;
            println!("   description: \"{}\"", desc) ;
        }
        match self.descender.set_root(&*old_root) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error resetting root: {}", e);
                fail = true;
            }
        }
        if fail {
            let bad_reg_list = bad_regs.join(" ");
            Err(format!("invalid registers: {}", bad_reg_list))
        }
        else {
            Ok(())
        }
    }
}

impl RegisterTool {
    pub fn new(descender: Box<dyn Descender<dyn Write>>) -> Result<Self, Vec<String>> {
        let mut errs: Vec<String> = Vec::new();

        /*
         * Validate the integrity of the file
         */
        let device = match descender.get_string_field_or_parent("","device") {
            Ok(d) => d,
            Err(e) => {
                errs.push(format!("device not found: {}", e));
                "".to_string()
            }
        } ;

        let base = match descender.get_int_field_or_parent("","base") {
            Ok(b) => b,
            Err(e) => {
                errs.push(format!("base not found: {}", e));
                0
            }
        }  as u64 ;

        let length = match descender.get_int_field_or_parent("","length") {
            Ok(l) => l,
            Err(e) => {
                errs.push(format!("length not found: {}", e));
                0
            }
        } as u64 ;

        /*
         * return all collected errors
         */
        if errs.len() > 0 {
            return Err(errs);
        }
        let register_tool = Self {descender, regs: Vec::new(), addr: std::ptr::null_mut(), test_mode: false, device, base, length } ;

        Ok(register_tool)
    }

    pub fn set_base_address(&mut self) -> Result<(), String> {
        self.addr = match mmap_memory(self.device.as_str(), self.base, self.length) {
            Ok(a) => a,
            Err(e) => {
                return Err(format!("Error mapping memory: {}", e));
            }
        } ;
        Ok(())
    }

    pub fn set_test_area(&mut self) {
        let mut memory = Vec::with_capacity(self.length as usize);
        memory.resize(self.length as usize, 0u8);
        self.addr = memory.as_mut_ptr();
        std::mem::forget(memory);
        self.test_mode = true;
    }


    ///
    /// gather the registers to set or read.
    ///
    /// return Ok or the collected errors
    ///
    pub fn gather_regs(&mut self, regsspecs: &Vec<&str>) -> Result<(), Vec<String>> {
        let mut errs: Vec<String> = Vec::new();

        let old_root = match &self.descender.get_string_field_or_parent("completion-metadata", "root") {
            Ok(r) => self.descender.set_root(r).unwrap(),
            Err(_s) => {"".to_string()}
        } ;

        for spec in regsspecs {
            let parts = spec.split("=").collect::<Vec<&str>>();
            if parts.len() > 2 {
                errs.push(format!("Bad argument {}", spec));
                continue ;
            }
            let is_set = parts.len() == 2 ;
            let value = if is_set {
                match parts[1].parse::<u32>() {
                    Ok(v) => Some(v),
                    Err(_) => {
                        errs.push(format!("Bad argument {}", spec)) ;
                        None
                    }
                }
            } else { None };

            let r = match RegisterOp::new(&*self.descender, value, parts[0]) {
                Ok(r) => r,
                Err(e) => { errs.push(e) ;
                    RegisterOp::noop()
                }
            } ;

            self.regs.push(r)
        }

        match self.descender.set_root(&*old_root) {
            Ok(_) => {},
            Err(e) => {
                println!("Error resetting root: {}", e);
                errs.push(format!("Error resetting old root: {}", e));
            }
        } ;

        if errs.len() > 0 {
            Err(errs)
        } else {
            Ok(())
        }
    }

    pub fn apply_registers<F>(&self, f: F) -> Result<Vec<Result<u32, String>>, String>
    where
        F: Fn(u32) -> Result<u32, String>,
    {
        let mut results : Vec<Result<u32, String>> = Vec::new();
        for reg in &self.regs {
            if !reg.value.is_some() {
                results.push(f(reg.get(self.addr)));
            }
            else {
                results.push(match reg.set(self.addr) {
                    Ok(i) => f(i),
                    Err(e) => Err(e),
                });
            }
        }
        Ok(results)
    }
}

///
/// Free any memory we might have allocated
///
impl Drop for RegisterTool {
    fn drop(&mut self) {
        if self.test_mode {
            unsafe {
                std::ptr::drop_in_place(self.addr);
            }
        }
    }
}