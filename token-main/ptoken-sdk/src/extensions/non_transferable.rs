//! Non-Transferable extension — permanently soul-bound tokens.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
};
use spl_token_2022::instruction as token_ix;
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Initialize the Non-Transferable extension on a mint.
///
/// Once set, tokens from this mint can never be transferred between accounts.
/// They can only be minted directly to an account and burned from it.
///
/// Must be called before `InitializeMint2`. No configuration parameters.
pub fn initialize_non_transferable_mint(mint: &AccountInfo) -> ProgramResult {
    invoke(
        &token_ix::initialize_non_transferable_mint(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
        )?,
        &[mint.clone()],
    )
}
