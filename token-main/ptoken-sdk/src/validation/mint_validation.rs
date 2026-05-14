//! Mint account validation helpers.

use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use crate::{
    errors::PTokenError,
    pinocchio_core::zero_copy_layout::{is_mint_initialized, read_mint_decimals, MINT_LEN},
};

/// Assert that a mint account is initialized.
pub fn assert_mint_initialized(mint: &AccountInfo) -> Result<(), PTokenError> {
    let data = mint.data.borrow();
    if data.len() < MINT_LEN {
        return Err(PTokenError::InvalidMint);
    }
    if !is_mint_initialized(&data) {
        return Err(PTokenError::NotInitialized);
    }
    Ok(())
}

/// Assert that a mint has the expected number of decimals.
pub fn assert_decimals(mint: &AccountInfo, expected: u8) -> Result<(), PTokenError> {
    let data = mint.data.borrow();
    let decimals = read_mint_decimals(&data).map_err(|_| PTokenError::InvalidMint)?;
    if decimals != expected {
        return Err(PTokenError::InvalidMint);
    }
    Ok(())
}

/// Assert that a mint account has a specific key.
pub fn assert_mint_key(mint: &AccountInfo, expected_key: &Pubkey) -> Result<(), PTokenError> {
    if mint.key != expected_key {
        return Err(PTokenError::MintMismatch);
    }
    Ok(())
}

/// Assert that a token account's mint field matches the expected mint key.
///
/// The mint field in a token account lives at bytes 0..32.
pub fn assert_token_account_mint(
    token_account: &AccountInfo,
    expected_mint: &Pubkey,
) -> Result<(), PTokenError> {
    let data = token_account.data.borrow();
    if data.len() < 32 {
        return Err(PTokenError::InvalidAccountDataLength);
    }
    if &data[0..32] != expected_mint.as_ref() {
        return Err(PTokenError::MintMismatch);
    }
    Ok(())
}
