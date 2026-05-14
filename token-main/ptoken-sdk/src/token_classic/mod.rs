//! SPL Token (classic) program operations.
//!
//! Program ID: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`

pub mod mint_creation;
pub mod token_account;
pub mod transfer;
pub mod approve_delegate;
pub mod revoke;
pub mod burn;
pub mod freeze_thaw;
pub mod close_account;
pub mod multisig;

pub use mint_creation::*;
pub use token_account::*;
pub use transfer::*;
pub use approve_delegate::*;
pub use revoke::*;
pub use burn::*;
pub use freeze_thaw::*;
pub use close_account::*;
pub use multisig::*;
