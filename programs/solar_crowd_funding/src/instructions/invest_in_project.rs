//! InvestProject instruction handler

use {
    crate::{
        error::SolarCrowdFundingError,
        state::{deposit_base::DepositBase, investor::Investor, project::Project},
    },
    anchor_lang::{prelude::*, system_program},
};

#[derive(Accounts)]
#[instruction(params: InvestProjectParams)]
pub struct InvestProject<'info> {
    #[account(mut, has_one = owner)]
    pub investor: Account<'info, Investor>,
    // mut makes it changeble (mutable)
    /// CHECK: project account for active status
    #[account(
        mut, constraint = project.is_initialized @ SolarCrowdFundingError::AccountNotInitialized
    )]
    pub project: Account<'info, Project>,
    //admin accs
    #[account(mut,
        constraint = admin_deposit_account.is_initialized @ SolarCrowdFundingError::AccountNotInitialized
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
pub struct InvestProjectParams {
    pub amount_invested: u32, // funds invested for the project by investor
}

pub fn invest_in_project(ctx: Context<InvestProject>, params: &InvestProjectParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.amount_invested == 0 {
        return Err(SolarCrowdFundingError::InvalidAmount.into());
    }

    let deposit_auth = &ctx.accounts.owner;
    let sys_program = &ctx.accounts.system_program;

    let project = &mut ctx.accounts.project;

    let project_funds: u32 = project.project_funds;
    let investor_funds_raised: u32 = project.investor_funds_raised;
    let amount_invested = params.amount_invested;

    project.investors.push(*ctx.accounts.owner.key);

    // Check if investors have met the set project funds
    if investor_funds_raised >= project_funds {
        return Err(SolarCrowdFundingError::DeclineInvestorFunds.into());
    }

    // Lets increment this account's investor_funds_raised with new  amounts invested
    project.investor_funds_raised = investor_funds_raised
        .checked_add(amount_invested)
        .ok_or(SolarCrowdFundingError::InvalidArithmeticOperation)?;

    let lamports: u64 = 1_000_000_000; // 1 SOL = 1,000,000,000 lamports
    let _amount = (amount_invested as u64)
        .checked_mul(lamports)
        .ok_or(SolarCrowdFundingError::InvalidArithmeticOperation)?;

    // transfer sol from investor to treasury vault
    let cpi_accounts = system_program::Transfer {
        from: deposit_auth.to_account_info(),
        to: ctx.accounts.admin_sol_vault.to_account_info(),
    };

    let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi, _amount.into())?;

    Ok(())
}
