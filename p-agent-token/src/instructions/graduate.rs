use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::require_signer;
use crate::state::AgentState;

pub struct GraduateAccounts<'a> {
    pub authority: &'a mut AccountView,
    pub agent_state: &'a mut AccountView,
    pub curve: &'a mut AccountView,
}

pub struct Graduate<'a> {
    pub accounts: GraduateAccounts<'a>,
}

impl<'a> TryFrom<&'a mut [AccountView]> for GraduateAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [authority, agent_state, curve, _vault, _amm_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(authority)?;
        Ok(Self { authority, agent_state, curve })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a mut [AccountView])> for Graduate<'a> {
    type Error = ProgramError;

    fn try_from((_data, accounts): (&'a [u8], &'a mut [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self { accounts: GraduateAccounts::try_from(accounts)? })
    }
}

impl<'a> Graduate<'a> {
    pub const DISCRIMINATOR: &'a u8 = &6;

    pub fn process(self) -> ProgramResult {
        let mut data = self.accounts.agent_state.try_borrow_mut()?;
        let agent = AgentState::load_mut(&mut data)?;
        agent.set_graduated();
        let _ = self.accounts.curve;
        Ok(())
    }
}
