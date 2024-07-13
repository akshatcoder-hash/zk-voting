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
    pub encrypted_votes: Vec<[u8; 64]>,  // Simplified for this example
    pub is_active: bool,
}

#[account]
pub struct UserState {
    pub voted_proposals: Vec<u64>,
    pub reward_points: u64,
}