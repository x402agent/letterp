//! x402 payment-gating primitives for LetterP agents.
//!
//! The SDK models the minimum on-chain facts a program should verify after an
//! x402 gateway or agent has produced a settlement receipt.

use crate::{errors::PTokenError, PTokenResult};

/// Payment asset used by a LetterP x402 settlement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentAsset {
    /// Native SOL lamports.
    Sol,
    /// SPL token mint bytes.
    SplToken([u8; 32]),
}

/// Required payment terms for a protected route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X402PaymentIntent {
    /// Required asset.
    pub asset: PaymentAsset,
    /// Minimum amount due in the asset's base units.
    pub amount_due: u64,
    /// Recipient account or vault bytes.
    pub pay_to: [u8; 32],
    /// Route hash binding the payment to a resource or instruction.
    pub route_hash: [u8; 32],
    /// Expiration timestamp in Unix seconds.
    pub expires_at_unix: u64,
}

/// Settlement state emitted by the x402 gateway path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X402SettlementStatus {
    /// Payment has not settled.
    Pending,
    /// Payment settled and can unlock the route.
    Accepted,
    /// Payment was rejected or reversed.
    Rejected,
}

/// Receipt facts verified before an agent can consume a gated route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X402Receipt {
    /// Asset that was paid.
    pub asset: PaymentAsset,
    /// Amount settled in base units.
    pub amount_paid: u64,
    /// Recipient account or vault bytes.
    pub paid_to: [u8; 32],
    /// Route hash bound to the paid resource.
    pub route_hash: [u8; 32],
    /// Settlement status.
    pub status: X402SettlementStatus,
}

/// Returns the gateway fee for an amount, rounded up to avoid undercharging.
pub fn gateway_fee(amount: u64, fee_bps: u16, minimum_fee: u64) -> PTokenResult<u64> {
    let raw = (amount as u128)
        .checked_mul(fee_bps as u128)
        .ok_or(PTokenError::ArithmeticOverflow)?;
    let rounded = ceil_div_u128(raw, 10_000)?;
    let fee = rounded.max(minimum_fee as u128);
    if fee > u64::MAX as u128 {
        return Err(PTokenError::ArithmeticOverflow);
    }
    Ok(fee as u64)
}

/// Validates that a receipt satisfies an intent at `now_unix`.
pub fn verify_receipt(
    intent: &X402PaymentIntent,
    receipt: &X402Receipt,
    now_unix: u64,
) -> PTokenResult<()> {
    if now_unix > intent.expires_at_unix {
        return Err(PTokenError::InvalidInstructionData);
    }
    if receipt.status != X402SettlementStatus::Accepted {
        return Err(PTokenError::InvalidInstructionData);
    }
    if receipt.asset != intent.asset
        || receipt.amount_paid < intent.amount_due
        || receipt.paid_to != intent.pay_to
        || receipt.route_hash != intent.route_hash
    {
        return Err(PTokenError::InvalidInstructionData);
    }
    Ok(())
}

fn ceil_div_u128(a: u128, b: u128) -> PTokenResult<u128> {
    if b == 0 {
        return Err(PTokenError::ArithmeticOverflow);
    }
    Ok(if a == 0 { 0 } else { 1 + (a - 1) / b })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepted_receipt_unlocks_intent() {
        let intent = X402PaymentIntent {
            asset: PaymentAsset::Sol,
            amount_due: 1_000,
            pay_to: [7; 32],
            route_hash: [9; 32],
            expires_at_unix: 100,
        };
        let receipt = X402Receipt {
            asset: PaymentAsset::Sol,
            amount_paid: 1_000,
            paid_to: [7; 32],
            route_hash: [9; 32],
            status: X402SettlementStatus::Accepted,
        };

        assert!(verify_receipt(&intent, &receipt, 99).is_ok());
    }
}
