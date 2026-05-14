//! Transfer Hook extension — invoke a custom program on every transfer.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_token_2022::extension::transfer_hook::instruction as hook_ix;

/// Initialize the Transfer Hook extension on a mint.
///
/// Must be called before `InitializeMint2`.
///
/// # Arguments
/// * `authority` — Can update the hook program. `None` = immutable.
/// * `hook_program_id` — Program invoked on every transfer. `None` to disable.
pub fn initialize_transfer_hook<'a>(
    mint: &AccountInfo<'a>,
    authority: Option<&Pubkey>,
    hook_program_id: Option<&Pubkey>,
) -> ProgramResult {
    invoke(
        &hook_ix::initialize(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.copied(),
            hook_program_id.copied(),
        )?,
        &[mint.clone()],
    )
}

/// Update the transfer hook program on an existing mint.
pub fn update_transfer_hook<'a>(
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    new_program_id: Option<&Pubkey>,
) -> ProgramResult {
    invoke(
        &hook_ix::update(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.key,
            &[],
            new_program_id.copied(),
        )?,
        &[mint.clone(), authority.clone()],
    )
}
