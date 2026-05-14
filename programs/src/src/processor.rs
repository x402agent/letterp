//! Instruction processor for the bonding-curve program.
//!
//! Each instruction handler receives the parsed accounts and data.
//! The actual curve math (quote_buy, quote_sell, apply_buy, apply_sell)
//! is implemented off-chain in `launchpad/` — here on-chain we validate
//! the invariants and update state atomically.

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use crate::instruction::BondingCurveInstruction;
use crate::state::{CurveState, CURVE_STATE_LEN};
use crate::error::CurveError;

const BPS: u64 = 10_000;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = BondingCurveInstruction::unpack(instruction_data)?;

    match instruction {
        BondingCurveInstruction::InitializeCurve {
            total_supply,
            virtual_sol_reserves,
            virtual_token_reserves,
            creator_fee_bps,
            protocol_fee_bps,
        } => {
            process_initialize_curve(
                accounts,
                total_supply,
                virtual_sol_reserves,
                virtual_token_reserves,
                creator_fee_bps,
                protocol_fee_bps,
            )
        }
        BondingCurveInstruction::Buy {
            sol_in,
            min_tokens_out,
        } => process_buy(accounts, sol_in, min_tokens_out),
        BondingCurveInstruction::Sell {
            tokens_in,
            min_sol_out,
        } => process_sell(accounts, tokens_in, min_sol_out),
        BondingCurveInstruction::Graduate => process_graduate(accounts),
        BondingCurveInstruction::ClaimCreatorFees => process_claim_fees(accounts),
    }
}

/// Validate the curve state is initialized and not graduated.
fn check_curve_is_active(state: &CurveState) -> Result<(), ProgramError> {
    if state.discriminant != 1 {
        return Err(CurveError::NotInitialized.into());
    }
    if state.graduated == 1 {
        return Err(CurveError::AlreadyGraduated.into());
    }
    Ok(())
}

// -----------------------------------------------------------------------
// Handlers
// -----------------------------------------------------------------------

fn process_initialize_curve(
    accounts: &[AccountInfo],
    total_supply: u64,
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    creator_fee_bps: u16,
    protocol_fee_bps: u16,
) -> ProgramResult {
    // TODO: implement on-chain initialization logic.
    // This involves:
    //   - Validating PDAs (curve, vault)
    //   - Storing curve parameters in the curve PDA account
    //   - Creating the vault PDA with system program
    //   - Setting mint authority to the curve PDA
    //
    // For now this is a stub — the full implementation requires Pinocchio
    // CPI helpers for system and token programs.
    //
    // See launchpad/src/programs/launchpad-ix.ts for the off-chain
    // instruction builder that constructs the correct accounts and data.

    pinocchio_log::sol_log("InitializeCurve (stub)");
    Ok(())
}

fn process_buy(
    accounts: &[AccountInfo],
    sol_in: u64,
    min_tokens_out: u64,
) -> ProgramResult {
    // TODO: implement on-chain buy logic.
    //   - Compute fee deduction from sol_in
    //   - Calculate tokens_out using the constant-product invariant
    //   - Validate min_tokens_out (slippage)
    //   - Transfer SOL from buyer to vault
    //   - Mint tokens to buyer's ATA via token program CPI
    //   - Update realSolReserves, realTokenReserves
    //   - Store accumulated fees (creator + protocol) in the vault

    pinocchio_log::sol_log("Buy (stub)");
    Ok(())
}

fn process_sell(
    accounts: &[AccountInfo],
    tokens_in: u64,
    min_sol_out: u64,
) -> ProgramResult {
    // TODO: implement on-chain sell logic.
    //   - Burn tokens from seller's ATA via token program CPI
    //   - Calculate sol_out using the constant-product invariant
    //   - Deduct fees from sol_out
    //   - Validate min_sol_out (slippage)
    //   - Transfer SOL from vault to seller
    //   - Update realSolReserves, realTokenReserves

    pinocchio_log::sol_log("Sell (stub)");
    Ok(())
}

fn process_graduate(accounts: &[AccountInfo]) -> ProgramResult {
    // TODO: implement graduation logic.
    //   - Mark curve as graduated
    //   - Transfer remaining liquidity to Raydium pool or the target
    //   - Disable further trades on the curve

    pinocchio_log::sol_log("Graduate (stub)");
    Ok(())
}

fn process_claim_fees(accounts: &[AccountInfo]) -> ProgramResult {
    // TODO: implement creator fee claiming.
    //   - Verify the caller is the creator_fee_wallet or authority
    //   - Transfer accumulated SOL fees from vault to recipient
    //   - Reset fee accumulator

    pinocchio_log::sol_log("ClaimCreatorFees (stub)");
    Ok(())
}
