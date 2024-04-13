//! Init instruction handler

use {
    crate::{
        error::SolarCrowdFundingError,
        state::{deposit_base::DepositBase, project::Project},
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
        space = 8 + Project::INIT_SPACE,
        constraint = !project.is_initialized @ SolarCrowdFundingError::AccountAlreadyInitialized,
        seeds = [b"project"],
        bump
    )]
    pub project: Account<'info, Project>,
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !admin_deposit_account.is_initialized @ SolarCrowdFundingError::AccountAlreadyInitialized
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
    pub project_reference: String, // unique reference of the project
    pub project_name: String,      // project name
    pub country: String,           // home country where project is implemented
    pub project_funds: u32,        // funds needed for completion of project
    pub bill_amount: u32, // this is the amount meant to be paid(monthly basis) by institution that acquired solar project
}

// project reference length
const PROJECT_REFERENCE_LENGTH: usize = 20;
// project name length
const PROJECT_NAME_LENGTH: usize = 30;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn init(ctx: Context<Init>, params: &InitParams) -> Result<()> {
    msg!("Validate inputs");
    if params.project_reference.as_bytes().len() > 0
        && params.project_reference.as_bytes().len() <= PROJECT_REFERENCE_LENGTH
    {
    } else {
        return Err(SolarCrowdFundingError::InvalidProjectReferenceLength.into());
    }
    if params.project_name.as_bytes().len() > 0
        && params.project_name.as_bytes().len() <= PROJECT_NAME_LENGTH
    {
    } else {
        return Err(SolarCrowdFundingError::InvalidProjectNameLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(SolarCrowdFundingError::InvalidCountryLength.into());
    }
    if params.project_funds == 0 {
        return Err(SolarCrowdFundingError::InvalidProjectFunds.into());
    }
    if params.bill_amount == 0 {
        return Err(SolarCrowdFundingError::InvalidProjectBillAmount.into());
    }

    let deposit_account = &mut ctx.accounts.admin_deposit_account;
    let project = &mut ctx.accounts.project;

    // admin deposit account
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.admin_pda_auth;
    deposit_account.admin_sol_vault_bump = Some(ctx.bumps.admin_sol_vault);
    deposit_account.is_initialized = true;

    // project
    project.project_reference = params.project_reference.to_string();
    project.project_name = params.project_name.to_string();
    project.country = params.country.to_string();
    project.project_funds = params.project_funds;
    project.bill_amount = params.bill_amount;
    project.active = true;
    project.is_initialized = true;
    project.is_launched = true;

    Ok(())
}
