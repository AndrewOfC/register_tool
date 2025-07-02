pub struct Register {
    pub offset: u64,
    pub set_mask: u32,
    pub read_mask: u32,
    pub shift: u32,
    pub isset: bool,
    value: u32,
}

impl Register {
    pub fn new(offset: u64, set_mask: u32, clr_mask: u32, shift: u32, isset: bool, value: u32) -> Register {
        Register { offset, set_mask, read_mask: clr_mask, shift, isset, value }
    }
    pub fn set(&self, addr: *mut u8) -> u32 {
        if self.value >= 0x01 << self.shift { panic!("value out of range"); }

        let addr2 = unsafe { addr.add(self.offset as usize) };
        let value = self.value << self.shift;
        unsafe {
            let curr_value = *( addr2 as *mut u32);
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