//! CPI into the Associated Token Account program.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use spl_associated_token_account::instruction as ata_ix;
use crate::constants::program_ids::{TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID};

/// CPI: create a new Associated Token Account.
///
/// Fails if the ATA already exists. Use [`cpi_create_ata_idempotent`]
/// to skip if it already exists.
pub fn cpi_create_associated_token_account(
    payer: &AccountInfo,
    wallet: &AccountInfo,
    mint: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
) -> ProgramResult {
    let token_program_id = token_program.key;
    invoke(
        &ata_ix::create_associated_token_account(
            payer.key,
            wallet.key,
            mint.key,
            token_program_id,
        ),
        &[
            payer.clone(),
            wallet.clone(),
            mint.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )
}

/// CPI: create an ATA only if it doesn't already exist (idempotent).
pub fn cpi_create_ata_idempotent(
    payer: &AccountInfo,
    wallet: &AccountInfo,
    mint: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
) -> ProgramResult {
    invoke(
        &ata_ix::create_associated_token_account_idempotent(
            payer.key,
            wallet.key,
            mint.key,
            token_program.key,
        ),
        &[
            payer.clone(),
            wallet.clone(),
            mint.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )
}
