use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::{require_program_account, require_signer, require_writable};

pub struct InitializeAgentMintAccounts<'a> {
    pub owner: &'a mut AccountView,
    pub mint: &'a mut AccountView,
}

pub struct InitializeAgentMint<'a> {
    pub accounts: InitializeAgentMintAccounts<'a>,
}

impl<'a> TryFrom<&'a mut [AccountView]> for InitializeAgentMintAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [owner, mint, _rent_sysvar, token_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(owner)?;
        require_writable(mint)?;
        require_program_account(token_program, &pinocchio_token::ID)?;
        Ok(Self { owner, mint })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a mut [AccountView])> for InitializeAgentMint<'a> {
    type Error = ProgramError;

    fn try_from((_data, accounts): (&'a [u8], &'a mut [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self {
            accounts: InitializeAgentMintAccounts::try_from(accounts)?,
        })
    }
}

impl<'a> InitializeAgentMint<'a> {
    pub const DISCRIMINATOR: u8 = 1;

    pub fn process(self) -> ProgramResult {
        let _ = self.accounts;
        Ok(())
    }
}
