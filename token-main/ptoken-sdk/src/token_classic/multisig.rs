//! SPL Token M-of-N multisig authority setup.

use crate::constants::program_ids::TOKEN_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_pack::Pack,
    pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};
use spl_token::instruction as token_ix;

/// Maximum number of signers in an SPL Token multisig (11).
pub const MAX_SIGNERS: usize = 11;

/// Initialize a multisig account with M-of-N signing threshold.
///
/// # Arguments
/// * `payer` — Pays for account creation
/// * `multisig` — The new multisig account to initialize
/// * `signer_keys` — Up to 11 signer public keys
/// * `m` — Minimum number of signers required
pub fn create_multisig<'a>(
    payer: &AccountInfo<'a>,
    multisig: &AccountInfo<'a>,
    signer_keys: &[&Pubkey],
    m: u8,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    rent_sysvar: &AccountInfo<'a>,
) -> ProgramResult {
    assert!(signer_keys.len() <= MAX_SIGNERS, "Too many signers");
    assert!(
        m as usize <= signer_keys.len(),
        "m cannot exceed number of signers"
    );

    let rent = Rent::get()?;
    let multisig_rent = rent.minimum_balance(spl_token::state::Multisig::LEN);

    invoke(
        &system_instruction::create_account(
            payer.key,
            multisig.key,
            multisig_rent,
            spl_token::state::Multisig::LEN as u64,
            &TOKEN_PROGRAM_ID,
        ),
        &[payer.clone(), multisig.clone(), system_program.clone()],
    )?;

    invoke(
        &token_ix::initialize_multisig(&TOKEN_PROGRAM_ID, multisig.key, signer_keys, m)?,
        &[multisig.clone(), rent_sysvar.clone()],
    )?;

    Ok(())
}
