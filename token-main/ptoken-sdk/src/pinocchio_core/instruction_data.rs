//! Instruction data parsing without Anchor IDL.

use crate::errors::PTokenError;

/// Read a u8 from a byte slice at `offset`.
pub fn read_u8(data: &[u8], offset: usize) -> Result<u8, PTokenError> {
    data.get(offset)
        .copied()
        .ok_or(PTokenError::InvalidInstructionData)
}

/// Read a u16 (little-endian) from a byte slice at `offset`.
pub fn read_u16(data: &[u8], offset: usize) -> Result<u16, PTokenError> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or(PTokenError::InvalidInstructionData)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

/// Read a u64 (little-endian) from a byte slice at `offset`.
pub fn read_u64(data: &[u8], offset: usize) -> Result<u64, PTokenError> {
    let bytes = data
        .get(offset..offset + 8)
        .ok_or(PTokenError::InvalidInstructionData)?;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
}

/// Read a 32-byte Pubkey from a byte slice at `offset`.
pub fn read_pubkey(data: &[u8], offset: usize) -> Result<[u8; 32], PTokenError> {
    let bytes = data
        .get(offset..offset + 32)
        .ok_or(PTokenError::InvalidInstructionData)?;
    let mut key = [0u8; 32];
    key.copy_from_slice(bytes);
    Ok(key)
}

/// Extract the instruction discriminant (first byte).
pub fn discriminant(data: &[u8]) -> Result<u8, PTokenError> {
    read_u8(data, 0)
}

/// Assert the data slice has at least `len` bytes.
pub fn require_len(data: &[u8], len: usize) -> Result<(), PTokenError> {
    if data.len() < len {
        Err(PTokenError::InvalidInstructionData)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u64() {
        let amount: u64 = 1_000_000;
        let bytes = amount.to_le_bytes();
        assert_eq!(read_u64(&bytes, 0).unwrap(), 1_000_000);
    }

    #[test]
    fn test_discriminant() {
        let data = &[3u8, 0, 0, 0];
        assert_eq!(discriminant(data).unwrap(), 3);
    }
}
