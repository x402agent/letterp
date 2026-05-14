//! Initialize new SPL Token mint accounts.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::instruction as token_ix;
use crate::{constants::program_ids::TOKEN_PROGRAM_ID, errors::PTokenError};

/// Parameters for creating a new mint.
#[derive(Debug, Clone)]
pub struct InitializeMintParams<'a> {
    /// The uninitialized mint account to initialize.
    pub mint: &'a AccountInfo<'a>,
    /// Authority that can mint new tokens. `None` for fixed supply.
    pub mint_authority: &'a Pubkey,
    /// Optional authority that can freeze token accounts.
    pub freeze_authority: Option<&'a Pubkey>,
    /// Number of decimal places (0–9 typical, 0–255 max).
    pub decimals: u8,
}

/// Create a new rent-exempt mint account and initialize it.
///
/// Performs two instructions:
/// 1. `system_program::create_account` — allocates space and funds rent
/// 2. `spl_token::initialize_mint` — sets authority and decimals
pub fn create_and_initialize_mint(
    payer: &AccountInfo,
    mint: &AccountInfo,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    rent_sysvar: &AccountInfo,
) -> ProgramResult {
    let rent = Rent::get()?;
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

    // 1. Allocate the mint account
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint.key,
            mint_rent,
            spl_token::state::Mint::LEN as u64,
            &TOKEN_PROGRAM_ID,
        ),
        &[payer.clone(), mint.clone(), system_program.clone()],
    )?;

    // 2. Initialize the mint
    invoke(
        &token_ix::initialize_mint(
            &TOKEN_PROGRAM_ID,
            mint.key,
            mint_authority,
            freeze_authority,
            decimals,
        )?,
        &[mint.clone(), rent_sysvar.clone()],
    )?;

    Ok(())
}

/// Initialize an already-allocated mint account (no create_account CPI).
pub fn initialize_mint(
    mint: &AccountInfo,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
    rent_sysvar: &AccountInfo,
) -> ProgramResult {
    invoke(
        &token_ix::initialize_mint(
            &TOKEN_PROGRAM_ID,
            mint.key,
            mint_authority,
            freeze_authority,
            decimals,
        )?,
        &[mint.clone(), rent_sysvar.clone()],
    )
}

/// Mint new tokens to a destination account.
///
/// Requires the mint authority to sign.
pub fn mint_to(
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

/// Mint tokens using a PDA authority (invoke_signed).
pub fn mint_to_signed(
    mint: &AccountInfo,
    destination: &AccountInfo,
    pda_authority: &AccountInfo,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    solana_program::program::invoke_signed(
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
