//! Error types for the pToken SDK.

pub mod token_errors;
pub mod extension_errors;

pub use token_errors::PTokenError;
pub use extension_errors::ExtensionError;

/// Convenience result type used throughout pToken SDK.
pub type PTokenResult<T> = Result<T, PTokenError>;
