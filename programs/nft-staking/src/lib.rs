#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8XgPUJ2tYyWNcxgJxP2dQA34sFNo5FMASmfZDiEzagtd");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>) -> Result<()> {
        // initialize_config::handler(ctx)
        Ok(())
    }
}
