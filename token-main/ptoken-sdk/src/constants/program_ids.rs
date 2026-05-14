//! Canonical Solana program IDs used by LetterP token helpers.

use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

/// Original SPL Token program.
pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// Token-2022 (Token Extensions) program.
pub const TOKEN_2022_PROGRAM_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

/// Associated Token Account program.
pub const ATA_PROGRAM_ID: Pubkey = pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJe1bN");

/// Solana System program.
pub const SYSTEM_PROGRAM_ID: Pubkey = pubkey!("11111111111111111111111111111111");

/// SPL Memo program.
pub const MEMO_PROGRAM_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

/// Sysvar: Rent.
pub const SYSVAR_RENT_ID: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

/// Sysvar: Clock.
pub const SYSVAR_CLOCK_ID: Pubkey = pubkey!("SysvarC1ock11111111111111111111111111111111");

/// LetterP p-agent program ID reserved for devnet.
pub const LETTERP_P_AGENT_DEVNET_ID: Pubkey =
    pubkey!("FaXsrwC4bZnprnoMtvPibxLyJjhPSwC7pURsZK7T49Gg");

/// LetterP p-token program ID reserved for devnet.
pub const LETTERP_P_TOKEN_DEVNET_ID: Pubkey =
    pubkey!("7BNvimHVAW7KHzG33RFYoQLEQMZZ1yk8MbaDETwujptY");

/// LetterP x402 gateway program ID reserved for devnet.
pub const LETTERP_X402_GATEWAY_DEVNET_ID: Pubkey =
    pubkey!("5hsc8ptpLrCYfeEZypEm4NtDjjMpsSwWVLLnEwLtzeMF");

/// LetterP bonding-curve program ID reserved for devnet.
pub const LETTERP_BONDING_CURVE_DEVNET_ID: Pubkey =
    pubkey!("2yVjXFU9cLM79DDpLHvAsCWgZaxU2cNRpd9nBhj7tC3m");

/// LetterP perpetuals program ID reserved for devnet.
pub const LETTERP_PERPETUALS_DEVNET_ID: Pubkey =
    pubkey!("GikZAua12fZz7rNNBPeSp9PSGP5RpH2Drje7x75Kq4wX");

/// Returns true if the given pubkey is any recognized token program.
pub fn is_token_program(key: &Pubkey) -> bool {
    key == &TOKEN_PROGRAM_ID || key == &TOKEN_2022_PROGRAM_ID
}
