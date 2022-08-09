use crate::*;
use nosana_common::NosanaError;

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        owner = staking::ID @ NosanaError::WrongOwner,
        has_one = authority @ NosanaError::Unauthorized,
    )]
    pub stats: Account<'info, StatsAccount>,
    pub authority: Signer<'info>,
    pub new_authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateAuthority>) -> Result<()> {
    // get stats account and update authority
    let stats: &mut Account<StatsAccount> = &mut ctx.accounts.stats;
    stats.update_authority(*ctx.accounts.new_authority.key);

    // finish
    Ok(())
}
