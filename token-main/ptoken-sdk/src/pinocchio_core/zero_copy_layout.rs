//! Zero-copy account deserialization via direct byte-slice interpretation.
//!
//! Avoids heap allocation by reading data in-place from AccountInfo.data.
//! Useful for high-performance programs where every compute unit counts.

use solana_program::program_error::ProgramError;

/// Base size of an SPL Token mint account in bytes.
pub const MINT_LEN: usize = 82;

/// Base size of an SPL Token account in bytes.
pub const ACCOUNT_LEN: usize = 165;

/// Base size of an SPL Token multisig account in bytes.
pub const MULTISIG_LEN: usize = 355;

/// Offset of the `supply` field within a Mint account.
pub const MINT_SUPPLY_OFFSET: usize = 36;

/// Offset of the `decimals` field within a Mint account.
pub const MINT_DECIMALS_OFFSET: usize = 44;

/// Offset of the `is_initialized` field within a Mint account.
pub const MINT_INITIALIZED_OFFSET: usize = 45;

/// Offset of the `amount` field within a Token Account.
pub const ACCOUNT_AMOUNT_OFFSET: usize = 64;

/// Offset of the `state` field within a Token Account.
pub const ACCOUNT_STATE_OFFSET: usize = 108;

/// Read u64 from a byte slice at a fixed offset (zero-copy, no allocation).
#[inline(always)]
pub fn read_u64_at(data: &[u8], offset: usize) -> Result<u64, ProgramError> {
    data.get(offset..offset + 8)
        .and_then(|b| b.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidAccountData)
}

/// Read token account balance directly from raw bytes.
pub fn read_token_amount(data: &[u8]) -> Result<u64, ProgramError> {
    read_u64_at(data, ACCOUNT_AMOUNT_OFFSET)
}

/// Read mint supply directly from raw bytes.
pub fn read_mint_supply(data: &[u8]) -> Result<u64, ProgramError> {
    read_u64_at(data, MINT_SUPPLY_OFFSET)
}

/// Read mint decimals directly from raw bytes.
pub fn read_mint_decimals(data: &[u8]) -> Result<u8, ProgramError> {
    data.get(MINT_DECIMALS_OFFSET)
        .copied()
        .ok_or(ProgramError::InvalidAccountData)
}

/// Check if a mint account is initialized.
pub fn is_mint_initialized(data: &[u8]) -> bool {
    data.get(MINT_INITIALIZED_OFFSET).copied().unwrap_or(0) != 0
}
