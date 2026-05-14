//! Transfer Fee extension — charge an automatic fee on every token transfer.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_token_2022::extension::transfer_fee::instruction as fee_ix;

/// Configuration for the Transfer Fee extension.
#[derive(Debug, Clone)]
pub struct TransferFeeConfig {
    /// Authority that can update the fee config.
    pub transfer_fee_config_authority: Option<Pubkey>,
    /// Authority that can withdraw withheld fees.
    pub withdraw_withheld_authority: Option<Pubkey>,
    /// Fee in basis points (1 bp = 0.01%). Max 10_000 (100%).
    pub transfer_fee_basis_points: u16,
    /// Maximum fee per transfer in raw token units.
    pub maximum_fee: u64,
}

/// Initialize the Transfer Fee extension on a mint.
///
/// Must be called before `InitializeMint2`.
pub fn initialize_transfer_fee_config<'a>(
    mint: &AccountInfo<'a>,
    config: &TransferFeeConfig,
) -> ProgramResult {
    invoke(
        &fee_ix::initialize_transfer_fee_config(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            config.transfer_fee_config_authority.as_ref(),
            config.withdraw_withheld_authority.as_ref(),
            config.transfer_fee_basis_points,
            config.maximum_fee,
        )?,
        &[mint.clone()],
    )
}

/// Update the transfer fee configuration on an existing mint.
pub fn set_transfer_fee<'a>(
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
) -> ProgramResult {
    invoke(
        &fee_ix::set_transfer_fee(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            authority.key,
            &[],
            transfer_fee_basis_points,
            maximum_fee,
        )?,
        &[mint.clone(), authority.clone()],
    )
}

/// Withdraw withheld tokens from token accounts to the fee vault.
pub fn withdraw_withheld_tokens_from_accounts<'a>(
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    token_accounts: &[&AccountInfo<'a>],
) -> ProgramResult {
    let token_account_keys: Vec<_> = token_accounts.iter().map(|a| a.key).collect();
    let mut accounts = vec![mint.clone(), destination.clone(), authority.clone()];
    accounts.extend(token_accounts.iter().map(|a| (*a).clone()));

    invoke(
        &fee_ix::withdraw_withheld_tokens_from_accounts(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            destination.key,
            authority.key,
            &[],
            &token_account_keys,
        )?,
        &accounts,
    )
}

/// Harvest withheld tokens from token accounts to the mint's withheld amount.
pub fn harvest_withheld_tokens_to_mint<'a>(
    mint: &AccountInfo<'a>,
    token_accounts: &[&AccountInfo<'a>],
) -> ProgramResult {
    let keys: Vec<_> = token_accounts.iter().map(|a| a.key).collect();
    let mut accounts = vec![mint.clone()];
    accounts.extend(token_accounts.iter().map(|a| (*a).clone()));

    invoke(
        &fee_ix::harvest_withheld_tokens_to_mint(&TOKEN_2022_PROGRAM_ID, mint.key, &keys)?,
        &accounts,
    )
}
