//! SPL Token Pack/Unpack trait helpers.
//!
//! The original SPL Token program uses fixed-size serialization via the
//! `Pack` trait. This module provides convenience wrappers.

use solana_program::{account_info::AccountInfo, program_error::ProgramError, program_pack::Pack};
use spl_token::state::{Account, Mint, Multisig};

/// Unpack a Mint account from raw bytes.
pub fn unpack_mint(data: &[u8]) -> Result<Mint, ProgramError> {
    Mint::unpack(data)
}

/// Unpack a Token Account from raw bytes.
pub fn unpack_token_account(data: &[u8]) -> Result<Account, ProgramError> {
    Account::unpack(data)
}

/// Unpack a Multisig account from raw bytes.
pub fn unpack_multisig(data: &[u8]) -> Result<Multisig, ProgramError> {
    Multisig::unpack(data)
}

/// Unpack a Mint from an AccountInfo.
pub fn unpack_mint_from_account(account: &AccountInfo) -> Result<Mint, ProgramError> {
    let data = account.data.borrow();
    Mint::unpack(&data)
}

/// Unpack a Token Account from an AccountInfo.
pub fn unpack_token_account_from_account(account: &AccountInfo) -> Result<Account, ProgramError> {
    let data = account.data.borrow();
    Account::unpack(&data)
}

/// Pack a Mint state into an AccountInfo's data.
pub fn pack_mint(mint: &Mint, account: &AccountInfo) -> Result<(), ProgramError> {
    let mut data = account.data.borrow_mut();
    Mint::pack(*mint, &mut data)
}

/// Pack a Token Account state into an AccountInfo's data.
pub fn pack_token_account(
    token_account: &Account,
    account: &AccountInfo,
) -> Result<(), ProgramError> {
    let mut data = account.data.borrow_mut();
    Account::pack(*token_account, &mut data)
}

/// Byte sizes for reference.
pub const MINT_PACKED_LEN: usize = Mint::LEN;
pub const ACCOUNT_PACKED_LEN: usize = Account::LEN;
pub const MULTISIG_PACKED_LEN: usize = Multisig::LEN;
