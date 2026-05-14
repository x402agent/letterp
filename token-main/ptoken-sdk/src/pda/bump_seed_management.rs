//! Store and retrieve canonical bump seeds from account data.
//!
//! Storing the bump avoids expensive re-derivation on every instruction.

use solana_program::{account_info::AccountInfo, program_error::ProgramError};

/// Write a bump seed to a specific byte offset in account data.
pub fn store_bump(account: &AccountInfo, offset: usize, bump: u8) -> Result<(), ProgramError> {
    let mut data = account.data.borrow_mut();
    *data
        .get_mut(offset)
        .ok_or(ProgramError::InvalidAccountData)? = bump;
    Ok(())
}

/// Read a bump seed from a specific byte offset in account data.
pub fn load_bump(account: &AccountInfo, offset: usize) -> Result<u8, ProgramError> {
    let data = account.data.borrow();
    data.get(offset)
        .copied()
        .ok_or(ProgramError::InvalidAccountData)
}

/// Build signer seeds array from seed slices + a stored bump reference.
///
/// # Usage
/// ```rust,ignore
/// let bump = load_bump(&config_account, BUMP_OFFSET)?;
/// let bump_ref = [bump];
/// let seeds = build_signer_seeds(&[AUTHORITY_SEED, mint.key.as_ref()], &bump_ref);
/// invoke_signed(ix, accounts, &[&seeds])?;
/// ```
pub fn build_signer_seeds<'a>(prefix_seeds: &[&'a [u8]], bump: &'a [u8]) -> Vec<&'a [u8]> {
    let mut seeds: Vec<&'a [u8]> = prefix_seeds.to_vec();
    seeds.push(bump);
    seeds
}
