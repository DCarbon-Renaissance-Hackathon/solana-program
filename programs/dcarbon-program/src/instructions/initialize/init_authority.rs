use ::anchor_lang::prelude::*;

use crate::{ AuthorityPda, WalletPda };

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitAuthorityParams {
    pub program_index: u8,
    pub signer: [u8; 64],
    pub admin_address: Pubkey,
}

#[derive(Accounts)]
#[instruction(params: InitAuthorityParams)]
pub struct InitAuthority<'info> {
    // init wallet pda to hold Sol from users when burning tokens
    #[account(
        init,
        space = WalletPda::BASE_SIZE,
        payer = payer,
        seeds = [WalletPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub wallet_pda: Account<'info, WalletPda>,

    #[account(
        init,
        space = AuthorityPda::BASE_SIZE,
        payer = payer,
        seeds = [AuthorityPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub authority_pda: Account<'info, AuthorityPda>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_authority_handle(
    ctx: Context<InitAuthority>,
    params: InitAuthorityParams
) -> Result<()> {
    let authority_pda = &mut ctx.accounts.authority_pda;
    //set backend signer
    authority_pda.backend_signer = params.signer;

    // set admin address
    authority_pda.admin_address = params.admin_address;

    Ok(())
}
