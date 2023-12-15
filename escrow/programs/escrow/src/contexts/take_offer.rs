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

    // Was 'taker_ata_a'
    #[account(
        // Since the taker may not have an account for the offered token!
        init_if_needed,
        payer = taker,
        associated_token::mint = offer_token,
        associated_token::authority = taker
    )]
    pub taker_offer_token_account: Account<'info, TokenAccount>,

    // Was 'taker_ata_b'
    #[account(
        mut,
        associated_token::mint = desired_token,
        associated_token::authority = taker
    )]
    pub taker_desired_token_account: Account<'info, TokenAccount>,

    // Was 'maker_ata_b'
    #[account(
        // Since the maker may not have an account for their desired token!
        init_if_needed,
        payer = taker,
        associated_token::mint = desired_token,
        associated_token::authority = maker
    )]
    pub maker_desired_token_account: Account<'info, TokenAccount>,

    // Was calld 'Escrow'
    #[account(
        
        seeds = [
            b"offer", 
            maker.key().as_ref(),
            offer.id.to_le_bytes().as_ref(),
        ],
        bump = offer.vault_bump,
        has_one = offer_token,
        // Ensures the taker provides the desired token and not something else
        has_one = desired_token,
    )]
    pub offer: Account<'info, Offer>,

    // Where we transfer tokens to while we wait for the offer to be taken or refunded
    #[account(
        mut,
        seeds = [
            b"vault", 
            offer.key().as_ref()
        ],
        bump = offer.vault_bump,
        token::mint = offer_token,
        token::authority = offer
    )]
    pub vault: Account<'info, TokenAccount>,

    // Program just means an account that is executable
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
