use pinocchio::error::ProgramError;

#[repr(u32)]
pub enum AgentError {
    NotInitialized       = 0,
    AlreadyInitialized   = 1,
    AlreadyBound         = 2,
    AlreadyGraduated     = 3,
    NotGraduatable       = 4,
    InvalidAuthority     = 5,
    InvalidMint          = 6,
    InvalidPda           = 7,
    SlippageExceeded     = 8,
    InsufficientReserves = 9,
    ArithmeticError      = 10,
    FeeTooHigh           = 11,
    InvalidTokenProgram  = 12,
    NotSigner            = 13,
    NotWritable          = 14,
    UriTooLong           = 15,
}

impl From<AgentError> for ProgramError {
    fn from(e: AgentError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
