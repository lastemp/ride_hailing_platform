use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Vehicle {
    //pub owner: Pubkey, // publickey of the vehicle
    #[max_len(20)]
    pub make: String, // make of the vehicle
    #[max_len(20)]
    pub model: String, // model of the vehicle
    pub manufacture_date: u16, // vehicle manufacture date in years i.e 2020
}
