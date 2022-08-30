use crate::*;
use nosana_staking::StakeAccount;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, seeds = [ b"stats" ], bump = stats.bump)]
    pub stats: Account<'info, StatsAccount>,
    #[account(mut, close = authority, has_one = authority @ NosanaError::Unauthorized)]
    pub reward: Account<'info, RewardAccount>,
    #[account(has_one = authority @ NosanaError::Unauthorized)]
    pub stake: Account<'info, StakeAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<Close>) -> Result<()> {
    // get reward and stats account
    let reward: &mut Account<RewardAccount> = &mut ctx.accounts.reward;
    let stats: &mut Account<StatsAccount> = &mut ctx.accounts.stats;

    // test that there are no pending rewards
    require!(
        reward.get_amount(stats.rate) == 0,
        NosanaError::RewardsToClaim
    );

    // safely close account
    stats.remove_rewards_account(reward.reflection, reward.xnos);
    Ok(())
}
