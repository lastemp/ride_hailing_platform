//! RequestTrip instruction handler

use {
    crate::{
        error::RideHailingPlatformError,
        state::driver::Driver,
        state::rider::Rider,
        state::trip::{GpsCoordinates, Trip},
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: RequestTripParams)]
pub struct RequestTrip<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Trip::INIT_SPACE,
        seeds = [b"trip", owner.key().as_ref()],
        bump
    )]
    pub trip: Account<'info, Trip>,
    #[account(mut, has_one = owner)]
    pub rider: Account<'info, Rider>,
    #[account(mut)]
    pub driver: Account<'info, Driver>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RequestTripParams {
    origin: GpsCoordinates,      // origin of trip
    destination: GpsCoordinates, // destination of trip
    amount: u32,                 // trip amount
}

pub fn request_trip(ctx: Context<RequestTrip>, params: &RequestTripParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");

    if params.origin.latitude.as_bytes().len() == 0 || params.origin.longitude.as_bytes().len() == 0
    {
        return Err(RideHailingPlatformError::InvalidGpsCoordinates.into());
    }

    if params.destination.latitude.as_bytes().len() == 0
        || params.destination.longitude.as_bytes().len() == 0
    {
        return Err(RideHailingPlatformError::InvalidGpsCoordinates.into());
    }

    if params.amount == 0 {
        return Err(RideHailingPlatformError::InvalidAmount.into());
    }

    // origin
    let origin_latitude = params.origin.latitude.replace(" ", "").to_string();
    let origin_longitude = params.origin.longitude.replace(" ", "").to_string();
    let is_valid_latitude = origin_latitude.trim().parse::<f32>().is_ok();
    let is_valid_longitude = origin_longitude.trim().parse::<f32>().is_ok();

    if !is_valid_latitude || !is_valid_longitude {
        return Err(RideHailingPlatformError::InvalidGpsCoordinates.into());
    }

    // destination
    let destination_latitude = params.destination.latitude.replace(" ", "").to_string();
    let destination_longitude = params.destination.longitude.replace(" ", "").to_string();
    let is_valid_latitude = destination_latitude.trim().parse::<f32>().is_ok();
    let is_valid_longitude = destination_longitude.trim().parse::<f32>().is_ok();

    if !is_valid_latitude || !is_valid_longitude {
        return Err(RideHailingPlatformError::InvalidGpsCoordinates.into());
    }

    let trip = &mut ctx.accounts.trip;
    let driver = &mut ctx.accounts.driver;

    // * - means dereferencing
    trip.rider = *ctx.accounts.owner.key;
    trip.driver = driver.owner;
    trip.amount = params.amount;
    trip.origin.latitude = origin_latitude;
    trip.origin.longitude = origin_longitude;
    trip.destination.latitude = destination_latitude;
    trip.destination.longitude = destination_longitude;
    trip.completed = true;

    Ok(())
}
