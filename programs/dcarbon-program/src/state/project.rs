use anchor_lang::prelude::*;

#[account]
pub struct ProjectState {
    pub program_index: u8,
    pub mint: Pubkey,
    pub devices: Vec<Device>,
    pub admin: Pubkey,
    pub eth_address: [u8; 20],
    // Fee percent for the Dcarbon foundation
    // 1 / 1e9 (1e9 equavalent 100%)
    pub fee: u64,
    pub fee_amount: u64,
    pub bump: u8,
}

impl ProjectState {
    pub const SEED_PREFIX: &'static [u8] = b"project";
    pub const BASE_SIZE: usize =
        8 + // padding
        1 + // u8
        4 + // empty string
        4 + // empty vec
        32 + // Pubkey
        8 + // u64
        8 + // u64
        1; // u8
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Device {
    // pub id: String,
    pub evm_address: [u8; 20],
    pub is_actived: bool,
    pub device_type: u16,
    pub limit_amount: u64,
    pub latest: i64,
    pub owner: Pubkey,
    pub nonce: u64,
}

impl Device {
    pub const SIZE: usize =
        100 + // String max 96 char
        1 + // bool
        2 + // u16
        8 + // u64
        8 + // i64
        32 + // Pubkey
        8; // u64
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct DeviceParams {
    // pub id: String,
    pub evm_address: [u8; 20],
    pub device_type: u16,
    pub limit_amount: u64,
    pub owner: String,
}
