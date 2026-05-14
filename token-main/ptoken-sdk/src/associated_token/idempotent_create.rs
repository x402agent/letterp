//! Idempotent ATA creation — create only if it doesn't exist.

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_associated_token_account::instruction as ata_ix;

/// Create an Associated Token Account only if it doesn't already exist.
///
/// Uses the `CreateIdempotent` instruction — safe to call even if the
/// ATA was already created in a previous transaction.
pub fn create_associated_token_account_idempotent<'a>(
    payer: &AccountInfo<'a>,
    wallet: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    token_program_id: &Pubkey,
    system_program: &AccountInfo<'a>,
) -> ProgramResult {
    invoke(
        &ata_ix::create_associated_token_account_idempotent(
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
        ],
    )
}
