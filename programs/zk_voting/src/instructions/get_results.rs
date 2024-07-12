use anchor_lang::prelude::*;
use crate::state::Proposal;

pub fn handler(ctx: Context<GetResults>, proposal_id: u64) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    
    require!(proposal.id == proposal_id, ErrorCode::InvalidProposal);

    msg!("Proposal {} results:", proposal_id);
    msg!("Yes votes: {}", proposal.yes_votes);
    msg!("No votes: {}", proposal.no_votes);

    Ok(())
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    pub proposal: Account<'info, Proposal>,
}