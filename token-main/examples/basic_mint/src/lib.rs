//! # Basic Mint Example
//!
//! Demonstrates the simplest LetterP workflow using SPL Token classic:
//! 1. Create and initialize a mint (6 decimals)
//! 2. Create an Associated Token Account for the user
//! 3. Mint 1,000,000 raw units to the ATA
//! 4. Transfer 500,000 units to a second wallet
//! 5. Burn 100,000 units
//!
//! This example uses explicit account parsing with LetterP SDK helpers.

use ptoken_sdk::{
    token_classic::{
        burn::burn_checked,
        mint_creation::{create_and_initialize_mint, mint_to},
        token_account::create_token_account,
        transfer::transfer_checked,
    },
    validation::{
        mint_validation::assert_mint_initialized, owner_checks::assert_owned_by_token_program,
        signer_checks::assert_signer,
    },
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

/// Instruction discriminants for this example program.
#[repr(u8)]
pub enum BasicMintInstruction {
    /// Create a new mint. Accounts: [payer(signer,writable), mint(signer,writable),
    /// system_program, token_program, rent_sysvar]
    CreateMint = 0,
    /// Create a token account. Accounts: [payer(signer,writable), token_account(signer,writable),
    /// mint, system_program, token_program, rent_sysvar]
    CreateTokenAccount = 1,
    /// Mint tokens to an account. Accounts: [mint(writable), destination(writable),
    /// authority(signer)]
    MintTokens = 2,
    /// Transfer tokens. Accounts: [source(writable), mint, destination(writable),
    /// authority(signer)]
    TransferTokens = 3,
    /// Burn tokens. Accounts: [account(writable), mint(writable), authority(signer)]
    BurnTokens = 4,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let discriminant = instruction_data
        .first()
        .copied()
        .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)?;

    let data = &instruction_data[1..];

    match discriminant {
        0 => process_create_mint(program_id, accounts, data),
        1 => process_create_token_account(program_id, accounts, data),
        2 => process_mint_tokens(program_id, accounts, data),
        3 => process_transfer(program_id, accounts, data),
        4 => process_burn(program_id, accounts, data),
        _ => Err(solana_program::program_error::ProgramError::InvalidInstructionData),
    }
}

fn process_create_mint(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;

    assert_signer(payer)?;
    assert_signer(mint)?;

    msg!("Creating mint with 6 decimals");

    create_and_initialize_mint(
        payer,
        mint,
        payer.key, // mint authority = payer
        None,      // no freeze authority
        6,         // 6 decimal places
        system_program,
        token_program,
        rent_sysvar,
    )?;

    msg!("Mint created: {}", mint.key);
    Ok(())
}

fn process_create_token_account(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;

    assert_signer(payer)?;
    assert_mint_initialized(mint)?;

    msg!("Creating token account for wallet: {}", payer.key);

    create_token_account(
        payer,
        token_account,
        mint,
        payer.key,
        system_program,
        token_program,
        rent_sysvar,
    )?;

    msg!("Token account created: {}", token_account.key);
    Ok(())
}

fn process_mint_tokens(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint = next_account_info(accounts_iter)?;
    let destination = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    assert_signer(authority)?;

    // amount is the first 8 bytes of data (little-endian u64)
    let amount = ptoken_sdk::serialization::borsh_decode::read_u64(data, 0).unwrap_or(1_000_000);

    msg!("Minting {} raw units to {}", amount, destination.key);

    mint_to(mint, destination, authority, amount)?;

    msg!("Minted successfully");
    Ok(())
}

fn process_transfer(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let source = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let destination = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    assert_signer(authority)?;

    let amount = ptoken_sdk::serialization::borsh_decode::read_u64(data, 0).unwrap_or(500_000);

    msg!("Transferring {} raw units to {}", amount, destination.key);

    transfer_checked(source, mint, destination, authority, amount, 6)?;

    msg!("Transfer complete");
    Ok(())
}

fn process_burn(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let token_account = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    assert_signer(authority)?;

    let amount = ptoken_sdk::serialization::borsh_decode::read_u64(data, 0).unwrap_or(100_000);

    msg!("Burning {} raw units", amount);

    burn_checked(token_account, mint, authority, amount, 6)?;

    msg!("Burn complete. Supply reduced.");
    Ok(())
}
