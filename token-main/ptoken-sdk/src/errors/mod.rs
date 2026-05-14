//! Error types for the LetterP token SDK.

pub mod extension_errors;
pub mod token_errors;

pub use extension_errors::ExtensionError;
pub use token_errors::PTokenError;

/// Convenience result type used throughout the LetterP token SDK.
pub type PTokenResult<T> = Result<T, PTokenError>;
