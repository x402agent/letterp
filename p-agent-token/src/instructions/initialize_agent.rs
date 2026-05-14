use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::require_signer;

pub struct InitializeAgentAccounts<'a> {
    pub owner: &'a AccountView,
    pub agent_state: &'a AccountView,
}

pub struct InitializeAgent<'a> {
    pub accounts: InitializeAgentAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountView]> for InitializeAgentAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [owner, agent_state, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(owner)?;
        Ok(Self { owner, agent_state })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for InitializeAgent<'a> {
    type Error = ProgramError;

    fn try_from((_data, accounts): (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self { accounts: InitializeAgentAccounts::try_from(accounts)? })
    }
}

impl<'a> InitializeAgent<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&self) -> ProgramResult {
        let _ = self.accounts;
        Ok(())
    }
}
