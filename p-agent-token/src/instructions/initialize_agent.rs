use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::{require_signer, require_writable};

pub struct InitializeAgentAccounts<'a> {
    pub owner: &'a mut AccountView,
    pub agent_state: &'a mut AccountView,
}

pub struct InitializeAgent<'a> {
    pub accounts: InitializeAgentAccounts<'a>,
}

impl<'a> TryFrom<&'a mut [AccountView]> for InitializeAgentAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [owner, agent_state, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(owner)?;
        require_writable(agent_state)?;
        Ok(Self { owner, agent_state })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a mut [AccountView])> for InitializeAgent<'a> {
    type Error = ProgramError;

    fn try_from((_data, accounts): (&'a [u8], &'a mut [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self {
            accounts: InitializeAgentAccounts::try_from(accounts)?,
        })
    }
}

impl<'a> InitializeAgent<'a> {
    pub const DISCRIMINATOR: u8 = 0;

    pub fn process(self) -> ProgramResult {
        let _ = self.accounts;
        Ok(())
    }
}
