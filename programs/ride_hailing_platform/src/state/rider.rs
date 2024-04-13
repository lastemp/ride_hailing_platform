use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Rider {
    pub owner: Pubkey, // publickey of the rider
    #[max_len(50)]
    pub full_names: String, // full names i.e first name, middlename, surname
    #[max_len(3)]
    pub country: String, // home country of rider
    pub active: bool,  // status of rider
    pub loyalty_points_awarded: u32, // loyalty points awarded
    pub loyalty_points_redeemed: u32, // loyalty points redeemed
}
