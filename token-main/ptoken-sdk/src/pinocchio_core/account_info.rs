//! Raw AccountInfo access helpers using Pinocchio's zero-overhead model.

use crate::errors::PTokenError;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

/// Assert that an account is a signer on the transaction.
pub fn assert_signer(account: &AccountInfo) -> Result<(), PTokenError> {
    if !account.is_signer {
        return Err(PTokenError::MissingSigner);
    }
    Ok(())
}

/// Assert that an account is owned by `expected_owner`.
pub fn assert_owned_by(account: &AccountInfo, expected_owner: &Pubkey) -> Result<(), PTokenError> {
    if account.owner != expected_owner {
        return Err(PTokenError::InvalidAccountOwner);
    }
    Ok(())
}

/// Assert that an account is writable.
pub fn assert_writable(account: &AccountInfo) -> Result<(), PTokenError> {
    if !account.is_writable {
        return Err(PTokenError::InvalidInstructionData);
    }
    Ok(())
}

/// Get the lamport balance of an account.
pub fn get_lamports(account: &AccountInfo) -> u64 {
    **account.lamports.borrow()
}

/// Add lamports to an account.
pub fn add_lamports(account: &AccountInfo, amount: u64) -> Result<(), ProgramError> {
    **account.lamports.borrow_mut() = account
        .lamports()
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(())
}

/// Subtract lamports from an account.
pub fn sub_lamports(account: &AccountInfo, amount: u64) -> Result<(), ProgramError> {
    **account.lamports.borrow_mut() = account
        .lamports()
        .checked_sub(amount)
        .ok_or(ProgramError::InsufficientFunds)?;
    Ok(())
}

/// Assert that an account's data length equals the expected size.
pub fn assert_data_len(account: &AccountInfo, expected: usize) -> Result<(), PTokenError> {
    if account.data_len() != expected {
        return Err(PTokenError::InvalidAccountDataLength);
    }
    Ok(())
}

/// Assert account data length is at least `min_size`.
pub fn assert_min_data_len(account: &AccountInfo, min_size: usize) -> Result<(), PTokenError> {
    if account.data_len() < min_size {
        return Err(PTokenError::InvalidAccountDataLength);
    }
    Ok(())
}
