//! SPL Token delegate approval.

use crate::constants::program_ids::TOKEN_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_token::instruction as token_ix;

/// Approve a delegate to spend up to `amount` tokens from `source`.
pub fn approve<'a>(
    source: &AccountInfo<'a>,
    delegate: &Pubkey,
    owner: &AccountInfo<'a>,
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
pub fn approve_checked<'a>(
    source: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    delegate: &Pubkey,
    owner: &AccountInfo<'a>,
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
