//! Reallocate Token-2022 token accounts to add new extensions.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use spl_token_2022::{extension::ExtensionType, instruction as token_ix};
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Add new extensions to an existing Token-2022 token account.
///
/// Uses the `Reallocate` instruction to resize account data and
/// append the required space for the new extensions.
///
/// The payer covers any additional rent lamports required.
pub fn reallocate(
    token_account: &AccountInfo,
    payer: &AccountInfo,
    owner: &AccountInfo,
    new_extensions: &[ExtensionType],
) -> ProgramResult {
    invoke(
        &token_ix::reallocate(
            &TOKEN_2022_PROGRAM_ID,
            token_account.key,
            payer.key,
            owner.key,
            &[],
            new_extensions,
        )?,
        &[token_account.clone(), payer.clone(), owner.clone()],
    )
}

/// Reallocate using a PDA owner.
pub fn reallocate_signed(
    token_account: &AccountInfo,
    payer: &AccountInfo,
    pda_owner: &AccountInfo,
    new_extensions: &[ExtensionType],
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    solana_program::program::invoke_signed(
        &token_ix::reallocate(
            &TOKEN_2022_PROGRAM_ID,
            token_account.key,
            payer.key,
            pda_owner.key,
            &[],
            new_extensions,
        )?,
        &[token_account.clone(), payer.clone(), pda_owner.clone()],
        signer_seeds,
    )
}
