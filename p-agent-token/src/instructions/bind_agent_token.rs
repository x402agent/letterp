use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::require_signer;
use crate::{errors::PAgentTokenError, require, state::AgentState};

pub struct BindAgentTokenAccounts<'a> {
    pub owner: &'a mut AccountView,
    pub agent_state: &'a mut AccountView,
}

pub struct BindAgentToken<'a> {
    pub accounts: BindAgentTokenAccounts<'a>,
}

impl<'a> TryFrom<&'a mut [AccountView]> for BindAgentTokenAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [owner, agent_state, _mint] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(owner)?;
        Ok(Self { owner, agent_state })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a mut [AccountView])> for BindAgentToken<'a> {
    type Error = ProgramError;

    fn try_from((_data, accounts): (&'a [u8], &'a mut [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self { accounts: BindAgentTokenAccounts::try_from(accounts)? })
    }
}

impl<'a> BindAgentToken<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;

    pub fn process(self) -> ProgramResult {
        let mut data = self.accounts.agent_state.try_borrow_mut()?;
        let agent = AgentState::load_mut(&mut data)?;
        require!(!agent.is_bound(), PAgentTokenError::AlreadyBound);
        agent.set_bound();
        Ok(())
    }
}
