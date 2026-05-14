//! Token account state validation.

use solana_program::account_info::AccountInfo;
use crate::{
    errors::PTokenError,
    pinocchio_core::zero_copy_layout::{
        read_token_amount, ACCOUNT_STATE_OFFSET, ACCOUNT_LEN,
    },
};

/// SPL Token account states.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountState {
    /// Account is uninitialized.
    Uninitialized = 0,
    /// Account is initialized and active.
    Initialized = 1,
    /// Account is frozen — transfers disabled.
    Frozen = 2,
}

/// Assert that a token account is initialized (not Uninitialized or Frozen).
pub fn assert_account_initialized(account: &AccountInfo) -> Result<(), PTokenError> {
    let data = account.data.borrow();
    if data.len() < ACCOUNT_LEN {
        return Err(PTokenError::NotInitialized);
    }
    let state = data[ACCOUNT_STATE_OFFSET];
    if state == AccountState::Uninitialized as u8 {
        return Err(PTokenError::NotInitialized);
    }
    Ok(())
}

/// Assert that a token account is not frozen.
pub fn assert_account_not_frozen(account: &AccountInfo) -> Result<(), PTokenError> {
    let data = account.data.borrow();
    if data.len() < ACCOUNT_LEN {
        return Err(PTokenError::InvalidAccountDataLength);
    }
    if data[ACCOUNT_STATE_OFFSET] == AccountState::Frozen as u8 {
        return Err(PTokenError::AccountFrozen);
    }
    Ok(())
}

/// Assert that a token account has a balance of at least `amount`.
pub fn assert_sufficient_balance(
    account: &AccountInfo,
    amount: u64,
) -> Result<(), PTokenError> {
    let data = account.data.borrow();
    let balance = read_token_amount(&data).map_err(|_| PTokenError::InvalidAccountDataLength)?;
    if balance < amount {
        return Err(PTokenError::InsufficientBalance);
    }
    Ok(())
}

/// Assert that a token account has a zero balance (required before closing).
pub fn assert_zero_balance(account: &AccountInfo) -> Result<(), PTokenError> {
    let data = account.data.borrow();
    let balance = read_token_amount(&data).map_err(|_| PTokenError::InvalidAccountDataLength)?;
    if balance != 0 {
        return Err(PTokenError::NonZeroBalance);
    }
    Ok(())
}
