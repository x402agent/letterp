//! # LetterP Token SDK
//!
//! Solana token primitives for LetterP programs that use SPL Token and
//! Token-2022 directly with explicit account, instruction, and arithmetic
//! boundaries.
//!
//! ## Modules
//! - [`pinocchio_core`] — Pinocchio runtime primitives
//! - [`token_classic`] — SPL Token (original program) operations
//! - [`token_2022`] — Token-2022 base operations
//! - [`extensions`] — All Token-2022 extensions
//! - [`cpi`] — Cross-Program Invocation helpers
//! - [`pda`] — Program Derived Address utilities
//! - [`associated_token`] — Associated Token Account wrappers
//! - [`serialization`] — Borsh and Pack/Unpack serialization
//! - [`math`] — Safe arithmetic and decimal helpers
//! - [`agent`] — Agent permission and execution policy primitives
//! - [`x402`] — x402 payment intent and receipt verification helpers
//! - [`bonding_curve`] — Bonding-curve quote math
//! - [`perpetuals`] — Perpetual position and funding math
//! - [`validation`] — Account and instruction validation
//! - [`errors`] — Custom error types
//! - [`constants`] — Program IDs, seeds, and defaults

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod agent;
#[cfg(not(kani))]
pub mod associated_token;
pub mod bonding_curve;
pub mod constants;
#[cfg(not(kani))]
pub mod cpi;
pub mod errors;
#[cfg(not(kani))]
pub mod extensions;
pub mod math;
#[cfg(not(kani))]
pub mod pda;
pub mod perpetuals;
#[cfg(not(kani))]
pub mod pinocchio_core;
#[cfg(not(kani))]
pub mod serialization;
#[cfg(not(kani))]
pub mod token_2022;
#[cfg(not(kani))]
pub mod token_classic;
#[cfg(not(kani))]
pub mod validation;
pub mod x402;

#[cfg(kani)]
mod kani_verification;

pub use errors::{PTokenError, PTokenResult};

/// Re-export commonly used Solana types for convenience
pub use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
