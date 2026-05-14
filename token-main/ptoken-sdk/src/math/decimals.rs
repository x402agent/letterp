//! Decimal conversion helpers for human-readable token amounts.

/// Convert a raw token amount (u64) to a UI float amount given decimals.
///
/// # Example
/// ```
/// // 1_000_000 raw units with 6 decimals = 1.0
/// let ui = raw_to_ui(1_000_000, 6);
/// assert_eq!(ui, 1.0f64);
/// ```
pub fn raw_to_ui(amount: u64, decimals: u8) -> f64 {
    amount as f64 / 10f64.powi(decimals as i32)
}

/// Convert a UI float amount to raw token units given decimals.
///
/// # Example
/// ```
/// let raw = ui_to_raw(1.5, 6);
/// assert_eq!(raw, 1_500_000u64);
/// ```
pub fn ui_to_raw(ui_amount: f64, decimals: u8) -> u64 {
    (ui_amount * 10f64.powi(decimals as i32)) as u64
}

/// Return the base-10 multiplier for a given decimal count.
///
/// # Example
/// ```
/// assert_eq!(decimal_multiplier(6), 1_000_000u64);
/// ```
pub fn decimal_multiplier(decimals: u8) -> u64 {
    const POWERS_OF_TEN: [u64; 20] = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
        100_000_000_000,
        1_000_000_000_000,
        10_000_000_000_000,
        100_000_000_000_000,
        1_000_000_000_000_000,
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
    ];

    if decimals < POWERS_OF_TEN.len() as u8 {
        POWERS_OF_TEN[decimals as usize]
    } else {
        u64::MAX
    }
}

/// Format a raw amount as a decimal string with the correct number of decimal places.
///
/// # Example
/// ```
/// assert_eq!(format_amount(1_500_000, 6), "1.500000");
/// ```
pub fn format_amount(amount: u64, decimals: u8) -> String {
    let multiplier = decimal_multiplier(decimals);
    let whole = amount / multiplier;
    let frac = amount % multiplier;
    format!("{}.{:0>width$}", whole, frac, width = decimals as usize)
}

/// Calculate transfer fee in raw units.
///
/// # Arguments
/// * `amount` — Transfer amount in raw units
/// * `basis_points` — Fee rate (1 bp = 0.01%)
/// * `max_fee` — Maximum fee cap in raw units
pub fn calculate_transfer_fee(amount: u64, basis_points: u16, max_fee: u64) -> u64 {
    let uncapped_fee = amount as u128 * basis_points as u128 / 10_000;
    uncapped_fee.min(max_fee as u128).min(u64::MAX as u128) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_to_ui() {
        assert_eq!(raw_to_ui(1_000_000, 6), 1.0);
        assert_eq!(raw_to_ui(1_500_000, 6), 1.5);
    }

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount(1_500_000, 6), "1.500000");
        assert_eq!(format_amount(1_000_000_000, 9), "1.000000000");
    }

    #[test]
    fn test_transfer_fee() {
        // 0.5% of 1000 tokens = 5
        assert_eq!(calculate_transfer_fee(1_000_000, 50, u64::MAX), 5000);
        // Capped at max
        assert_eq!(calculate_transfer_fee(1_000_000, 50, 1000), 1000);
    }
}
