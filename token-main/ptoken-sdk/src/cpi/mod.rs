//! Cross-Program Invocation helpers.
//!
//! Thin wrappers around Solana's `invoke` and `invoke_signed` for calling
//! token programs from within a Pinocchio-based on-chain program.

pub mod invoke_associated_token;
pub mod invoke_system_program;
pub mod invoke_token_2022;
pub mod invoke_token_program;

pub use invoke_associated_token::*;
pub use invoke_system_program::*;
pub use invoke_token_2022::*;
pub use invoke_token_program::*;
