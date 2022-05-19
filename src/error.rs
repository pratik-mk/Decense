use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecenseError {
    #[error("Invalid Intruction")]
    InvalidInstruction,

    #[error("Invalid Number")]
    InvalidNumber,
}

impl From<DecenseError> for ProgramError {
    fn from(e: DecenseError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
