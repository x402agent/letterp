//! Zero-copy on-chain state layouts.
//!
//! All structs are `repr(C)`. Alignment padding is made explicit via `_pad_*`
//! fields so that `size_of::<T>()` matches the documented byte layout.

// ---------------------------------------------------------------------------
// AgentState — 200 bytes total (repr(C) with explicit alignment padding)
// ---------------------------------------------------------------------------
//
//  offset 0  : u8        discriminant   (0=uninit, 1=init, 2=bound)
//  offset 1  : [u8;32]   owner          (creator / authority pubkey)
//  offset 33 : [u8;32]   core_asset     (MPL Core asset address, zero if unset)
//  offset 65 : [u8;32]   mint           (token mint, zero until bound)
//  offset 97 : [u8;32]   executive      (delegate wallet, zero if none)
//  offset 129: [u8;32]   creator_vault  (creator fee vault PDA)
//  offset 161: [u8;7]    _pad_align     (alignment pad before u64)
//  offset 168: u64       created_at     (unix timestamp)
//  offset 176: u8        graduated      (0/1)
//  offset 177: u8        bump           (agent PDA bump)
//  offset 178: [u8;22]   _pad           (tail pad to 200 bytes)

/// Agent state account.
#[repr(C)]
pub struct AgentState {
    pub discriminant:  u8,
    pub owner:         [u8; 32],
    pub core_asset:    [u8; 32],
    pub mint:          [u8; 32],
    pub executive:     [u8; 32],
    pub creator_vault: [u8; 32],
    pub _pad_align:    [u8; 7],   // explicit alignment pad before u64
    pub created_at:    u64,
    pub graduated:     u8,
    pub bump:          u8,
    pub _pad:          [u8; 22],
}

impl AgentState {
    pub const LEN: usize = 200;

    pub fn from_bytes(data: &[u8]) -> Option<&Self> {
        if data.len() < Self::LEN {
            return None;
        }
        // SAFETY: AgentState is repr(C) and we verified the slice is long enough.
        Some(unsafe { &*(data.as_ptr() as *const AgentState) })
    }

    pub fn from_bytes_mut(data: &mut [u8]) -> Option<&mut Self> {
        if data.len() < Self::LEN {
            return None;
        }
        // SAFETY: AgentState is repr(C) and we verified the slice is long enough.
        Some(unsafe { &mut *(data.as_mut_ptr() as *mut AgentState) })
    }
}

// ---------------------------------------------------------------------------
// CurveState — 168 bytes total (repr(C) with explicit alignment padding)
// ---------------------------------------------------------------------------
//
//  offset 0  : u8        discriminant
//  offset 1  : [u8;32]   mint
//  offset 33 : [u8;32]   creator_fee_wallet  (= agent's creator_vault PDA)
//  offset 65 : [u8;7]    _pad_align          (alignment pad before u64)
//  offset 72 : u64       virtual_sol_reserves
//  offset 80 : u64       virtual_token_reserves
//  offset 88 : u64       real_sol_reserves
//  offset 96 : u64       real_token_reserves
//  offset 104: u64       total_supply
//  offset 112: u64       tokens_sold
//  offset 120: u16       creator_fee_bps
//  offset 122: u16       protocol_fee_bps
//  offset 124: u8        graduated
//  offset 125: u8        curve_bump
//  offset 126: u8        vault_bump
//  offset 127: [u8;41]   _pad

/// Bonding curve state account.
#[repr(C)]
pub struct CurveState {
    pub discriminant:           u8,
    pub mint:                   [u8; 32],
    pub creator_fee_wallet:     [u8; 32],
    pub _pad_align:             [u8; 7],  // explicit alignment pad before u64
    pub virtual_sol_reserves:   u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves:      u64,
    pub real_token_reserves:    u64,
    pub total_supply:           u64,
    pub tokens_sold:            u64,
    pub creator_fee_bps:        u16,
    pub protocol_fee_bps:       u16,
    pub graduated:              u8,
    pub curve_bump:             u8,
    pub vault_bump:             u8,
    pub _pad:                   [u8; 41],
}

impl CurveState {
    pub const LEN: usize = 168;

    pub fn from_bytes(data: &[u8]) -> Option<&Self> {
        if data.len() < Self::LEN {
            return None;
        }
        // SAFETY: CurveState is repr(C) and we verified the slice is long enough.
        Some(unsafe { &*(data.as_ptr() as *const CurveState) })
    }

    pub fn from_bytes_mut(data: &mut [u8]) -> Option<&mut Self> {
        if data.len() < Self::LEN {
            return None;
        }
        // SAFETY: CurveState is repr(C) and we verified the slice is long enough.
        Some(unsafe { &mut *(data.as_mut_ptr() as *mut CurveState) })
    }
}

// Compile-time layout assertions.
const _: () = {
    assert!(core::mem::size_of::<AgentState>() == AgentState::LEN);
    assert!(core::mem::size_of::<CurveState>() == CurveState::LEN);
};
