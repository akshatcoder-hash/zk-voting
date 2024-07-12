use anchor_lang::prelude::*;
use crate::zk::elgamal::ElGamalCiphertext;

#[account]
pub struct DaoState {
    pub authority: Pubkey,
    pub proposal_count: u64,
    pub public_key: u64,
    pub private_key: u64,
}

#[account]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub encrypted_votes: Vec<ElGamalCiphertext>,
    pub is_active: bool,
}

#[account]
pub struct UserState {
    pub has_voted: bool,
    pub reward_points: u64,
}