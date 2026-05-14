//! Permanent Delegate extension — irrevocable global delegate for a mint.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use spl_token_2022::instruction as token_ix;
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Initialize the Permanent Delegate extension on a mint.
///
/// The permanent delegate can transfer or burn tokens from ANY holder's
/// account at any time, regardless of whether the holder approves.
///
/// Must be called before `InitializeMint2`.
/// The delegate is immutable once set.
pub fn initialize_permanent_delegate(
    mint: &AccountInfo,
    delegate: &Pubkey,
) -> ProgramResult {
    invoke(
        &token_ix::initialize_permanent_delegate(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            delegate,
        )?,
        &[mint.clone()],
    )
}
