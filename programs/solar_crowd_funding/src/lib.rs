//! solar_crowd_funding program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("HNMc2VpNfvPxLizJchUZG5G8tzyFSLfEhShnECiB8EHB");

#[program]
pub mod solar_crowd_funding {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }

    // public instructions
    pub fn register_investor(
        ctx: Context<RegisterInvestor>,
        params: RegisterInvestorParams,
    ) -> Result<()> {
        instructions::register_investor(ctx, &params)
    }

    pub fn register_institution(
        ctx: Context<RegisterInstitution>,
        params: RegisterInstitutionParams,
    ) -> Result<()> {
        instructions::register_institution(ctx, &params)
    }

    pub fn invest_in_project(
        ctx: Context<InvestProject>,
        params: InvestProjectParams,
    ) -> Result<()> {
        instructions::invest_in_project(ctx, &params)
    }

    pub fn pay_solar_bill(ctx: Context<PaySolarBill>, params: PaySolarBillParams) -> Result<()> {
        instructions::pay_solar_bill(ctx, &params)
    }
}
