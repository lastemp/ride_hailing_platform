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
}
