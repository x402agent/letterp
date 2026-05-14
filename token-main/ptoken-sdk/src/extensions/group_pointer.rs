//! Group Pointer extension — point a mint to a token group account.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_token_2022::extension::group_pointer::instruction as gp_ix;

/// Initialize the Group Pointer extension on a mint.
///
/// Must be called before `InitializeMint2`.
pub fn initialize_group_pointer<'a>(
    mint: &AccountInfo<'a>,
    authority: Option<&Pubkey>,
    group_address: Option<&Pubkey>,
) -> ProgramResult {
    invoke(
        &gp_ix::initialize(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.copied(),
            group_address.copied(),
        )?,
        &[mint.clone()],
    )
}

/// Update the group address pointer.
pub fn update_group_pointer<'a>(
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    new_group_address: Option<Pubkey>,
) -> ProgramResult {
    invoke(
        &gp_ix::update(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.key,
            &[],
            new_group_address,
        )?,
        &[mint.clone(), authority.clone()],
    )
}
