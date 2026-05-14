//! Borsh serialization helpers for instruction payloads.

use borsh::BorshSerialize;
use solana_program::program_error::ProgramError;

/// Serialize a Borsh-serializable value to a Vec<u8>.
pub fn to_bytes<T: BorshSerialize>(value: &T) -> Result<Vec<u8>, ProgramError> {
    borsh::to_vec(value).map_err(|_| ProgramError::InvalidInstructionData)
}

/// Serialize to a fixed-size array. Panics if the size doesn't match.
pub fn to_fixed_bytes<T: BorshSerialize, const N: usize>(
    value: &T,
) -> Result<[u8; N], ProgramError> {
    let bytes = to_bytes(value)?;
    if bytes.len() != N {
        return Err(ProgramError::InvalidInstructionData);
    }
    let mut arr = [0u8; N];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

/// Write a u64 as little-endian bytes into a mutable slice at `offset`.
pub fn write_u64(buf: &mut [u8], offset: usize, value: u64) -> Result<(), ProgramError> {
    let bytes = value.to_le_bytes();
    buf.get_mut(offset..offset + 8)
        .ok_or(ProgramError::InvalidAccountData)?
        .copy_from_slice(&bytes);
    Ok(())
}

/// Write a u8 into a mutable slice at `offset`.
pub fn write_u8(buf: &mut [u8], offset: usize, value: u8) -> Result<(), ProgramError> {
    *buf.get_mut(offset)
        .ok_or(ProgramError::InvalidAccountData)? = value;
    Ok(())
}

/// Write a Pubkey (32 bytes) into a mutable slice at `offset`.
pub fn write_pubkey(
    buf: &mut [u8],
    offset: usize,
    key: &solana_program::pubkey::Pubkey,
) -> Result<(), ProgramError> {
    buf.get_mut(offset..offset + 32)
        .ok_or(ProgramError::InvalidAccountData)?
        .copy_from_slice(key.as_ref());
    Ok(())
}
