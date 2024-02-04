use anchor_lang::prelude::*;
// This is the Data Structure of a User that will be  part on Wetawelu Platform
#[account]
pub struct User {
    pub authority: Pubkey,
    pub username: String,
    pub contributors: u8,
    pub contributions: u8,
    pub bump: u8,
    pub time_stamp: i64,
    pub image: String,
}

impl User {
    pub const LEN: usize = 8 + 8 + 4 + 256 + 32;
}
