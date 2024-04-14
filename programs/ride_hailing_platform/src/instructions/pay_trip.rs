//! PayTrip instruction handler

use {
    crate::{
        error::RideHailingPlatformError,
        state::{
            configs::Configs, deposit_base::DepositBase, driver::Driver, rider::Rider, trip::Trip,
        },
    },
    anchor_lang::{prelude::*, system_program},
};

#[derive(Accounts)]
#[instruction(params: PayTripParams)]
pub struct PayTrip<'info> {
    #[account(mut, has_one = owner, constraint = rider.owner == trip.rider @ RideHailingPlatformError::InvalidTripRider)]
    pub rider: Account<'info, Rider>,
    // mut makes it changeble (mutable)
    /// CHECK: trip account for completed status
    #[account(
        mut, constraint = trip.completed @ RideHailingPlatformError::InvalidTripStatus
    )]
    pub trip: Account<'info, Trip>,
    #[account(mut)]
    pub configs: Account<'info, Configs>,
    #[account(mut, constraint = driver.owner == trip.driver @ RideHailingPlatformError::InvalidTripDriver)]
    pub driver: Account<'info, Driver>,
    //admin accs
    #[account(mut,
        constraint = admin_deposit_account.is_initialized @ RideHailingPlatformError::AccountNotInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"admin-auth", admin_deposit_account.key().as_ref()], bump = admin_deposit_account.admin_auth_bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin-sol-vault", admin_pda_auth.key().as_ref()], bump = admin_deposit_account.admin_sol_vault_bump.unwrap())]
    pub admin_sol_vault: SystemAccount<'info>,
    //admin accs
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PayTripParams {
    pub amount_paid: u32, // trip amount paid by rider
}

pub fn pay_trip(ctx: Context<PayTrip>, params: &PayTripParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.amount_paid == 0 {
        return Err(RideHailingPlatformError::InvalidAmount.into());
    }

    let deposit_auth = &ctx.accounts.owner;
    let sys_program = &ctx.accounts.system_program;

    let trip = &mut ctx.accounts.trip;
    let rider = &mut ctx.accounts.rider;
    let configs = &mut ctx.accounts.configs;
    let driver = &mut ctx.accounts.driver;

    // this is the amount meant to be paid by rider
    let bill_amount: u32 = trip.amount;
    let loyalty_points_awarded: u32 = rider.loyalty_points_awarded;
    let single_trip_to_loyalty_points_mapping: u32 =
        configs.single_trip_to_loyalty_points_mapping as u32;
    let total_loyalty_points_awarded = configs.total_loyalty_points_awarded;
    let commission_earned = driver.commission_earned;
    let amount_paid = params.amount_paid;

    // Check if amount paid is equal to bill amount
    if bill_amount != amount_paid {
        return Err(RideHailingPlatformError::InvalidTripBillAmountPaid.into());
    }

    // Lets increment this account's total_loyalty_points_awarded with new loyalty points
    configs.total_loyalty_points_awarded = total_loyalty_points_awarded
        .checked_add(single_trip_to_loyalty_points_mapping)
        .ok_or(RideHailingPlatformError::InvalidArithmeticOperation)?;

    // Lets increment this account's loyalty_points_awarded with new loyalty points
    rider.loyalty_points_awarded = loyalty_points_awarded
        .checked_add(single_trip_to_loyalty_points_mapping)
        .ok_or(RideHailingPlatformError::InvalidArithmeticOperation)?;

    trip.paid = true;

    // Lets increment this account's commission_earned with amount_paid
    driver.commission_earned = commission_earned
        .checked_add(amount_paid)
        .ok_or(RideHailingPlatformError::InvalidArithmeticOperation)?;

    let lamports: u64 = 1_000_000_000; // 1 SOL = 1,000,000,000 lamports
    let _amount = (amount_paid as u64)
        .checked_mul(lamports)
        .ok_or(RideHailingPlatformError::InvalidArithmeticOperation)?;

    // transfer sol from rider to treasury vault
    let cpi_accounts = system_program::Transfer {
        from: deposit_auth.to_account_info(),
        to: ctx.accounts.admin_sol_vault.to_account_info(),
    };

    let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi, _amount.into())?;

    Ok(())
}
