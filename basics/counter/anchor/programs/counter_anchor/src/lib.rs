#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("BmDHboaj1kBUoinJKKSRqKfMeRKJqQqEbUj1VgzeQe4A");

#[cfg(feature = "certora")]
mod certora;

#[cfg(feature = "certora")]
use nondet::Nondet;

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize_counter(_ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + Counter::INIT_SPACE,
        payer = payer
    )]
    pub counter: Account<'info, Counter>,
    pub owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = owner)]
    pub counter: Account<'info, Counter>,
    pub owner: Signer<'info>
}

#[account]
#[derive(InitSpace)]
#[cfg_attr(feature = "certora", derive(Nondet))]
pub struct Counter {
    owner: Pubkey,
    count: u64,
}
