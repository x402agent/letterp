//! Group Member Pointer extension — point a mint to a group member account.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_token_2022::extension::group_member_pointer::instruction as gmp_ix;

/// Initialize the Group Member Pointer extension on a mint.
///
/// Must be called before `InitializeMint2`.
pub fn initialize_group_member_pointer<'a>(
    mint: &AccountInfo<'a>,
    authority: Option<&Pubkey>,
    member_address: Option<&Pubkey>,
) -> ProgramResult {
    invoke(
        &gmp_ix::initialize(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.copied(),
            member_address.copied(),
        )?,
        &[mint.clone()],
    )
}

/// Update the group member address pointer.
pub fn update_group_member_pointer<'a>(
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    new_member_address: Option<Pubkey>,
) -> ProgramResult {
    invoke(
        &gmp_ix::update(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.key,
            &[],
            new_member_address,
        )?,
        &[mint.clone(), authority.clone()],
    )
}
