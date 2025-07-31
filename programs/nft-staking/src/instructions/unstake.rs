use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::{accounts::MasterEdition, types::Collection},
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::{
    error::ErrorCode, StakeAccount, StakeConfig, UserAccount, STAKE_ACCOUNT_SEED,
    STAKE_CONFIG_SEED, USER_ACCOUNT_SEED,
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,
    //? So the metadata account should have created before the mint
    #[account(
        mut,
        seeds=[b"metadata", metadata_token_program.key().as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = metadata_token_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key == collection_mint.key(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        mut,
        seeds=[b"metadata", metadata_token_program.key().as_ref(), mint.key().as_ref(), b"edition"],
        bump,
        seeds::program = metadata_token_program.key(),
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [USER_ACCOUNT_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [STAKE_CONFIG_SEED.as_bytes()],
        bump = stake_config.bump
    )]
    pub stake_config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [STAKE_ACCOUNT_SEED.as_bytes(), mint.key().as_ref(), stake_config.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub metadata_token_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    pub fn process_stake(&mut self) -> Result<()> {
        Ok(())
    }
}
