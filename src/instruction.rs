use solana_program::program_error::ProgramError;

use crate::error::DecenseError;

pub enum DecenseInstruction {
    Initialize { market_valuation: u64 },
}

impl DecenseInstruction {
    fn get_number(rest: &[u8]) -> Result<u64, ProgramError> {
        let amount = rest
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(DecenseError::InvalidNumber)?;
        Ok(amount)

    }

    pub fn unpack_instruction(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (ins_no, rest) = instruction_data.split_first().ok_or(DecenseError::InvalidInstruction)?;
        Ok(
            match ins_no {
                0 => Self::Initialize {
                    market_valuation: Self::get_number(rest)?
                },
                _ => return Err(DecenseError::InvalidInstruction.into())
            }
        )
    }
}
