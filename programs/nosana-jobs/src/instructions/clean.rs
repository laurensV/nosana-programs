use crate::*;

#[derive(Accounts)]
pub struct Clean<'info> {
    #[account(
        mut,
        close = payer,
        has_one = payer @ NosanaError::Unauthorized,
        constraint = job.status == JobStatus::Done as u8 @ NosanaError::JobInWrongState,
        constraint = market.job_expiration < Clock::get()?.unix_timestamp - job.time_end
            @ NosanaError::JobNotExpired,
    )]
    pub job: Account<'info, JobAccount>,
    /// CHECK: this account is verified as the original payer for the job
    pub payer: AccountInfo<'info>,
    pub market: Account<'info, MarketAccount>,
}

pub fn handler(_ctx: Context<Clean>) -> Result<()> {
    Ok(())
}
