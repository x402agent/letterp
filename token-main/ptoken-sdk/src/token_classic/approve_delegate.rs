//! SPL Token delegate approval.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use spl_token::instruction as token_ix;
use crate::constants::program_ids::TOKEN_PROGRAM_ID;

/// Approve a delegate to spend up to `amount` tokens from `source`.
pub fn approve(
    source: &AccountInfo,
    delegate: &Pubkey,
    owner: &AccountInfo,
    amount: u64,
) -> ProgramResult {
    invoke(
        &token_ix::approve(
            &TOKEN_PROGRAM_ID,
            source.key,
            delegate,
            owner.key,
            &[],
            amount,
        )?,
        &[source.clone(), owner.clone()],
    )
}

/// Approve a delegate with a checked instruction (verifies decimals).
pub fn approve_checked(
    source: &AccountInfo,
    mint: &AccountInfo,
    delegate: &Pubkey,
    owner: &AccountInfo,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    invoke(
        &token_ix::approve_checked(
            &TOKEN_PROGRAM_ID,
            source.key,
            mint.key,
            delegate,
            owner.key,
            &[],
            amount,
            decimals,
        )?,
        &[source.clone(), mint.clone(), owner.clone()],
    )
}
