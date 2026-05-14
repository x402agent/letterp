#![no_std]

use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::{Deposit, Withdraw};

entrypoint!(process_instruction);

pub const ID: Address = Address::new_from_array([0; 32]);

fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, data)) => Withdraw::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

