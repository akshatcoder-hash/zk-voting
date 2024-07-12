use anchor_lang::prelude::*;
use crate::state::UserState;

pub fn handler(ctx: Context<RewardParticipant>) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;
    
    user_state.reward_points += 1;

    msg!("Rewarded participant. New reward points: {}", user_state.reward_points);

    Ok(())
}

#[derive(Accounts)]
pub struct RewardParticipant<'info> {
    #[account(mut)]
    pub user_state: Account<'info, UserState>,
    pub user: Signer<'info>,
}