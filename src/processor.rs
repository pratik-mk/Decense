use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};

use spl_associated_token_account::instruction::create_associated_token_account;

use crate::state::{PlatformState, UserState};
use crate::{error::DecenseError, instruction::DecenseInstruction, state::BuyerState};

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

            DecenseInstruction::InitializeUser {
                market_valuation,
                supply,
            } => {
                msg!("Instruction: InitializeUser");
                Self::process_initialize_user(program_id, accounts, market_valuation, supply)?;
            }

            DecenseInstruction::Exchange {
                asked_price,
                quantity,
            } => {
                msg!("Instruction: Exchange");
                Self::process_exchange(program_id, accounts, asked_price, quantity)?;
            }

            DecenseInstruction::SendRecieveToken { action, amount } => {
                msg!("Instruction: SendRecieveToken");
                Self::process_send_receive_tokens(program_id, accounts, action, amount)?;
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
        supply: u64,
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
        unpacked_user_state_account.cmp = market_valuation
            .checked_div(supply)
            .ok_or(DecenseError::MathError)?
            .checked_mul(1000000000)
            .ok_or(DecenseError::MathError)?;
        unpacked_user_state_account.liquidate_percentage = 50;

        UserState::pack(
            unpacked_user_state_account,
            &mut user_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    fn process_exchange(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        asked_price: u64,
        quantity: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let exchanger_account = next_account_info(account_info_iter)?;

        let exchanger_state = next_account_info(account_info_iter)?;

        let exchanger_token_ata = next_account_info(account_info_iter)?;

        let sk_account = next_account_info(account_info_iter)?;

        let sk_mint = next_account_info(account_info_iter)?;

        let sk_state_account = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let pda_token_ata = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let rent_sysvar_account = next_account_info(account_info_iter)?;

        let associated_token_account_program_account = next_account_info(account_info_iter)?;

        let system_program_account = next_account_info(account_info_iter)?;

        let (pda, bump_seeds) =
            Pubkey::find_program_address(&[sk_account.key.as_ref()], program_id);

        if pda != *pda_account.key {
            return Err(DecenseError::InvalidPDA.into());
        }

        if exchanger_state.data_is_empty() {
            // create user state account
            let create_user_state_account_ix = system_instruction::create_account_with_seed(
                exchanger_account.key,
                exchanger_state.key,
                exchanger_account.key,
                "DECENSE BUYER",
                Rent::default().minimum_balance(BuyerState::LEN),
                BuyerState::LEN as u64,
                program_id,
            );

            invoke(
                &create_user_state_account_ix,
                &[
                    exchanger_account.clone(),
                    exchanger_state.clone(),
                    system_program_account.clone(),
                ],
            )?;

            let mut unpacked_exchanger_state =
                BuyerState::unpack(&exchanger_state.try_borrow_data()?)?;

            unpacked_exchanger_state.is_initialized = true;
            unpacked_exchanger_state.buyer = *exchanger_account.key;

            BuyerState::pack(
                unpacked_exchanger_state,
                &mut exchanger_account.try_borrow_mut_data()?,
            )?;
        }

        if exchanger_token_ata.data_is_empty() {
            // create exchanger ata for user
            let create_exchanger_ata_ix = create_associated_token_account(
                exchanger_account.key,
                exchanger_account.key,
                sk_mint.key,
            );

            invoke(
                &create_exchanger_ata_ix,
                &[
                    exchanger_account.clone(),
                    exchanger_token_ata.clone(),
                    exchanger_account.clone(),
                    sk_mint.clone(),
                    system_program_account.clone(),
                    token_program_account.clone(),
                    rent_sysvar_account.clone(),
                    associated_token_account_program_account.clone(),
                ],
            )?;
        }

        let mut unpacked_sk_state_account =
            UserState::unpack(&sk_state_account.try_borrow_data()?)?;

        let mut unpacked_exchanger_state = BuyerState::unpack(&exchanger_state.try_borrow_data()?)?;

        let unpacked_pda_token_ata =
            spl_token::state::Account::unpack(&pda_token_ata.try_borrow_data()?)?;

        if quantity > unpacked_pda_token_ata.amount {
            return Err(DecenseError::InsufficientTokenBalance.into());
        }

        let transfer_sol =
            system_instruction::transfer(exchanger_account.key, sk_account.key, asked_price);

        invoke(
            &transfer_sol,
            &[
                exchanger_account.clone(),
                sk_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        let mut new_cmp = (asked_price.checked_sub(unpacked_sk_state_account.cmp))
            .ok_or(DecenseError::MathError)?
            .checked_div(unpacked_pda_token_ata.amount)
            .ok_or(DecenseError::MathError)?
            .checked_mul(quantity)
            .ok_or(DecenseError::MathError)?;

        if asked_price > unpacked_sk_state_account.cmp {
            new_cmp = unpacked_sk_state_account
                .cmp
                .checked_add(new_cmp)
                .ok_or(DecenseError::MathError)?;
        } else {
            new_cmp = unpacked_sk_state_account
                .cmp
                .checked_sub(new_cmp)
                .ok_or(DecenseError::MathError)?;
        }

        let unpacked_exchanger_token_ata = spl_token::state::Account::unpack(&exchanger_token_ata.try_borrow_data()?)?;

        if unpacked_exchanger_token_ata.amount == 0 {
            unpacked_sk_state_account.holders = unpacked_sk_state_account
                .holders
                .checked_add(1)
                .ok_or(DecenseError::MathError)?;
        }

        let transfer_token_to_user = spl_token::instruction::transfer_checked(
            &spl_token::id(),
            pda_token_ata.key,
            sk_mint.key,
            exchanger_token_ata.key,
            pda_account.key,
            &[],
            quantity,
            4,
        )?;

        invoke_signed(
            &transfer_token_to_user,
            &[
                pda_token_ata.clone(),
                exchanger_token_ata.clone(),
                pda_account.clone(),
                token_program_account.clone(),
            ],
            &[&[sk_account.key.as_ref(), &[bump_seeds]]],
        )?;

        unpacked_sk_state_account.cmp = new_cmp;
        UserState::pack(
            unpacked_sk_state_account,
            &mut sk_state_account.try_borrow_mut_data()?,
        )?;

        unpacked_exchanger_state.current_holding_in_tokens = quantity;
        BuyerState::pack(
            unpacked_exchanger_state,
            &mut exchanger_state.try_borrow_mut_data()?,
        )?;

        Ok(())
    }
    fn process_send_receive_tokens(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        action: u64,
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let sk_account = next_account_info(account_info_iter)?;

        let sk_state_account = next_account_info(account_info_iter)?;

        let sk_mint = next_account_info(account_info_iter)?;

        let exchanger_account = next_account_info(account_info_iter)?;

        let exchanger_token_ata = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let pda_token_ata = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let (pda, bump_seeds) =
            Pubkey::find_program_address(&[sk_account.key.as_ref()], program_id);

        if *pda_account.key != pda {
            return Err(DecenseError::InvalidPDA.into());
        }

        match action {
            0 => {
                let transfer_token_to_user = spl_token::instruction::transfer_checked(
                    &spl_token::id(),
                    exchanger_token_ata.key,
                    sk_mint.key,
                    pda_token_ata.key,
                    exchanger_account.key,
                    &[],
                    amount,
                    4,
                )?;

                invoke(
                    &transfer_token_to_user,
                    &[
                        exchanger_token_ata.clone(),
                        pda_token_ata.clone(),
                        exchanger_account.clone(),
                        token_program_account.clone(),
                    ],
                )?;

                let unpacked_exchanger_token_ata =
                    spl_token::state::Account::unpack(&exchanger_token_ata.try_borrow_data()?)?;

                if unpacked_exchanger_token_ata.amount == 0 {
                    let mut unpacked_sk_state_account =
                        UserState::unpack(&sk_state_account.try_borrow_data()?)?;
                    unpacked_sk_state_account.holders = unpacked_sk_state_account
                        .holders
                        .checked_sub(1)
                        .ok_or(DecenseError::MathError)?;

                    

                    UserState::pack(
                        unpacked_sk_state_account,
                        &mut sk_state_account.try_borrow_mut_data()?,
                    )?;
                }
            }

            1 => {
                let unpacked_exchanger_token_ata =
                    spl_token::state::Account::unpack(&exchanger_token_ata.try_borrow_data()?)?;

                if unpacked_exchanger_token_ata.amount == 0 {
                    let mut unpacked_sk_state_account =
                        UserState::unpack(&sk_state_account.try_borrow_data()?)?;
                    unpacked_sk_state_account.holders = unpacked_sk_state_account
                        .holders
                        .checked_add(1)
                        .ok_or(DecenseError::MathError)?;

                    UserState::pack(
                        unpacked_sk_state_account,
                        &mut sk_state_account.try_borrow_mut_data()?,
                    )?;
                }

                let transfer_token_to_user = spl_token::instruction::transfer_checked(
                    &spl_token::id(),
                    pda_token_ata.key,
                    sk_mint.key,
                    exchanger_token_ata.key,
                    pda_account.key,
                    &[],
                    amount,
                    4,
                )?;

                invoke_signed(
                    &transfer_token_to_user,
                    &[
                        pda_token_ata.clone(),
                        exchanger_token_ata.clone(),
                        pda_account.clone(),
                        token_program_account.clone(),
                    ],
                    &[&[sk_account.key.as_ref(), &[bump_seeds]]],
                )?;
            }

            _ => return Err(DecenseError::InvalidInstruction.into()),
        }

        Ok(())
    }
}
