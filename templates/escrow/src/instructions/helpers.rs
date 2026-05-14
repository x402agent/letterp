use crate::{errors::EscrowError, require};
use pinocchio::{error::ProgramError, AccountView};

#[inline(always)]
pub fn require_signer(account: &AccountView) -> Result<(), ProgramError> {
    require!(account.is_signer(), EscrowError::MissingSigner);
    Ok(())
}

