use core::mem::size_of;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::{require_nonzero_amount, require_signer};

pub struct BuyAccounts<'a> {
    pub buyer: &'a mut AccountView,
    pub curve: &'a mut AccountView,
}

pub struct BuyData {
    pub lamports_in: u64,
}

pub struct Buy<'a> {
    pub accounts: BuyAccounts<'a>,
    pub data: BuyData,
}

impl<'a> TryFrom<&'a mut [AccountView]> for BuyAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [buyer, curve, _vault, _buyer_token_account, _mint, _token_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(buyer)?;
        Ok(Self { buyer, curve })
    }
}

impl TryFrom<&[u8]> for BuyData {
    type Error = ProgramError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let lamports_in = u64::from_le_bytes(data.try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
        require_nonzero_amount(lamports_in)?;
        Ok(Self { lamports_in })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a mut [AccountView])> for Buy<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a mut [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self { accounts: BuyAccounts::try_from(accounts)?, data: BuyData::try_from(data)? })
    }
}

impl<'a> Buy<'a> {
    pub const DISCRIMINATOR: &'a u8 = &4;

    pub fn process(self) -> ProgramResult {
        let _ = self.accounts;
        let _ = self.data;
        Ok(())
    }
}
