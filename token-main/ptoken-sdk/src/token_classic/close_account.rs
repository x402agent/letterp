//! SPL Token account closing.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
};
use spl_token::instruction as token_ix;
use crate::constants::program_ids::TOKEN_PROGRAM_ID;

/// Close a token account with zero balance, returning rent lamports
/// to `destination`.
///
/// The account must have zero token balance before closing.
pub fn close_account(
    account: &AccountInfo,
    destination: &AccountInfo,
    owner: &AccountInfo,
) -> ProgramResult {
    invoke(
        &token_ix::close_account(
            &TOKEN_PROGRAM_ID,
            account.key,
            destination.key,
            owner.key,
            &[],
        )?,
        &[account.clone(), destination.clone(), owner.clone()],
    )
}

/// Close a token account using a PDA authority.
pub fn close_account_signed(
    account: &AccountInfo,
    destination: &AccountInfo,
    pda_owner: &AccountInfo,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    solana_program::program::invoke_signed(
        &token_ix::close_account(
            &TOKEN_PROGRAM_ID,
            account.key,
            destination.key,
            pda_owner.key,
            &[],
        )?,
        &[account.clone(), destination.clone(), pda_owner.clone()],
        signer_seeds,
    )
}
