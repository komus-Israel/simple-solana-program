use anchor_lang::prelude::*;

declare_id!("21HrvicsFGysih8H4CfUEM41KG6cpZUqqvUz4eiikydC");

#[program]
pub mod spl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
