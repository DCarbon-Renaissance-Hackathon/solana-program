use anchor_lang::prelude::*;

#[account]
pub struct AuthorityPda {
    pub backend_signer: [u8; 64],
    pub admin_address: Pubkey,
}

impl AuthorityPda {
    pub const SEED_PREFIX: &'static [u8] = b"authorityPda";
    pub const BASE_SIZE: usize = 8 + 64 + 32; // padding
}
