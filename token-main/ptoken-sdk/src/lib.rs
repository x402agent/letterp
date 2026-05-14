//! # pToken SDK
//!
//! A high-performance Solana token SDK built on Pinocchio principles —
//! zero-dependency, close-to-the-metal Rust primitives for SPL Token
//! and Token-2022 programs.
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
//! - [`validation`] — Account and instruction validation
//! - [`errors`] — Custom error types
//! - [`constants`] — Program IDs, seeds, and defaults

#![deny(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(not(kani))]
pub mod pinocchio_core;
#[cfg(not(kani))]
pub mod token_classic;
#[cfg(not(kani))]
pub mod token_2022;
#[cfg(not(kani))]
pub mod extensions;
#[cfg(not(kani))]
pub mod cpi;
#[cfg(not(kani))]
pub mod pda;
#[cfg(not(kani))]
pub mod associated_token;
#[cfg(not(kani))]
pub mod serialization;
pub mod math;
#[cfg(not(kani))]
pub mod validation;
pub mod errors;
pub mod constants;

#[cfg(kani)]
mod kani_verification;

pub use errors::{PTokenError, PTokenResult};

/// Re-export commonly used Solana types for convenience
pub use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
