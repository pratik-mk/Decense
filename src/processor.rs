use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};

use crate::instruction::DecenseInstruction;
use crate::state::PlatformState;

pub struct Processor;

impl Processor {
    pub fn unpack_and_process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        match DecenseInstruction::unpack_instruction(instruction_data)? {
            DecenseInstruction::InitializePlatform => {
                msg!("Instruction: InitializePlatform");
                Self::process_initialize_platform(program_id, accounts)?;
            }

            DecenseInstruction::InitializeUser { market_valuation } => {
                msg!("Instruction: InitializeUser");
                Self::process_initialize(program_id, accounts, market_valuation)?;
            }
        }

        Ok(())
    }

    fn process_initialize_platform(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let admin_account = next_account_info(account_info_iter)?;

        let platform_state_account = next_account_info(account_info_iter)?;

        let sol_treasury_wallet = next_account_info(account_info_iter)?;

        let system_program_account = next_account_info(account_info_iter)?;

        if platform_state_account.data_is_empty() {
            let create_platform_state_account_ix = system_instruction::create_account_with_seed(
                admin_account.key,
                platform_state_account.key,
                admin_account.key,
                "DECENSE PLATFORM",
                Rent::default().minimum_balance(PlatformState::LEN),
                PlatformState::LEN as u64,
                program_id,
            );

            invoke(
                &create_platform_state_account_ix,
                &[
                    admin_account.clone(),
                    platform_state_account.clone(),
                    system_program_account.clone(),
                ],
            )?;
        }

        let mut unpacked_platform_state_account =
            PlatformState::unpack_unchecked(&platform_state_account.try_borrow_data()?)?;

        unpacked_platform_state_account.is_initialized = true;
        unpacked_platform_state_account.platform_treasury_sol_wallet = *sol_treasury_wallet.key;

        PlatformState::pack(
            unpacked_platform_state_account,
            &mut platform_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        number: u64,
    ) -> ProgramResult {
        Ok(())
    }
}
