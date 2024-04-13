//! PaySolarBill instruction handler

use {
    crate::{
        error::SolarCrowdFundingError,
        state::{deposit_base::DepositBase, institution::Institution, project::Project},
    },
    anchor_lang::{prelude::*, system_program},
};

#[derive(Accounts)]
#[instruction(params: PaySolarBillParams)]
pub struct PaySolarBill<'info> {
    #[account(mut, has_one = owner)]
    pub institution: Account<'info, Institution>,
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
pub struct PaySolarBillParams {
    pub amount_paid: u32, // solar bill amount paid by institution that acquired solar project
}

pub fn pay_solar_bill(ctx: Context<PaySolarBill>, params: &PaySolarBillParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.amount_paid == 0 {
        return Err(SolarCrowdFundingError::InvalidAmount.into());
    }

    let deposit_auth = &ctx.accounts.owner;
    let sys_program = &ctx.accounts.system_program;

    let project = &mut ctx.accounts.project;

    // this is the amount meant to be paid(monthly basis) by institution that acquired solar project
    let bill_amount: u32 = project.bill_amount;
    // this is the amount paid by institution that acquired solar project
    let bill_amount_paid: u32 = project.bill_amount_paid;
    let amount_paid = params.amount_paid;

    // Check if amount paid is equal to bill amount
    if bill_amount != amount_paid {
        return Err(SolarCrowdFundingError::InvalidProjectBillAmountPaid.into());
    }

    // Lets increment this account's bill_amount_paid with new bill amounts paid
    project.bill_amount_paid = bill_amount_paid
        .checked_add(amount_paid)
        .ok_or(SolarCrowdFundingError::InvalidArithmeticOperation)?;

    let lamports: u64 = 1_000_000_000; // 1 SOL = 1,000,000,000 lamports
    let _amount = (amount_paid as u64)
        .checked_mul(lamports)
        .ok_or(SolarCrowdFundingError::InvalidArithmeticOperation)?;

    // transfer sol from institution to treasury vault
    let cpi_accounts = system_program::Transfer {
        from: deposit_auth.to_account_info(),
        to: ctx.accounts.admin_sol_vault.to_account_info(),
    };

    let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi, _amount.into())?;

    Ok(())
}
