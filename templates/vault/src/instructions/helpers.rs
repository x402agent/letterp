use crate::{errors::VaultError, require};
use pinocchio::{error::ProgramError, AccountView};

#[inline(always)]
pub fn require_signer(account: &AccountView) -> Result<(), ProgramError> {
    require!(account.is_signer(), VaultError::MissingSigner);
    Ok(())
}

