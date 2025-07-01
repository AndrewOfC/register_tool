mod unittests;
pub mod unsafes;

pub fn parse_bits(bitsstr: &str) -> Result<(u32, u32, u32), &'static str> {
    let parts: Vec<&str> = bitsstr.split(':').collect();

    if parts.len() != 2 {
        return Err("Invalid bit range format. Expected 'hi:lo'");
    }

    let hi: u32 = match parts[0].trim().parse() {
        Ok(n) => n,
        Err(_) => return Err("Invalid high bit value"),
    };

    let lo: u32 = match parts[1].trim().parse() {
        Ok(n) => n,
        Err(_) => return Err("Invalid low bit value"),
    };

    if hi >= 32 || lo >= 32 {
        return Err("Bit positions must be less than 32");
    }

    if hi < lo {
        return Err("High bit must be greater than or equal to low bit");
    }

    let width = hi - lo + 1;
    let mask = ((1u32 << width) - 1) << lo;

    Ok((mask, width, lo))
}

