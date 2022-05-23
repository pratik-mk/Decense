use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecenseError {
    #[error("Invalid Intruction")]
    InvalidInstruction,

    #[error("Invalid Number")]
    InvalidNumber,

    #[error("Insufficient token balance")]
    InsufficientTokenBalance,

    #[error("Math Error")]
    MathError,

    #[error("Invalid PDA")]
    InvalidPDA,
}

impl From<DecenseError> for ProgramError {
    fn from(e: DecenseError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
