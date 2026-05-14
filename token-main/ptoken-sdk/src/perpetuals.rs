//! Perpetual trading math for LetterP agent guardrails.

use crate::{errors::PTokenError, PTokenResult};

/// Fixed-point price scale used by SDK quote helpers.
pub const PRICE_SCALE: u64 = 1_000_000;

/// Perpetual position side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSide {
    /// Long exposure.
    Long,
    /// Short exposure.
    Short,
}

/// Minimal perpetual position state used for risk checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// Position side.
    pub side: PositionSide,
    /// Collateral in quote-token base units.
    pub collateral: u64,
    /// Notional exposure in quote-token base units.
    pub notional: u64,
    /// Entry price scaled by [`PRICE_SCALE`].
    pub entry_price: u64,
}

impl Position {
    /// Returns leverage in basis points: 10_000 means 1x.
    pub fn leverage_bps(&self) -> PTokenResult<u64> {
        if self.collateral == 0 {
            return Err(PTokenError::InvalidInstructionData);
        }
        let leverage = (self.notional as u128)
            .checked_mul(10_000)
            .ok_or(PTokenError::ArithmeticOverflow)?
            / self.collateral as u128;
        u64_from_u128(leverage)
    }

    /// Calculates unrealized PnL at `mark_price`, scaled by quote-token units.
    pub fn unrealized_pnl(&self, mark_price: u64) -> PTokenResult<i128> {
        if self.entry_price == 0 {
            return Err(PTokenError::InvalidInstructionData);
        }
        let diff = mark_price.abs_diff(self.entry_price);
        let magnitude = (self.notional as u128)
            .checked_mul(diff as u128)
            .ok_or(PTokenError::ArithmeticOverflow)?
            / self.entry_price as u128;
        if magnitude > i128::MAX as u128 {
            return Err(PTokenError::ArithmeticOverflow);
        }
        let pnl = magnitude as i128;
        Ok(match (self.side, mark_price >= self.entry_price) {
            (PositionSide::Long, true) | (PositionSide::Short, false) => pnl,
            _ => -pnl,
        })
    }

    /// Returns true when equity is below the maintenance margin requirement.
    pub fn is_liquidatable(
        &self,
        mark_price: u64,
        maintenance_margin_bps: u16,
    ) -> PTokenResult<bool> {
        let pnl = self.unrealized_pnl(mark_price)?;
        let equity = self.collateral as i128 + pnl;
        let required = (self.notional as u128)
            .checked_mul(maintenance_margin_bps as u128)
            .ok_or(PTokenError::ArithmeticOverflow)?
            / 10_000;
        Ok(equity <= required as i128)
    }
}

/// Funding payment paid by longs when positive and by shorts when negative.
pub fn funding_payment(notional: u64, funding_rate_bps: i32) -> PTokenResult<i128> {
    let abs_rate = funding_rate_bps.unsigned_abs() as u128;
    let payment = (notional as u128)
        .checked_mul(abs_rate)
        .ok_or(PTokenError::ArithmeticOverflow)?
        / 10_000;
    if payment > i128::MAX as u128 {
        return Err(PTokenError::ArithmeticOverflow);
    }
    let signed = payment as i128;
    Ok(if funding_rate_bps >= 0 {
        signed
    } else {
        -signed
    })
}

fn u64_from_u128(value: u128) -> PTokenResult<u64> {
    if value > u64::MAX as u128 {
        return Err(PTokenError::ArithmeticOverflow);
    }
    Ok(value as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_position_gains_when_mark_rises() {
        let position = Position {
            side: PositionSide::Long,
            collateral: 100,
            notional: 500,
            entry_price: PRICE_SCALE,
        };

        assert!(position.unrealized_pnl(PRICE_SCALE * 2).unwrap() > 0);
        assert_eq!(position.leverage_bps().unwrap(), 50_000);
    }
}
