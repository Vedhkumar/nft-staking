use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{StakeConfig, STAKE_CONFIG_SEED};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,       
        payer = admin,
        space = 8  + StakeConfig::INIT_SPACE,
        seeds = [STAKE_CONFIG_SEED.as_bytes()],
        bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::decimals = 6, 
        mint::authority = config,
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl <'info> InitializeConfig<'info> {
    pub fn process_initialize_config(&mut self, points_per_stake: u64,
    freeze_period: u64,
    max_stake: u8, bumps: &InitializeConfigBumps)-> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake,
            freeze_period,
            max_stake,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });
        Ok(())
    }
}
