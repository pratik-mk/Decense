use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PlatformState {
    pub is_initialized: bool,
    pub platform_treasury_sol_wallet: Pubkey,
}

impl Sealed for PlatformState {}
impl IsInitialized for PlatformState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for PlatformState {
    const LEN: usize = 33;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, PlatformState::LEN];

        let (is_initialized, platform_treasury_sol_wallet) = array_refs![src, 1, 32];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(PlatformState {
            is_initialized,
            platform_treasury_sol_wallet: Pubkey::new_from_array(*platform_treasury_sol_wallet),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, PlatformState::LEN];

        let (is_initialized_dst, platform_treasury_sol_wallet_dst) = mut_array_refs![dst, 1, 32];

        let PlatformState {
            is_initialized,
            platform_treasury_sol_wallet,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        platform_treasury_sol_wallet_dst.copy_from_slice(platform_treasury_sol_wallet.as_ref());
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct UserState {
    pub is_initialized: bool,
    pub user: Pubkey,
    pub market_valuation: u64,
    pub supply: u64,
    pub user_token_mint: Pubkey,
    pub user_ata: Pubkey,
    pub user_treasury_percentage: u8,
    pub liquidate_percentage: u8,
    pub pda_ata: Pubkey,
    pub cmp: u64,
    pub holders: u64,
}

impl Sealed for UserState {}
impl IsInitialized for UserState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for UserState {
    const LEN: usize = 163;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, UserState::LEN];

        let (
            is_initialized,
            user,
            market_valuation,
            supply,
            user_token_mint,
            user_ata,
            user_treasury_percentage,
            liquidate_percentage,
            pda_ata,
            cmp,
            holders,
        ) = array_refs![src, 1, 32, 8, 8, 32, 32, 1, 1, 32, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(UserState {
            is_initialized,
            user: Pubkey::new_from_array(*user),
            market_valuation: u64::from_le_bytes(*market_valuation),
            supply: u64::from_le_bytes(*supply),
            user_token_mint: Pubkey::new_from_array(*user_token_mint),
            user_ata: Pubkey::new_from_array(*user_ata),
            user_treasury_percentage: user_treasury_percentage[0],
            liquidate_percentage: liquidate_percentage[0],
            pda_ata: Pubkey::new_from_array(*pda_ata),
            cmp: u64::from_le_bytes(*cmp),
            holders: u64::from_le_bytes(*holders),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, UserState::LEN];

        let (
            is_initialized_dst,
            user_dst,
            market_valuation_dst,
            supply_dst,
            user_token_mint_dst,
            user_ata_dst,
            user_treasury_percentage_dst,
            liquidate_percentage_dst,
            pda_ata_dst,
            cmp_dst,
            holders_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 32, 32, 1, 1, 32, 8, 8];

        let UserState {
            is_initialized,
            user,
            market_valuation,
            supply,
            user_token_mint,
            user_ata,
            user_treasury_percentage,
            liquidate_percentage,
            pda_ata,
            cmp,
            holders,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        user_dst.copy_from_slice(user.as_ref());
        *market_valuation_dst = market_valuation.to_le_bytes();
        *supply_dst = supply.to_le_bytes();
        user_token_mint_dst.copy_from_slice(user_token_mint.as_ref());
        user_ata_dst.copy_from_slice(user_ata.as_ref());
        user_treasury_percentage_dst[0] = *user_treasury_percentage;
        liquidate_percentage_dst[0] = *liquidate_percentage;
        pda_ata_dst.copy_from_slice(pda_ata.as_ref());
        *cmp_dst = cmp.to_le_bytes();
        *holders_dst = holders.to_le_bytes();
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BuyerState {
    pub is_initialized: bool,
    pub buyer: Pubkey,
    pub current_holding_in_tokens: u64
}

impl Sealed for BuyerState {}
impl IsInitialized for BuyerState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for BuyerState {
    const LEN: usize = 41;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, BuyerState::LEN];

        let (
            is_initialized,
            buyer,
            current_holding_in_tokens
        ) = array_refs![src, 1, 32, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(BuyerState {
            is_initialized,
            buyer: Pubkey::new_from_array(*buyer),
            current_holding_in_tokens: u64::from_le_bytes(*current_holding_in_tokens)
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, BuyerState::LEN];

        let (
            is_initialized_dst,
            buyer_dst,
            current_holding_in_tokens_dst
        ) = mut_array_refs![dst, 1, 32, 8];

        let BuyerState {
            is_initialized,
            buyer,
            current_holding_in_tokens
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        buyer_dst.copy_from_slice(buyer.as_ref());
        *current_holding_in_tokens_dst = current_holding_in_tokens.to_le_bytes();
    }
}