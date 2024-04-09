use ::{
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{
            create_metadata_accounts_v3,
            mpl_token_metadata::types::DataV2,
            CreateMetadataAccountsV3,
            Metadata,
        },
        token::{ Mint, Token },
    },
    std::str::FromStr,
};

use crate::{ AuthorityPda, Device, DeviceParams, MintPda, ProjectState };

#[derive(Accounts)]
#[instruction(params: InitCarbonStateParams)]
pub struct InitCarbonState<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init_if_needed,
        payer = owner,
        mint::decimals = params.decimals,
        mint::authority = authority_pda,
        seeds = [MintPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [AuthorityPda::SEED_PREFIX, params.program_index.to_be_bytes().as_ref()],
        bump
    )]
    pub authority_pda: Account<'info, AuthorityPda>,

    #[account(
        init,
        payer = owner,
        space = ProjectState::BASE_SIZE + Device::SIZE * params.devices.len(),
        seeds = [ProjectState::SEED_PREFIX, mint.key().to_bytes().as_ref()],
        bump
    )]
    pub project_state: Account<'info, ProjectState>,

    /// CHECK
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitCarbonStateParams {
    program_index: u8,
    decimals: u8,
    fee: u64,
    name: String,
    symbol: String,
    uri: String,
    eth_address: [u8; 20],
    devices: Vec<DeviceParams>,
}

pub fn initialize_carbon_state_handle(
    ctx: Context<InitCarbonState>,
    params: InitCarbonStateParams
) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let owner = &ctx.accounts.owner;
    let metadata_account = &ctx.accounts.metadata_account;
    let token_metadata_program = &ctx.accounts.token_metadata_program;
    let system_program = &ctx.accounts.system_program;
    let rent = &ctx.accounts.rent;
    let project_state = &mut ctx.accounts.project_state;
    let authority_pda = &ctx.accounts.authority_pda;

    let signed_seeds: &[&[&[u8]]] = &[
        &[
            AuthorityPda::SEED_PREFIX,
            &params.program_index.to_be_bytes(),
            &[ctx.bumps.authority_pda],
        ],
    ];
    let cpi_context = CpiContext::new(
        token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: metadata_account.to_account_info(),
            mint: mint.to_account_info(),
            mint_authority: authority_pda.to_account_info(),
            update_authority: authority_pda.to_account_info(),
            payer: owner.to_account_info(),
            system_program: system_program.to_account_info(),
            rent: rent.to_account_info(),
        }
    ).with_signer(signed_seeds);

    let data_v2 = DataV2 {
        name: params.name,
        symbol: params.symbol,
        uri: params.uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };
    create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

    // update config
    project_state.mint = mint.key();
    project_state.admin = owner.key();
    project_state.eth_address = params.eth_address;
    project_state.program_index = params.program_index;
    project_state.fee_amount = 0;
    if params.fee > (i32::pow(10, 9) as u64) {
        project_state.fee = i32::pow(10, 9) as u64;
    } else {
        project_state.fee = params.fee;
    }
    for device in params.devices.iter() {
        project_state.devices.push(Device {
            // id: device.id.clone(),
            evm_address: device.evm_address.clone(),
            is_actived: true,
            device_type: device.device_type,
            limit_amount: device.limit_amount,
            latest: 0,
            owner: Pubkey::from_str(&device.owner).unwrap(),
            nonce: 0,
        });
    }

    Ok(())
}
