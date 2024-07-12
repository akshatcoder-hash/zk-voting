use anchor_lang::prelude::*;
use crate::state::{Proposal, UserState};
use crate::zk::elgamal::{ElGamalPubkey, ElGamalCiphertext};
use crate::zk::proofs::VoteProof;
use crate::errors::ErrorCode;

pub fn handler(ctx: Context<CastVote>, proposal_id: u64, encrypted_vote: ElGamalCiphertext, vote_proof: VoteProof) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let user_state = &mut ctx.accounts.user_state;

    require!(proposal.is_active, ErrorCode::ProposalNotActive);
    require!(!user_state.has_voted, ErrorCode::AlreadyVoted);

    // Verify the vote proof
    let dao_pubkey = ElGamalPubkey {
        g: 5, // This should be stored in the DAO state in a real implementation
        h: ctx.accounts.dao_state.public_key,
    };
    require!(
        crate::zk::proofs::verify_vote_proof(&vote_proof, &encrypted_vote, &dao_pubkey),
        ErrorCode::InvalidVoteProof
    );

    // Store the encrypted vote
    proposal.encrypted_votes.push(encrypted_vote);

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
    pub dao_state: Account<'info, DaoState>,
    pub user: Signer<'info>,
}