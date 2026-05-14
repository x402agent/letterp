//! Signer authorization checks.

use crate::errors::PTokenError;
use solana_program::account_info::AccountInfo;

/// Assert that a single account is a transaction signer.
pub fn assert_signer(account: &AccountInfo) -> Result<(), PTokenError> {
    if !account.is_signer {
        return Err(PTokenError::MissingSigner);
    }
    Ok(())
}

/// Assert that all accounts in a slice are transaction signers.
pub fn assert_all_signers(accounts: &[&AccountInfo]) -> Result<(), PTokenError> {
    for account in accounts {
        assert_signer(account)?;
    }
    Ok(())
}

/// Assert that at least `m` of the provided accounts are signers (multisig).
pub fn assert_m_of_n_signers(accounts: &[&AccountInfo], m: usize) -> Result<(), PTokenError> {
    let signer_count = accounts.iter().filter(|a| a.is_signer).count();
    if signer_count < m {
        return Err(PTokenError::MultisigThresholdNotMet);
    }
    Ok(())
}

/// Assert that `authority` is a signer and matches the expected pubkey.
pub fn assert_authority(
    account: &AccountInfo,
    expected: &solana_program::pubkey::Pubkey,
) -> Result<(), PTokenError> {
    if account.key != expected {
        return Err(PTokenError::InvalidOwner);
    }
    assert_signer(account)
}
