use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PlatformState {
    pub is_initialized: bool,
    pub platform_treasury_sol_wallet: Pubkey
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

        let (
            is_initialized,
            platform_treasury_sol_wallet
        ) = array_refs![src, 1, 32];

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

        let (
            is_initialized_dst,
            platform_treasury_sol_wallet_dst
        ) = mut_array_refs![dst, 1, 32];

        let PlatformState {
            is_initialized,
            platform_treasury_sol_wallet
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        platform_treasury_sol_wallet_dst.copy_from_slice(platform_treasury_sol_wallet.as_ref());
    }
}
