use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Driver {
    pub owner: Pubkey, // publickey of the driver
    #[max_len(50)]
    pub full_names: String, // full names i.e first name, middlename, surname
    #[max_len(3)]
    pub country: String, // home country of driver
    pub active: bool,  // status of driver
    pub commission_earned: u32, // commission earned through completion of trips
}
