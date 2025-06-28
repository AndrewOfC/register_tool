use std::ptr;

/// Reads a 32-bit word from the specified memory address
///
/// # Safety
///
/// This function is unsafe because it performs raw memory access.
/// The caller must ensure that:
/// - The memory address is properly aligned for 32-bit access
/// - The memory address points to valid readable memory
/// - No other threads are concurrently accessing this memory location
pub unsafe fn read_word(address: usize) -> Result<u32, &'static str> {
    if address % 4 != 0 {
        return Err("Memory address must be 4-byte aligned");
    }

    Ok(ptr::read_volatile(address as *const u32))
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
pub unsafe fn write_word(address: usize, value: u32) -> Result<(), &'static str> {
    if address % 4 != 0 {
        return Err("Memory address must be 4-byte aligned");
    }

    ptr::write_volatile(address as *mut u32, value);
    Ok(())
}