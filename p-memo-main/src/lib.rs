//! LetterP memo program.
//!
//! The program accepts UTF-8 instruction data, requires every supplied account
//! to be a signer, and logs signer identities plus the memo body using
//! Pinocchio's low-overhead entrypoint.

#![no_std]

mod entrypoint;

pinocchio::no_allocator!();
pinocchio::lazy_program_entrypoint!(process_instruction);

pub use entrypoint::process_instruction;
