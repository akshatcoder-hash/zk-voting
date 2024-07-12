use anchor_lang::prelude::*;
use crate::state::DaoState;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let dao_state = &mut ctx.accounts.dao_state;
    dao_state.authority = *ctx.accounts.authority.key;
    dao_state.proposal_count = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub dao_state: Account<'info, DaoState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}