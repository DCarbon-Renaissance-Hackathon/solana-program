use ::anchor_lang::prelude::*;

use crate::{ AuthorityPda, DcarbonError, WalletPda };

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct WithdrawPoolParams {
    program_index: u8,
}

#[derive(Accounts)]
#[instruction(params: WithdrawPoolParams)]
pub struct WithdrawPool<'info> {
    #[account(
        mut,
        seeds = [WalletPda::SEED_PREFIX,  params.program_index.to_be_bytes().as_ref()],
        bump
      )]
    pub wallet_pda: Account<'info, WalletPda>,

    #[account(
        mut,
        address = authority_pda.admin_address @DcarbonError::PermissionDenied,
    )]
    pub owner: Signer<'info>,

    #[account(
        seeds = [AuthorityPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub authority_pda: Account<'info, AuthorityPda>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_pool_handle(ctx: Context<WithdrawPool>, _params: WithdrawPoolParams) -> Result<()> {
    let owner = &ctx.accounts.owner;
    let wallet_pda = &ctx.accounts.wallet_pda;

    let lamports = ctx.accounts.wallet_pda.get_lamports();

    ctx.accounts.wallet_pda.sub_lamports(lamports - WalletPda::BASE_RENT)?;

    ctx.accounts.owner.add_lamports(lamports - WalletPda::BASE_RENT)?;

    msg!(
        "transfer from wallet pda (address: {:?}) to owner (address: {:?}) with amount ({:?} lamports)",
        wallet_pda.key(),
        owner.key(),
        lamports
    );

    Ok(())
}
