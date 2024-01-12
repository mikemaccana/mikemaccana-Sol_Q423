use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use crate::{state::Offer, constants::ANCHOR_DISCRIMINATOR_SIZE};

// Make an Offer instance
// _____________________________________________________
// Whoever signs this is the 'maker' of the offer
// Maker deposits an asset (of whatever token mint) they are offering to a vault
// Maker specified amount and what token they desire


// Was called "Initialize"
// The constraints for the Initialize instruction
// 'info is just a name we're giving for the lifetime
// but 'info is the convention to name the lifetime for Anchor
// under the hood we have 'account info' and 'account meta'
// https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
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

    // MakeOffer() will transfer the balance out of the Makers ATA account
    // Was 'maker_ata_a'
    // Add account constraints to ensure we can change it, and the account matches offer_token
    #[account(
        mut,
        associated_token::mint = offer_token,
        associated_token::authority = maker
    )]
    pub maker_offered_token_account: Account<'info, TokenAccount>,

    // Was called 'Escrow'
    #[account(
        init,
        payer = maker, 
        // Per https://www.anchor-lang.com/docs/space see InitSpace
        space = ANCHOR_DISCRIMINATOR_SIZE + Offer::INIT_SPACE,
        seeds = [
            b"offer", 
            maker.key().as_ref(),
            id.to_le_bytes().as_ref(),
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

    // Program just means an account that is executable
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// Combined the 'Initialize' and 'saveEscrow' methods in Dean's code
pub fn handler(
    context: Context<MakeOfferAccountConstraints>, 
    // Was 'seed'
    id: u64,
    // Was 'Deposit'
    deposit_amount: u64,
    // Was 'Receive'
    desired_amount: u64
) -> Result<()> {

    let offer = context.accounts.offer;    
    let offer_token = context.accounts.offer_token;
    let desired_token = context.accounts.desired_token;

    // TODO: understand what account_info is
    let token_program_account_info = context.accounts.token_program.to_account_info();
    
    let maker_account_info = context.accounts.maker.to_account_info();
    let maker_offered_token_account_info = context.accounts.maker_offered_token_account.to_account_info();
    let maker_desired_token_account_info = context.accounts.maker_desired_token_account.to_account_info();

    let taker_account_info = context.accounts.taker.to_account_info();
    let taker_offer_token_account_info = context.accounts.taker_offer_token_account.to_account_info();
    let taker_desired_token_account_info = context.accounts.taker_desired_token_account.to_account_info();

    let vault_account_info = context.accounts.vault.to_account_info();

    // Send some tokens to our vault account
    let transfer_accounts = Transfer {
        from: maker_offered_token_account_info,
        to: vault_account_info,
        authority: maker_account_info,
    };
    let cpi_context = CpiContext::new(token_program_account_info, transfer_accounts);
    transfer(cpi_context, deposit_amount);

    // make a variable 'id' set from the unix timestamp
    // We'll use this to identify the offer
    // TODO: what if the same account makes an offer at the same time?
    // to the millisecond or whatever? Edge case but still worth considering
    let id: u64 = Clock::get()?.unix_timestamp as u64;

    // Set the values for an Offer account (which will hold details of this offer)
    // set_inner() saves us from having to do context.accounts.offer.a = a, b = b etc. since we can just use a Struct
    offer.set_inner(Offer {
        id,
        offer_token: offer_token.key(),
        desired_token: desired_token.key(),
        desired_amount,
        bump: &MakeOfferAccountConstraintsBumps::offer,
        vault_bump: &MakeOfferAccountConstraintsBump::vault,
    })

    // TODO: Just double check Dean's code to make sure we're not missing anything at the end here
}
