//! SPL Token transfer operations.

use crate::constants::program_ids::TOKEN_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
};
use spl_token::instruction as token_ix;

/// Transfer tokens from `source` to `destination`.
///
/// The `authority` must be the source account owner or an approved delegate.
pub fn transfer<'a>(
    source: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &token_ix::transfer(
            &TOKEN_PROGRAM_ID,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source.clone(), destination.clone(), authority.clone()],
    )
}

/// Transfer tokens using a PDA authority (invoke_signed).
pub fn transfer_signed<'a>(
    source: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    pda_authority: &AccountInfo<'a>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    invoke_signed(
        &token_ix::transfer(
            &TOKEN_PROGRAM_ID,
            source.key,
            destination.key,
            pda_authority.key,
            &[],
            amount,
        )?,
        &[source.clone(), destination.clone(), pda_authority.clone()],
        signer_seeds,
    )
}

/// Transfer tokens with a checked instruction (verifies decimals on-chain).
pub fn transfer_checked<'a>(
    source: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    invoke(
        &token_ix::transfer_checked(
            &TOKEN_PROGRAM_ID,
            source.key,
            mint.key,
            destination.key,
            authority.key,
            &[],
            amount,
            decimals,
        )?,
        &[
            source.clone(),
            mint.clone(),
            destination.clone(),
            authority.clone(),
        ],
    )
}
