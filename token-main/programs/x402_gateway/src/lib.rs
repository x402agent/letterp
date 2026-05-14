//! LetterP x402 gateway program entrypoint.

use ptoken_sdk::x402::{
    verify_receipt, PaymentAsset, X402PaymentIntent, X402Receipt, X402SettlementStatus,
};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

/// Verifies a compact SOL-denominated x402 receipt preflight.
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 17 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount_due = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let amount_paid = u64::from_le_bytes(
        instruction_data[8..16]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let accepted = instruction_data[16] == 1;
    let intent = X402PaymentIntent {
        asset: PaymentAsset::Sol,
        amount_due,
        pay_to: [7; 32],
        route_hash: [9; 32],
        expires_at_unix: u64::MAX,
    };
    let receipt = X402Receipt {
        asset: PaymentAsset::Sol,
        amount_paid,
        paid_to: [7; 32],
        route_hash: [9; 32],
        status: if accepted {
            X402SettlementStatus::Accepted
        } else {
            X402SettlementStatus::Rejected
        },
    };

    verify_receipt(&intent, &receipt, 0).map_err(ProgramError::from)?;
    msg!("LetterP x402 receipt accepted");
    Ok(())
}
