use core::mem::size_of;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use super::helpers::{require_nonzero_amount, require_signer};

pub struct SellAccounts<'a> {
    pub seller: &'a AccountView,
    pub curve: &'a AccountView,
}

pub struct SellData {
    pub tokens_in: u64,
}

pub struct Sell<'a> {
    pub accounts: SellAccounts<'a>,
    pub data: SellData,
}

impl<'a> TryFrom<&'a [AccountView]> for SellAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [seller, curve, _vault, _seller_token_account, _mint, _token_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        require_signer(seller)?;
        Ok(Self { seller, curve })
    }
}

impl TryFrom<&[u8]> for SellData {
    type Error = ProgramError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let tokens_in = u64::from_le_bytes(data.try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
        require_nonzero_amount(tokens_in)?;
        Ok(Self { tokens_in })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for Sell<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
        Ok(Self { accounts: SellAccounts::try_from(accounts)?, data: SellData::try_from(data)? })
    }
}

impl<'a> Sell<'a> {
    pub const DISCRIMINATOR: &'a u8 = &5;

    pub fn process(&self) -> ProgramResult {
        let _ = self.accounts;
        let _ = self.data;
        Ok(())
    }
}
