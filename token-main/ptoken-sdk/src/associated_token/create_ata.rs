//! Create Associated Token Accounts.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use spl_associated_token_account::instruction as ata_ix;

/// Create a new Associated Token Account for `wallet` and `mint`.
///
/// The ATA address is derived deterministically.
/// Fails if the ATA already exists.
pub fn create_associated_token_account(
    payer: &AccountInfo,
    wallet: &AccountInfo,
    mint: &AccountInfo,
    token_program_id: &Pubkey,
    system_program: &AccountInfo,
    ata_program: &AccountInfo,
) -> ProgramResult {
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
            ata_program.clone(),
        ],
    )
}
