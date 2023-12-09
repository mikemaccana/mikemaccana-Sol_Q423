// See https://wonderful-stock-089.notion.site/Maker-Taker-Escrow-In-Plain-English-3ac1c043a5a4432698125050787e3eb7
// See https://www.anchor-lang.com/docs/account-constraints#spl-constraints

use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token}, associated_token::AssociatedToken};

// 'anchor sync' to update
declare_id!("Hh2kzENRayrRsGJz2eUumxtATkBCTgAu3N5R7SrCcmvG");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
pub const U8_SIZE: usize = 1;
pub const U64_SIZE: usize = 8;
pub const PUBKEY_SIZE: usize = 32;


#[program]
pub mod mikes_cool_escrow {
    use anchor_spl::token::{Transfer, transfer};
    use super::*;

    // Make an Offer instance
    // Was called Initialize
    // Whoever signs this is the 'maker'
    // Deposits an asset (of whatever token mint)
    // Requests an amount of a particular token mint  

    // TODO: why is data a u64 int? Wouldn't it be an array of u64s?
    pub fn makeOffer(
        context: Context<MakeOfferAccountConstraints>, 
        id: u64, 
        // Was Deposit
        deposit_amount: u64,
        // Was Receive
        desired_amount: u64
    ) -> Result<()> {

        // Set the values for an Offer account (which will hold details of this offer)
        // set_inner() saves us from having to do context.accounts.offer.a = a, b = b etc. since we can just use a Struct
        context.accounts.offer.set_inner(Offer {
            id,
            offer_token: context.accounts.offer_token.key(),
            desired_token: context.accounts.desired_token.key(),
            desired_amount,
            bump: context.bumps.offer,
            vault_bump: context.bumps.vault
        });


        // SSend some tokens to our vault account
        let transfer_accounts = Transfer {
            from: context.accounts.maker_token_account.to_account_info(),
            to: context.accounts.vault.to_account_info(),
            authority: context.accounts.maker.to_account_info()
        };
        let cpi_context = CpiContext::new(
            context.accounts.token_program.to_account_info(), 
            transfer_accounts
        );
         transfer(cpi_context, deposit_amount);

    }

    // Whoever signs this is the 'taker'
    // Taker deposits assets (is that in the same transaction)

    // TODO: does taker run TokenProgram.transfer or an instruction from this program?
    // pub fn takeOffer(context: Context<TakeOfferConstraints>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn refundOffer(context: Context<RefundOfferConstraints>) -> Result<()> {
    //     Ok(())
    // }
}


// Was called "Initialize"
// The constraints for the Initialize instruction
// info is just a name we're giving for the lifetime
// but info is the convention to name the lifetime for Anchor
// under the hood we have 'account info' and 'account meta'
// https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
// TODO: 27:44 in vieo - why are these Structs named like they are functions?
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOfferAccountConstraints<'info> {
    // We can use the Signer macro or the Signer type
    // both do the validation - the #[account] macros are the constraint
    // #[account(mut Signer)]
    //
    // Signer type allows any account to be able to sign (including program accounts)
    #[account(mut)]
    pub maker: Signer<'info>,

    // The currencies we're swapping
    // Was called 'mint_a'
    pub offer_token: Account<'info, Mint>,
    // Was called 'mint_b'
    pub desired_token: Account<'info, Mint>,

    // WMakeOffer() will transfer the balance out of the Makers ATA account
    // Was 'maker_ata_a'
    // Add account constraints to ensure we can change it, and the account matches offer_token 
    #[account(
        mut,
        associated_token::mint = offer_token,
        associated_token::authority = maker
    )] 
    pub maker_token_account: Account<'info, TokenAccount>,

    // Was calld 'Escrow'
    #[account(
        init,
        payer = maker,
        space = Offer::INIT_SPACE,
        seeds = [
            b"offer", 
            maker.key().as_ref()
            id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub offer: Account<'info, Offer>,

    // Where we transfer tokens to while we wait for the offer to be taken or refunded
    #[account(
        init,
        payer = maker,
        seeds = [
            b"vault", 
            offer.key().as_ref() 
        ],
        bump,
        token::mint = offer_token,
        token::authority = offer
    )]
    pub vault: Account<'info, TokenAccount>,

    

    // Dean style above, Richard style below
    // pub offer: Account<'info, Offer>,
    //     #[account(
    //     init_if_needed,
    //     payer = maker,
    //     associated_token::mint = offer_token,
    //     associated_token::authority = auth
    // )]

    // Program just means an account that is executable
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    // Richard style below
    // Used by program to sign
    // This is unique for every offer, so if one auth_key is compromised
    // other offers are not compromised. 
    // Only this program can sign with this program ID 
    /// CHECK: insert reason that I am not a bad person
    // #[account(seeds=[b"auth", offer.key().as_ref()], bump)]
    // pub auth_key: UncheckedAccount<'info>,

    // // key() function is better than key property for various reasons!
    // // we use 'le' as convention (also x64 is little endian)
    // #[account(
    //     init, 
    //     payer=signer, 
    //     seeds=[
    //         b"auth", 
    //         maker.key().as_ref(), 
    //         id.to_le_SIZE().as_ref()
    //     ], 
    //     bump, 
    //     space=Offer::INIT_SPACE)
    // ]
    
    

    
}


// because Initialize uses <'info>
// we have to specify it - twice!

// TODO: why are we implementing methods on our constraints?
impl <'info> MakeOfferAccountConstraints<'info>  {
    
}



// Gives us all our serialization / deserialization traits
#[account]
// Was called Escrow
pub struct Offer {
    // Identifier 
    // Was called Seed
    // No need for pub as we will be using methods in impl
    id: u64,

    // Was mint_a
    offer_token: Pubkey,
    // 'Amount' maker is offering is determined by what makers put in the vault
    // TODO: Why isn't offer token mint determined by what maker puts in the vault?

    // What the maker wants back
    // Was mint_b
    desired_token: Pubkey,

    desired_amount: u64,

    // Person making offer
    // maker: Pubkey,

    // Faster to not have to recalculate the bumps 
    bump: u8,
    vault_bump: u8 
} 

// Make space for the Offer/Escrow 
impl Space for Offer {
    // ANCHOR DISCRUMINATOR is always 8 bytes
    // 8 bytes for the u64 = 64 bits is 8 bytes
    // 32 for every pubkey
    // u8s are 1 bytes
    // TODO: can    I use sizeOf() or similar?
    const INIT_SPACE: usize = 
        ANCHOR_DISCRIMINATOR_SIZE 
        + 1 * U64_SIZE // id
        + 2 * PUBKEY_SIZE // offer_token, desired_token
        + 1 * U64_SIZE // desired_amount
        + 2 * U8_SIZE; // bump, vault_bump
}
