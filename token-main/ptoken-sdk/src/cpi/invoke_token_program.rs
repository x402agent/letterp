//! CPI wrappers for SPL Token classic instructions.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use spl_token::instruction as token_ix;
use crate::constants::program_ids::TOKEN_PROGRAM_ID;

/// CPI: transfer tokens via SPL Token program.
pub fn cpi_transfer(
    source: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
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
pub fn cpi_transfer_signed(
    source: &AccountInfo,
    destination: &AccountInfo,
    pda_authority: &AccountInfo,
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
pub fn cpi_mint_to(
    mint: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
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
pub fn cpi_mint_to_signed(
    mint: &AccountInfo,
    destination: &AccountInfo,
    pda_authority: &AccountInfo,
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
pub fn cpi_burn(
    account: &AccountInfo,
    mint: &AccountInfo,
    authority: &AccountInfo,
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
pub fn cpi_close_account(
    account: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
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
pub fn cpi_set_authority(
    account: &AccountInfo,
    current_authority: &AccountInfo,
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
