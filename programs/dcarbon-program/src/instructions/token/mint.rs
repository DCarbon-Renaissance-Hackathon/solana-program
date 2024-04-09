use ::{
    anchor_lang::{
        prelude::*,
        solana_program::clock::Clock,
        solana_program::{
            sysvar::instructions::{ ID as IX_ID, load_instruction_at_checked },
            instruction::Instruction,
            keccak,
        },
    },
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{ mint_to, Mint, MintTo, Token, TokenAccount },
    },
};

use crate::{ verify_secp256k1_instruction, AuthorityPda, DcarbonError, MintPda, ProjectState };

#[derive(Accounts)]
#[instruction(params: MintTokenParams)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut, 
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
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
       mut,
        seeds = [ProjectState::SEED_PREFIX, project_state.mint.to_bytes().as_ref()],
        bump
    )]
    pub project_state: Account<'info, ProjectState>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: sysvar account from solana
    #[account(address = IX_ID)]
    pub sysvar: AccountInfo<'info>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintTokenParams {
    program_index: u8,
    // device_id: String,
    device_id: [u8; 20],
    amount: u64,
    nonce: u64,
    name: String,
    symbol: String,
    uri: String,
    signature: [u8; 64],
    recovery_id: u8,
}

pub fn mint_token_handle(ctx: Context<MintToken>, params: MintTokenParams) -> Result<()> {
    let project_state = &mut ctx.accounts.project_state;
    let owner = &ctx.accounts.owner;
    let mint = &ctx.accounts.mint;
    let token_account = &ctx.accounts.token_account;
    let token_program = &ctx.accounts.token_program;
    let sysvar = &ctx.accounts.sysvar;
    let authority_pda = &ctx.accounts.authority_pda;

    // -- Check conditions --
    // check if device existed
    let fee_rate = project_state.fee;
    let eth_address = project_state.eth_address;
    // let device = project_state.devices.iter_mut().find(|item| item.id == params.device_id);
    let device = project_state.devices.iter_mut().find(|item| item.evm_address == params.device_id);
    require!(device.is_some(), DcarbonError::DeviceNotFound);

    let device = device.unwrap();

    // check device owner
    require!(
        device.owner == owner.key() && device.is_actived,
        DcarbonError::WrongDeviceOwnerOrDeviceSuspended
    );
    // check nonce is used or not
    require!(device.nonce + 1 == params.nonce, DcarbonError::InvalidNonce);

    // check claim today or not
    let current_timestamp = Clock::get()?.unix_timestamp;
    let today = current_timestamp / 86400;
    let last_mint_day = device.latest / 86400;
    require!(today > last_mint_day, DcarbonError::AlreadyMintToday);

    // -- Verify signature --
    // create message
    let message = keccak::hashv(
        &[
            // params.device_id.as_bytes(),
            &params.device_id,
            params.amount.to_be_bytes().as_ref(),
            params.nonce.to_be_bytes().as_ref(),
        ]
    );
    let ethereum_message = [
        "\x19Ethereum Signed Message:\n32".as_bytes(),
        &message.to_bytes(),
    ].concat();

    let ix: Instruction = load_instruction_at_checked(0, &sysvar.to_account_info())?;
    // verify signature
    verify_secp256k1_instruction(
        &ix,
        &eth_address,
        &ethereum_message,
        &params.signature,
        params.recovery_id
    )?;

    // if amount > limit, use limit of device
    let mut amount = params.amount;
    if amount > device.limit_amount {
        amount = device.limit_amount;
    }

    let fee_amount = (((amount as u128) * (fee_rate as u128)) / (i32::pow(10, 9) as u128)) as u64;

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

    mint_to(mint_to_cpi, amount - fee_amount)?;

    // -- Update config --
    device.nonce += 1;
    device.latest = current_timestamp;
    project_state.fee_amount += fee_amount;

    Ok(())
}
