//! Initialize Token-2022 mints with pre-configured extensions.

use crate::constants::program_ids::TOKEN_2022_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
    rent::Rent, system_instruction, sysvar::Sysvar,
};
use spl_token_2022::{extension::ExtensionType, instruction as token_ix, state::Mint};

/// Calculate the required account size for a Token-2022 mint with the given extensions.
pub fn mint_size_with_extensions<'a>(
    extensions: &[ExtensionType],
) -> Result<usize, solana_program::program_error::ProgramError> {
    ExtensionType::try_calculate_account_len::<Mint>(extensions)
}

/// Allocate and initialize a Token-2022 mint account with extensions.
///
/// Extensions must be initialized *before* the base mint via their respective
/// `InitializeXxx` instructions. This function handles only allocation and
/// the final `InitializeMint2` call.
///
/// For extension-specific initialization (e.g. `InitializeTransferFeeConfig`),
/// use the corresponding module in `crate::extensions`.
pub fn create_mint_with_extensions<'a>(
    payer: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
    extensions: &[ExtensionType],
    system_program: &AccountInfo<'a>,
) -> ProgramResult {
    let space = mint_size_with_extensions(extensions)?;
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);

    // Allocate the mint account with extra space for extensions
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint.key,
            lamports,
            space as u64,
            &TOKEN_2022_PROGRAM_ID,
        ),
        &[payer.clone(), mint.clone(), system_program.clone()],
    )?;

    // Initialize the base mint (use InitializeMint2 which doesn't require rent sysvar)
    invoke(
        &token_ix::initialize_mint2(
            &TOKEN_2022_PROGRAM_ID,
            mint.key,
            mint_authority,
            freeze_authority,
            decimals,
        )?,
        &[mint.clone()],
    )?;

    Ok(())
}
