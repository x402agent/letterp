use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::require_signer;

pub struct InitializeAgentMintAccounts<'a> {
    pub owner: &'a AccountView,
    pub mint: &'a AccountView,
}

pub struct InitializeAgentMint<'a> {
    pub accounts: InitializeAgentMintAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountView]> for InitializeAgentMintAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [owner, mint, _rent_sysvar, _token_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(owner)?;
        Ok(Self { owner, mint })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for InitializeAgentMint<'a> {
    type Error = ProgramError;

    fn try_from((_data, accounts): (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self { accounts: InitializeAgentMintAccounts::try_from(accounts)? })
    }
}

impl<'a> InitializeAgentMint<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&self) -> ProgramResult {
        let _ = self.accounts;
        Ok(())
    }
}
