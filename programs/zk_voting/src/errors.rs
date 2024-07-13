use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided proposal is not active")]
    ProposalNotActive,
    #[msg("User has already voted on this proposal")]
    AlreadyVoted,
    #[msg("Invalid proposal ID")]
    InvalidProposal,
    #[msg("Invalid vote proof")]
    InvalidVoteProof,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid private key")]
    InvalidPrivateKey,
}