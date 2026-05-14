use core::mem::size_of;

use crate::{
    instructions::{Buy, Sell},
    state::{AgentState, CurveState},
};

#[kani::proof]
fn agent_flags_are_one_way_and_independent() {
    let flags: u8 = kani::any();

    let bound = AgentState::with_flag(flags, AgentState::FLAG_BOUND);
    let graduated = AgentState::with_flag(flags, AgentState::FLAG_GRADUATED);
    let both = AgentState::with_flag(bound, AgentState::FLAG_GRADUATED);

    kani::cover!(
        !AgentState::has_flag(flags, AgentState::FLAG_BOUND),
        "initial unbound path is reachable"
    );
    kani::cover!(
        AgentState::has_flag(flags, AgentState::FLAG_BOUND),
        "initial bound path is reachable"
    );
    kani::cover!(
        !AgentState::has_flag(flags, AgentState::FLAG_GRADUATED),
        "initial ungraduated path is reachable"
    );
    kani::cover!(
        AgentState::has_flag(flags, AgentState::FLAG_GRADUATED),
        "initial graduated path is reachable"
    );

    assert!(AgentState::has_flag(bound, AgentState::FLAG_BOUND));
    assert_eq!(
        AgentState::has_flag(bound, AgentState::FLAG_GRADUATED),
        AgentState::has_flag(flags, AgentState::FLAG_GRADUATED)
    );
    assert!(AgentState::has_flag(graduated, AgentState::FLAG_GRADUATED));
    assert_eq!(
        AgentState::has_flag(graduated, AgentState::FLAG_BOUND),
        AgentState::has_flag(flags, AgentState::FLAG_BOUND)
    );
    assert!(AgentState::has_flag(both, AgentState::FLAG_BOUND));
    assert!(AgentState::has_flag(both, AgentState::FLAG_GRADUATED));
    assert_eq!(AgentState::with_flag(bound, AgentState::FLAG_BOUND), bound);
}

#[kani::proof]
fn state_layout_lengths_match_struct_sizes() {
    kani::cover!(AgentState::LEN > 0, "agent state length is nonzero");
    kani::cover!(
        CurveState::LEN < AgentState::LEN,
        "curve state is smaller than agent state"
    );

    assert_eq!(AgentState::LEN, size_of::<AgentState>());
    assert_eq!(CurveState::LEN, size_of::<CurveState>());
    assert!(AgentState::FLAG_BOUND != AgentState::FLAG_GRADUATED);
    assert_eq!(AgentState::FLAG_BOUND & AgentState::FLAG_GRADUATED, 0);
}

#[kani::proof]
fn buy_data_accepts_exactly_nonzero_u64_payloads() {
    let amount: u16 = kani::any();
    let amount = amount as u64;
    let bytes = amount.to_le_bytes();

    let parsed = crate::instructions::buy::BuyData::try_from(&bytes[..]);
    kani::cover!(parsed.is_ok(), "buy parser accepts nonzero u64 payload");
    kani::cover!(parsed.is_err(), "buy parser rejects zero u64 payload");

    if amount == 0 {
        assert!(parsed.is_err());
    } else {
        assert_eq!(parsed.unwrap().lamports_in, amount);
    }

    assert!(crate::instructions::buy::BuyData::try_from(&bytes[..7]).is_err());
}

#[kani::proof]
fn sell_data_accepts_exactly_nonzero_u64_payloads() {
    let amount: u64 = kani::any();
    let bytes = amount.to_le_bytes();

    let parsed = crate::instructions::sell::SellData::try_from(&bytes[..]);
    kani::cover!(parsed.is_ok(), "sell parser accepts nonzero u64 payload");
    kani::cover!(parsed.is_err(), "sell parser rejects zero u64 payload");

    if amount == 0 {
        assert!(parsed.is_err());
    } else {
        assert_eq!(parsed.unwrap().tokens_in, amount);
    }

    assert!(crate::instructions::sell::SellData::try_from(&bytes[..7]).is_err());
}

#[kani::proof]
fn curve_decoding_and_fee_math_are_total() {
    let amount_bytes: [u8; 8] = kani::any();
    let bps_bytes: [u8; 2] = kani::any();
    let amount: u8 = kani::any();
    let amount = amount as u64;
    let high_bps: bool = kani::any();
    let bps_offset: u8 = kani::any();
    let bps = if high_bps {
        CurveState::DEFAULT_DENOMINATOR_BPS + 1 + bps_offset as u16
    } else {
        bps_offset as u16
    };

    kani::cover!(
        bps <= CurveState::DEFAULT_DENOMINATOR_BPS,
        "standard bps path is reachable"
    );
    kani::cover!(
        bps > CurveState::DEFAULT_DENOMINATOR_BPS,
        "high bps path is reachable"
    );
    kani::cover!(amount == 0, "zero amount fee path is reachable");
    kani::cover!(amount > 0, "nonzero amount fee path is reachable");

    assert_eq!(
        CurveState::amount_from_le_bytes(amount_bytes),
        u64::from_le_bytes(amount_bytes)
    );
    assert_eq!(
        CurveState::bps_from_le_bytes(bps_bytes),
        u16::from_le_bytes(bps_bytes)
    );

    let fee = CurveState::apply_bps(amount, bps);
    let expected = ((amount as u128 * bps as u128) / CurveState::DEFAULT_DENOMINATOR_BPS as u128)
        .min(u64::MAX as u128) as u64;
    assert_eq!(fee, expected);
    assert_eq!(
        CurveState::net_after_bps_fee(amount, bps),
        (amount.saturating_sub(fee), fee)
    );
    if bps <= CurveState::DEFAULT_DENOMINATOR_BPS {
        assert!(fee <= amount);
    }
}

#[kani::proof]
fn curve_fee_math_handles_u64_extremes() {
    assert_eq!(CurveState::apply_bps(u64::MAX, 0), 0);
    assert_eq!(
        CurveState::apply_bps(u64::MAX, CurveState::DEFAULT_DENOMINATOR_BPS),
        u64::MAX
    );
    assert_eq!(CurveState::apply_bps(u64::MAX, u16::MAX), u64::MAX);
    assert_eq!(
        CurveState::net_after_bps_fee(u64::MAX, CurveState::DEFAULT_DENOMINATOR_BPS),
        (0, u64::MAX)
    );
}

#[kani::proof]
fn constant_product_quotes_are_bounded_and_total() {
    let virtual_sol: u8 = kani::any();
    let virtual_token: u8 = kani::any();
    let net_sol_in: u8 = kani::any();
    let tokens_in: u8 = kani::any();
    kani::assume(virtual_sol <= 15);
    kani::assume(virtual_token <= 15);
    kani::assume(net_sol_in <= 15);
    kani::assume(tokens_in <= 15);
    let virtual_sol = virtual_sol as u64;
    let virtual_token = virtual_token as u64;
    let net_sol_in = net_sol_in as u64;
    let tokens_in = tokens_in as u64;

    let buy = CurveState::buy_tokens_out(virtual_sol, virtual_token, net_sol_in);
    kani::cover!(buy.is_none(), "buy quote undefined path is reachable");
    kani::cover!(buy.is_some(), "buy quote defined path is reachable");
    kani::cover!(
        matches!(buy, Some(out) if out == 0),
        "buy quote zero-output path is reachable"
    );
    kani::cover!(
        matches!(buy, Some(out) if out > 0),
        "buy quote positive-output path is reachable"
    );
    if virtual_sol == 0 && net_sol_in == 0 {
        assert!(buy.is_none());
    } else {
        let out = buy.unwrap();
        let denominator = virtual_sol as u128 + net_sol_in as u128;
        let invariant = virtual_sol as u128 * virtual_token as u128;
        let expected = virtual_token.saturating_sub((invariant / denominator) as u64);
        assert_eq!(out, expected);
        assert!(out <= virtual_token);
    }

    let sell = CurveState::sell_sol_out(virtual_sol, virtual_token, tokens_in);
    kani::cover!(sell.is_none(), "sell quote undefined path is reachable");
    kani::cover!(sell.is_some(), "sell quote defined path is reachable");
    kani::cover!(
        matches!(sell, Some(out) if out == 0),
        "sell quote zero-output path is reachable"
    );
    kani::cover!(
        matches!(sell, Some(out) if out > 0),
        "sell quote positive-output path is reachable"
    );
    if virtual_token == 0 && tokens_in == 0 {
        assert!(sell.is_none());
    } else {
        let out = sell.unwrap();
        let denominator = virtual_token as u128 + tokens_in as u128;
        let invariant = virtual_sol as u128 * virtual_token as u128;
        let expected = virtual_sol.saturating_sub((invariant / denominator) as u64);
        assert_eq!(out, expected);
        assert!(out <= virtual_sol);
    }
}

#[kani::proof]
fn instruction_discriminators_are_unique() {
    let discriminators = [
        crate::instructions::InitializeAgent::DISCRIMINATOR,
        crate::instructions::InitializeAgentMint::DISCRIMINATOR,
        crate::instructions::BindAgentToken::DISCRIMINATOR,
        crate::instructions::DelegateExecutor::DISCRIMINATOR,
        Buy::DISCRIMINATOR,
        Sell::DISCRIMINATOR,
        crate::instructions::Graduate::DISCRIMINATOR,
    ];

    kani::cover!(
        discriminators[0] == 0,
        "initialize agent discriminator is reachable"
    );
    kani::cover!(
        discriminators[6] == 6,
        "graduate discriminator is reachable"
    );

    assert_eq!(discriminators[0], 0);
    assert_eq!(discriminators[1], 1);
    assert_eq!(discriminators[2], 2);
    assert_eq!(discriminators[3], 3);
    assert_eq!(discriminators[4], 4);
    assert_eq!(discriminators[5], 5);
    assert_eq!(discriminators[6], 6);
    assert!(discriminators[0] != discriminators[1]);
    assert!(discriminators[1] != discriminators[2]);
    assert!(discriminators[2] != discriminators[3]);
    assert!(discriminators[3] != discriminators[4]);
    assert!(discriminators[4] != discriminators[5]);
    assert!(discriminators[5] != discriminators[6]);
}
