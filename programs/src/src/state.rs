//! On-chain state layout for the bonding-curve account.
//!
//! Byte layout (152 bytes total):
//!   0   - u8   discriminant       (0 = Uninitialized, 1 = Initialized)
//!   1   - bool graduated          (1 byte)
//!   2   - Pubkey authority        (32 bytes)
//!   34  - Pubkey creatorFeeWallet (32 bytes)
//!   66  - Pubkey mint             (32 bytes)
//!   98  - u64 realSolReserves     (8 bytes, LE)
//!   106 - u64 realTokenReserves   (8 bytes, LE)
//!   114 - u64 virtualSolReserves  (8 bytes, LE)
//!   122 - u64 virtualTokenReserves(8 bytes, LE)
//!   130 - u64 totalSupply         (8 bytes, LE)
//!   138 - u16 creatorFeeBps       (2 bytes, LE)
//!   140 - u16 protocolFeeBps      (2 bytes, LE)
//!   142 - [u8; 10] padding        (10 bytes)

use pinocchio::Address;

pub const CURVE_STATE_LEN: usize = 152;

/// The BondingCurve account state.
#[repr(C)]
pub struct CurveState {
    pub discriminant: u8,      // 0 = uninitialized, 1 = initialized
    pub graduated: u8,         // 0 = false, 1 = true (bool)
    pub authority: Address,     // 32 bytes
    pub creator_fee_wallet: Address, // 32 bytes
    pub mint: Address,          // 32 bytes
    pub real_sol_reserves: u64,     // 8 bytes
    pub real_token_reserves: u64,   // 8 bytes
    pub virtual_sol_reserves: u64,  // 8 bytes
    pub virtual_token_reserves: u64,// 8 bytes
    pub total_supply: u64,          // 8 bytes
    pub creator_fee_bps: u16,       // 2 bytes
    pub protocol_fee_bps: u16,      // 2 bytes
    // padding: 10 bytes (implicit)
}

impl CurveState {
    /// Deserialize from a raw byte slice. Returns None if invalid.
    pub fn from_bytes(data: &[u8]) -> Option<&Self> {
        if data.len() < CURVE_STATE_LEN {
            return None;
        }
        // Safety: the struct is repr(C) and we've checked length.
        Some(unsafe { &*(data.as_ptr() as *const CurveState) })
    }

    /// Deserialize mutably.
    pub fn from_bytes_mut(data: &mut [u8]) -> Option<&mut Self> {
        if data.len() < CURVE_STATE_LEN {
            return None;
        }
        Some(unsafe { &mut *(data.as_mut_ptr() as *mut CurveState) })
    }

    /// Create an uninitialized zeroed buffer for a new curve account.
    pub fn zeroed() -> [u8; CURVE_STATE_LEN] {
        [0u8; CURVE_STATE_LEN]
    }
}
