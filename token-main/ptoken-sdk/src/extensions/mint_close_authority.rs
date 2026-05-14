//! Mint Close Authority extension — allow closing a mint account.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use spl_token_2022::instruction as token_ix;
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Initialize the Mint Close Authority extension on a mint.
///
/// Must be called before `InitializeMint2`.
///
/// # Arguments
/// * `close_authority` — Account authorized to close the mint. `None` disables closing.
pub fn initialize_mint_close_authority(
    mint: &AccountInfo,
    close_authority: Option<&Pubkey>,
) -> ProgramResult {
    invoke(
        &token_ix::initialize_mint_close_authority(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            close_authority,
        )?,
        &[mint.clone()],
    )
}

/// Close a mint account, reclaiming rent.
///
/// Requires the mint supply to be 0 and the close authority to sign.
pub fn close_mint(
    mint: &AccountInfo,
    destination: &AccountInfo,
    close_authority: &AccountInfo,
) -> ProgramResult {
    invoke(
        &spl_token_2022::instruction::close_account(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            destination.key,
            close_authority.key,
            &[],
        )?,
        &[mint.clone(), destination.clone(), close_authority.clone()],
    )
}
