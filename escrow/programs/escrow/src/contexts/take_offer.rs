use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::state::Offer;

#[derive(Accounts)]
pub struct TakeOfferAccountConstraints<'info> {
    // Taker is the signer here
    #[account(mut)]
    pub taker: Signer<'info>,

    pub maker: SystemAccount<'info>,

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
