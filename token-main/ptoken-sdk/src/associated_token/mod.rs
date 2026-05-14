//! Associated Token Account (ATA) helpers.
//!
//! ATA Program ID: `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJe1bN`

pub mod create_ata;
pub mod get_ata_address;
pub mod idempotent_create;

pub use create_ata::*;
pub use get_ata_address::*;
pub use idempotent_create::*;
