//! Confidential Transfer extension — hide transfer amounts with ZK proofs.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use solana_zk_token_sdk::{
    encryption::auth_encryption::AeCiphertext, zk_token_elgamal::pod::ElGamalPubkey,
};
use spl_token_2022::extension::confidential_transfer::instruction as ct_ix;

/// Initialize the Confidential Transfer extension on a mint.
///
/// Must be called before `InitializeMint2`.
///
/// # Arguments
/// * `authority` — Can approve confidential transfer accounts. `None` = auto-approve.
/// * `auto_approve_new_accounts` — If true, all new accounts can transfer confidentially.
/// * `auditor_elgamal_pubkey` — Optional auditor that can decrypt all transfers.
pub fn initialize_confidential_transfer_mint<'a>(
    mint: &AccountInfo<'a>,
    authority: Option<&Pubkey>,
    auto_approve_new_accounts: bool,
    auditor_elgamal_pubkey: Option<ElGamalPubkey>,
) -> ProgramResult {
    invoke(
        &ct_ix::initialize_mint(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.copied(),
            auto_approve_new_accounts,
            auditor_elgamal_pubkey,
        )?,
        &[mint.clone()],
    )
}

/// Apply a pending confidential balance to the account's available balance.
///
/// After receiving confidential transfers, the recipient must call this
/// to merge their pending balance into the available balance.
pub fn apply_pending_balance<'a>(
    token_account: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
    expected_pending_balance_credit_counter: u64,
    new_decryptable_available_balance: AeCiphertext,
) -> ProgramResult {
    invoke(
        &ct_ix::apply_pending_balance(
            &TOKEN_2022_PROGRAM_ID,
            token_account.key,
            expected_pending_balance_credit_counter,
            new_decryptable_available_balance,
            owner.key,
            &[],
        )?,
        &[token_account.clone(), owner.clone()],
    )
}
