//! RegisterDriver instruction handler

use {
    crate::{error::RideHailingPlatformError, state::driver::Driver, state::vehicle::Vehicle},
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: RegisterDriverParams)]
pub struct RegisterDriver<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Driver::INIT_SPACE,
        seeds = [b"driver", owner.key().as_ref()],
        bump
    )]
    pub driver: Account<'info, Driver>,
    #[account(
        init,
        payer = owner,
        space = 8 + Vehicle::INIT_SPACE,
        seeds = [b"vehicle", owner.key().as_ref()],
        bump
    )]
    pub vehicle: Account<'info, Vehicle>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VehicleDetails {
    make: String,          // make of the vehicle
    model: String,         // model of the vehicle
    manufacture_date: u16, // vehicle manufacture date in years i.e 2020
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterDriverParams {
    full_names: String,      // full names i.e first name, middlename, surname
    country: String,         // country of driver
    vehicle: VehicleDetails, // vehicle details
}

// full names length
const FULL_NAMES_LENGTH: usize = 50;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_driver(ctx: Context<RegisterDriver>, params: &RegisterDriverParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.full_names.as_bytes().len() > 0
        && params.full_names.as_bytes().len() <= FULL_NAMES_LENGTH
    {
    } else {
        return Err(RideHailingPlatformError::InvalidFullNamesLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(RideHailingPlatformError::InvalidCountryLength.into());
    }

    let driver = &mut ctx.accounts.driver;
    let vehicle = &mut ctx.accounts.vehicle;

    // driver
    driver.owner = *ctx.accounts.owner.key;
    driver.full_names = params.full_names.to_string();
    driver.country = params.country.to_string();
    driver.active = true;

    // vehicle
    vehicle.make = params.vehicle.make.to_string();
    vehicle.model = params.vehicle.model.to_string();
    vehicle.manufacture_date = params.vehicle.manufacture_date;

    Ok(())
}
