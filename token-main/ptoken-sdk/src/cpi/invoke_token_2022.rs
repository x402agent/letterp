//! CPI wrappers for Token-2022 program instructions.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
};
use spl_token_2022::instruction as token_ix;

/// CPI: transfer tokens via Token-2022 program.
#[allow(deprecated)]
pub fn cpi_transfer_2022<'a>(
    source: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &token_ix::transfer(
            &TOKEN_2022_PROGRAM_ID,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source.clone(), destination.clone(), authority.clone()],
    )
}

/// CPI: transfer_checked via Token-2022 (preferred for extensions).
pub fn cpi_transfer_checked_2022<'a>(
    source: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    invoke(
        &token_ix::transfer_checked(
            &TOKEN_2022_PROGRAM_ID,
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

/// CPI: transfer_checked using a PDA authority (Token-2022).
pub fn cpi_transfer_checked_signed_2022<'a>(
    source: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    pda_authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    invoke_signed(
        &token_ix::transfer_checked(
            &TOKEN_2022_PROGRAM_ID,
            source.key,
            mint.key,
            destination.key,
            pda_authority.key,
            &[],
            amount,
            decimals,
        )?,
        &[
            source.clone(),
            mint.clone(),
            destination.clone(),
            pda_authority.clone(),
        ],
        signer_seeds,
    )
}

/// CPI: mint tokens via Token-2022.
pub fn cpi_mint_to_2022<'a>(
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &token_ix::mint_to(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[mint.clone(), destination.clone(), authority.clone()],
    )
}

/// CPI: mint using PDA authority (Token-2022).
pub fn cpi_mint_to_signed_2022<'a>(
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    pda_authority: &AccountInfo<'a>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    invoke_signed(
        &token_ix::mint_to(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            destination.key,
            pda_authority.key,
            &[],
            amount,
        )?,
        &[mint.clone(), destination.clone(), pda_authority.clone()],
        signer_seeds,
    )
}

/// CPI: burn tokens via Token-2022.
pub fn cpi_burn_2022<'a>(
    account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &token_ix::burn(
            &TOKEN_2022_PROGRAM_ID,
            account.key,
            mint.key,
            authority.key,
            &[],
            amount,
        )?,
        &[account.clone(), mint.clone(), authority.clone()],
    )
}
