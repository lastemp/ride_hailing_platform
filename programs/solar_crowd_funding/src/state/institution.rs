use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Institution {
    pub owner: Pubkey, // publickey of the institution
    #[max_len(30)]
    pub institution_name: String, // institution name
    #[max_len(3)]
    pub country: String, // home country of institution
    pub active: bool,  // status of institution
}
