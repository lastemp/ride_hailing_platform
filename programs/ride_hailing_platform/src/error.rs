use anchor_lang::prelude::*;

#[error_code]
pub enum RideHailingPlatformError {
    // rider/driver
    #[msg("Invalid full names length")]
    InvalidFullNamesLength,
    #[msg("Invalid country length")]
    InvalidCountryLength,

    // configs
    #[msg("Driver share has invalid value.")]
    InvalidDriverShare,
    #[msg("Single trip to loyalty points mapping has invalid value.")]
    InvalidSingleTripToLoyaltyPointsMapping,

    //
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,

    //
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
