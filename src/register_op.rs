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
use aep_rust_common::descender::Descender;

#[derive(Debug, PartialEq)]
pub enum RegisterAccess {
    ReadOnly,
    ReadWrite,
    WriteOnly,
    Write1Clear,
    Unspecified,
}


pub struct RegisterOp {
    pub offset: u64,
    pub set_mask: u32,
    pub read_mask: u32,
    pub shift: u32,
    pub value: Option<u32>,
    shadow_offset: Option<u64>,
    access_type: RegisterAccess,
}


pub fn parse_bits(bitsstr: &str) -> Result<(u32, u32), String> {
    let parts: Vec<&str> = bitsstr.split(':').collect();

    if parts.len() != 2 {
        return Err("Invalid bit range format. Expected 'hi:lo'".parse().unwrap());
    }

    let hi: u32 = match parts[0].trim().parse() {
        Ok(n) => n,
        Err(_) => return Err("Invalid high bit value".parse().unwrap()),
    };

    let lo: u32 = match parts[1].trim().parse() {
        Ok(n) => n,
        Err(_) => return Err("Invalid low bit value".parse().unwrap()),
    };

    if hi >= 32 || lo >= 32 {
        return Err("Bit positions must be less than 32".parse().unwrap());
    }

    if hi < lo {
        return Err("High bit must be greater than or equal to low bit".parse().unwrap());
    }

    let width = (hi - lo) + 1;
    if width == 32 {
        return Ok((0xFFFFFFFF as u32, 0u32))
    }
    let mask = ((1u32 << width) - 1) << lo;

    Ok((mask, lo))
}

impl RegisterOp {
    pub fn new(descender: &dyn Descender<dyn Write>, value: Option<u32>, path:&str) -> Result<RegisterOp, String> {

        let offset_r = descender.get_int_field_or_parent(path, "offset");
        let offset = match offset_r {
            Ok(o) => o,
            Err(e) => return Err(format!("Invalid offset for register {}: {}", path, e)),
        } ;

        let bits_r = descender.get_string_field_or_parent(path, "bits");
        let bits = bits_r.unwrap_or_else(|_| "31:0".to_string());

        let mask_r = parse_bits(&*bits) ;
        let (mask, shift) = match mask_r {
            Ok((m, s)) => (m, s),
            Err(e) => return Err(e)
        } ;


        let read_only_r = descender.get_string_field_or_parent(path, "read-write");
        let access_type = match read_only_r {
            Ok(access_str) => match access_str.as_str() {
                "ro" => RegisterAccess::ReadOnly,
                "wo" => RegisterAccess::WriteOnly,
                "w1c" => RegisterAccess::Write1Clear,
                "rw" => RegisterAccess::ReadWrite,
                "wr" => RegisterAccess::ReadWrite,
                _ => return Err(format!("Invalid read-write value '{}' for register {} must be ro, rw, wo or w1c", access_str, path))
            },
            Err(_) => RegisterAccess::Unspecified,
        };

        if access_type == RegisterAccess::ReadOnly && value.is_some() {
            return Err(format!("Register {} is read only and cannot be set", path));
        }

        let shadow_path = descender.get_string_field_or_parent(path, "shadow");
        let shadow_offset = match shadow_path {
            Err(_) => None, // no shadow reg found
            Ok(shadow_p) => match descender.get_int_field_or_parent(&shadow_p, "offset") {
                Ok(o) => Some(o as u64),
                Err(e) => return Err(format!("Invalid shadow-offset for register {}: {}", path, e)),
            }
        } ;


        if access_type == RegisterAccess::WriteOnly && !value.is_some() && shadow_offset.is_none() {
            return Err(format!("Register {} is write only and cannot be read", path));
        }

        match value {
            None => (),
            Some(v) => {
                if v > (0x01 << shift) {
                    return Err(format!("Value {} is out of range for register {}", v, path));
                }
            }
        }

        Ok(RegisterOp {
            offset: offset as u64,
            set_mask: !mask,
            read_mask: mask,
            shift: shift,
            access_type: access_type,
            value: value,
            shadow_offset: shadow_offset,
        })
    }


    pub fn set(&self, addr: *mut u8) -> Result<u32, String> {

        let read_offset = match self.shadow_offset {
            None => self.offset,
            Some(o) => o,
        } ;

        let addr_read = unsafe { addr.add(read_offset as usize) };
        
        let addr_write = unsafe { addr.add(self.offset as usize) };
        let value = match self.value {
            None => panic!("set with no value"), // panic appropriate
            Some(v) => v,
        } ;

        let bits = value << self.shift;
        unsafe {
            let curr_value = *(addr_read as *mut u32);
            let new_value = (curr_value & self.set_mask) | bits;
            *(addr_write as *mut u32) = new_value ;
        }
        Ok(value)
    }

    pub fn get(&self, addr: *mut u8) -> u32 {

        let offset = match self.shadow_offset {
            None => self.offset,
            Some(o) => o,
        } ;

        let addr2 = unsafe { addr.add(offset as usize) };
        let value = unsafe { *(addr2 as *mut u32) };
        (value & self.read_mask) >> self.shift
    }
}