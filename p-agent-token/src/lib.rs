//! P Agent Token — Pinocchio bonding-curve agent program for Solana mainnet.
//!
//! Instructions:
//!   0 - InitializeAgent
//!   1 - InitializeAgentMint
//!   2 - BindAgentToken  (irreversible)
//!   3 - DelegateExecutor
//!   4 - Buy
//!   5 - Sell
//!   6 - Graduate

extern crate alloc;

pub mod error;
pub mod instruction;
pub mod pdas;
pub mod processor;
pub mod state;

// Program ID placeholder — replace with the deployed program's real address
// before mainnet deployment.
// NOTE: This is a placeholder (all zeros). The owner must deploy with the real keypair.
pinocchio::address::declare_id!("11111111111111111111111111111112");

#[cfg(not(feature = "no-entrypoint"))]
pinocchio::entrypoint!(processor::process_instruction);
