//! RegisterRider instruction handler

use {
    crate::{error::RideHailingPlatformError, state::rider::Rider},
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: RegisterRiderParams)]
pub struct RegisterRider<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Rider::INIT_SPACE,
        seeds = [b"rider", owner.key().as_ref()],
        bump
    )]
    pub rider: Account<'info, Rider>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterRiderParams {
    full_names: String, // full names i.e first name, middlename, surname
    country: String,    // country of rider
}

// full names length
const FULL_NAMES_LENGTH: usize = 50;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_rider(ctx: Context<RegisterRider>, params: &RegisterRiderParams) -> Result<()> {
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

    let rider = &mut ctx.accounts.rider;

    // * - means dereferencing
    rider.owner = *ctx.accounts.owner.key;
    rider.full_names = params.full_names.to_string();
    rider.country = params.country.to_string();
    rider.active = true;

    Ok(())
}
