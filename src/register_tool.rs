use std::io::Write;
use clap::parser::ValuesRef;
use regex::Regex;
use aep_rust_common::descender;
use aep_rust_common::descender::Descender;
use crate::register::Register;
use crate::unsafes::mmap_memory;

pub struct RegisterTool {
    descender: Box<dyn Descender<dyn Write>>,
    regs: Vec<Register>,
    addr: *mut u8,
    test_mode: bool,
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

    pub fn gather_regs(&mut self, regsspecs: Vec<&str>) -> Result<(), String> {

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
                    Ok(v) => v,
                    Err(_) => return Err(format!("Bad argument {}", spec)),
                }
            } else { 0 };

            let r = match Register::new(&*self.descender, isSet, value, parts[0]) {
                Ok(r) => r,
                Err(e) => return Err(e),
            } ;

            self.regs.push(r)
        }
        self.descender.set_root(&*old_root) ;
        Ok(())
    }

    pub fn apply_registers<F>(&self, f: F) -> Result<(), String>
    where
        F: Fn(u64),
    {
        for reg in &self.regs {
            if !reg.isset {
                f(reg.get(self.addr) as u64);
            }
            else {
                reg.set(self.addr);
            }
        }
        Ok(())
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