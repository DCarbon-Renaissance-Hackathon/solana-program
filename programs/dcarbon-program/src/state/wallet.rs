use anchor_lang::prelude::*;

#[account]
pub struct WalletPda {}

impl WalletPda {
    pub const SEED_PREFIX: &'static [u8] = b"walletPda";
    pub const BASE_SIZE: usize = 8; // padding
    pub const BASE_RENT: u64 = 946560;
}

#[account]
pub struct TokenAccountPda {
    pub bump: u8,
}

impl TokenAccountPda {
    pub const SEED_PREFIX: &'static [u8] = b"tokenAccount";
}
