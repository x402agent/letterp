use core::mem::{align_of, size_of};
use pinocchio::{error::ProgramError, Address};

#[repr(C)]
pub struct AgentState {
    pub owner: Address,
    pub agent_asset: Address,
    pub agent_token_mint: Address,
    pub executive: Address,
    pub metadata_hash: [u8; 32],
    flags: [u8; 1],
    bump: [u8; 1],
}

impl AgentState {
    pub const LEN: usize = size_of::<Self>();
    pub const FLAG_BOUND: u8 = 1;
    pub const FLAG_GRADUATED: u8 = 2;

    #[inline(always)]
    pub const fn has_flag(flags: u8, flag: u8) -> bool {
        flags & flag != 0
    }

    #[inline(always)]
    pub const fn with_flag(flags: u8, flag: u8) -> u8 {
        flags | flag
    }

    #[inline(always)]
    pub fn is_aligned(data: &[u8]) -> bool {
        (data.as_ptr() as usize) % align_of::<Self>() == 0
    }

    #[inline(always)]
    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if !Self::is_aligned(data) {
            return Err(ProgramError::InvalidAccountData);
        }
        // SAFETY: length and alignment were checked above. The layout is repr(C),
        // all fields are byte arrays or Address values, and callers must still
        // validate account ownership before trusting the loaded state.
        Ok(unsafe { &*(data.as_ptr() as *const Self) })
    }

    #[inline(always)]
    pub fn load_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if !Self::is_aligned(data) {
            return Err(ProgramError::InvalidAccountData);
        }
        // SAFETY: length and alignment were checked above. The mutable borrow
        // comes from AccountView's borrow guard, so no aliasing mutable reference
        // should exist while this reference is live.
        Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) })
    }

    #[inline(always)]
    pub fn flags(&self) -> u8 {
        self.flags[0]
    }

    #[inline(always)]
    pub fn is_bound(&self) -> bool {
        Self::has_flag(self.flags(), Self::FLAG_BOUND)
    }

    #[inline(always)]
    pub fn set_bound(&mut self) {
        self.flags[0] = Self::with_flag(self.flags[0], Self::FLAG_BOUND);
    }

    #[inline(always)]
    pub fn set_graduated(&mut self) {
        self.flags[0] = Self::with_flag(self.flags[0], Self::FLAG_GRADUATED);
    }

    #[inline(always)]
    pub fn bump(&self) -> u8 {
        self.bump[0]
    }
}

#[repr(C)]
pub struct CurveState {
    pub mint: Address,
    pub vault: Address,
    virtual_sol: [u8; 8],
    virtual_token: [u8; 8],
    real_sol: [u8; 8],
    real_token: [u8; 8],
    fee_bps: [u8; 2],
    creator_fee_bps: [u8; 2],
    flags: [u8; 1],
    bump: [u8; 1],
}

impl CurveState {
    pub const LEN: usize = size_of::<Self>();
    pub const DEFAULT_DENOMINATOR_BPS: u16 = 10_000;

    #[inline(always)]
    pub fn amount_from_le_bytes(bytes: [u8; 8]) -> u64 {
        u64::from_le_bytes(bytes)
    }

    #[inline(always)]
    pub fn bps_from_le_bytes(bytes: [u8; 2]) -> u16 {
        u16::from_le_bytes(bytes)
    }

    #[inline(always)]
    pub fn apply_bps(amount: u64, bps: u16) -> u64 {
        ((amount as u128 * bps as u128) / Self::DEFAULT_DENOMINATOR_BPS as u128)
            .min(u64::MAX as u128) as u64
    }

    #[inline(always)]
    pub fn net_after_bps_fee(amount: u64, bps: u16) -> (u64, u64) {
        let fee = Self::apply_bps(amount, bps);
        (amount.saturating_sub(fee), fee)
    }

    #[inline(always)]
    pub fn buy_tokens_out(virtual_sol: u64, virtual_token: u64, net_sol_in: u64) -> Option<u64> {
        let denominator = virtual_sol as u128 + net_sol_in as u128;
        if denominator == 0 {
            return None;
        }
        let invariant = virtual_sol as u128 * virtual_token as u128;
        let new_virtual_token = invariant / denominator;
        Some(virtual_token.saturating_sub(new_virtual_token.min(u64::MAX as u128) as u64))
    }

    #[inline(always)]
    pub fn sell_sol_out(virtual_sol: u64, virtual_token: u64, tokens_in: u64) -> Option<u64> {
        let denominator = virtual_token as u128 + tokens_in as u128;
        if denominator == 0 {
            return None;
        }
        let invariant = virtual_sol as u128 * virtual_token as u128;
        let new_virtual_sol = invariant / denominator;
        Some(virtual_sol.saturating_sub(new_virtual_sol.min(u64::MAX as u128) as u64))
    }

    #[inline(always)]
    pub fn is_aligned(data: &[u8]) -> bool {
        (data.as_ptr() as usize) % align_of::<Self>() == 0
    }

    #[inline(always)]
    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if !Self::is_aligned(data) {
            return Err(ProgramError::InvalidAccountData);
        }
        // SAFETY: length and alignment were checked above. The layout is repr(C),
        // all integer fields are stored as raw little-endian byte arrays, and
        // callers must validate account ownership before trusting the state.
        Ok(unsafe { &*(data.as_ptr() as *const Self) })
    }

    #[inline(always)]
    pub fn load_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if !Self::is_aligned(data) {
            return Err(ProgramError::InvalidAccountData);
        }
        // SAFETY: length and alignment were checked above. The mutable borrow
        // comes from AccountView's borrow guard, so no aliasing mutable reference
        // should exist while this reference is live.
        Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) })
    }

    #[inline(always)]
    pub fn virtual_sol(&self) -> u64 {
        Self::amount_from_le_bytes(self.virtual_sol)
    }

    #[inline(always)]
    pub fn virtual_token(&self) -> u64 {
        Self::amount_from_le_bytes(self.virtual_token)
    }

    #[inline(always)]
    pub fn fee_bps(&self) -> u16 {
        Self::bps_from_le_bytes(self.fee_bps)
    }
}
