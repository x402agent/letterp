//! LetterP p-token program entrypoint.

use ptoken_sdk::math::{calculate_transfer_fee, decimal_multiplier};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

/// Processes p-token math preflight instructions.
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 11 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let fee_bps = u16::from_le_bytes(
        instruction_data[8..10]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let decimals = instruction_data[10];
    let multiplier = decimal_multiplier(decimals);
    let fee = calculate_transfer_fee(amount, fee_bps, amount);

    msg!(
        "LetterP p-token preflight multiplier={} fee={}",
        multiplier,
        fee
    );
    Ok(())
}
