//! Derive Associated Token Account addresses.

use crate::constants::program_ids::{TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID};
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address_with_program_id;

/// Derive the ATA address for a wallet and mint (SPL Token classic).
pub fn get_associated_token_address(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(wallet, mint, &TOKEN_PROGRAM_ID)
}

/// Derive the ATA address for a wallet and mint (Token-2022).
pub fn get_associated_token_address_2022(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(wallet, mint, &TOKEN_2022_PROGRAM_ID)
}

/// Derive the ATA address for any token program.
pub fn get_ata(wallet: &Pubkey, mint: &Pubkey, token_program_id: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(wallet, mint, token_program_id)
}
