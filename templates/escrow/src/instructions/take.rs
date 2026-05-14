use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::require_signer;

pub struct TakeAccounts<'a> {
    pub taker: &'a mut AccountView,
    pub escrow: &'a mut AccountView,
    pub vault: &'a mut AccountView,
}

pub struct Take<'a> {
    pub accounts: TakeAccounts<'a>,
}

impl<'a> TryFrom<&'a mut [AccountView]> for TakeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [taker, escrow, vault, _maker_receive_account, _taker_receive_account, _token_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(taker)?;
        Ok(Self { taker, escrow, vault })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a mut [AccountView])> for Take<'a> {
    type Error = ProgramError;

    fn try_from((_data, accounts): (&'a [u8], &'a mut [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self { accounts: TakeAccounts::try_from(accounts)? })
    }
}

impl<'a> Take<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(self) -> ProgramResult {
        let _ = self.accounts;
        Ok(())
    }
}

