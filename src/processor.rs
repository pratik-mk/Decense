use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, pubkey::Pubkey, msg};

use crate::instruction::DecenseInstruction;

pub struct Processor;

impl Processor {
    pub fn unpack_and_process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        match DecenseInstruction::unpack_instruction(instruction_data)? {
            DecenseInstruction::Initialize { market_valuation } => {
                msg!("Instruction: Initialize");
                Self::process_initialize(program_id, accounts, market_valuation)?;
            }
        }

        Ok(())
    }

    fn process_initialize(program_id: &Pubkey, accounts: &[AccountInfo], number: u64) -> ProgramResult {
        Ok(())
    }
}
