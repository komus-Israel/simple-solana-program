use anchor_lang::prelude::*;

//  Specifies the program's on-chain address
declare_id!("5QZMpEhUMqVmnMtLJ4snAQEnkWsxrP9KYYCqzweZhgsP");

/// Annotates the module containing all instructions handler for the program
#[program]
pub mod hello {
    use super::*;

    /// Each public function within this module corresponds to an instruction that can be invoked


    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

//  Applied to structs to indicate a list of accounts required by an instruction
//  This macro is applied to a struct to specify the accounts that must be provided
//  when an instruction is invoked
#[derive(Accounts)]

//  Each field in the struct represents an account required by an instruction
//  The naming of each field is arbitrary but should be descriptive of the account's purpose
pub struct Initialize <'info>{

    //  applied to structs to create custom account types for the program
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//  Applied to structs that define the structure of the data stored in custom accounts created by the program
#[account]
pub struct NewAccount {
    data: u64
}