//! SPL Token delegate revocation.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
};
use spl_token::instruction as token_ix;
use crate::constants::program_ids::TOKEN_PROGRAM_ID;

/// Revoke any previously approved delegate from a token account.
///
/// After this call, only the account owner can operate on the account.
pub fn revoke(source: &AccountInfo, owner: &AccountInfo) -> ProgramResult {
    invoke(
        &token_ix::revoke(
            &TOKEN_PROGRAM_ID,
            source.key,
            owner.key,
            &[],
        )?,
        &[source.clone(), owner.clone()],
    )
}
