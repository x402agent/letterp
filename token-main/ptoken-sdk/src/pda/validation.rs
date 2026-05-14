//! PDA validation — assert accounts match expected PDAs.

use crate::errors::PTokenError;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

/// Assert that `account` is the PDA derived from `seeds` and `program_id`.
///
/// Returns the canonical bump seed on success.
/// Prevents seed substitution and spoofing attacks.
pub fn assert_pda(
    account: &AccountInfo,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> Result<u8, PTokenError> {
    let (expected_key, bump) = Pubkey::find_program_address(seeds, program_id);
    if account.key != &expected_key {
        return Err(PTokenError::InvalidOwner);
    }
    Ok(bump)
}

/// Assert that `account` is a PDA derived from `seeds` + the provided `bump`.
///
/// Faster than `assert_pda` since it skips the bump search.
pub fn assert_pda_with_bump(
    account: &AccountInfo,
    seeds: &[&[u8]],
    bump: u8,
    program_id: &Pubkey,
) -> Result<(), PTokenError> {
    let mut seeds_with_bump: Vec<&[u8]> = seeds.to_vec();
    let bump_slice = &[bump][..];
    seeds_with_bump.push(bump_slice);

    let derived = Pubkey::create_program_address(&seeds_with_bump, program_id)
        .map_err(|_| PTokenError::InvalidOwner)?;

    if account.key != &derived {
        return Err(PTokenError::InvalidOwner);
    }
    Ok(())
}
