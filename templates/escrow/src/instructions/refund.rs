use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::require_signer;

pub struct RefundAccounts<'a> {
    pub maker: &'a mut AccountView,
    pub escrow: &'a mut AccountView,
    pub vault: &'a mut AccountView,
}

pub struct Refund<'a> {
    pub accounts: RefundAccounts<'a>,
}

impl<'a> TryFrom<&'a mut [AccountView]> for RefundAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [maker, escrow, vault, _maker_receive_account, _token_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(maker)?;
        Ok(Self { maker, escrow, vault })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a mut [AccountView])> for Refund<'a> {
    type Error = ProgramError;

    fn try_from((_data, accounts): (&'a [u8], &'a mut [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self { accounts: RefundAccounts::try_from(accounts)? })
    }
}

impl<'a> Refund<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;

    pub fn process(self) -> ProgramResult {
        let _ = self.accounts;
        Ok(())
    }
}

