//! CPI Guard extension — protect token accounts from unauthorized CPI manipulation.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};
use spl_token_2022::extension::cpi_guard::instruction as cpi_guard_ix;

/// Enable CPI Guard on a token account.
///
/// When enabled, the token account can only be operated on by the owner
/// calling directly — not by programs invoking on their behalf via CPI.
///
/// Protected operations: Transfer, Burn, Approve, Close, SetAuthority.
pub fn enable_cpi_guard<'a>(
    token_account: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
) -> ProgramResult {
    invoke(
        &cpi_guard_ix::enable_cpi_guard(&TOKEN_2022_PROGRAM_ID, token_account.key, owner.key, &[])?,
        &[token_account.clone(), owner.clone()],
    )
}

/// Disable CPI Guard on a token account.
pub fn disable_cpi_guard<'a>(
    token_account: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
) -> ProgramResult {
    invoke(
        &cpi_guard_ix::disable_cpi_guard(
            &TOKEN_2022_PROGRAM_ID,
            token_account.key,
            owner.key,
            &[],
        )?,
        &[token_account.clone(), owner.clone()],
    )
}
