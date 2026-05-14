//! CPI into the Solana System Program.

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
    system_instruction,
};

/// CPI: create a new account owned by `owner_program`.
pub fn cpi_create_account<'a>(
    payer: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    owner_program: &Pubkey,
    lamports: u64,
    space: u64,
) -> ProgramResult {
    invoke(
        &system_instruction::create_account(
            payer.key,
            new_account.key,
            lamports,
            space,
            owner_program,
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
    )
}

/// CPI: transfer SOL from `from` to `to`.
pub fn cpi_transfer_sol<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    lamports: u64,
) -> ProgramResult {
    invoke(
        &system_instruction::transfer(from.key, to.key, lamports),
        &[from.clone(), to.clone(), system_program.clone()],
    )
}

/// CPI: transfer SOL using a PDA authority.
pub fn cpi_transfer_sol_signed<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    lamports: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    solana_program::program::invoke_signed(
        &system_instruction::transfer(from.key, to.key, lamports),
        &[from.clone(), to.clone(), system_program.clone()],
        signer_seeds,
    )
}

/// CPI: allocate space for an account.
pub fn cpi_allocate<'a>(
    account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    space: u64,
) -> ProgramResult {
    invoke(
        &system_instruction::allocate(account.key, space),
        &[account.clone(), system_program.clone()],
    )
}

/// CPI: assign an account to a program.
pub fn cpi_assign<'a>(
    account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    owner: &Pubkey,
) -> ProgramResult {
    invoke(
        &system_instruction::assign(account.key, owner),
        &[account.clone(), system_program.clone()],
    )
}
