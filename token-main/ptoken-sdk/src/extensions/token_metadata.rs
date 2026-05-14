//! Token Metadata extension — embed metadata directly in the mint.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};

/// Initialize token metadata on a mint account.
///
/// The mint must have the MetadataPointer extension pointing to itself,
/// and sufficient space allocated for the metadata fields.
///
/// # Arguments
/// * `name` — Token name (e.g. "USD Coin")
/// * `symbol` — Ticker symbol (e.g. "USDC")
/// * `uri` — Off-chain JSON metadata URI
pub fn initialize_token_metadata<'a>(
    mint: &AccountInfo<'a>,
    update_authority: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    name: String,
    symbol: String,
    uri: String,
) -> ProgramResult {
    invoke(
        &spl_token_metadata_interface::instruction::initialize(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            update_authority.key,
            mint.key,
            mint_authority.key,
            name,
            symbol,
            uri,
        ),
        &[
            mint.clone(),
            update_authority.clone(),
            mint_authority.clone(),
        ],
    )
}

/// Update a metadata field on the mint.
pub fn update_token_metadata_field<'a>(
    mint: &AccountInfo<'a>,
    update_authority: &AccountInfo<'a>,
    field: spl_token_metadata_interface::state::Field,
    value: String,
) -> ProgramResult {
    invoke(
        &spl_token_metadata_interface::instruction::update_field(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            update_authority.key,
            field,
            value,
        ),
        &[mint.clone(), update_authority.clone()],
    )
}
