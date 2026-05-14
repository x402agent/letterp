//! Kani proof harnesses for arithmetic invariants used by token programs.

use crate::agent::{AgentCapabilityFlags, AgentPolicy};
use crate::bonding_curve::{
    constant_product_buy_quote, constant_product_sell_quote, LinearBondingCurve,
};
use crate::math::{
    apply_bps_fee, calculate_transfer_fee, ceil_div, decimal_multiplier, in_range, muldiv,
    safe_add, safe_div, safe_mul, safe_sub, saturating_add, saturating_sub, would_exceed_max,
};
use crate::perpetuals::{funding_payment, Position, PositionSide, PRICE_SCALE};
use crate::x402::{
    gateway_fee, verify_receipt, PaymentAsset, X402PaymentIntent, X402Receipt, X402SettlementStatus,
};

#[kani::proof]
fn safe_arithmetic_matches_checked_operations() {
    let a: u64 = kani::any();
    let b: u64 = kani::any();

    let add = safe_add(a, b);
    kani::cover!(add.is_ok(), "safe_add success path is reachable");
    kani::cover!(add.is_err(), "safe_add overflow path is reachable");
    assert_eq!(add.ok(), a.checked_add(b));

    let sub = safe_sub(a, b);
    kani::cover!(sub.is_ok(), "safe_sub success path is reachable");
    kani::cover!(sub.is_err(), "safe_sub underflow path is reachable");
    assert_eq!(sub.ok(), a.checked_sub(b));

    let overflow_mul: bool = kani::any();
    let mul_a = if overflow_mul {
        u64::MAX
    } else {
        u16::MAX as u64
    };
    let mul_b = if overflow_mul {
        2
    } else {
        kani::any::<u16>() as u64
    };
    let mul = safe_mul(mul_a, mul_b);
    kani::cover!(mul.is_ok(), "safe_mul success path is reachable");
    kani::cover!(mul.is_err(), "safe_mul overflow path is reachable");
    assert_eq!(mul.ok(), mul_a.checked_mul(mul_b));
}

#[kani::proof]
fn division_helpers_are_total_and_precise() {
    let case: u8 = kani::any();
    let case = case % 4;
    let (a, b) = match case {
        0 => (10, 0),
        1 => (9, 3),
        2 => (10, 3),
        _ => (0, 5),
    };
    let (a, b) = (a as u64, b as u64);

    let div = safe_div(a, b);
    kani::cover!(div.is_ok(), "safe_div nonzero divisor path is reachable");
    kani::cover!(div.is_err(), "safe_div zero divisor path is reachable");
    if b == 0 {
        assert!(div.is_err());
    } else {
        assert_eq!(div.unwrap(), a / b);
    }

    let ceil = ceil_div(a, b);
    kani::cover!(a == 0, "ceil_div zero dividend path is reachable");
    kani::cover!(b == 0, "ceil_div zero divisor path is reachable");
    kani::cover!(
        a > 0 && b > 0 && a % b == 0,
        "ceil_div exact path is reachable"
    );
    kani::cover!(
        a > 0 && b > 0 && a % b != 0,
        "ceil_div rounded path is reachable"
    );
    if a == 0 || b == 0 {
        assert_eq!(ceil, 0);
    } else {
        assert_eq!(ceil, 1 + (a - 1) / b);
        assert!(ceil >= a / b);
    }

    let muldiv_case: u8 = kani::any();
    let muldiv_case = muldiv_case % 3;
    let (md_a, md_b, md_c) = match muldiv_case {
        0 => (1_000, 50, 10_000),
        1 => (1, 1, 0),
        _ => (u64::MAX, 2, 1),
    };
    let md = muldiv(md_a, md_b, md_c);
    kani::cover!(md.is_ok(), "muldiv success path is reachable");
    kani::cover!(md.is_err(), "muldiv error path is reachable");
    match muldiv_case {
        0 => assert_eq!(md.unwrap(), 5),
        1 | 2 => assert!(md.is_err()),
        _ => unreachable!(),
    }
}

#[kani::proof]
fn standard_fee_calculations_do_not_truncate_or_exceed_caps() {
    let amount: u16 = kani::any();
    let max_fee: u16 = kani::any();
    let amount = amount as u64;
    let max_fee = max_fee as u64;
    let bps = 10_000u16;

    let fee = calculate_transfer_fee(amount, bps, max_fee);
    let expected = (amount as u128 * bps as u128 / 10_000)
        .min(max_fee as u128)
        .min(u64::MAX as u128) as u64;

    kani::cover!(fee == 0, "zero fee path is reachable");
    kani::cover!(fee == max_fee, "capped fee path is reachable");
    kani::cover!(fee < max_fee, "uncapped fee path is reachable");
    assert_eq!(fee, expected);
    assert!(fee <= max_fee);
    assert!(fee <= amount);

    let (net, applied_fee) = apply_bps_fee(amount, bps, max_fee);
    kani::cover!(applied_fee == amount, "full amount fee path is reachable");
    kani::cover!(applied_fee <= amount, "fee within amount path is reachable");
    assert_eq!(applied_fee, expected);
    assert_eq!(net, amount - applied_fee);
    assert!(net <= amount);
}

#[kani::proof]
fn fee_calculations_cover_u64_extremes() {
    assert_eq!(calculate_transfer_fee(u64::MAX, 10_000, u64::MAX), u64::MAX);
    assert_eq!(apply_bps_fee(u64::MAX, 10_000, u64::MAX), (0, u64::MAX));
    assert_eq!(
        calculate_transfer_fee(u64::MAX, u16::MAX, u64::MAX),
        u64::MAX
    );
    assert_eq!(apply_bps_fee(u64::MAX, u16::MAX, u64::MAX), (0, u64::MAX));
    assert_eq!(calculate_transfer_fee(u64::MAX, u16::MAX, 42), 42);
    assert_eq!(apply_bps_fee(u64::MAX, u16::MAX, 42), (u64::MAX - 42, 42));
}

#[kani::proof]
fn extended_fee_calculations_cover_high_bps_edges() {
    let amount: u16 = kani::any();
    let max_fee: u16 = kani::any();
    let amount = amount as u64;
    let max_fee = max_fee as u64;
    let bps = 20_000u16;

    let fee = calculate_transfer_fee(amount, bps, max_fee);
    let expected = (amount as u128 * bps as u128 / 10_000)
        .min(max_fee as u128)
        .min(u64::MAX as u128) as u64;

    kani::cover!(
        fee > amount,
        "high bps fee greater than amount path is reachable"
    );
    kani::cover!(fee == max_fee, "high bps capped path is reachable");
    kani::cover!(fee < max_fee, "high bps uncapped path is reachable");
    assert_eq!(fee, expected);
    assert!(fee <= max_fee);

    let (net, applied_fee) = apply_bps_fee(amount, bps, max_fee);
    assert_eq!(applied_fee, expected);
    assert_eq!(net, amount.saturating_sub(applied_fee));
}

#[kani::proof]
fn bounded_u64_helpers_match_mathematical_intent() {
    let current: u64 = kani::any();
    let delta: u64 = kani::any();
    let max: u64 = kani::any();
    let value: u64 = kani::any();
    let min: u64 = kani::any();

    let exceeds = would_exceed_max(current, delta, max);
    let mathematical_exceeds = current as u128 + delta as u128 > max as u128;
    kani::cover!(exceeds, "would_exceed_max true path is reachable");
    kani::cover!(!exceeds, "would_exceed_max false path is reachable");
    assert_eq!(exceeds, mathematical_exceeds);

    assert_eq!(
        saturating_add(current, delta),
        current.saturating_add(delta)
    );
    assert_eq!(
        saturating_sub(current, delta),
        current.saturating_sub(delta)
    );

    kani::cover!(
        min <= max && in_range(value, min, max),
        "in_range true path is reachable"
    );
    kani::cover!(
        min <= max && !in_range(value, min, max),
        "in_range false path is reachable"
    );
    if min <= max {
        assert_eq!(in_range(value, min, max), value >= min && value <= max);
    }
}

#[kani::proof]
fn decimal_multiplier_is_total_and_monotonic_until_saturation() {
    let decimals: u8 = kani::any();
    let next_decimals = decimals.saturating_add(1);

    let multiplier = decimal_multiplier(decimals);
    let next_multiplier = decimal_multiplier(next_decimals);

    kani::cover!(
        decimals <= 19,
        "exact decimal multiplier range is reachable"
    );
    kani::cover!(
        decimals > 19,
        "saturated decimal multiplier range is reachable"
    );
    assert!(multiplier >= 1);

    if decimals < u8::MAX {
        assert!(next_multiplier >= multiplier);
    }

    if decimals <= 19 {
        let expected = match decimals {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1_000,
            4 => 10_000,
            5 => 100_000,
            6 => 1_000_000,
            7 => 10_000_000,
            8 => 100_000_000,
            9 => 1_000_000_000,
            10 => 10_000_000_000,
            11 => 100_000_000_000,
            12 => 1_000_000_000_000,
            13 => 10_000_000_000_000,
            14 => 100_000_000_000_000,
            15 => 1_000_000_000_000_000,
            16 => 10_000_000_000_000_000,
            17 => 100_000_000_000_000_000,
            18 => 1_000_000_000_000_000_000,
            19 => 10_000_000_000_000_000_000,
            _ => unreachable!(),
        };
        assert_eq!(multiplier, expected);
    } else {
        assert_eq!(multiplier, u64::MAX);
    }
}

#[kani::proof]
fn agent_policy_rejects_vacuous_trading_authority() {
    let risk_limit_bps: u16 = kani::any();
    let grants_trading: bool = kani::any();
    let spending_limit_lamports = if grants_trading { 0 } else { 1 };
    let capabilities = if grants_trading {
        AgentCapabilityFlags::BONDING_CURVE_TRADING
    } else {
        AgentCapabilityFlags::X402_SETTLEMENT
    };
    let policy = AgentPolicy {
        agent_id: [1; 32],
        owner: [2; 32],
        capabilities,
        spending_limit_lamports,
        risk_limit_bps,
    };

    let valid = policy.validate().is_ok();
    kani::cover!(valid, "valid non-trading agent policy is reachable");
    kani::cover!(
        !valid,
        "invalid trading policy without spend limit is reachable"
    );

    if risk_limit_bps > 10_000 || grants_trading {
        assert!(!valid);
    }
}

#[kani::proof]
fn x402_receipts_only_unlock_matching_unexpired_payments() {
    let accepted: bool = kani::any();
    let expired: bool = kani::any();
    let underpaid: bool = kani::any();
    let amount_paid = if underpaid { 999 } else { 1_000 };
    let now_unix = if expired { 101 } else { 100 };
    let intent = X402PaymentIntent {
        asset: PaymentAsset::Sol,
        amount_due: 1_000,
        pay_to: [7; 32],
        route_hash: [9; 32],
        expires_at_unix: 100,
    };
    let receipt = X402Receipt {
        asset: PaymentAsset::Sol,
        amount_paid,
        paid_to: [7; 32],
        route_hash: [9; 32],
        status: if accepted {
            X402SettlementStatus::Accepted
        } else {
            X402SettlementStatus::Rejected
        },
    };

    let verified = verify_receipt(&intent, &receipt, now_unix).is_ok();
    kani::cover!(verified, "matching x402 receipt unlock path is reachable");
    kani::cover!(!verified, "rejected x402 receipt path is reachable");
    assert_eq!(verified, accepted && !expired && !underpaid);

    let fee = gateway_fee(10_000, 25, 1).unwrap();
    assert_eq!(fee, 25);
}

#[kani::proof]
fn bonding_curve_quotes_are_monotonic_for_bounded_inputs() {
    let case: u8 = kani::any();
    let case = case % 4;
    let curve = LinearBondingCurve {
        base_price: 10,
        slope_numerator: if case == 0 { 0 } else { 2 },
        slope_denominator: 1,
    };
    let supply = if case == 3 { 20 } else { 1 };
    let tokens = if case == 2 { 0 } else { 3 };

    let price_now = curve.price_at_supply(supply).unwrap();
    let price_next = curve.price_at_supply(supply + 1).unwrap();
    kani::cover!(case == 0, "flat linear curve path is reachable");
    kani::cover!(case != 0, "increasing linear curve path is reachable");
    assert!(price_next >= price_now);

    let quote = curve.buy_quote(supply, tokens).unwrap();
    kani::cover!(case == 2, "zero-token buy quote path is reachable");
    kani::cover!(case != 2, "nonzero-token buy quote path is reachable");
    if tokens == 0 {
        assert_eq!(quote, 0);
    }

    let buy = constant_product_buy_quote(1_000, 1_000, 10, 30).unwrap();
    let sell = constant_product_sell_quote(1_000, 1_000, 10, 30).unwrap();
    assert!(buy > 0);
    assert!(sell > 0);
}

#[kani::proof]
fn perpetual_position_math_preserves_side_direction() {
    let side_long: bool = kani::any();
    let mark_up: bool = kani::any();
    let position = Position {
        side: if side_long {
            PositionSide::Long
        } else {
            PositionSide::Short
        },
        collateral: 100,
        notional: 500,
        entry_price: PRICE_SCALE,
    };
    let mark_price = if mark_up {
        PRICE_SCALE + PRICE_SCALE / 10
    } else {
        PRICE_SCALE - PRICE_SCALE / 10
    };
    let pnl = position.unrealized_pnl(mark_price).unwrap();

    kani::cover!(pnl > 0, "profitable perpetual position path is reachable");
    kani::cover!(pnl < 0, "losing perpetual position path is reachable");
    assert_eq!(pnl > 0, side_long == mark_up);
    assert_eq!(position.leverage_bps().unwrap(), 50_000);
    assert_eq!(funding_payment(10_000, 25).unwrap(), 25);
    assert_eq!(funding_payment(10_000, -25).unwrap(), -25);
}
