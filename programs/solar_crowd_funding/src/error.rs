use anchor_lang::prelude::*;

#[error_code]
pub enum SolarCrowdFundingError {
    // project
    #[msg("Invalid project reference length")]
    InvalidProjectReferenceLength,
    #[msg("Invalid project name length")]
    InvalidProjectNameLength,
    #[msg("Invalid country length")]
    InvalidCountryLength,
    #[msg("Invalid project funds amount.")]
    InvalidProjectFunds,
    #[msg("Project must be active.")]
    InvalidProjectStatus,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Project funds fully raised.")]
    DeclineInvestorFunds,
    #[msg("Invalid project bill amount.")]
    InvalidProjectBillAmount,
    #[msg("Amount paid is not equal to project bill amount set.")]
    InvalidProjectBillAmountPaid,

    // investor
    #[msg("Invalid full names length")]
    InvalidFullNamesLength,

    // institution
    #[msg("Invalid institution name length")]
    InvalidInstitutionNameLength,

    //
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,

    //
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
