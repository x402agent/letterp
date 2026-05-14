//! Close Token-2022 token accounts.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
};
use spl_token_2022::instruction as token_ix;
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Close a Token-2022 token account, returning rent to `destination`.
///
/// The account must have a zero token balance.
/// Some extensions (e.g. CPI Guard) may impose additional restrictions.
pub fn close_account_2022(
    token_account: &AccountInfo,
    destination: &AccountInfo,
    owner: &AccountInfo,
) -> ProgramResult {
    invoke(
        &token_ix::close_account(
            &TOKEN_2022_PROGRAM_ID,
            token_account.key,
            destination.key,
            owner.key,
            &[],
        )?,
        &[token_account.clone(), destination.clone(), owner.clone()],
    )
}
