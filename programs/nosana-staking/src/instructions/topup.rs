use crate::*;
use anchor_spl::token::{Token, TokenAccount};
use nosana_common::{nos, transfer_tokens, NosanaError};

#[derive(Accounts)]
pub struct Topup<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [ b"vault", nos::ID.key().as_ref(), authority.key().as_ref() ],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = authority @ NosanaError::Unauthorized,
        constraint = stake.time_unstake == 0 @ NosanaError::StakeAlreadyUnstaked,
    )]
    pub stake: Account<'info, StakeAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Topup>, amount: u64) -> Result<()> {
    // get stake account and topup stake
    let stake: &mut Account<StakeAccount> = &mut ctx.accounts.stake;
    stake.topup(amount);

    // transfer tokens to the vault
    transfer_tokens(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.from.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        0, // skip signature
        amount,
    )
}
