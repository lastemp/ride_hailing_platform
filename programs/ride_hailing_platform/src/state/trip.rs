use anchor_lang::prelude::*;

#[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct GpsCoordinates {
    #[max_len(10)]
    pub latitude: String, // latitude
    #[max_len(10)]
    pub longitude: String, // longitude
}

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Trip {
    pub rider: Pubkey,               // publickey of rider
    pub driver: Pubkey,              // publickey of driver
    pub origin: GpsCoordinates,      // origin of trip
    pub destination: GpsCoordinates, // destination of trip
    pub amount: u32,                 // trip amount
    pub completed: bool,             // completion status of trip
    pub paid: bool,                  // trip paid status
}
