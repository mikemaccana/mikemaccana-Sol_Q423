// See https://wonderful-stock-089.notion.site/Maker-Taker-Escrow-In-Plain-English-3ac1c043a5a4432698125050787e3eb7
// See https://www.anchor-lang.com/docs/account-constraints#spl-constraints

use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token}, associated_token::AssociatedToken};

declare_id!("Hh2kzENRayrRsGJz2eUumxtATkBCTgAu3N5R7SrCcmvG");

pub const ANCHOR_DISCRIMINATOR_BYTES: usize = 8;
pub const U8_BYTES: usize = 1;
pub const U64_BYTES: usize = 8;
pub const PUBKEY_BYTES: usize = 32;


#[program]
pub mod escrow {
    use super::*;

    // Make an Offer instance
    pub fn initialize(ctx: Context<InitializeOptions>) -> Result<()> {
        Ok(())
    }
}


// Was called "Initialize"
// The constraints for the Initialize instruction
// info is just a name we're giving for the lifetime
// but info is the convention to name the lifetime for Anchor
// under the hood we have 'account info' and 'account meta'
#[derive(Accounts)]
pub struct InitializeOptions<'info> {
    // We can use the Signer macro or the Signer type
    // both do the validation - the #[account] macros are the constraint
    // #[account(mut Signer)]
    //
    // Signer type allows any account to be able to sign (including program accounts)
    #[account(mut)]
    pub maker: Signer<'info>,

    // Mint's are effectively the 'currency' 
    pub offer_token: Account<'info, Mint>,
    pub desired_token: Account<'info, Mint>,

    // What holds the balance
    pub maker_token_account: Account<'info, TokenAccount>,

    // Used by program to sign
    // This is unique for every offer, so if one auth_key is compromised
    // other offers are not compromised. 
    // Only this program can sign with this program ID 
    /// CHECK: I am not a bad person
    #[account(seeds=[b"auth", offer.key().as_ref()], bump)]
    pub auth_key: UncheckedAccount<'info>,

    // key() function is better than key property for various reasons!
    // we use 'le' as convention
    #[account(
        init, 
        payer=signer, 
        seeds=[
            b"auth", 
            maker.key().as_ref(), 
            id.to_le_bytes().as_ref()
        ], 
        bump, 
        space=Offer::INIT_SPACE)
    ]
    pub offer: Account<'info, Offer>,
        #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = offer_token,
        associated_token::authority = auth
    )]
    pub vault: Account<'info, TokenAccount>,

    // Program just means an account that is executable
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}


// because Initialize uses <'info>
// we have to specify it - twice!
impl <'info> InitializeOptions<'info>  {
    
}



#[account]
// Was called Escrow
pub struct Offer {
    // Identifier 
    // Was called Seed
    // No need for pub as we will be using methods in impl
    id: u64,

    // Person making offer
    maker: Pubkey,
    offer_token: Pubkey,
    // Amount maker is offering is determined by what they put in the vault

    // What the maker wants back
    desired_token: Pubkey,
    amount: u64,   

    // Faster to not have to recalculate the bumps 
    vault_bump: u8,
    auth_bump: u8,
    escrow_bump: u8  
} 

// Make space for the Offer/Escrow 
impl Space for Offer {
    // ANCHOR DISCRUMINATOR is always 8 bytes
    // 8 bytes for the u64 = 64 bits is 8 bytes
    // 32 for every pubkey
    // u8s are 1 bytes
    const INIT_SPACE: usize = 
        ANCHOR_DISCRIMINATOR_BYTES 
        + U64_BYTES 
        + 3 * PUBKEY_BYTES 
        + U64_BYTES 
        + 3 * U8_BYTES;
}
