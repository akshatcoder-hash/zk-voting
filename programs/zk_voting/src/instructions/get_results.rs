use anchor_lang::prelude::*;
use crate::state::{Proposal, DaoState};
use crate::zk::elgamal::{ElGamalPubkey, decrypt};
use crate::errors::ErrorCode;

pub fn handler(ctx: Context<GetResults>, proposal_id: u64) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    
    require!(proposal.id == proposal_id, ErrorCode::InvalidProposal);

    let dao_private_key = ctx.accounts.dao_state.private_key; 
    let dao_pubkey = ElGamalPubkey {
        g: 5,
        h: ctx.accounts.dao_state.public_key,
    };

    let mut yes_votes = 0;
    let mut no_votes = 0;

    for encrypted_vote in &proposal.encrypted_votes {
        let decrypted_vote = decrypt(encrypted_vote, dao_private_key);
        if decrypted_vote == 1 {
            yes_votes += 1;
        } else {
            no_votes += 1;
        }
    }

    msg!("Proposal {} results:", proposal_id);
    msg!("Yes votes: {}", yes_votes);
    msg!("No votes: {}", no_votes);

    Ok(())
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    pub proposal: Account<'info, Proposal>,
    pub dao_state: Account<'info, DaoState>,
}