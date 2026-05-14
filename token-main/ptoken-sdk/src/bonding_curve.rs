//! Bonding-curve math for LetterP token launches.

use crate::{errors::PTokenError, PTokenResult};

/// Basis-point denominator.
pub const BPS_DENOMINATOR: u128 = 10_000;

/// Linear bonding curve: `price = base_price + supply * slope`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearBondingCurve {
    /// Starting price in lamports or quote-token base units.
    pub base_price: u64,
    /// Slope numerator.
    pub slope_numerator: u64,
    /// Slope denominator. Must be nonzero.
    pub slope_denominator: u64,
}

impl LinearBondingCurve {
    /// Returns the marginal price at `supply`.
    pub fn price_at_supply(&self, supply: u64) -> PTokenResult<u64> {
        if self.slope_denominator == 0 {
            return Err(PTokenError::ArithmeticOverflow);
        }
        let slope = (supply as u128)
            .checked_mul(self.slope_numerator as u128)
            .ok_or(PTokenError::ArithmeticOverflow)?
            / self.slope_denominator as u128;
        let price = self.base_price as u128 + slope;
        u64_from_u128(price)
    }

    /// Quotes the total cost to buy `tokens` starting from `current_supply`.
    pub fn buy_quote(&self, current_supply: u64, tokens: u64) -> PTokenResult<u64> {
        if self.slope_denominator == 0 {
            return Err(PTokenError::ArithmeticOverflow);
        }
        let tokens_u128 = tokens as u128;
        let base = tokens_u128
            .checked_mul(self.base_price as u128)
            .ok_or(PTokenError::ArithmeticOverflow)?;
        let arithmetic_sum = tokens_u128
            .checked_mul(current_supply as u128)
            .and_then(|v| v.checked_add(tokens_u128.saturating_sub(1) * tokens_u128 / 2))
            .ok_or(PTokenError::ArithmeticOverflow)?;
        let slope_cost = arithmetic_sum
            .checked_mul(self.slope_numerator as u128)
            .ok_or(PTokenError::ArithmeticOverflow)?
            / self.slope_denominator as u128;
        let total = base
            .checked_add(slope_cost)
            .ok_or(PTokenError::ArithmeticOverflow)?;
        u64_from_u128(total)
    }
}

/// Quotes input needed to buy `tokens_out` from constant-product reserves.
pub fn constant_product_buy_quote(
    quote_reserve: u64,
    token_reserve: u64,
    tokens_out: u64,
    fee_bps: u16,
) -> PTokenResult<u64> {
    if tokens_out >= token_reserve || fee_bps as u128 >= BPS_DENOMINATOR {
        return Err(PTokenError::InvalidInstructionData);
    }
    let numerator = (quote_reserve as u128)
        .checked_mul(tokens_out as u128)
        .ok_or(PTokenError::ArithmeticOverflow)?;
    let denominator = (token_reserve - tokens_out) as u128;
    let raw = ceil_div_u128(numerator, denominator)?;
    gross_up_for_fee(raw, fee_bps)
}

/// Quotes output from selling `tokens_in` into constant-product reserves.
pub fn constant_product_sell_quote(
    quote_reserve: u64,
    token_reserve: u64,
    tokens_in: u64,
    fee_bps: u16,
) -> PTokenResult<u64> {
    if token_reserve == 0 || fee_bps as u128 >= BPS_DENOMINATOR {
        return Err(PTokenError::InvalidInstructionData);
    }
    let effective_in = apply_input_fee(tokens_in as u128, fee_bps)?;
    let numerator = (quote_reserve as u128)
        .checked_mul(effective_in)
        .ok_or(PTokenError::ArithmeticOverflow)?;
    let denominator = (token_reserve as u128)
        .checked_add(effective_in)
        .ok_or(PTokenError::ArithmeticOverflow)?;
    u64_from_u128(numerator / denominator)
}

fn apply_input_fee(amount: u128, fee_bps: u16) -> PTokenResult<u128> {
    amount
        .checked_mul(BPS_DENOMINATOR - fee_bps as u128)
        .map(|v| v / BPS_DENOMINATOR)
        .ok_or(PTokenError::ArithmeticOverflow)
}

fn gross_up_for_fee(net_amount: u128, fee_bps: u16) -> PTokenResult<u64> {
    let denominator = BPS_DENOMINATOR - fee_bps as u128;
    let numerator = net_amount
        .checked_mul(BPS_DENOMINATOR)
        .ok_or(PTokenError::ArithmeticOverflow)?;
    let gross = ceil_div_u128(numerator, denominator)?;
    u64_from_u128(gross)
}

fn ceil_div_u128(a: u128, b: u128) -> PTokenResult<u128> {
    if b == 0 {
        return Err(PTokenError::ArithmeticOverflow);
    }
    Ok(if a == 0 { 0 } else { 1 + (a - 1) / b })
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
    fn linear_quote_increases_with_supply() {
        let curve = LinearBondingCurve {
            base_price: 10,
            slope_numerator: 2,
            slope_denominator: 1,
        };

        assert!(curve.price_at_supply(10).unwrap() > curve.price_at_supply(1).unwrap());
        assert_eq!(curve.buy_quote(0, 3).unwrap(), 36);
    }
}
