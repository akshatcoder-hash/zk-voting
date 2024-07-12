use anchor_lang::prelude::*;

declare_id!("2FCNqcTDHfNSHKGCj33DRbkgNJkW62JJswtz6X47iCRQ");

#[program]
pub mod zk_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
