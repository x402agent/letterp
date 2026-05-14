//! Token-2022 token account initialization.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_pack::Pack,
    pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};
use spl_token_2022::{instruction as token_ix, state::Account};

/// Create and initialize a Token-2022 token account.
pub fn create_token_account_2022<'a>(
    payer: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    owner: &Pubkey,
    system_program: &AccountInfo<'a>,
) -> ProgramResult {
    let rent = Rent::get()?;
    let space = spl_token_2022::extension::ExtensionType::try_calculate_account_len::<Account>(&[])
        .unwrap_or(Account::LEN);
    let lamports = rent.minimum_balance(space);

    invoke(
        &system_instruction::create_account(
            payer.key,
            token_account.key,
            lamports,
            space as u64,
            &TOKEN_2022_PROGRAM_ID,
        ),
        &[payer.clone(), token_account.clone(), system_program.clone()],
    )?;

    invoke(
        &token_ix::initialize_account3(&TOKEN_2022_PROGRAM_ID, token_account.key, mint.key, owner)?,
        &[token_account.clone(), mint.clone()],
    )?;

    Ok(())
}
