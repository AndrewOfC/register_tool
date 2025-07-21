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
use clap::parser::ValuesRef;
use libc::fanotify_init;
use regex::Regex;
use yaml_rust::Yaml;
use aep_rust_common::descender;
use aep_rust_common::descender::Descender;
use crate::register_op::RegisterOp;
use crate::unsafes::mmap_memory;

pub struct RegisterTool {
    descender: Box<dyn Descender<dyn Write>>,
    regs: Vec<RegisterOp>,
    addr: *mut u8,
    test_mode: bool,
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
                   println!("   offset: NOT FOUND") ; 
                   fail = true ;
                   bad_regs.push(path) ;
               }
            } ;
            println!("   read-write: {}", rw) ;
            println!("   width: {}", width) ;
            println!("   bits: {}", bits) ;
            println!("   description: \"{}\"", desc) ;
        }
        self.descender.set_root(&*old_root) ;
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
    pub fn new(descender: Box<dyn Descender<dyn Write>>) -> Self {
        Self {descender, regs: Vec::new(), addr: std::ptr::null_mut(), test_mode: false }
    }

    pub fn set_base_address(&mut self) {
        let device = self.descender.get_string_field_or_parent("","device").expect("device not found") ;
        let base = self.descender.get_int_field_or_parent("","base").expect("base not found") ;
        let length = self.descender.get_int_field_or_parent("","length").expect("length not found") ;
        self.addr = mmap_memory(device.as_str(), base as u64, length as u64).expect("mmap failed") ;
    }

    pub fn set_test_area(&mut self) {
        let length = self.descender.get_int_field_or_parent("", "length").expect("length not found");
        let mut memory = Vec::with_capacity(length as usize);
        memory.resize(length as usize, 0u8);
        self.addr = memory.as_mut_ptr();
        std::mem::forget(memory);
        self.test_mode = true;
    }

    pub fn gather_regs(&mut self, regsspecs: &Vec<&str>) -> Result<(), String> {

        let old_root = match &self.descender.get_string_field_or_parent("completion-metadata", "root") {
            Ok(r) => self.descender.set_root(r).unwrap(),
            Err(_s) => {"".to_string()}
        } ;

        for spec in regsspecs {
            let parts = spec.split("=").collect::<Vec<&str>>();
            if parts.len() > 2 {
                return Err(format!("Bad argument {}", spec));
            }
            let isSet = parts.len() == 2 ;
            let value = if isSet {
                match parts[1].parse::<u32>() {
                    Ok(v) => Some(v),
                    Err(_) => return Err(format!("Bad argument {}", spec)),
                }
            } else { None };

            let r = match RegisterOp::new(&*self.descender, value, parts[0]) {
                Ok(r) => r,
                Err(e) => return Err(e),
            } ;

            self.regs.push(r)
        }
        self.descender.set_root(&*old_root) ;
        Ok(())
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
impl Drop for RegisterTool {
    fn drop(&mut self) {
        if self.test_mode {
            unsafe {
                std::ptr::drop_in_place(self.addr);
            }
        }
    }
}