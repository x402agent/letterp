//! Custom program entrypoint pattern for Pinocchio-style programs.
//!
//! Instead of Anchor's `#[program]` macro, Pinocchio programs use a simple
//! `process_instruction` function dispatched by a manual match on the
//! instruction discriminant.

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

/// Trait for Pinocchio-style instruction processors.
/// Each instruction variant implements this trait.
pub trait InstructionProcessor {
    /// Process the instruction given the program ID, accounts, and raw data.
    fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult;
}

/// Macro to declare a Pinocchio-style entrypoint.
///
/// # Usage
/// ```rust,ignore
/// ptoken_sdk::declare_entrypoint!(process_instruction);
///
/// fn process_instruction(
///     program_id: &Pubkey,
///     accounts: &[AccountInfo],
///     data: &[u8],
/// ) -> ProgramResult {
///     match data[0] {
///         0 => handle_initialize(program_id, accounts, &data[1..]),
///         1 => handle_transfer(program_id, accounts, &data[1..]),
///         _ => Err(ProgramError::InvalidInstructionData),
///     }
/// }
/// ```
#[macro_export]
macro_rules! declare_entrypoint {
    ($process_fn:ident) => {
        solana_program::entrypoint!($process_fn);
    };
}
