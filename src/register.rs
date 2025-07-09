use std::io::Write;
use aep_rust_common::descender::Descender;

pub struct Register {
    pub offset: u64,
    pub set_mask: u32,
    pub read_mask: u32,
    pub shift: u32,
    pub isset: bool,
    pub read_only: bool,
    value: u32,
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

impl Register {
    pub fn new(descender: &dyn Descender<dyn Write>, is_set: bool, value: u32, path:&str) -> Result<Register, String> {

        let offset_r = descender.get_int_field_or_parent(path, "offset");
        let offset = match offset_r {
            Ok(o) => o,
            Err(e) => return Err(e),
        } ;

        let bits_r = descender.get_string_field_or_parent(path, "bits");
        let bits = match bits_r {
            Ok(b) => b,
            Err(e) => "31:0".to_string(),
        } ;

        let mask_r = parse_bits(&*bits) ;
        let (mask, shift) = match mask_r {
            Ok((m, s)) => (m, s),
            Err(e) => return Err(e)
        } ;


        let read_only_r = descender.get_string_field_or_parent(path, "read-write") ;
        let read_only = match read_only_r {
            Ok(o) => o == "ro",
            Err(e) => return Err(e),
        } ;

        if read_only && is_set {
            return Err(format!("Register {} is read only and cannot be set", path));
        }

        if is_set && value >= 0x01 << shift {
            return Err(format!("Value {} is out of range for register {}", value, path));
        }

        Ok(Register {
            offset: offset as u64,
            set_mask: !mask,
            read_mask: mask,
            shift: shift,
            isset: is_set,
            read_only: read_only,
            value: value,
        })
    }


    pub fn set(&self, addr: *mut u8) -> u32 {

        let addr2 = unsafe { addr.add(self.offset as usize) };
        let value = self.value << self.shift;
        unsafe {
            let curr_value = *(addr2 as *mut u32);
            let new_value = (curr_value & self.set_mask) | value;
            *(addr2 as *mut u32) = new_value ;
        }
        value
    }

    pub fn get(&self, addr: *mut u8) -> u32 {
        let addr2 = unsafe { addr.add(self.offset as usize) };
        let value = unsafe { *(addr2 as *mut u32) };
        (value & self.read_mask) >> self.shift
    }
}