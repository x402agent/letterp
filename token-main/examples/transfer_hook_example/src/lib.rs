//! # Transfer Hook Example
//!
//! Two programs in one file to demonstrate the full Transfer Hook lifecycle:
//!
//! ## Program A: Mint Creator
//! Creates a Token-2022 mint with a Transfer Hook pointing to Program B.
//!
//! ## Program B: The Hook Program
//! Implements the `spl-transfer-hook-interface` `Execute` instruction.
//! Logs the transfer amount and enforces a minimum transfer size.
//!
//! ## Flow
//! 1. Deploy Program B (hook)
//! 2. Create mint via Program A, pointing hook to Program B
//! 3. Every `transfer_checked` call invokes Program B automatically

use ptoken_sdk::{
    extensions::transfer_hook::{initialize_transfer_hook, update_transfer_hook},
    token_2022::mint_with_extensions::create_mint_with_extensions,
    validation::signer_checks::assert_signer,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::extension::ExtensionType;

// ─── Mint Creator Program ───────────────────────────────────────────────────

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data
        .first()
        .copied()
        .ok_or(ProgramError::InvalidInstructionData)?
    {
        0 => process_create_hook_mint(accounts, &instruction_data[1..]),
        1 => process_update_hook(accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Create a Token-2022 mint with a transfer hook program configured.
///
/// Accounts: [payer(signer,writable), mint(signer,writable), system_program]
/// Data: [hook_program_id: 32 bytes]
fn process_create_hook_mint(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    assert_signer(payer)?;

    // Parse hook program ID from instruction data
    let hook_program_id = ptoken_sdk::serialization::borsh_decode::read_pubkey(data, 0)?;

    msg!("Creating hook mint with hook program: {}", hook_program_id);

    // Allocate mint with TransferHook extension space
    create_mint_with_extensions(
        payer,
        mint,
        payer.key,
        None,
        6,
        &[ExtensionType::TransferHook],
        system_program,
    )?;

    // Initialize the transfer hook extension
    initialize_transfer_hook(
        mint,
        Some(payer.key),        // authority can update the hook
        Some(&hook_program_id), // the hook program to invoke on every transfer
    )?;

    msg!("Hook mint created: {}", mint.key);
    Ok(())
}

/// Update the hook program on an existing mint.
fn process_update_hook(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    assert_signer(authority)?;

    let new_hook_program_id = ptoken_sdk::serialization::borsh_decode::read_pubkey(data, 0)?;

    update_transfer_hook(mint, authority, Some(&new_hook_program_id))?;

    msg!("Hook program updated to: {}", new_hook_program_id);
    Ok(())
}

// ─── Hook Program (Program B) ───────────────────────────────────────────────

/// Minimum transfer amount enforced by this hook (1000 raw units).
pub const MIN_TRANSFER_AMOUNT: u64 = 1_000;

/// Execute hook — called automatically by Token-2022 on every transfer.
///
/// This is the entry point for the hook program.
/// It receives source, destination, mint, and extra accounts,
/// plus the transfer amount as instruction data.
pub fn execute_hook(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let _source = next_account_info(accounts_iter)?;
    let _mint = next_account_info(accounts_iter)?;
    let _destination = next_account_info(accounts_iter)?;
    let _owner = next_account_info(accounts_iter)?;
    // extra_account_metas account follows if configured

    // The Transfer Hook Interface passes amount as the last 8 bytes
    let amount = ptoken_sdk::serialization::borsh_decode::read_u64(
        instruction_data,
        instruction_data.len().saturating_sub(8),
    )
    .unwrap_or(0);

    msg!("Transfer hook fired — amount: {} raw units", amount);

    // Enforce minimum transfer amount
    if amount < MIN_TRANSFER_AMOUNT {
        msg!(
            "Transfer rejected: {} < minimum {}",
            amount,
            MIN_TRANSFER_AMOUNT
        );
        return Err(ProgramError::Custom(1));
    }

    msg!(
        "Transfer hook passed. Amount {} >= minimum {}.",
        amount,
        MIN_TRANSFER_AMOUNT
    );
    Ok(())
}
