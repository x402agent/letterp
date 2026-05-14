//! # Confidential Mint Example
//!
//! Demonstrates the Token-2022 Confidential Transfer extension.
//! Transfer amounts are hidden from the public ledger using ElGamal encryption
//! and zero-knowledge proofs.
//!
//! ## Flow
//! 1. Create mint with ConfidentialTransfer extension (auto-approve = true)
//! 2. Create and configure token accounts with ElGamal keypairs
//! 3. Deposit tokens into confidential balance
//! 4. Transfer confidentially (amount invisible on-chain)
//! 5. Apply pending balance (receiver merges incoming transfer)
//! 6. Withdraw back to visible balance
//!
//! ## Important
//! ZK proof generation happens client-side using the solana-zk-token-sdk.
//! The proofs are submitted as separate instruction accounts.

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use ptoken_sdk::{
    extensions::confidential_transfer::initialize_confidential_transfer_mint,
    token_2022::mint_with_extensions::create_mint_with_extensions,
    validation::signer_checks::assert_signer,
};
use spl_token_2022::extension::ExtensionType;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.first().copied().ok_or(ProgramError::InvalidInstructionData)? {
        0 => process_create_confidential_mint(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Create a Token-2022 mint with Confidential Transfer enabled.
///
/// Uses `auto_approve_new_accounts = true` so any token account
/// can participate without requiring per-account approval.
///
/// Accounts: [payer(signer,writable), mint(signer,writable), system_program]
fn process_create_confidential_mint(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    assert_signer(payer)?;

    msg!("Step 1: Allocate mint with ConfidentialTransfer extension space");

    create_mint_with_extensions(
        payer,
        mint,
        payer.key,
        None,
        6,
        &[ExtensionType::ConfidentialTransferMint],
        system_program,
    )?;

    msg!("Step 2: Initialize confidential transfer config (auto-approve)");

    initialize_confidential_transfer_mint(
        mint,
        Some(payer.key),  // authority can update config
        true,             // auto_approve_new_accounts
        None,             // no auditor
    )?;

    msg!("Confidential mint created: {}", mint.key);
    msg!("All token accounts can transfer confidentially without per-account approval.");
    msg!("");
    msg!("Next steps (client-side):");
    msg!("  1. Generate ElGamal keypair for each token account");
    msg!("  2. Call ConfigureAccount to register ElGamal pubkey");
    msg!("  3. Call Deposit to move tokens into confidential balance");
    msg!("  4. Generate ZK transfer proof client-side");
    msg!("  5. Call Transfer with proof to send confidentially");
    msg!("  6. Receiver calls ApplyPendingBalance to finalize");
    Ok(())
}
