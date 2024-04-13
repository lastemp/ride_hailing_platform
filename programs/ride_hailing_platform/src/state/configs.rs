use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Configs {
    pub driver_share: u32,                         // driver share split on fees
    pub single_trip_to_loyalty_points_mapping: u8, // used to compute loyalty points from a single trip
    pub total_loyalty_points_awarded: u32,         // total loyalty points awarded
    pub total_loyalty_points_redeemed: u32,        // total loyalty points redeemed
    pub active: bool,                              // status of configs
    pub is_initialized: bool,
}
