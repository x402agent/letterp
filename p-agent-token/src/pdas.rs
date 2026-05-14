//! PDA seed helpers and address verification.

use pinocchio::{address::Address, error::ProgramError};

use crate::error::AgentError;

/// Verify that `account`'s key matches the PDA derived from `seeds` + `bump`
/// under `program_id`. Returns `AgentError::InvalidPda` on mismatch.
pub fn verify_pda(
    account_key: &[u8],
    seeds: &[&[u8]],
    program_id: &Address,
    bump: u8,
) -> Result<(), ProgramError> {
    // Build seeds with bump appended.
    let bump_slice: &[u8] = core::slice::from_ref(&bump);

    // We need to build a combined seeds slice that includes the bump.
    // Use a fixed-size array on the stack (max seeds + 1 for bump).
    const MAX_SEEDS: usize = 16;
    let n = seeds.len();
    if n >= MAX_SEEDS {
        return Err(AgentError::InvalidPda.into());
    }

    let mut all_seeds: [&[u8]; MAX_SEEDS] = [&[]; MAX_SEEDS];
    for (i, s) in seeds.iter().enumerate() {
        all_seeds[i] = s;
    }
    all_seeds[n] = bump_slice;
    let seeds_with_bump = &all_seeds[..=n];

    let derived = Address::create_program_address(seeds_with_bump, program_id)
        .map_err(|_| ProgramError::from(AgentError::InvalidPda))?;

    if derived.as_ref() != account_key {
        return Err(AgentError::InvalidPda.into());
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Seed constructors matching the TypeScript PDA builders exactly.
// ---------------------------------------------------------------------------

pub fn agent_seeds(owner: &[u8]) -> [&[u8]; 2] {
    [b"agent", owner]
}

pub fn agent_token_seeds(mint: &[u8]) -> [&[u8]; 2] {
    [b"agent-token", mint]
}

pub fn curve_seeds(mint: &[u8]) -> [&[u8]; 2] {
    [b"bonding-curve", mint]
}

pub fn vault_seeds(mint: &[u8]) -> [&[u8]; 3] {
    [b"bonding-curve", mint, b"vault"]
}

pub fn creator_vault_seeds(creator: &[u8]) -> [&[u8]; 2] {
    [b"creator-vault", creator]
}

pub fn exec_delegation_seeds<'a>(agent: &'a [u8], delegate: &'a [u8]) -> [&'a [u8]; 3] {
    [b"exec-delegation", agent, delegate]
}
