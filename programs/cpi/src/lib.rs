use anchor_lang::prelude::*;

declare_id!("8YxNd8ZrJSya1fHnxrt83QBf47TLYpvdPUaZDxzUM66g");

#[program]
pub mod cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
