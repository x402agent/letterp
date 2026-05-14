//! Error types specific to Token-2022 extensions.

use num_derive::FromPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Errors for Token-2022 extension operations.
#[derive(Error, Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ExtensionError {
    /// Requested extension not found on account.
    #[error("Extension not found on account")]
    ExtensionNotFound,

    /// Extension already initialized on this account.
    #[error("Extension already initialized")]
    ExtensionAlreadyInitialized,

    /// Unrecognized or unsupported extension type.
    #[error("Invalid extension type")]
    InvalidExtensionType,

    /// Transfer fee configuration is invalid.
    #[error("Invalid transfer fee configuration")]
    InvalidFeeConfig,

    /// Confidential transfers are not enabled on this mint.
    #[error("Confidential transfers not enabled")]
    ConfidentialTransferDisabled,

    /// Confidential transfer proof verification failed.
    #[error("Confidential transfer proof verification failed")]
    ProofVerificationFailed,

    /// Transfer hook program invocation failed.
    #[error("Transfer hook program failed")]
    TransferHookFailed,

    /// Transfer hook program ID is invalid.
    #[error("Invalid transfer hook program")]
    InvalidTransferHookProgram,

    /// Transfer requires a memo but none was provided.
    #[error("Transfer requires a memo instruction")]
    MemoRequired,

    /// CPI guard is active; operation blocked.
    #[error("CPI guard violation: operation not allowed via CPI")]
    CpiGuardViolation,

    /// Owner cannot be changed; immutable owner set.
    #[error("Immutable owner: cannot change token account owner")]
    ImmutableOwnerViolation,

    /// Token is non-transferable (soul-bound).
    #[error("Token is non-transferable")]
    NonTransferable,

    /// Interest rate configuration is invalid.
    #[error("Invalid interest rate configuration")]
    InvalidInterestRate,

    /// Permanent delegate cannot be changed after initialization.
    #[error("Permanent delegate is immutable")]
    PermanentDelegateImmutable,

    /// Metadata field update requires valid update authority.
    #[error("Invalid metadata update authority")]
    InvalidMetadataAuthority,

    /// Account does not have enough space for the new extension.
    #[error("Insufficient account space for extension")]
    InsufficientExtensionSpace,
}

impl From<ExtensionError> for ProgramError {
    fn from(e: ExtensionError) -> Self {
        ProgramError::Custom(1000 + e as u32)
    }
}
