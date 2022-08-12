use crate::*;
use nosana_common::error::NosanaError;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        mut,
        has_one = authority @ NosanaError::Unauthorized,
        constraint = stake.time_unstake == 0 @ NosanaError::StakeAlreadyUnstaked,
    )]
    pub stake: Account<'info, StakeAccount>,
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<Unstake>) -> Result<()> {
    // get stake account, and unstake stake
    let stake: &mut Account<StakeAccount> = &mut ctx.accounts.stake;
    stake.unstake(Clock::get()?.unix_timestamp);
    Ok(())
}
