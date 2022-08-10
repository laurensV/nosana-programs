use crate::*;
use nosana_common::{authority, nos, NosanaError};

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = authority, space = STATS_SIZE, seeds = [ b"stats" ], bump)]
    pub stats: Account<'info, StatsAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Init>) -> Result<()> {
    // get stats account and init
    let stats: &mut Account<StatsAccount> = &mut ctx.accounts.stats;
    stats.init(authority::ID);
    Ok(())
}
