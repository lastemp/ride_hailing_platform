use anchor_lang::prelude::*;

#[error_code]
pub enum RideHailingPlatformError {
    // rider/driver
    #[msg("Invalid full names length")]
    InvalidFullNamesLength,
    #[msg("Invalid country length")]
    InvalidCountryLength,
    #[msg("Invalid rider for the trip")]
    InvalidTripRider,
    #[msg("Invalid driver for the trip")]
    InvalidTripDriver,
    #[msg("Driver has no active status.")]
    InvalidDriverStatus,

    // configs
    #[msg("Driver share has invalid value.")]
    InvalidDriverShare,
    #[msg("Single trip to loyalty points mapping has invalid value.")]
    InvalidSingleTripToLoyaltyPointsMapping,

    // trip
    #[msg("Gps coordinates has invalid values.")]
    InvalidGpsCoordinates,
    #[msg("Amount paid is not equal to trip bill amount set.")]
    InvalidTripBillAmountPaid,
    #[msg("Trip must be completed.")]
    InvalidTripStatus,

    // amount
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
    #[msg("Invalid withdrawal amount.")]
    InvalidWithdrawalAmount,

    //
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,

    //
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
