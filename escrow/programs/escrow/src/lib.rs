// See https://www.anchor-lang.com/docs/account-constraints#spl-constraints
// See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

// 'anchor sync' to update
declare_id!("Hh2kzENRayrRsGJz2eUumxtATkBCTgAu3N5R7SrCcmvG");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
pub const U8_SIZE: usize = 1;
pub const U64_SIZE: usize = 8;
pub const PUBKEY_SIZE: usize = 32;

#[program]
pub mod mikes_cool_escrow {
    use super::*;
    use anchor_spl::token::{transfer, Transfer};

    // Make an Offer instance
    // Was called Initialize
    // Whoever signs this is the 'maker'
    // Deposits an asset (of whatever token mint)
    // Requests an amount of a particular token mint
    pub fn make_offer(
        context: Context<MakeOfferAccountConstraints>,
        // Was 'Deposit'
        deposit_amount: u64,
        // Was 'Receive'
        desired_amount: u64,
    ) -> Result<()> {
        // make a variable 'id' set from the unix timestamp
        // We'll use this to identify the offer
        // TODO: same account makes an offer at the same time
        let id: u64 = Clock::get()?.unix_timestamp as u64;

        // Set the values for an Offer account (which will hold details of this offer)
        // set_inner() saves us from having to do context.accounts.offer.a = a, b = b etc. since we can just use a Struct
        context.accounts.offer.set_inner(Offer {
            id,
            offer_token: context.accounts.offer_token.key(),
            desired_token: context.accounts.desired_token.key(),
            desired_amount,
            bump: context.bumps.offer,
            vault_bump: context.bumps.vault,
        });

        // Send some tokens to our vault account
        let transfer_accounts = Transfer {
            from: context.accounts.maker_token_account.to_account_info(),
            to: context.accounts.vault.to_account_info(),
            authority: context.accounts.maker.to_account_info(),
        };
        let cpi_context = CpiContext::new(
            context.accounts.token_program.to_account_info(),
            transfer_accounts,
        );
        transfer(cpi_context, deposit_amount)
    }

    pub fn refund_offer(context: Context<RefundOfferAccountConstraints>) -> Result<()> {
        // See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
        // 55m42s
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"offer",
            context.accounts.maker.to_account_info().key.as_ref(),
            &context.accounts.offer.id.to_le_bytes()[..],
        ]];

        // Release the tokens to our vault account
        let transfer_accounts = Transfer {
            from: context.accounts.vault.to_account_info(),
            to: context.accounts.maker_token_account.to_account_info(),
            authority: context.accounts.offer.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            context.accounts.token_program.to_account_info(),
            transfer_accounts,
            &signer_seeds,
        );

        transfer(cpi_context, context.accounts.vault.amount)?;

        Ok(())
    }

    // Whoever signs this is the 'taker'
    // Taker deposits assets (is that in the same transaction)
    // pub fn takeOffer(context: Context<TakeConstraints>) -> Result<()> {
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
    maker: Signer<'info>,

    // The currencies we're swapping
    // Was called 'mint_a'
    offer_token: Account<'info, Mint>,
    // Was called 'mint_b'
    desired_token: Account<'info, Mint>,

    // WMakeOffer() will transfer the balance out of the Makers ATA account
    // Was 'maker_ata_a'
    // Add account constraints to ensure we can change it, and the account matches offer_token
    #[account(
        mut,
        associated_token::mint = offer_token,
        associated_token::authority = maker
    )]
    maker_token_account: Account<'info, TokenAccount>,

    // Was calld 'Escrow'
    #[account(
        init,
        payer = maker,
        space = Offer::INIT_SPACE,
        seeds = [
            b"offer", 
            maker.key().as_ref(),  
            id.to_le_bytes().as_ref(),
        ],
        bump
    )]
    offer: Account<'info, Offer>,

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
    vault: Account<'info, TokenAccount>,

    // Program just means an account that is executable
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct RefundOfferAccountConstraints<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    offer_token: Account<'info, Mint>,

    // We'll refund to the maker token account
    #[account(
        mut,
        associated_token::mint = offer_token,
        associated_token::authority = maker
    )]
    maker_token_account: Account<'info, TokenAccount>,

    // We'll close the offer account
    #[account(
        mut,
        close = maker,
        seeds = [
            b"offer", 
            maker.key().as_ref(),
            offer.id.to_le_bytes().as_ref(),
        ],
        bump  
    )]
    offer: Account<'info, Offer>,

    // We'll empty the vault
    #[account(
        seeds = [
            b"vault", 
            offer.key().as_ref(),
        ],
        bump = offer.vault_bump,
        token::mint = offer_token,
        token::authority = offer
    )]
    vault: Account<'info, TokenAccount>,

    // Program just means an account that is executable
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
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
    vault_bump: u8,
}

// Make space for the Offer/Escrow
impl Space for Offer {
    // ANCHOR DISCRUMINATOR is always 8 bytes
    // 8 bytes for the u64 = 64 bits is 8 bytes
    // 32 for every pubkey
    // u8s are 1 bytes
    // TODO: can    I use sizeOf() or similar?
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR_SIZE 
        + 1 * U64_SIZE // id
        + 2 * PUBKEY_SIZE // offer_token, desired_token
        + 1 * U64_SIZE // desired_amount
        + 2 * U8_SIZE; // bump, vault_bump
}
