use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod zk;

use state::{DaoState, Proposal, UserState};

declare_id!("ABTDuP27AJX7ZDcSFcqcEYt86aC9M19EjQ4vDn6KLcZw");

#[program]
pub mod zk_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let dao_state = &mut ctx.accounts.dao_state;
        dao_state.authority = *ctx.accounts.authority.key;
        dao_state.proposal_count = 0;

        msg!("DAO initialized with authority: {:?}", dao_state.authority);
        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
        let dao_state = &mut ctx.accounts.dao_state;
        let proposal = &mut ctx.accounts.proposal;

        proposal.id = dao_state.proposal_count;
        proposal.description = description;
        proposal.encrypted_votes = Vec::new();
        proposal.is_active = true;

        dao_state.proposal_count += 1;

        Ok(())
    }

    pub fn cast_vote(
        ctx: Context<CastVote>,
        proposal_id: u64,
        encrypted_vote: [u8; 64]
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let user_state = &mut ctx.accounts.user_state;

        require!(proposal.is_active, errors::ErrorCode::ProposalNotActive);
        require!(!user_state.voted_proposals.contains(&proposal_id), errors::ErrorCode::AlreadyVoted);

        proposal.encrypted_votes.push(encrypted_vote);
        user_state.voted_proposals.push(proposal_id);
        user_state.reward_points += 1;

        Ok(())
    }

    pub fn get_results(ctx: Context<GetResults>, proposal_id: u64) -> Result<()> {
        let proposal = &ctx.accounts.proposal;
        
        require!(proposal.id == proposal_id, errors::ErrorCode::InvalidProposal);

        let total_votes = proposal.encrypted_votes.len();
        msg!("Proposal {} has {} total votes", proposal_id, total_votes);

        Ok(())
    }

    pub fn reward_participant(ctx: Context<RewardParticipant>) -> Result<()> {
        let user_state = &mut ctx.accounts.user_state;
        user_state.reward_points += 1;
        msg!("Rewarded participant. New reward points: {}", user_state.reward_points);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8
    )]
    pub dao_state: Account<'info, DaoState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub dao_state: Account<'info, DaoState>,
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 200 + 32 + 1 + 1000 // Adjust space calculation based on your needs
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + (4 + 8 * 10), // Adjust space as needed
        seeds = [b"user_state", user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,
    pub dao_state: Account<'info, DaoState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    pub proposal: Account<'info, Proposal>,
    pub dao_state: Account<'info, DaoState>,
}

#[derive(Accounts)]
pub struct RewardParticipant<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + (4 + 8 * 10),
        seeds = [b"user_state", user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}