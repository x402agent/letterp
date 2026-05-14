//! PDA derivation helpers.

use solana_program::pubkey::Pubkey;
use crate::constants::{program_ids::TOKEN_2022_PROGRAM_ID, seeds::*};

/// Derive a user-specific mint PDA and its bump seed.
///
/// Seeds: `["mint", user_pubkey]`
pub fn find_mint_pda(user: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MINT_SEED, user.as_ref()], program_id)
}

/// Derive a protocol vault PDA and its bump seed.
///
/// Seeds: `["vault", mint_pubkey]`
pub fn find_vault_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VAULT_SEED, mint.as_ref()], program_id)
}

/// Derive a metadata PDA and its bump seed.
///
/// Seeds: `["metadata", mint_pubkey]`
pub fn find_metadata_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[METADATA_SEED, mint.as_ref()], program_id)
}

/// Derive an authority PDA and its bump seed.
///
/// Seeds: `["authority", mint_pubkey]`
pub fn find_authority_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[AUTHORITY_SEED, mint.as_ref()], program_id)
}

/// Derive a fee vault PDA and its bump seed.
///
/// Seeds: `["fee_vault", mint_pubkey]`
pub fn find_fee_vault_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[FEE_VAULT_SEED, mint.as_ref()], program_id)
}

/// Derive a config PDA and its bump seed.
///
/// Seeds: `["config"]`
pub fn find_config_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG_SEED], program_id)
}

/// Derive a generic PDA from arbitrary seeds.
pub fn find_pda(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, program_id)
}

/// Create a PDA from seeds + bump (no search, panics if off-curve).
pub fn create_pda(seeds: &[&[u8]], bump: u8, program_id: &Pubkey) -> Result<Pubkey, solana_program::pubkey::PubkeyError> {
    let seeds_with_bump: Vec<&[u8]> = seeds.iter().copied().chain(std::iter::once(&[bump][..])).collect();
    Pubkey::create_program_address(&seeds_with_bump, program_id)
}
