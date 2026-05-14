//! Error types for the LetterP token SDK.

pub mod token_errors;
pub mod extension_errors;

pub use token_errors::PTokenError;
pub use extension_errors::ExtensionError;

/// Convenience result type used throughout the LetterP token SDK.
pub type PTokenResult<T> = Result<T, PTokenError>;
