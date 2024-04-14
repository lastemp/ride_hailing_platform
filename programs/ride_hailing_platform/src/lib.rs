//! ride_hailing_platform program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("5o1B27Xsk1P4vqo9UBDTSXa5TbZmBV9UdrveUEkaXMQS");

#[program]
pub mod ride_hailing_platform {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }

    // public instructions
    pub fn register_rider(ctx: Context<RegisterRider>, params: RegisterRiderParams) -> Result<()> {
        instructions::register_rider(ctx, &params)
    }

    pub fn register_driver(
        ctx: Context<RegisterDriver>,
        params: RegisterDriverParams,
    ) -> Result<()> {
        instructions::register_driver(ctx, &params)
    }

    pub fn request_trip(ctx: Context<RequestTrip>, params: RequestTripParams) -> Result<()> {
        instructions::request_trip(ctx, &params)
    }

    pub fn pay_trip(ctx: Context<PayTrip>, params: PayTripParams) -> Result<()> {
        instructions::pay_trip(ctx, &params)
    }

    pub fn withdraw_driver_funds(
        ctx: Context<WithdrawDriverFunds>,
        params: WithdrawDriverFundsParams,
    ) -> Result<()> {
        instructions::withdraw_funds(ctx, &params)
    }
}
