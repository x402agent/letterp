#![no_std]

use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::{
    BindAgentToken, Buy, DelegateExecutor, Graduate, InitializeAgent, InitializeAgentMint, Sell,
};

entrypoint!(process_instruction);

pub const ID: Address = Address::new_from_array([0; 32]);

fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((InitializeAgent::DISCRIMINATOR, data)) => InitializeAgent::try_from((data, accounts))?.process(),
        Some((InitializeAgentMint::DISCRIMINATOR, data)) => InitializeAgentMint::try_from((data, accounts))?.process(),
        Some((BindAgentToken::DISCRIMINATOR, data)) => BindAgentToken::try_from((data, accounts))?.process(),
        Some((DelegateExecutor::DISCRIMINATOR, data)) => DelegateExecutor::try_from((data, accounts))?.process(),
        Some((Buy::DISCRIMINATOR, data)) => Buy::try_from((data, accounts))?.process(),
        Some((Sell::DISCRIMINATOR, data)) => Sell::try_from((data, accounts))?.process(),
        Some((Graduate::DISCRIMINATOR, data)) => Graduate::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
