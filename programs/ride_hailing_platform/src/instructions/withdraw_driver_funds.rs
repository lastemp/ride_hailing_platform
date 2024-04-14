//! WithdrawDriverFunds instruction handler

use {
    crate::{
        error::RideHailingPlatformError,
        state::{deposit_base::DepositBase, driver::Driver},
    },
    anchor_lang::{prelude::*, system_program},
};

#[derive(Accounts)]
#[instruction(params: WithdrawDriverFundsParams)]
pub struct WithdrawDriverFunds<'info> {
    #[account(mut, has_one = owner, constraint = driver.active @ RideHailingPlatformError::InvalidDriverStatus)]
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
pub struct WithdrawDriverFundsParams {
    pub withdrawal_amount: u64, // withdrawal amount
}

pub fn withdraw_funds(
    ctx: Context<WithdrawDriverFunds>,
    params: &WithdrawDriverFundsParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.withdrawal_amount == 0 {
        return Err(RideHailingPlatformError::InvalidWithdrawalAmount.into());
    }

    let sys_program = &ctx.accounts.system_program;
    let deposit_account = &ctx.accounts.admin_deposit_account;
    let pda_auth = &mut ctx.accounts.admin_pda_auth;
    let sol_vault = &mut ctx.accounts.admin_sol_vault;

    let driver = &mut ctx.accounts.driver;

    let commission_earned: u32 = driver.commission_earned;

    // convert withdrawal_amount lamports to Sol
    let lamports = 1_000_000_000; // 1 SOL = 1,000,000,000 lamports
    let withdrawal_amount_sol = params
        .withdrawal_amount
        .checked_div(lamports)
        .ok_or(RideHailingPlatformError::InvalidArithmeticOperation)?;

    // Driver's commission earned should exceed withdrawal amount
    if commission_earned as u64 > withdrawal_amount_sol {
    } else {
        return Err(RideHailingPlatformError::InsufficientFunds.into());
    }

    // Deduct withdrawn amount from driver's commission earned
    driver.commission_earned = commission_earned
        .checked_sub(withdrawal_amount_sol as u32)
        .ok_or(RideHailingPlatformError::InvalidArithmeticOperation)?;

    let amount = params.withdrawal_amount;

    // Driver withdraws funds(commission earned) from treasury vault
    let cpi_accounts = system_program::Transfer {
        from: sol_vault.to_account_info(),
        to: ctx.accounts.owner.to_account_info(),
    };

    let seeds = &[
        b"admin-sol-vault",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_sol_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(sys_program.to_account_info(), cpi_accounts, signer);

    system_program::transfer(cpi, amount)?;

    Ok(())
}
