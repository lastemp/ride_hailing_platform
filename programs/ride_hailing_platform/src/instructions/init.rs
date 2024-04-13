//! Init instruction handler

use {
    crate::{
        error::RideHailingPlatformError,
        state::{configs::Configs, deposit_base::DepositBase},
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: InitParams)]
pub struct Init<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Configs::INIT_SPACE,
        constraint = !configs.is_initialized @ RideHailingPlatformError::AccountAlreadyInitialized,
        seeds = [b"configs"],
        bump
    )]
    pub configs: Account<'info, Configs>,
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !admin_deposit_account.is_initialized @ RideHailingPlatformError::AccountAlreadyInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"admin-auth", admin_deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin-sol-vault", admin_pda_auth.key().as_ref()], bump)]
    pub admin_sol_vault: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitParams {
    pub driver_share: u32,                         // driver share split on fees
    pub single_trip_to_loyalty_points_mapping: u8, // used to compute loyalty points from a single trip
}

pub fn init(ctx: Context<Init>, params: &InitParams) -> Result<()> {
    msg!("Validate inputs");

    if params.driver_share == 0 {
        return Err(RideHailingPlatformError::InvalidDriverShare.into());
    }
    if params.single_trip_to_loyalty_points_mapping == 0 {
        return Err(RideHailingPlatformError::InvalidSingleTripToLoyaltyPointsMapping.into());
    }

    let deposit_account = &mut ctx.accounts.admin_deposit_account;
    let configs = &mut ctx.accounts.configs;

    // admin deposit account
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.admin_pda_auth;
    deposit_account.admin_sol_vault_bump = Some(ctx.bumps.admin_sol_vault);
    deposit_account.is_initialized = true;

    // configs
    configs.driver_share = params.driver_share;
    configs.single_trip_to_loyalty_points_mapping = params.single_trip_to_loyalty_points_mapping;
    configs.active = true;
    configs.is_initialized = true;

    Ok(())
}
