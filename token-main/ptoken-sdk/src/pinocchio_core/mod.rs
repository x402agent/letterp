//! Pinocchio runtime primitives.
//!
//! Pinocchio is a zero-dependency Solana program framework that gives
//! direct control over account deserialization and instruction parsing
//! without Anchor macro overhead.

pub mod account_info;
pub mod instruction_data;
pub mod program_entrypoint;
pub mod syscalls;
pub mod zero_copy_layout;

pub use account_info::*;
pub use instruction_data::*;
pub use zero_copy_layout::*;
