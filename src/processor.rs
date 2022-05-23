use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};

use spl_associated_token_account::instruction::create_associated_token_account;

use crate::instruction::DecenseInstruction;
use crate::state::{PlatformState, UserState};

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

            DecenseInstruction::InitializeUser { market_valuation, supply } => {
                msg!("Instruction: InitializeUser");
                Self::process_initialize_user(program_id, accounts, market_valuation, supply)?;
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

    fn process_initialize_user(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        market_valuation: u64,
        supply: u64
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user_account = next_account_info(account_info_iter)?;

        let user_mint = next_account_info(account_info_iter)?;

        let user_state_account = next_account_info(account_info_iter)?;

        let platform_state_account = next_account_info(account_info_iter)?;

        let platform_sol_treasury_wallet_account = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let user_ata = next_account_info(account_info_iter)?;

        let pda_ata = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let rent_sysvar_account = next_account_info(account_info_iter)?;

        let associated_token_account_program_account = next_account_info(account_info_iter)?;

        let system_program_account = next_account_info(account_info_iter)?;

        let unpacked_platform_state_account =
            PlatformState::unpack(&platform_state_account.try_borrow_data()?)?;

        if unpacked_platform_state_account.platform_treasury_sol_wallet
            != *platform_sol_treasury_wallet_account.key
        {
            return Err(ProgramError::Custom(22));
        }

        // transfer 1 sol from user to platform as a part of initialization fees
        let transfer_initialize_amount_to_platform_ix = system_instruction::transfer(
            user_account.key,
            platform_sol_treasury_wallet_account.key,
            1000000000,
        );

        invoke(
            &transfer_initialize_amount_to_platform_ix,
            &[
                user_account.clone(),
                platform_sol_treasury_wallet_account.clone(),
                user_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        // create user state account
        let create_user_state_account_ix = system_instruction::create_account_with_seed(
            user_account.key,
            user_state_account.key,
            user_account.key,
            "DECENSE USER",
            Rent::default().minimum_balance(UserState::LEN),
            UserState::LEN as u64,
            program_id,
        );

        invoke(
            &create_user_state_account_ix,
            &[
                user_account.clone(),
                user_state_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        // initialize user mint
        let initialize_mint_ix = spl_token::instruction::initialize_mint2(
            &spl_token::id(),
            user_mint.key,
            user_account.key,
            Some(user_account.key),
            4,
        )?;

        invoke(
            &initialize_mint_ix,
            &[
                user_mint.clone(),
                user_account.clone(),
                token_program_account.clone(),
            ],
        )?;

        // create mint ata for user
        let create_user_ata_ix =
            create_associated_token_account(user_account.key, user_account.key, user_mint.key);

        invoke(
            &create_user_ata_ix,
            &[
                user_account.clone(),
                user_ata.clone(),
                user_account.clone(),
                user_mint.clone(),
                system_program_account.clone(),
                token_program_account.clone(),
                rent_sysvar_account.clone(),
                associated_token_account_program_account.clone(),
            ],
        )?;

        // create mint ata for pda
        let create_user_ata_ix =
            create_associated_token_account(user_account.key, pda_ata.key, user_mint.key);

        invoke(
            &create_user_ata_ix,
            &[
                user_account.clone(),
                pda_ata.clone(),
                pda_account.clone(),
                user_mint.clone(),
                system_program_account.clone(),
                token_program_account.clone(),
                rent_sysvar_account.clone(),
                associated_token_account_program_account.clone(),
            ],
        )?;

        // mint market cap amount of tokens into users token ata
        let mint_tokens_ix = spl_token::instruction::mint_to_checked(
            &spl_token::id(),
            user_mint.key,
            user_ata.key,
            user_account.key,
            &[],
            supply * 10000,
            4,
        )?;

        invoke(
            &mint_tokens_ix,
            &[
                user_mint.clone(),
                user_account.clone(),
                user_ata.clone(),
                token_program_account.clone(),
            ],
        )?;

        // transfer 50% amount of token mint to pda ata
        let transfer_tokens_to_pda_ata_ix = spl_token::instruction::transfer_checked(
            &spl_token::id(),
            user_ata.key,
            user_mint.key,
            pda_ata.key,
            user_account.key,
            &[],
            (supply / 2) * 10000,
            4,
        )?;

        invoke(
            &transfer_tokens_to_pda_ata_ix,
            &[
                user_ata.clone(),
                user_mint.clone(),
                pda_ata.clone(),
                user_account.clone(),
                token_program_account.clone(),
            ],
        )?;

        let mut unpacked_user_state_account =
            UserState::unpack(&user_state_account.try_borrow_data()?)?;

        unpacked_user_state_account.is_initialized = true;
        unpacked_user_state_account.user_treasury_percentage = 50;
        unpacked_user_state_account.user_token_mint = *user_mint.key;
        unpacked_user_state_account.user_ata = *user_ata.key;
        unpacked_user_state_account.user = *user_account.key;
        unpacked_user_state_account.pda_ata = *pda_ata.key;
        unpacked_user_state_account.market_valuation = market_valuation;
        unpacked_user_state_account.supply = supply;
        unpacked_user_state_account.liquidate_percentage = 50;

        UserState::pack(
            unpacked_user_state_account,
            &mut user_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }
}
