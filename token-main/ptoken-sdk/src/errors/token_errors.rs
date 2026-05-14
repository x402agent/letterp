//! Core token operation error types.

use num_derive::FromPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Errors for core SPL Token and Token-2022 operations.
#[derive(Error, Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum PTokenError {
    /// Account has not been initialized.
    #[error("Account has not been initialized")]
    NotInitialized,

    /// Account is already initialized.
    #[error("Account is already initialized")]
    AlreadyInitialized,

    /// Invalid mint account provided.
    #[error("Invalid mint account")]
    InvalidMint,

    /// Invalid token account owner.
    #[error("Invalid token account owner")]
    InvalidOwner,

    /// Invalid or expired delegate.
    #[error("Invalid or expired delegate")]
    InvalidDelegate,

    /// Insufficient token balance for operation.
    #[error("Insufficient token balance")]
    InsufficientBalance,

    /// Token account is frozen.
    #[error("Token account is frozen")]
    AccountFrozen,

    /// Mint mismatch between accounts.
    #[error("Mint mismatch between token accounts")]
    MintMismatch,

    /// Owner does not match account.
    #[error("Owner does not match account")]
    OwnerMismatch,

    /// Mint has a fixed supply; minting is not allowed.
    #[error("Mint has a fixed supply")]
    FixedSupply,

    /// Arithmetic overflow in token calculation.
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,

    /// Arithmetic underflow in token calculation.
    #[error("Arithmetic underflow")]
    ArithmeticUnderflow,

    /// Account data length is invalid.
    #[error("Invalid account data length")]
    InvalidAccountDataLength,

    /// Instruction data is malformed.
    #[error("Invalid instruction data")]
    InvalidInstructionData,

    /// Missing required signer.
    #[error("Missing required signer")]
    MissingSigner,

    /// Account is not owned by the expected program.
    #[error("Account not owned by expected program")]
    InvalidAccountOwner,

    /// Token account balance is not zero; cannot close.
    #[error("Token account balance must be zero to close")]
    NonZeroBalance,

    /// Freeze authority not set on mint.
    #[error("Freeze authority not set on this mint")]
    NoFreezeAuthority,

    /// Operation requires multisig and threshold not met.
    #[error("Multisig threshold not met")]
    MultisigThresholdNotMet,

    /// Delegate amount exceeded.
    #[error("Delegate allowance exceeded")]
    DelegateAllowanceExceeded,
}

impl From<PTokenError> for ProgramError {
    fn from(e: PTokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
