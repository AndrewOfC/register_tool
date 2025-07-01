use libc::{mmap, off_t, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::ffi::CStr;
use std::ptr;

/// Maps a memory region using mmap system call
///
/// # Safety
///
/// This function is unsafe because it performs system memory mapping.
/// The caller must ensure that:
/// - The address and length are valid for memory mapping
/// - The resulting mapped memory is accessed properly
/// - The mapping is properly unmapped when no longer needed
pub fn mmap_memory(device: &str, address: u64, length: u64) -> Result<*mut u8, String> { 
    unsafe {
        let fd = libc::open(device.as_ptr() as *const libc::c_char, libc::O_RDWR) ;
        if fd < 0 {
            return Err("Failed to open /dev/mem".parse().unwrap());
        }
    
        let addr = mmap(
            0 as *mut libc::c_void,
            length as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            address as off_t
        );
        
        if addr == libc::MAP_FAILED {
            let errno =  *libc::__errno_location() ;
            let err_msg =  CStr::from_ptr(libc::strerror(errno))
                .to_str()
                .unwrap_or("Invalid error message");
            return Err(format!("Memory mapping failed: {err_msg} (errno: {errno})"));
        }
    
        Ok(addr as *mut u8)
    }
}

/// Reads a 32-bit word from the specified memory address
///
/// # Safety
///
/// This function is unsafe because it performs raw memory access.
/// The caller must ensure that:
/// - The memory address is properly aligned for 32-bit access
/// - The memory address points to valid readable memory
/// - No other threads are concurrently accessing this memory location
pub fn read_word(address: usize) -> Result<u32, &'static str> {
    if address % 4 != 0 {
        return Err("Memory address must be 4-byte aligned");
    }

    unsafe {Ok(ptr::read_volatile(address as *const u32))}
}

/// Writes a 32-bit word to the specified memory address
///
/// # Safety
///
/// This function is unsafe because it performs raw memory access.
/// The caller must ensure that:
/// - The memory address is properly aligned for 32-bit access
/// - The memory address points to valid writable memory
/// - No other threads are concurrently accessing this memory location
pub fn write_word(address: usize, value: u32) -> Result<(), &'static str> {
    if address % 4 != 0 {
        return Err("Memory address must be 4-byte aligned");
    }

    unsafe {
        ptr::write_volatile(address as *mut u32, value);
        Ok(())
    }
}