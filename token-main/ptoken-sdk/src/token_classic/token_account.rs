//! SPL Token account creation and initialization.

use crate::constants::program_ids::TOKEN_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_pack::Pack,
    pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};
use spl_token::instruction as token_ix;

/// Create and initialize a new token account.
///
/// Steps:
/// 1. Allocate account via System program
/// 2. Initialize via Token program
pub fn create_token_account<'a>(
    payer: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    owner: &Pubkey,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    rent_sysvar: &AccountInfo<'a>,
) -> ProgramResult {
    let rent = Rent::get()?;
    let account_rent = rent.minimum_balance(spl_token::state::Account::LEN);

    invoke(
        &system_instruction::create_account(
            payer.key,
            token_account.key,
            account_rent,
            spl_token::state::Account::LEN as u64,
            token_program.key,
        ),
        &[payer.clone(), token_account.clone(), system_program.clone()],
    )?;

    invoke(
        &token_ix::initialize_account(token_program.key, token_account.key, mint.key, owner)?,
        &[token_account.clone(), mint.clone(), rent_sysvar.clone()],
    )?;

    Ok(())
}

/// Initialize an already-allocated token account.
pub fn initialize_account<'a>(
    token_account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    owner: &Pubkey,
    rent_sysvar: &AccountInfo<'a>,
) -> ProgramResult {
    invoke(
        &token_ix::initialize_account(&TOKEN_PROGRAM_ID, token_account.key, mint.key, owner)?,
        &[token_account.clone(), mint.clone(), rent_sysvar.clone()],
    )
}
