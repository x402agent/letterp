//! SPL Token (classic) program operations.
//!
//! Program ID: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`

pub mod approve_delegate;
pub mod burn;
pub mod close_account;
pub mod freeze_thaw;
pub mod mint_creation;
pub mod multisig;
pub mod revoke;
pub mod token_account;
pub mod transfer;

pub use approve_delegate::*;
pub use burn::*;
pub use close_account::*;
pub use freeze_thaw::*;
pub use mint_creation::*;
pub use multisig::*;
pub use revoke::*;
pub use token_account::*;
pub use transfer::*;
