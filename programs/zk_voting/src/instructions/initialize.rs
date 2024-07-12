use anchor_lang::prelude::*;
use crate::state::DaoState;
use crate::zk::elgamal::generate_keypair;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let dao_state = &mut ctx.accounts.dao_state;
    dao_state.authority = *ctx.accounts.authority.key;
    dao_state.proposal_count = 0;

    // Generate a keypair for the DAO
    let (public_key, private_key) = generate_keypair();
    dao_state.public_key = public_key.h;
    dao_state.private_key = private_key;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8
    )]
    pub dao_state: Account<'info, DaoState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}