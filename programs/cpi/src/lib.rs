use anchor_lang::prelude::*;

declare_id!("8YxNd8ZrJSya1fHnxrt83QBf47TLYpvdPUaZDxzUM66g");

#[program]
pub mod cpi {
    use crate::instruction::SolTransfer;

    use super::*;

    pub fn sol_transfer(ctx: Context<SolTransfer>) -> Result<()> {
        Ok(());
    }
   
}
#[derive(Accounts)]
pub struct SolTransfer <'info> {

    #[account(mut)]
    sender: Signer<'info>,

    #[account(mut)]
    recipient: SystemAccount<'info>,
    
    system_program: Program<'info, System>,
}

