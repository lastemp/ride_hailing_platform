// admin instructions
pub mod init;

// public instructions
pub mod pay_trip;
pub mod register_driver;
pub mod register_rider;
pub mod request_trip;
pub mod withdraw_driver_funds;

// bring everything in scope
pub use {
    init::*, pay_trip::*, register_driver::*, register_rider::*, request_trip::*,
    withdraw_driver_funds::*,
};
