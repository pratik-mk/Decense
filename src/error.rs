use std::io::Seek;

use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecenseError {
    #[error("Invalid Intruction")]
    InvalidInstruction
}

impl From<DecenseError> for ProgramError {
    fn from(e: DecenseError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
