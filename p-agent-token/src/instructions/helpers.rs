use crate::{errors::PAgentTokenError, require};
use pinocchio::{error::ProgramError, AccountView, Address};

#[inline(always)]
pub fn require_signer(account: &AccountView) -> Result<(), ProgramError> {
    require!(account.is_signer(), PAgentTokenError::MissingSigner);
    Ok(())
}

#[inline(always)]
pub fn require_writable(account: &AccountView) -> Result<(), ProgramError> {
    require!(account.is_writable(), ProgramError::InvalidArgument);
    Ok(())
}

#[inline(always)]
pub fn require_owned_by(account: &AccountView, owner: &Address) -> Result<(), ProgramError> {
    require!(account.owned_by(owner), ProgramError::InvalidAccountOwner);
    Ok(())
}

#[inline(always)]
pub fn require_program_account(
    account: &AccountView,
    program_id: &Address,
) -> Result<(), ProgramError> {
    require!(
        account.address() == program_id,
        ProgramError::IncorrectProgramId
    );
    require!(account.executable(), ProgramError::InvalidAccountData);
    Ok(())
}

#[inline(always)]
pub fn require_nonzero_amount(amount: u64) -> Result<(), ProgramError> {
    require!(amount > 0, PAgentTokenError::InvalidAmount);
    Ok(())
}
