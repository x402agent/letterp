//! # Transfer With Fee Example
//!
//! Demonstrates the Token-2022 Transfer Fee extension:
//! 1. Allocate a mint with extension space
//! 2. Initialize `TransferFeeConfig` (50 bps = 0.5%, max 1000 units)
//! 3. Initialize the base mint via `InitializeMint2`
//! 4. Mint tokens to a source account
//! 5. Transfer tokens — fee is automatically withheld in the destination
//! 6. Harvest withheld fees to the mint
//! 7. Withdraw fees from mint to fee authority wallet

use ptoken_sdk::{
    cpi::invoke_token_2022::{cpi_mint_to_2022, cpi_transfer_checked_2022},
    extensions::transfer_fee::{
        harvest_withheld_tokens_to_mint, initialize_transfer_fee_config, set_transfer_fee,
        withdraw_withheld_tokens_from_accounts, TransferFeeConfig,
    },
    token_2022::mint_with_extensions::create_mint_with_extensions,
    validation::signer_checks::assert_signer,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use spl_token_2022::extension::ExtensionType;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let discriminant = instruction_data
        .first()
        .copied()
        .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)?;

    let data = &instruction_data[1..];

    match discriminant {
        0 => process_create_fee_mint(accounts),
        1 => process_mint_tokens(accounts, data),
        2 => process_transfer_with_fee(accounts, data),
        3 => process_harvest_fees(accounts),
        _ => Err(solana_program::program_error::ProgramError::InvalidInstructionData),
    }
}

/// Instruction 0: Create a Token-2022 mint with TransferFeeConfig.
///
/// Accounts: [payer(signer,writable), mint(signer,writable),
///            fee_config_authority, withdraw_withheld_authority, system_program]
fn process_create_fee_mint(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let fee_authority = next_account_info(accounts_iter)?;
    let withdraw_authority = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    assert_signer(payer)?;

    let fee_config = TransferFeeConfig {
        transfer_fee_config_authority: Some(*fee_authority.key),
        withdraw_withheld_authority: Some(*withdraw_authority.key),
        transfer_fee_basis_points: 50, // 0.5%
        maximum_fee: 1_000,            // max 1000 raw units per transfer
    };

    // Step 1: Allocate mint account with space for TransferFeeConfig extension
    create_mint_with_extensions(
        payer,
        mint,
        payer.key, // mint authority
        None,      // no freeze authority
        6,         // 6 decimals
        &[ExtensionType::TransferFeeConfig],
        system_program,
    )?;

    // Step 2: Initialize the fee config extension
    initialize_transfer_fee_config(mint, &fee_config)?;

    msg!("Fee mint created: {} (0.5% fee, max 1000)", mint.key);
    Ok(())
}

/// Instruction 1: Mint tokens to a destination account.
fn process_mint_tokens(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint = next_account_info(accounts_iter)?;
    let destination = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    assert_signer(authority)?;

    let amount = ptoken_sdk::serialization::borsh_decode::read_u64(data, 0).unwrap_or(10_000_000);

    cpi_mint_to_2022(mint, destination, authority, amount)?;
    msg!("Minted {} units to {}", amount, destination.key);
    Ok(())
}

/// Instruction 2: Transfer tokens — fee auto-withheld in destination.
fn process_transfer_with_fee(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let source = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let destination = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    assert_signer(authority)?;

    let amount = ptoken_sdk::serialization::borsh_decode::read_u64(data, 0).unwrap_or(1_000_000);

    let fee = ptoken_sdk::math::decimals::calculate_transfer_fee(amount, 50, 1_000);
    msg!(
        "Transferring {} units (fee withheld: {} units)",
        amount,
        fee
    );

    cpi_transfer_checked_2022(source, mint, destination, authority, amount, 6)?;
    msg!(
        "Transfer complete. Fee of {} units withheld in destination account.",
        fee
    );
    Ok(())
}

/// Instruction 3: Harvest withheld fees from token accounts to the mint.
fn process_harvest_fees(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint = next_account_info(accounts_iter)?;
    let token_accounts: Vec<&AccountInfo> = accounts_iter.collect();
    let token_refs: Vec<&AccountInfo> = token_accounts.iter().copied().collect();

    harvest_withheld_tokens_to_mint(mint, &token_refs)?;
    msg!("Fees harvested to mint for {} accounts", token_refs.len());
    Ok(())
}
