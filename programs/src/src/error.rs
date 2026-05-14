//! Program-specific error codes.

use pinocchio::program_error::ProgramError;

#[repr(u32)]
pub enum CurveError {
    /// The curve is not initialized.
    NotInitialized = 0,
    /// The curve has already graduated.
    AlreadyGraduated = 1,
    /// Insufficient reserves for the requested trade.
    InsufficientReserves = 2,
    /// Slippage exceeded.
    SlippageExceeded = 3,
    /// Arithmetic overflow/underflow.
    ArithmeticError = 4,
    /// Invalid authority.
    InvalidAuthority = 5,
    /// Creator fee too high.
    FeeTooHigh = 6,
}

impl From<CurveError> for ProgramError {
    fn from(e: CurveError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
