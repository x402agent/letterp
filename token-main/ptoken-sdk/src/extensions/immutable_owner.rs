//! Immutable Owner extension — prevent owner reassignment on token accounts.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
};
use spl_token_2022::instruction as token_ix;
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Initialize the Immutable Owner extension on a token account.
///
/// Must be called before `InitializeAccount3`.
/// After this, `SetAuthority` can never change the account's owner.
///
/// This is the default for Token-2022 ATAs.
pub fn initialize_immutable_owner(token_account: &AccountInfo) -> ProgramResult {
    invoke(
        &token_ix::initialize_immutable_owner(
            &TOKEN_2022_PROGRAM_ID,
            token_account.key,
        )?,
        &[token_account.clone()],
    )
}
