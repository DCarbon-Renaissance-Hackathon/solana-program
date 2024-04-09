use ::{
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{
            create_master_edition_v3,
            create_metadata_accounts_v3,
            CreateMasterEditionV3,
            CreateMetadataAccountsV3,
            Metadata,
        },
        token::{ Mint, Token },
    },
    mpl_token_metadata::types::{ Creator, DataV2 },
};

use crate::AuthorityPda;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateMetadataParams {
    program_index: u8,
    name: String,
    symbol: String,
    uri: String,
}

#[derive(Accounts)]
#[instruction(params: UpdateMetadataParams)]
pub struct UpdateMetadata<'info> {
    #[account(
        mut,
        mint::decimals = 0,
        mint::authority = authority_pda
    )]
    pub mint_nft: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [AuthorityPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub authority_pda: Account<'info, AuthorityPda>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: From metaplex
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: From metaplex
    #[account(mut)]
    pub master_edition_account: AccountInfo<'info>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn update_metadata_handle(
    ctx: Context<UpdateMetadata>,
    params: UpdateMetadataParams
) -> Result<()> {
    let mint_nft = &ctx.accounts.mint_nft;
    let metadata_account = &ctx.accounts.metadata_account;
    let master_edition_account = &ctx.accounts.master_edition_account;
    let token_metadata_program = &ctx.accounts.token_metadata_program;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let rent = &ctx.accounts.rent;
    let authority_pda = &ctx.accounts.authority_pda;

    let cpi_program = token_metadata_program.to_account_info().clone();
    let cpi_accounts = CreateMetadataAccountsV3 {
        metadata: metadata_account.to_account_info(),
        mint: mint_nft.to_account_info(),
        mint_authority: authority_pda.to_account_info(),
        update_authority: authority_pda.to_account_info(),
        payer: payer.to_account_info(),
        system_program: system_program.to_account_info(),
        rent: rent.to_account_info(),
    };

    let seed_mint = AuthorityPda::SEED_PREFIX;
    let seeds = &[seed_mint, &params.program_index.to_be_bytes(), &[ctx.bumps.authority_pda]];

    let signer = &[&seeds[..]];
    let cpi_ctx_update_metadata = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    let creators = vec![Creator {
        address: authority_pda.key(),
        verified: false,
        share: 100,
    }];

    let data_v2 = DataV2 {
        name: params.name.clone(),
        symbol: params.symbol.clone(),
        uri: params.uri.clone(),
        seller_fee_basis_points: 0,

        creators: Some(creators.clone()),
        collection: None,
        uses: None,
    };
    create_metadata_accounts_v3(cpi_ctx_update_metadata, data_v2, false, true, None)?;

    let cpi_program = token_metadata_program.to_account_info().clone();
    let cpi_accounts = CreateMasterEditionV3 {
        edition: master_edition_account.to_account_info(),
        mint: mint_nft.to_account_info(),
        mint_authority: authority_pda.to_account_info(),
        update_authority: authority_pda.to_account_info(),
        payer: payer.to_account_info(),
        metadata: metadata_account.to_account_info(),
        token_program: token_program.to_account_info(),
        system_program: system_program.to_account_info(),
        rent: rent.to_account_info(),
    };

    let seed_mint = AuthorityPda::SEED_PREFIX;
    let seeds = &[seed_mint, &params.program_index.to_be_bytes(), &[ctx.bumps.authority_pda]];

    let signer = &[&seeds[..]];

    let cpi_ctx_update_master_edition = CpiContext::new_with_signer(
        cpi_program,
        cpi_accounts,
        signer
    );

    create_master_edition_v3(cpi_ctx_update_master_edition, None)?;

    Ok(())
}
