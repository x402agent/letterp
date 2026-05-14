//! # Token With Metadata Example
//!
//! Demonstrates Token-2022 embedded metadata (no Metaplex needed):
//! 1. Allocate mint with MetadataPointer + (space for) TokenMetadata extensions
//! 2. Initialize MetadataPointer (pointing to the mint itself)
//! 3. Initialize base mint via InitializeMint2
//! 4. Initialize TokenMetadata (name, symbol, URI)
//! 5. Update a metadata field
//!
//! Result: A mint that IS its own metadata account.

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use ptoken_sdk::{
    extensions::{
        metadata_pointer::initialize_metadata_pointer,
        token_metadata::initialize_token_metadata,
    },
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
    let discriminant = instruction_data
        .first()
        .copied()
        .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)?;

    match discriminant {
        0 => process_create_metadata_mint(accounts),
        _ => Err(solana_program::program_error::ProgramError::InvalidInstructionData),
    }
}

/// Create a Token-2022 mint with embedded name, symbol, and URI.
///
/// Accounts: [payer(signer,writable), mint(signer,writable),
///            update_authority(signer), system_program]
fn process_create_metadata_mint(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let update_authority = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    assert_signer(payer)?;
    assert_signer(update_authority)?;

    msg!("Step 1: Allocate mint with MetadataPointer extension space");

    // Allocate mint with extension space
    // Note: TokenMetadata space is added separately since its size is dynamic
    create_mint_with_extensions(
        payer,
        mint,
        payer.key,
        None,
        0,   // 0 decimals for this NFT-style token
        &[ExtensionType::MetadataPointer],
        system_program,
    )?;

    msg!("Step 2: Initialize MetadataPointer (pointing to mint itself)");

    initialize_metadata_pointer(
        mint,
        Some(update_authority.key),  // authority can update the pointer
        Some(mint.key),              // metadata lives in the mint account
    )?;

    msg!("Step 3: Initialize token metadata");

    initialize_token_metadata(
        mint,
        update_authority,
        payer,   // mint authority
        "pToken Example".to_string(),
        "PTKN".to_string(),
        "https://example.com/ptoken-metadata.json".to_string(),
    )?;

    msg!("Metadata mint created: {}", mint.key);
    msg!("Name: pToken Example | Symbol: PTKN");
    Ok(())
}
