//! Confidential Transfer Fee extension — fees on confidential transfers.
//!
//! Requires both `transfer_fee` and `confidential_transfer` extensions
//! to be initialized on the same mint.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Initialize the Confidential Transfer Fee extension on a mint.
///
/// Must be called after `initialize_transfer_fee_config` and
/// `initialize_confidential_transfer_mint`, but before `InitializeMint2`.
///
/// # Arguments
/// * `authority` — Can withdraw confidential withheld fees.
/// * `withdraw_withheld_authority_elgamal_pubkey` — ElGamal key for fee decryption.
pub fn initialize_confidential_transfer_fee_config(
    mint: &AccountInfo,
    authority: Option<&Pubkey>,
    withdraw_withheld_authority_elgamal_pubkey: &spl_token_2022::solana_zk_token_sdk::encryption::elgamal::ElGamalPubkey,
) -> ProgramResult {
    invoke(
        &spl_token_2022::extension::confidential_transfer_fee::instruction::initialize_confidential_transfer_fee_config(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.copied(),
            withdraw_withheld_authority_elgamal_pubkey,
        )?,
        &[mint.clone()],
    )
}
