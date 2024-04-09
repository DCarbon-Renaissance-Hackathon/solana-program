use anchor_lang::prelude::*;

use instructions::*;
use state::*;
use util::*;

mod instructions;
mod state;
mod util;

declare_id!("k2tYYVZSVqZyMaaDNTQ3owSKqTaGSmmbWRnfjnVn2ri");

#[program]
pub mod dcarbon_program {
    use super::*;

    // init authority of program
    pub fn init_authority(ctx: Context<InitAuthority>, params: InitAuthorityParams) -> Result<()> {
        init_authority_handle(ctx, params)?;
        Ok(())
    }

    // init default state for carbon program
    pub fn initialize_carbon_state(
        ctx: Context<InitCarbonState>,
        params: InitCarbonStateParams
    ) -> Result<()> {
        initialize_carbon_state_handle(ctx, params)?;
        Ok(())
    }

    // init token account to hold carbon token fund
    pub fn init_token_account(
        ctx: Context<InitTokenAccount>,
        params: InitTokenAccountParams
    ) -> Result<()> {
        init_token_account_handle(ctx, params)?;

        Ok(())
    }

    // mint token for device owner
    pub fn mint_token(ctx: Context<MintToken>, params: MintTokenParams) -> Result<()> {
        mint_token_handle(ctx, params)?;
        Ok(())
    }

    pub fn add_devices(ctx: Context<AddDevices>, params: AddDevicesParams) -> Result<()> {
        add_devices_handle(ctx, params)?;
        Ok(())
    }

    // withdraw carbon token fee to admin
    pub fn withdraw_fee(ctx: Context<WithdrawFee>, params: WithdrawFeeParams) -> Result<()> {
        withdraw_fee_handle(ctx, params)?;
        Ok(())
    }

    pub fn enable_device(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
        enable_device_handle(ctx, params)?;
        Ok(())
    }

    pub fn suspend_device(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
        suspend_device_handle(ctx, params)?;
        Ok(())
    }

    pub fn set_limit(ctx: Context<UpdateConfig>, params: SetLimitParams) -> Result<()> {
        set_limit_handle(ctx, params)?;
        Ok(())
    }

    // burn carbon token and mint certificate nft
    pub fn mint_nft(ctx: Context<MintNft>, params: MintNftParams) -> Result<()> {
        mint_nft_handle(ctx, params)?;
        Ok(())
    }

    // update metadata for certificate nft
    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        params: UpdateMetadataParams
    ) -> Result<()> {
        update_metadata_handle(ctx, params)?;
        Ok(())
    }

    // withdraw pool form wallet pda to admin
    pub fn withdraw_pool(ctx: Context<WithdrawPool>, params: WithdrawPoolParams) -> Result<()> {
        withdraw_pool_handle(ctx, params)?;
        Ok(())
    }
}
