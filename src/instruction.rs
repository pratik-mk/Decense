use solana_program::program_error::ProgramError;

use crate::error::DecenseError;

pub enum DecenseInstruction {
    InitializePlatform,
    InitializeUser { market_valuation: u64, supply: u64 },
    Exchange { asked_price: u64, quantity: u64 },
}

impl DecenseInstruction {
    fn get_first_u64(rest: &[u8]) -> Result<u64, ProgramError> {
        let amount = rest
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(DecenseError::InvalidNumber)?;
        Ok(amount)
    }

    fn get_second_u64(rest: &[u8]) -> Result<u64, ProgramError> {
        let amount = rest
            .get(8..16)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(DecenseError::InvalidNumber)?;
        Ok(amount)
    }

    pub fn unpack_instruction(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (ins_no, rest) = instruction_data
            .split_first()
            .ok_or(DecenseError::InvalidInstruction)?;
        Ok(match ins_no {
            0 => Self::InitializePlatform,
            1 => Self::InitializeUser {
                market_valuation: Self::get_first_u64(rest)?,
                supply: Self::get_second_u64(rest)?,
            },
            2 => Self::Exchange {
                asked_price: Self::get_first_u64(rest)?,
                quantity: Self::get_second_u64(rest)?,
            },
            _ => return Err(DecenseError::InvalidInstruction.into()),
        })
    }
}
