//! Required Memo extension — enforce memo instructions on incoming transfers.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};
use spl_token_2022::extension::memo_transfer::instruction as memo_ix;

/// Enable the Required Memo extension on a token account.
///
/// After enabling, all incoming transfers to this account must include
/// a Memo program instruction in the same transaction.
pub fn enable_required_transfer_memos<'a>(
    token_account: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
) -> ProgramResult {
    invoke(
        &memo_ix::enable_required_transfer_memos(
            &TOKEN_2022_PROGRAM_ID,
            token_account.key,
            owner.key,
            &[],
        )?,
        &[token_account.clone(), owner.clone()],
    )
}

/// Disable the Required Memo extension on a token account.
pub fn disable_required_transfer_memos<'a>(
    token_account: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
) -> ProgramResult {
    invoke(
        &memo_ix::disable_required_transfer_memos(
            &TOKEN_2022_PROGRAM_ID,
            token_account.key,
            owner.key,
            &[],
        )?,
        &[token_account.clone(), owner.clone()],
    )
}
