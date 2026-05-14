//! LetterP p-agent program entrypoint.

use ptoken_sdk::agent::{AgentCapabilityFlags, AgentPolicy};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

/// Processes p-agent policy validation instructions.
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 11 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let capability = match instruction_data[0] {
        0 => AgentCapabilityFlags::X402_SETTLEMENT,
        1 => AgentCapabilityFlags::BONDING_CURVE_TRADING,
        2 => AgentCapabilityFlags::PERPETUAL_TRADING,
        3 => AgentCapabilityFlags::TOKEN_EXTENSION_ADMIN,
        _ => return Err(ProgramError::InvalidInstructionData),
    };
    let spending_limit_lamports = u64::from_le_bytes(
        instruction_data[1..9]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let risk_limit_bps = u16::from_le_bytes(
        instruction_data[9..11]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let policy = AgentPolicy {
        agent_id: [0; 32],
        owner: [0; 32],
        capabilities: capability,
        spending_limit_lamports,
        risk_limit_bps,
    };
    policy.validate().map_err(ProgramError::from)?;

    msg!("LetterP p-agent policy accepted");
    Ok(())
}
