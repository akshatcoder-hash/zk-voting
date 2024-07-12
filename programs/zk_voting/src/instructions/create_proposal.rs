use anchor_lang::prelude::*;
use crate::state::{DaoState, Proposal};

pub fn handler(ctx: Context<CreateProposal>, description: String) -> Result<()> {
    let dao_state = &mut ctx.accounts.dao_state;
    let proposal = &mut ctx.accounts.proposal;

    proposal.id = dao_state.proposal_count;
    proposal.description = description;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.is_active = true;

    dao_state.proposal_count += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub dao_state: Account<'info, DaoState>,
    #[account(init, payer = authority, space = 8 + 8 + 200 + 8 + 8 + 1)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}