//! SPL Token account creation and initialization.

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
use crate::constants::program_ids::TOKEN_PROGRAM_ID;

/// Create and initialize a new token account.
///
/// Steps:
/// 1. Allocate account via System program
/// 2. Initialize via Token program
pub fn create_token_account(
    payer: &AccountInfo,
    token_account: &AccountInfo,
    mint: &AccountInfo,
    owner: &Pubkey,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    rent_sysvar: &AccountInfo,
) -> ProgramResult {
    let rent = Rent::get()?;
    let account_rent = rent.minimum_balance(spl_token::state::Account::LEN);

    invoke(
        &system_instruction::create_account(
            payer.key,
            token_account.key,
            account_rent,
            spl_token::state::Account::LEN as u64,
            &TOKEN_PROGRAM_ID,
        ),
        &[payer.clone(), token_account.clone(), system_program.clone()],
    )?;

    invoke(
        &token_ix::initialize_account(
            &TOKEN_PROGRAM_ID,
            token_account.key,
            mint.key,
            owner,
        )?,
        &[
            token_account.clone(),
            mint.clone(),
            rent_sysvar.clone(),
        ],
    )?;

    Ok(())
}

/// Initialize an already-allocated token account.
pub fn initialize_account(
    token_account: &AccountInfo,
    mint: &AccountInfo,
    owner: &Pubkey,
    rent_sysvar: &AccountInfo,
) -> ProgramResult {
    invoke(
        &token_ix::initialize_account(
            &TOKEN_PROGRAM_ID,
            token_account.key,
            mint.key,
            owner,
        )?,
        &[token_account.clone(), mint.clone(), rent_sysvar.clone()],
    )
}
