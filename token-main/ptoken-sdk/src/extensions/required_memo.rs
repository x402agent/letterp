//! Required Memo extension — enforce memo instructions on incoming transfers.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
};
use spl_token_2022::extension::memo_transfer::instruction as memo_ix;
use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;

/// Enable the Required Memo extension on a token account.
///
/// After enabling, all incoming transfers to this account must include
/// a Memo program instruction in the same transaction.
pub fn enable_required_transfer_memos(
    token_account: &AccountInfo,
    owner: &AccountInfo,
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
pub fn disable_required_transfer_memos(
    token_account: &AccountInfo,
    owner: &AccountInfo,
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
