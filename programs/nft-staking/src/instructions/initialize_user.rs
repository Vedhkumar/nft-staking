use anchor_lang::prelude::*;

use crate::{UserAccount, USER_ACCOUNT_SEED};

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [USER_ACCOUNT_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    pub fn process_initialize_user(&mut self, bumps: &InitializeUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            amount_staked: 0,
            points: 0,
            bump: bumps.user_account,
        });
        Ok(())
    }
}
