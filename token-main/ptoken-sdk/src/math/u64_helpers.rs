//! u64 utility functions common in Solana token programs.

/// Maximum possible token supply (u64::MAX).
pub const MAX_SUPPLY: u64 = u64::MAX;

/// Convert basis points to a percentage float.
///
/// # Example
/// ```
/// assert_eq!(bps_to_percent(50), 0.005f64);
/// ```
pub fn bps_to_percent(bps: u16) -> f64 {
    bps as f64 / 10_000.0
}

/// Apply basis points fee to an amount, returning (net_amount, fee).
pub fn apply_bps_fee(amount: u64, bps: u16, max_fee: u64) -> (u64, u64) {
    let uncapped_fee = amount as u128 * bps as u128 / 10_000;
    let fee = uncapped_fee.min(max_fee as u128).min(u64::MAX as u128) as u64;
    let net = amount.saturating_sub(fee);
    (net, fee)
}

/// Check if adding `delta` to `current` would exceed `max`.
pub fn would_exceed_max(current: u64, delta: u64, max: u64) -> bool {
    current > max || delta > max - current
}

/// Round up integer division: ceil(a / b).
pub fn ceil_div(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    1 + (a - 1) / b
}

/// Returns true if the value is within [min, max] inclusive.
pub fn in_range(value: u64, min: u64, max: u64) -> bool {
    value >= min && value <= max
}

/// Clamp value to [min, max].
pub fn clamp(value: u64, min: u64, max: u64) -> u64 {
    value.max(min).min(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_bps_fee() {
        let (net, fee) = apply_bps_fee(10_000, 100, u64::MAX); // 1%
        assert_eq!(fee, 100);
        assert_eq!(net, 9_900);
    }

    #[test]
    fn test_ceil_div() {
        assert_eq!(ceil_div(10, 3), 4);
        assert_eq!(ceil_div(9, 3), 3);
    }
}
