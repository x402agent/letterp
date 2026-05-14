//! Borsh deserialization helpers for account data.

use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

/// Deserialize a Borsh value from a byte slice.
pub fn from_bytes<T: BorshDeserialize>(data: &[u8]) -> Result<T, ProgramError> {
    T::try_from_slice(data).map_err(|_| ProgramError::InvalidAccountData)
}

/// Deserialize a Borsh value from an AccountInfo's data.
pub fn from_account<T: BorshDeserialize>(
    account: &solana_program::account_info::AccountInfo,
) -> Result<T, ProgramError> {
    let data = account.data.borrow();
    from_bytes::<T>(&data)
}

/// Read a u64 (little-endian) from a byte slice at `offset`.
pub fn read_u64(data: &[u8], offset: usize) -> Result<u64, ProgramError> {
    data.get(offset..offset + 8)
        .and_then(|b| b.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidAccountData)
}

/// Read a u32 (little-endian) from a byte slice at `offset`.
pub fn read_u32(data: &[u8], offset: usize) -> Result<u32, ProgramError> {
    data.get(offset..offset + 4)
        .and_then(|b| b.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(ProgramError::InvalidAccountData)
}

/// Read a Pubkey (32 bytes) from a byte slice at `offset`.
pub fn read_pubkey(data: &[u8], offset: usize) -> Result<solana_program::pubkey::Pubkey, ProgramError> {
    let bytes = data
        .get(offset..offset + 32)
        .ok_or(ProgramError::InvalidAccountData)?;
    Ok(solana_program::pubkey::Pubkey::from(<[u8; 32]>::try_from(bytes).unwrap()))
}
