use anchor_lang::prelude::*;

use crate::{DcarbonError, ProjectState};

#[derive(Accounts)]
#[instruction(params: UpdateConfigParams)]
pub struct UpdateConfig<'info> {
    #[account(mut, address = project_state.admin)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [ProjectState::SEED_PREFIX, project_state.mint.to_bytes().as_ref()],
        bump
    )]
    pub project_state: Account<'info, ProjectState>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateConfigParams {
    // pub device_id: String,
    pub device_id: [u8; 20],
}

pub fn enable_device_handle(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
    let project_state = &mut ctx.accounts.project_state;

    let device = project_state
        .devices
        .iter_mut()
        // .find(|item| item.id == params.device_id);
        .find(|item| item.evm_address == params.device_id);
    require!(device.is_some(), DcarbonError::DeviceNotFound);

    let device = device.unwrap();
    require!(device.limit_amount > 0, DcarbonError::LimitShouldBePositive);

    device.is_actived = true;
    Ok(())
}

pub fn suspend_device_handle(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
    let project_state = &mut ctx.accounts.project_state;

    let device = project_state
        .devices
        .iter_mut()
        // .find(|item| item.id == params.device_id);
        .find(|item| item.evm_address == params.device_id);
    require!(device.is_some(), DcarbonError::DeviceNotFound);

    let device = device.unwrap();
    device.is_actived = false;
    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct SetLimitParams {
    // pub device_id: String,
    pub device_id: [u8; 20],
    pub limit: u64,
}

pub fn set_limit_handle(ctx: Context<UpdateConfig>, params: SetLimitParams) -> Result<()> {
    let project_state = &mut ctx.accounts.project_state;

    let device = project_state
        .devices
        .iter_mut()
        // .find(|item| item.id == params.device_id);
        .find(|item| item.evm_address == params.device_id);
    require!(device.is_some(), DcarbonError::DeviceNotFound);

    let device = device.unwrap();
    device.limit_amount = params.limit;
    Ok(())
}
