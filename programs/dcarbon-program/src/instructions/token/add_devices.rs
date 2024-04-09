use ::{ anchor_lang::prelude::*, std::str::FromStr };

use crate::{ Device, DeviceParams, ProjectState };

#[derive(Accounts)]
#[instruction(params: AddDevicesParams)]
pub struct AddDevices<'info> {
    #[account(mut, address = project_state.admin)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        realloc = ProjectState::BASE_SIZE + Device::SIZE *(project_state.devices.len() +  params.devices.len()), 
        realloc::payer = owner, 
        realloc::zero = false,
        seeds = [ProjectState::SEED_PREFIX, project_state.mint.to_bytes().as_ref()],
        bump
    )]
    pub project_state: Account<'info, ProjectState>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddDevicesParams {
    devices: Vec<DeviceParams>,
}

pub fn add_devices_handle(ctx: Context<AddDevices>, params: AddDevicesParams) -> Result<()> {
    let project_state = &mut ctx.accounts.project_state;

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
