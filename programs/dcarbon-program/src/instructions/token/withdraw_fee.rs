use ::{
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{ mint_to, Mint, MintTo, Token, TokenAccount },
    },
};

use crate::{ AuthorityPda, MintPda, ProjectState };

#[derive(Accounts)]
#[instruction(params:WithdrawFeeParams)]
pub struct WithdrawFee<'info> {
    #[account(
        mut, 
        seeds = [MintPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = admin
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut, address = project_state.admin)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [ProjectState::SEED_PREFIX, project_state.mint.to_bytes().as_ref()],
        bump
    )]
    pub project_state: Account<'info, ProjectState>,
    #[account(
        mut,
        seeds = [AuthorityPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub authority_pda: Account<'info, AuthorityPda>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct WithdrawFeeParams {
    program_index: u8,
}

pub fn withdraw_fee_handle(ctx: Context<WithdrawFee>, params: WithdrawFeeParams) -> Result<()> {
    let project_state = &mut ctx.accounts.project_state;
    let mint = &ctx.accounts.mint;
    let token_account = &ctx.accounts.token_account;
    let token_program = &ctx.accounts.token_program;
    let authority_pda = &ctx.accounts.authority_pda;

    let signed_seeds: &[&[&[u8]]] = &[
        &[
            AuthorityPda::SEED_PREFIX,
            &params.program_index.to_be_bytes(),
            &[ctx.bumps.authority_pda],
        ],
    ];
    let mint_to_cpi = CpiContext::new(token_program.to_account_info().clone(), MintTo {
        mint: mint.to_account_info().clone(),
        to: token_account.to_account_info().clone(),
        authority: authority_pda.to_account_info().clone(),
    }).with_signer(signed_seeds);

    mint_to(mint_to_cpi, project_state.fee_amount)?;

    // -- Update config --
    project_state.fee_amount = 0;

    Ok(())
}
