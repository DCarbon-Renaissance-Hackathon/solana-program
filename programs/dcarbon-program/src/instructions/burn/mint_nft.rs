use ::{
    anchor_lang::{
        prelude::*,
        solana_program::{ keccak, secp256k1_recover::secp256k1_recover },
        system_program::{ transfer as transfer_sol, Transfer as TransferSol },
    },
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{ burn, mint_to, Burn, Mint, MintTo, Token, TokenAccount },
    },
};

use crate::{ AuthorityPda, DcarbonError, MintNftEvent, MintPda, TokenAccountPda, WalletPda };

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintNftParams {
    program_index: u8,
    amount: u64,
    price: u64,
    signature: [u8; 64],
    recovery_id: u8,
    project_id: String,
    certificate_id: String,
}

#[derive(Accounts)]
#[instruction(params: MintNftParams)]
pub struct MintNft<'info> {
    ///CHECK
    #[account(
        mut,
        seeds = [MintPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub mint_pda: Account<'info, Mint>,

    ///CHECK
    #[account(
        mut,
        token::mint=mint_pda,
        token::authority = token_account_pda,
        seeds = [TokenAccountPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub token_account_pda: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = buyer,
        mint::decimals = 0,
        mint::authority = authority_pda,
        mint::freeze_authority = authority_pda
    )]
    pub mint_nft: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [AuthorityPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub authority_pda: Account<'info, AuthorityPda>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint_nft,
        associated_token::authority = buyer
    )]
    pub token_account_nft: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [WalletPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
      )]
    pub wallet_pda: Account<'info, WalletPda>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_nft_handle(ctx: Context<MintNft>, params: MintNftParams) -> Result<()> {
    let mint_nft = &ctx.accounts.mint_nft;
    let token_account_nft = &ctx.accounts.token_account_nft;
    let buyer = &ctx.accounts.buyer;
    let mint_pda = &ctx.accounts.mint_pda;
    let token_account_pda = &ctx.accounts.token_account_pda;
    let token_program = &ctx.accounts.token_program;

    let system_program = &ctx.accounts.system_program;
    let wallet_pda = &ctx.accounts.wallet_pda;
    let authority_pda = &ctx.accounts.authority_pda;

    // verify signature here-----

    let msg1 = params.project_id.to_string();
    let msg2 = buyer.key().to_string();
    let msg3 = params.amount.to_string();
    let msg4 = params.price.to_string();

    let message = format!("{msg1}{msg2}{msg3}{msg4}");
    msg!("message {}", message);

    let message_hash = {
        let mut hasher = keccak::Hasher::default();
        hasher.hash(message.as_bytes());
        hasher.result()
    };

    let recovered_pubkey = secp256k1_recover(
        &message_hash.0,
        params.recovery_id,
        &params.signature
    ).map_err(|_| ProgramError::InvalidArgument)?;

    require!(recovered_pubkey.0 == authority_pda.backend_signer, DcarbonError::WrongSignature);
    // end verify signature------

    // start burn tokens -------------------
    let cpi_program = token_program.to_account_info().clone();

    let cpi_accounts = Burn {
        mint: mint_pda.to_account_info().clone(),
        from: token_account_pda.to_account_info().clone(),
        authority: token_account_pda.to_account_info().clone(),
    };

    let seed_mint = TokenAccountPda::SEED_PREFIX;
    let seeds = &[seed_mint, &params.program_index.to_be_bytes(), &[ctx.bumps.token_account_pda]];

    let signer = &[&seeds[..]];
    let cpi_ctx_burn = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    burn(cpi_ctx_burn, params.amount)?;

    // start mint NFT ----------------------
    let cpi_program = token_program.to_account_info().clone();

    let cpi_accounts = MintTo {
        mint: mint_nft.to_account_info().clone(),
        to: token_account_nft.to_account_info().clone(),
        authority: authority_pda.to_account_info().clone(),
    };

    let seed_mint = AuthorityPda::SEED_PREFIX;
    let seeds = &[seed_mint, &params.program_index.to_be_bytes(), &[ctx.bumps.authority_pda]];
    let signer = &[&seeds[..]];

    // let cpi_ctx_nft = CpiContext::new(cpi_program, cpi_accounts);
    let cpi_ctx_nft = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    mint_to(cpi_ctx_nft, 1)?;

    // transfer
    // --- transfer with params.amount ---
    let transfer_price_context = CpiContext::new(system_program.to_account_info(), TransferSol {
        from: buyer.to_account_info(),
        to: wallet_pda.to_account_info(),
    });
    transfer_sol(transfer_price_context, params.price)?;

    emit!(MintNftEvent {
        owner: buyer.key(),
        mint: mint_nft.key(),
        project_id: params.project_id,
        amount: params.amount,
        price: params.price,
        certificate_id: params.certificate_id,
    });

    Ok(())
}
