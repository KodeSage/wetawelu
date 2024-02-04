use anchor_lang::prelude::*;

// This is the Data Structure of a Project that will be hosted on Wetawelu Platform
#[account]
pub struct Project {
    pub beneficiary: Pubkey,
    pub creator: String,
    pub name: String,
    pub description: String,
    pub image_link: String,
    pub twitter: String,
    pub discord: String,
    pub github: String,
    pub raised: u64,
    pub goal: String,
    pub contributions: u8,
    pub bump: u8,
    pub time_stamp: i64,
    pub category: String,
    pub balance: u64,
    pub last_updated: i64,
}

impl Project {
    pub const LEN: usize = 8 + 8 + 32 + 6500;
}
