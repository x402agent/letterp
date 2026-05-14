//! SPL Token freeze and thaw operations.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
};
use spl_token::instruction as token_ix;
use crate::constants::program_ids::TOKEN_PROGRAM_ID;

/// Freeze a token account, preventing transfers and burns.
///
/// Requires the mint's freeze authority.
pub fn freeze_account(
    account: &AccountInfo,
    mint: &AccountInfo,
    freeze_authority: &AccountInfo,
) -> ProgramResult {
    invoke(
        &token_ix::freeze_account(
            &TOKEN_PROGRAM_ID,
            account.key,
            mint.key,
            freeze_authority.key,
            &[],
        )?,
        &[account.clone(), mint.clone(), freeze_authority.clone()],
    )
}

/// Thaw a frozen token account, re-enabling transfers and burns.
///
/// Requires the mint's freeze authority.
pub fn thaw_account(
    account: &AccountInfo,
    mint: &AccountInfo,
    freeze_authority: &AccountInfo,
) -> ProgramResult {
    invoke(
        &token_ix::thaw_account(
            &TOKEN_PROGRAM_ID,
            account.key,
            mint.key,
            freeze_authority.key,
            &[],
        )?,
        &[account.clone(), mint.clone(), freeze_authority.clone()],
    )
}
