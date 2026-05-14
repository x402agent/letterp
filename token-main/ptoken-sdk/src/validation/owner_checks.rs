//! Account program-ownership checks.

use crate::{constants::program_ids::*, errors::PTokenError};
use solana_program::account_info::AccountInfo;

/// Assert an account is owned by the SPL Token program.
pub fn assert_owned_by_token_program(account: &AccountInfo) -> Result<(), PTokenError> {
    if account.owner != &TOKEN_PROGRAM_ID {
        return Err(PTokenError::InvalidAccountOwner);
    }
    Ok(())
}

/// Assert an account is owned by the Token-2022 program.
pub fn assert_owned_by_token_2022(account: &AccountInfo) -> Result<(), PTokenError> {
    if account.owner != &TOKEN_2022_PROGRAM_ID {
        return Err(PTokenError::InvalidAccountOwner);
    }
    Ok(())
}

/// Assert an account is owned by either token program.
pub fn assert_owned_by_any_token_program(account: &AccountInfo) -> Result<(), PTokenError> {
    if !is_token_program(account.owner) {
        return Err(PTokenError::InvalidAccountOwner);
    }
    Ok(())
}

/// Assert an account is owned by the System program.
pub fn assert_owned_by_system(account: &AccountInfo) -> Result<(), PTokenError> {
    if account.owner != &SYSTEM_PROGRAM_ID {
        return Err(PTokenError::InvalidAccountOwner);
    }
    Ok(())
}

/// Assert an account is owned by a specific program.
pub fn assert_owned_by(
    account: &AccountInfo,
    expected_owner: &solana_program::pubkey::Pubkey,
) -> Result<(), PTokenError> {
    if account.owner != expected_owner {
        return Err(PTokenError::InvalidAccountOwner);
    }
    Ok(())
}
