use anchor_lang::prelude::*;
use crate::state::{Proposal, UserState};

pub fn handler(ctx: Context<CastVote>, proposal_id: u64, vote: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let user_state = &mut ctx.accounts.user_state;

    require!(proposal.is_active, ErrorCode::ProposalNotActive);
    require!(!user_state.has_voted, ErrorCode::AlreadyVoted);

    if vote {
        proposal.yes_votes += 1;
    } else {
        proposal.no_votes += 1;
    }

    user_state.has_voted = true;
    user_state.reward_points += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user_state: Account<'info, UserState>,
    pub user: Signer<'info>,
}