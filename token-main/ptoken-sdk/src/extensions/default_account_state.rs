//! Default Account State extension — new token accounts start frozen.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};
use spl_token_2022::{
    extension::default_account_state::instruction as das_ix, state::AccountState,
};

/// Initialize the Default Account State extension on a mint.
///
/// Must be called before `InitializeMint2`.
///
/// # Arguments
/// * `state` — The default state for newly created token accounts.
///   Use `AccountState::Frozen` for KYC-gated tokens.
pub fn initialize_default_account_state<'a>(
    mint: &AccountInfo<'a>,
    state: AccountState,
) -> ProgramResult {
    invoke(
        &das_ix::initialize_default_account_state(&TOKEN_2022_PROGRAM_ID, mint.key, &state)?,
        &[mint.clone()],
    )
}

/// Update the default account state (requires freeze authority).
pub fn update_default_account_state<'a>(
    mint: &AccountInfo<'a>,
    freeze_authority: &AccountInfo<'a>,
    state: AccountState,
) -> ProgramResult {
    invoke(
        &das_ix::update_default_account_state(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            freeze_authority.key,
            &[],
            &state,
        )?,
        &[mint.clone(), freeze_authority.clone()],
    )
}
