use anchor_lang::{prelude::*, solana_program::program_option::COption};



declare_id!("21HrvicsFGysih8H4CfUEM41KG6cpZUqqvUz4eiikydC");

#[program]
pub mod zeno {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

/// MINT ACCOUNT
/// A mint acccount is an account type in Solana's Token program 
/// that uniquely represents a token on the network
/// and stores global metadata about the token, such as its total supply, decimals, and other attributes.

/// Mint data
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mint {
    /// optional authority used to mint new tokens
    pub mint_authority: COption<Pubkey>,
    /// total supply
    pub supply: u64,
    /// number of base 10 digits to the right of the decimal place
    pub decimals: u8,
    /// is `true` if this struct has been initialized
    pub is_initialized: bool,
    /// optional authority that can freeze token accounts
    pub freeze_authority: COption<Pubkey>,
}



#[derive(Accounts)]
pub struct Initialize {}
