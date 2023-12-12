use anchor_lang::prelude::*;

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
