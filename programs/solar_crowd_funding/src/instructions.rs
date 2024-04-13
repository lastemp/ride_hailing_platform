// admin instructions
//pub mod approve_tree_owner;
pub mod init;

// public instructions
pub mod invest_in_project;
pub mod pay_solar_bill;
pub mod register_institution;
pub mod register_investor;

// bring everything in scope
pub use {
    init::*, invest_in_project::*, pay_solar_bill::*, register_institution::*,
    register_investor::*,
};
