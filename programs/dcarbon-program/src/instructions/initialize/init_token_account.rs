use ::{ anchor_lang::prelude::*, anchor_spl::token::{ Mint, Token, TokenAccount } };

use crate::{ MintPda, TokenAccountPda };

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitTokenAccountParams {
    program_index: u8,
}

#[derive(Accounts)]
#[instruction(params: InitTokenAccountParams)]
pub struct InitTokenAccount<'info> {
    #[account(
        mut,
        seeds = [MintPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = owner,
        token::mint = mint,
        token::authority = token_account,
        seeds = [TokenAccountPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn init_token_account_handle(
    _ctx: Context<InitTokenAccount>,
    _params: InitTokenAccountParams
) -> Result<()> {
    Ok(())
}
