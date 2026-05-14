//! LetterP bonding-curve program entrypoint.

use ptoken_sdk::bonding_curve::LinearBondingCurve;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

/// Quotes a linear bonding-curve buy.
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 40 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let read_u64 = |offset: usize| -> Result<u64, ProgramError> {
        Ok(u64::from_le_bytes(
            instruction_data[offset..offset + 8]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        ))
    };
    let curve = LinearBondingCurve {
        base_price: read_u64(0)?,
        slope_numerator: read_u64(8)?,
        slope_denominator: read_u64(16)?,
    };
    let current_supply = read_u64(24)?;
    let tokens = read_u64(32)?;
    let quote = curve
        .buy_quote(current_supply, tokens)
        .map_err(ProgramError::from)?;

    msg!("LetterP bonding curve quote={}", quote);
    Ok(())
}
