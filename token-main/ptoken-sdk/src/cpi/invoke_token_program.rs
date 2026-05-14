//! CPI wrappers for SPL Token classic instructions.

use crate::constants::program_ids::TOKEN_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use spl_token::instruction as token_ix;

/// CPI: transfer tokens via SPL Token program.
pub fn cpi_transfer<'a>(
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

/// CPI: transfer tokens using a PDA authority.
pub fn cpi_transfer_signed<'a>(
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

/// CPI: mint tokens to a destination account.
pub fn cpi_mint_to<'a>(
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &token_ix::mint_to(
            &TOKEN_PROGRAM_ID,
            mint.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[mint.clone(), destination.clone(), authority.clone()],
    )
}

/// CPI: mint tokens using a PDA mint authority.
pub fn cpi_mint_to_signed<'a>(
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    pda_authority: &AccountInfo<'a>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    invoke_signed(
        &token_ix::mint_to(
            &TOKEN_PROGRAM_ID,
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

/// CPI: burn tokens.
pub fn cpi_burn<'a>(
    account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &token_ix::burn(
            &TOKEN_PROGRAM_ID,
            account.key,
            mint.key,
            authority.key,
            &[],
            amount,
        )?,
        &[account.clone(), mint.clone(), authority.clone()],
    )
}

/// CPI: close token account.
pub fn cpi_close_account<'a>(
    account: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
) -> ProgramResult {
    invoke(
        &token_ix::close_account(
            &TOKEN_PROGRAM_ID,
            account.key,
            destination.key,
            authority.key,
            &[],
        )?,
        &[account.clone(), destination.clone(), authority.clone()],
    )
}

/// CPI: set a new authority on a mint or token account.
pub fn cpi_set_authority<'a>(
    account: &AccountInfo<'a>,
    current_authority: &AccountInfo<'a>,
    new_authority: Option<&Pubkey>,
    authority_type: spl_token::instruction::AuthorityType,
) -> ProgramResult {
    invoke(
        &token_ix::set_authority(
            &TOKEN_PROGRAM_ID,
            account.key,
            new_authority,
            authority_type,
            current_authority.key,
            &[],
        )?,
        &[account.clone(), current_authority.clone()],
    )
}
