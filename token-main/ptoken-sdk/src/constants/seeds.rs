//! PDA seed byte arrays for pToken programs.

/// Seed for user-specific mint PDAs.
pub const MINT_SEED: &[u8] = b"mint";

/// Seed for protocol vault PDAs.
pub const VAULT_SEED: &[u8] = b"vault";

/// Seed for metadata PDAs.
pub const METADATA_SEED: &[u8] = b"metadata";

/// Seed for authority PDAs.
pub const AUTHORITY_SEED: &[u8] = b"authority";

/// Seed for program config PDAs.
pub const CONFIG_SEED: &[u8] = b"config";

/// Seed for fee vault PDAs.
pub const FEE_VAULT_SEED: &[u8] = b"fee_vault";

/// Seed for escrow PDAs.
pub const ESCROW_SEED: &[u8] = b"escrow";

/// Seed for token group PDAs.
pub const GROUP_SEED: &[u8] = b"group";

/// Seed for group member PDAs.
pub const MEMBER_SEED: &[u8] = b"member";
