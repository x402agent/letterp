//! Metadata Pointer extension — point a mint to its metadata account.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_token_2022::extension::metadata_pointer::instruction as mp_ix;

/// Initialize the Metadata Pointer extension on a mint.
///
/// Must be called before `InitializeMint2`.
///
/// # Arguments
/// * `authority` — Can update the metadata address. `None` = immutable.
/// * `metadata_address` — Account holding the token metadata. Can be the mint itself.
pub fn initialize_metadata_pointer<'a>(
    mint: &AccountInfo<'a>,
    authority: Option<&Pubkey>,
    metadata_address: Option<&Pubkey>,
) -> ProgramResult {
    invoke(
        &mp_ix::initialize(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.copied(),
            metadata_address.copied(),
        )?,
        &[mint.clone()],
    )
}

/// Update the metadata address pointer.
pub fn update_metadata_pointer<'a>(
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    new_metadata_address: Option<Pubkey>,
) -> ProgramResult {
    invoke(
        &mp_ix::update(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.key,
            &[],
            new_metadata_address,
        )?,
        &[mint.clone(), authority.clone()],
    )
}
