use anchor_lang::prelude::*;

#[account]
pub struct DaoState {
    pub authority: Pubkey,
    pub proposal_count: u64,
}

#[account]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub is_active: bool,
}

#[account]
pub struct UserState {
    pub has_voted: bool,
    pub reward_points: u64,
}