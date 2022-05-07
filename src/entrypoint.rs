use solana_program::{
    entrypoint, account_info::AccountInfo, pubkey::Pubkey, entrypoint::ProgramResult
};

use crate::processor::Processor;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    Processor::unpack_and_process_instruction(program_id, accounts, instruction_data)?;

    Ok(())
}