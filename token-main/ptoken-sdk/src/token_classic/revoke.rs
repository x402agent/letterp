//! SPL Token delegate revocation.

use crate::constants::program_ids::TOKEN_PROGRAM_ID;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};
use spl_token::instruction as token_ix;

/// Revoke any previously approved delegate from a token account.
///
/// After this call, only the account owner can operate on the account.
pub fn revoke<'a>(source: &AccountInfo<'a>, owner: &AccountInfo<'a>) -> ProgramResult {
    invoke(
        &token_ix::revoke(&TOKEN_PROGRAM_ID, source.key, owner.key, &[])?,
        &[source.clone(), owner.clone()],
    )
}
