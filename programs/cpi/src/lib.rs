use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("8YxNd8ZrJSya1fHnxrt83QBf47TLYpvdPUaZDxzUM66g");

#[program]
pub mod cpi {
    use super::*;

    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from = ctx.accounts.sender.to_account_info();
        let to = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from,
                to: to,
            },
        );
        transfer(cpi_context, amount)?;
        Ok(())
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

