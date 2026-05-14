//! SPL Token burn operations.

use crate::constants::program_ids::TOKEN_PROGRAM_ID;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};
use spl_token::instruction as token_ix;

/// Burn tokens from `account`, reducing the mint's total supply.
pub fn burn<'a>(
    account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &token_ix::burn(
            &TOKEN_PROGRAM_ID,
            account.key,
            mint.key,
            authority.key,
            &[],
            amount,
        )?,
        &[account.clone(), mint.clone(), authority.clone()],
    )
}

/// Burn tokens with a checked instruction (verifies decimals on-chain).
pub fn burn_checked<'a>(
    account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    invoke(
        &token_ix::burn_checked(
            &TOKEN_PROGRAM_ID,
            account.key,
            mint.key,
            authority.key,
            &[],
            amount,
            decimals,
        )?,
        &[account.clone(), mint.clone(), authority.clone()],
    )
}
