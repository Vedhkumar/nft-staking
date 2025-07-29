use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    //? Number of NFTs staked by the user
    pub amount_staked: u8,
    pub points: u64,
    pub bump: u8,
}
