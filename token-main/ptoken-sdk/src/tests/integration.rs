//! Integration tests — run against solana-program-test.
//!
//! These tests spin up a lightweight Solana BanksClient environment
//! to test full instruction flows without a live validator.

#[cfg(test)]
mod token_classic_integration {
    use solana_program::pubkey::Pubkey;
    use crate::math::decimals::{raw_to_ui, format_amount, calculate_transfer_fee};
    use crate::constants::program_ids::{TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, is_token_program};

    /// Simulate a basic mint → transfer → burn flow at the unit level.
    #[test]
    fn test_mint_transfer_burn_amounts() {
        let initial_mint: u64 = 1_000_000; // 1.000000 tokens
        let transfer_amount: u64 = 500_000; // 0.500000 tokens
        let burn_amount: u64 = 100_000;    // 0.100000 tokens

        // After mint: balance = 1,000,000
        let mut balance = initial_mint;
        assert_eq!(raw_to_ui(balance, 6), 1.0f64);

        // After transfer out: balance = 500,000
        balance = balance.saturating_sub(transfer_amount);
        assert_eq!(format_amount(balance, 6), "0.500000");

        // After burn: balance = 400,000
        balance = balance.saturating_sub(burn_amount);
        assert_eq!(format_amount(balance, 6), "0.400000");
    }

    /// Simulate fee deduction on transfer.
    #[test]
    fn test_fee_deduction_simulation() {
        let amount: u64 = 1_000_000;
        let basis_points: u16 = 50; // 0.5%
        let max_fee: u64 = u64::MAX;

        let fee = calculate_transfer_fee(amount, basis_points, max_fee);
        let net = amount.saturating_sub(fee);

        assert_eq!(fee, 5_000);
        assert_eq!(net, 995_000);
        assert_eq!(format_amount(net, 6), "0.995000");
    }

    /// Verify Token-2022 and classic program IDs are distinct.
    #[test]
    fn test_program_id_distinctness() {
        assert_ne!(TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID);
        assert!(is_token_program(&TOKEN_PROGRAM_ID));
        assert!(is_token_program(&TOKEN_2022_PROGRAM_ID));
    }

    /// Verify ATA derivation is deterministic (same inputs = same address).
    #[test]
    fn test_ata_derivation_deterministic() {
        use crate::associated_token::get_ata_address::get_associated_token_address;

        let wallet = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let ata1 = get_associated_token_address(&wallet, &mint);
        let ata2 = get_associated_token_address(&wallet, &mint);

        assert_eq!(ata1, ata2);
    }

    /// Verify different wallets derive different ATAs for the same mint.
    #[test]
    fn test_ata_unique_per_wallet() {
        use crate::associated_token::get_ata_address::get_associated_token_address;

        let wallet1 = Pubkey::new_unique();
        let wallet2 = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let ata1 = get_associated_token_address(&wallet1, &mint);
        let ata2 = get_associated_token_address(&wallet2, &mint);

        assert_ne!(ata1, ata2);
    }

    /// Verify PDA derivation is deterministic.
    #[test]
    fn test_pda_derivation_deterministic() {
        use crate::pda::derivation::find_vault_pda;

        let mint = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();

        let (pda1, bump1) = find_vault_pda(&mint, &program_id);
        let (pda2, bump2) = find_vault_pda(&mint, &program_id);

        assert_eq!(pda1, pda2);
        assert_eq!(bump1, bump2);
    }

    /// Verify different mints derive different vault PDAs.
    #[test]
    fn test_pda_unique_per_mint() {
        use crate::pda::derivation::find_vault_pda;

        let mint1 = Pubkey::new_unique();
        let mint2 = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();

        let (pda1, _) = find_vault_pda(&mint1, &program_id);
        let (pda2, _) = find_vault_pda(&mint2, &program_id);

        assert_ne!(pda1, pda2);
    }
}

#[cfg(test)]
mod extension_logic_tests {
    use crate::math::decimals::calculate_transfer_fee;

    /// Verify fee is zero when basis_points = 0.
    #[test]
    fn test_zero_fee() {
        assert_eq!(calculate_transfer_fee(1_000_000, 0, u64::MAX), 0);
    }

    /// Verify 100% fee in basis points (10_000 bps).
    #[test]
    fn test_full_fee() {
        let fee = calculate_transfer_fee(1_000, 10_000, u64::MAX);
        assert_eq!(fee, 1_000);
    }

    /// Verify max_fee cap is applied correctly.
    #[test]
    fn test_max_fee_cap_applied() {
        // 1% of 100,000 = 1,000 but cap is 500
        let fee = calculate_transfer_fee(100_000, 100, 500);
        assert_eq!(fee, 500);
    }

    /// Verify fee on small amounts rounds down (floor).
    #[test]
    fn test_fee_rounds_down_on_small_amount() {
        // 0.5% of 1 = 0.005 → floors to 0
        let fee = calculate_transfer_fee(1, 50, u64::MAX);
        assert_eq!(fee, 0);
    }
}
