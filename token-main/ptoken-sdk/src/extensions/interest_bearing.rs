//! Interest Bearing extension — tokens accumulate interest over time.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use spl_token_2022::extension::interest_bearing_mint::instruction as ib_ix;
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Initialize the Interest Bearing extension on a mint.
///
/// Must be called before `InitializeMint2`.
///
/// # Arguments
/// * `rate_authority` — Can update the interest rate. `None` = immutable.
/// * `rate` — Annual rate in basis points (signed: negative rates allowed).
pub fn initialize_interest_bearing_mint(
    mint: &AccountInfo,
    rate_authority: Option<&Pubkey>,
    rate: i16,
) -> ProgramResult {
    invoke(
        &ib_ix::initialize(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            rate_authority.copied(),
            rate,
        )?,
        &[mint.clone()],
    )
}

/// Update the interest rate on an existing mint.
pub fn update_rate(
    mint: &AccountInfo,
    rate_authority: &AccountInfo,
    rate: i16,
) -> ProgramResult {
    invoke(
        &ib_ix::update_rate(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            rate_authority.key,
            &[],
            rate,
        )?,
        &[mint.clone(), rate_authority.clone()],
    )
}
