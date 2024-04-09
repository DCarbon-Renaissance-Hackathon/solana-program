use anchor_lang::prelude::*;

#[event]
pub struct MintNftEvent {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub price: u64,
    pub project_id: String,
    pub certificate_id: String,
}
