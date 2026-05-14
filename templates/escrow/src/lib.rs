#![no_std]

use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::{Make, Refund, Take};

entrypoint!(process_instruction);

pub const ID: Address = Address::new_from_array([0; 32]);

fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Make::DISCRIMINATOR, data)) => Make::try_from((data, accounts))?.process(),
        Some((Take::DISCRIMINATOR, data)) => Take::try_from((data, accounts))?.process(),
        Some((Refund::DISCRIMINATOR, data)) => Refund::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

