//! Unit tests — pure Rust, no Solana runtime required.

#[cfg(test)]
mod math_tests {
    use crate::math::{
        checked_arithmetic::*,
        decimals::*,
        u64_helpers::*,
    };

    // ── checked_arithmetic ──────────────────────────────────────────────────

    #[test]
    fn test_safe_add_basic() {
        assert_eq!(safe_add(100, 200).unwrap(), 300);
    }

    #[test]
    fn test_safe_add_overflow_returns_err() {
        assert!(safe_add(u64::MAX, 1).is_err());
    }

    #[test]
    fn test_safe_sub_basic() {
        assert_eq!(safe_sub(500, 200).unwrap(), 300);
    }

    #[test]
    fn test_safe_sub_underflow_returns_err() {
        assert!(safe_sub(0, 1).is_err());
    }

    #[test]
    fn test_safe_mul_basic() {
        assert_eq!(safe_mul(100, 100).unwrap(), 10_000);
    }

    #[test]
    fn test_safe_mul_overflow_returns_err() {
        assert!(safe_mul(u64::MAX, 2).is_err());
    }

    #[test]
    fn test_safe_div_basic() {
        assert_eq!(safe_div(1_000, 10).unwrap(), 100);
    }

    #[test]
    fn test_safe_div_by_zero_returns_err() {
        assert!(safe_div(100, 0).is_err());
    }

    #[test]
    fn test_muldiv_fee_calculation() {
        // 0.5% of 1,000,000 = 5,000
        assert_eq!(muldiv(1_000_000, 50, 10_000).unwrap(), 5_000);
    }

    #[test]
    fn test_muldiv_exact_percent() {
        // 1% of 1,000,000 = 10,000
        assert_eq!(muldiv(1_000_000, 100, 10_000).unwrap(), 10_000);
    }

    #[test]
    fn test_saturating_add_clamps() {
        assert_eq!(saturating_add(u64::MAX, 1), u64::MAX);
    }

    #[test]
    fn test_saturating_sub_clamps() {
        assert_eq!(saturating_sub(0, 100), 0);
    }

    // ── decimals ────────────────────────────────────────────────────────────

    #[test]
    fn test_raw_to_ui_6_decimals() {
        assert_eq!(raw_to_ui(1_000_000, 6), 1.0f64);
    }

    #[test]
    fn test_raw_to_ui_9_decimals() {
        assert_eq!(raw_to_ui(1_000_000_000, 9), 1.0f64);
    }

    #[test]
    fn test_ui_to_raw_round_trip() {
        let original: u64 = 1_500_000;
        let ui = raw_to_ui(original, 6);
        let back = ui_to_raw(ui, 6);
        assert_eq!(back, original);
    }

    #[test]
    fn test_format_amount_6_decimals() {
        assert_eq!(format_amount(1_500_000, 6), "1.500000");
    }

    #[test]
    fn test_format_amount_zero() {
        assert_eq!(format_amount(0, 6), "0.000000");
    }

    #[test]
    fn test_calculate_transfer_fee_half_percent() {
        // 0.5% of 1,000,000 = 5,000
        assert_eq!(calculate_transfer_fee(1_000_000, 50, u64::MAX), 5_000);
    }

    #[test]
    fn test_calculate_transfer_fee_capped() {
        // Without cap: 5,000; with cap 1,000 => 1,000
        assert_eq!(calculate_transfer_fee(1_000_000, 50, 1_000), 1_000);
    }

    #[test]
    fn test_decimal_multiplier() {
        assert_eq!(decimal_multiplier(0), 1);
        assert_eq!(decimal_multiplier(6), 1_000_000);
        assert_eq!(decimal_multiplier(9), 1_000_000_000);
    }

    // ── u64_helpers ─────────────────────────────────────────────────────────

    #[test]
    fn test_bps_to_percent() {
        assert!((bps_to_percent(50) - 0.005f64).abs() < f64::EPSILON);
        assert!((bps_to_percent(10_000) - 1.0f64).abs() < f64::EPSILON);
    }

    #[test]
    fn test_apply_bps_fee_one_percent() {
        let (net, fee) = apply_bps_fee(10_000, 100, u64::MAX);
        assert_eq!(fee, 100);
        assert_eq!(net, 9_900);
    }

    #[test]
    fn test_apply_bps_fee_with_cap() {
        let (net, fee) = apply_bps_fee(1_000_000, 100, 50);
        assert_eq!(fee, 50);
        assert_eq!(net, 999_950);
    }

    #[test]
    fn test_ceil_div() {
        assert_eq!(ceil_div(10, 3), 4);
        assert_eq!(ceil_div(9, 3), 3);
        assert_eq!(ceil_div(1, 2), 1);
        assert_eq!(ceil_div(0, 5), 0);
    }

    #[test]
    fn test_in_range() {
        assert!(in_range(5, 1, 10));
        assert!(in_range(1, 1, 10));
        assert!(in_range(10, 1, 10));
        assert!(!in_range(0, 1, 10));
        assert!(!in_range(11, 1, 10));
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 1, 10), 5);
        assert_eq!(clamp(0, 1, 10), 1);
        assert_eq!(clamp(20, 1, 10), 10);
    }

    #[test]
    fn test_would_exceed_max() {
        assert!(would_exceed_max(90, 20, 100));
        assert!(!would_exceed_max(80, 20, 100));
    }
}

#[cfg(test)]
mod serialization_tests {
    use crate::pinocchio_core::instruction_data::*;

    #[test]
    fn test_read_u64_round_trip() {
        let amount: u64 = 999_999_999;
        let bytes = amount.to_le_bytes();
        assert_eq!(read_u64(&bytes, 0).unwrap(), amount);
    }

    #[test]
    fn test_read_u16_little_endian() {
        let val: u16 = 1_000;
        let bytes = val.to_le_bytes();
        assert_eq!(read_u16(&bytes, 0).unwrap(), 1_000);
    }

    #[test]
    fn test_discriminant_extraction() {
        let data = [5u8, 0x00, 0x01, 0x02];
        assert_eq!(discriminant(&data).unwrap(), 5);
    }

    #[test]
    fn test_require_len_passes() {
        assert!(require_len(&[0u8; 10], 10).is_ok());
        assert!(require_len(&[0u8; 10], 5).is_ok());
    }

    #[test]
    fn test_require_len_fails() {
        assert!(require_len(&[0u8; 4], 8).is_err());
    }

    #[test]
    fn test_read_u8_basic() {
        let data = [42u8, 0, 0];
        assert_eq!(read_u8(&data, 0).unwrap(), 42);
    }

    #[test]
    fn test_read_out_of_bounds_returns_err() {
        let data = [1u8, 2u8];
        assert!(read_u64(&data, 0).is_err());
    }
}

#[cfg(test)]
mod constants_tests {
    use crate::constants::{program_ids::*, seeds::*};
    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;

    #[test]
    fn test_token_program_id_correct() {
        let expected = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        assert_eq!(TOKEN_PROGRAM_ID, expected);
    }

    #[test]
    fn test_token_2022_program_id_correct() {
        let expected = Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").unwrap();
        assert_eq!(TOKEN_2022_PROGRAM_ID, expected);
    }

    #[test]
    fn test_ata_program_id_correct() {
        let expected = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJe1bN").unwrap();
        assert_eq!(ATA_PROGRAM_ID, expected);
    }

    #[test]
    fn test_is_token_program_classic() {
        assert!(is_token_program(&TOKEN_PROGRAM_ID));
    }

    #[test]
    fn test_is_token_program_2022() {
        assert!(is_token_program(&TOKEN_2022_PROGRAM_ID));
    }

    #[test]
    fn test_is_token_program_false_for_system() {
        assert!(!is_token_program(&SYSTEM_PROGRAM_ID));
    }

    #[test]
    fn test_seeds_are_valid_utf8() {
        assert!(std::str::from_utf8(MINT_SEED).is_ok());
        assert!(std::str::from_utf8(VAULT_SEED).is_ok());
        assert!(std::str::from_utf8(METADATA_SEED).is_ok());
        assert!(std::str::from_utf8(AUTHORITY_SEED).is_ok());
    }
}

#[cfg(test)]
mod zero_copy_tests {
    use crate::pinocchio_core::zero_copy_layout::*;

    #[test]
    fn test_read_u64_at_correct_offset() {
        let mut data = vec![0u8; 100];
        let value: u64 = 42_000_000;
        data[ACCOUNT_AMOUNT_OFFSET..ACCOUNT_AMOUNT_OFFSET + 8]
            .copy_from_slice(&value.to_le_bytes());
        assert_eq!(read_token_amount(&data).unwrap(), 42_000_000);
    }

    #[test]
    fn test_mint_not_initialized_when_zero() {
        let data = vec![0u8; MINT_LEN + 10];
        assert!(!is_mint_initialized(&data));
    }

    #[test]
    fn test_mint_initialized_when_flag_set() {
        let mut data = vec![0u8; MINT_LEN + 10];
        data[MINT_INITIALIZED_OFFSET] = 1;
        assert!(is_mint_initialized(&data));
    }

    #[test]
    fn test_read_mint_decimals() {
        let mut data = vec![0u8; MINT_LEN];
        data[MINT_DECIMALS_OFFSET] = 9;
        assert_eq!(read_mint_decimals(&data).unwrap(), 9);
    }
}
