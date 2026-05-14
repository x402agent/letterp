//! LetterP perpetuals program entrypoint.

use ptoken_sdk::perpetuals::{Position, PositionSide};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

/// Runs a perpetual position risk preflight.
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 35 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let side = match instruction_data[0] {
        0 => PositionSide::Long,
        1 => PositionSide::Short,
        _ => return Err(ProgramError::InvalidInstructionData),
    };
    let read_u64 = |offset: usize| -> Result<u64, ProgramError> {
        Ok(u64::from_le_bytes(
            instruction_data[offset..offset + 8]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        ))
    };
    let position = Position {
        side,
        collateral: read_u64(1)?,
        notional: read_u64(9)?,
        entry_price: read_u64(17)?,
    };
    let mark_price = read_u64(25)?;
    let maintenance_margin_bps = u16::from_le_bytes(
        instruction_data[33..35]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let liquidatable = position
        .is_liquidatable(mark_price, maintenance_margin_bps)
        .map_err(ProgramError::from)?;

    msg!("LetterP perpetual risk liquidatable={}", liquidatable);
    Ok(())
}
