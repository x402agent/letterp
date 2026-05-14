//! Checked arithmetic operations for safe token calculations.

use crate::errors::PTokenError;
use crate::errors::token_errors::PTokenError as Err;

/// Safely add two u64 values, returning [`PTokenError::ArithmeticOverflow`] on overflow.
pub fn safe_add(a: u64, b: u64) -> Result<u64, PTokenError> {
    a.checked_add(b).ok_or(Err::ArithmeticOverflow)
}

/// Safely subtract two u64 values, returning [`PTokenError::ArithmeticUnderflow`] on underflow.
pub fn safe_sub(a: u64, b: u64) -> Result<u64, PTokenError> {
    a.checked_sub(b).ok_or(Err::ArithmeticUnderflow)
}

/// Safely multiply two u64 values, returning [`PTokenError::ArithmeticOverflow`] on overflow.
pub fn safe_mul(a: u64, b: u64) -> Result<u64, PTokenError> {
    a.checked_mul(b).ok_or(Err::ArithmeticOverflow)
}

/// Safely divide two u64 values. Returns 0 if divisor is 0 (no panic).
pub fn safe_div(a: u64, b: u64) -> Result<u64, PTokenError> {
    if b == 0 {
        return Err(Err::ArithmeticOverflow);
    }
    Ok(a / b)
}

/// Calculate `a * b / c` with intermediate u128 to avoid overflow.
pub fn muldiv(a: u64, b: u64, c: u64) -> Result<u64, PTokenError> {
    if c == 0 {
        return Err(Err::ArithmeticOverflow);
    }
    let result = (a as u128)
        .checked_mul(b as u128)
        .ok_or(Err::ArithmeticOverflow)?
        / c as u128;
    if result > u64::MAX as u128 {
        return Err(Err::ArithmeticOverflow);
    }
    Ok(result as u64)
}

/// Saturating add — clamps at u64::MAX instead of overflowing.
pub fn saturating_add(a: u64, b: u64) -> u64 {
    a.saturating_add(b)
}

/// Saturating sub — clamps at 0 instead of underflowing.
pub fn saturating_sub(a: u64, b: u64) -> u64 {
    a.saturating_sub(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_add_ok() {
        assert_eq!(safe_add(100, 200).unwrap(), 300);
    }

    #[test]
    fn test_safe_add_overflow() {
        assert!(safe_add(u64::MAX, 1).is_err());
    }

    #[test]
    fn test_safe_sub_underflow() {
        assert!(safe_sub(0, 1).is_err());
    }

    #[test]
    fn test_muldiv() {
        // 1000 * 50 / 10_000 = 5 (0.5% of 1000)
        assert_eq!(muldiv(1000, 50, 10_000).unwrap(), 5);
    }
}
