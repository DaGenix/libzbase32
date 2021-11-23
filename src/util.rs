use crate::error::{zbase32_error, ZBase32Error, ZBase32ErrorInfo};

const fn u64_to_usize(val: u64) -> Option<usize> {
    if usize::BITS >= u64::BITS {
        Some(val as usize)
    } else {
        // In this case, u64::BITS > usize::BITS - so, any usize
        // value can be safely casted to a u64.
        let max_usize: u64 = usize::MAX as u64;
        if val <= max_usize {
            Some(val as usize)
        } else {
            None
        }
    }
}

/// Calculate the number of octets that are required to hold the specified
/// number of bits.
///
/// This function will return an Err value if the specified number of bits
/// would result in needing more than [`usize::MAX`] octets.
pub const fn required_octets_buffer_len(bits: u64) -> Result<usize, ZBase32Error> {
    let remainder = bits % 8;
    let needed_octets = bits / 8 + if remainder == 0 { 0 } else { 1 };
    if let Some(result) = u64_to_usize(needed_octets) {
        Ok(result)
    } else {
        Err(zbase32_error(ZBase32ErrorInfo::BitsOverflow))
    }
}

/// Calculate the number of quintets tht are required to hold the specified
/// number of bits.
///
/// This function will return an Err value if the specified number of bits
/// would result in needing more than [`usize::MAX`] quintets.
pub const fn required_quintets_buffer_len(bits: u64) -> Result<usize, ZBase32Error> {
    let remainder = bits % 5;
    let needed_octets = bits / 5 + if remainder == 0 { 0 } else { 1 };
    if let Some(result) = u64_to_usize(needed_octets) {
        Ok(result)
    } else {
        Err(zbase32_error(ZBase32ErrorInfo::BitsOverflow))
    }
}
