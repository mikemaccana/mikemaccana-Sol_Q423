use crate::state::Offer;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct RefundOfferAccountConstraints {
    #[account(mut)]
    maker: Signer<'info>,

    // Was called 'mint_a'
    offer_token: Account<'info, Mint>,

    // We'll refund to the maker token account
    // Was 'maker_ata_a'
    #[account(
        mut,
        associated_token::mint = offer_token,
        associated_token::authority = maker
    )]
    maker_offered_token_account: Account<'info, TokenAccount>,

    // We'll close the offer account
    // Was 'Escrow'
    #[account(
        mut,
        close = maker,
        has_one = offer_token,
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

pub fn handler(context: Context<Initialize>) -> Result<()> {
    // Refund
    // See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
    // 55m42s
    let offer = context.accounts.offer;

    // Close the vault

    // TODO: understand what account_info is
    let token_program_account_info = context.accounts.token_program.to_account_info();

    let maker_account_info = context.accounts.maker.to_account_info();
    let maker_offered_token_account_info = context
        .accounts
        .maker_offered_token_account
        .to_account_info();
    let maker_desired_token_account_info = context
        .accounts
        .maker_desired_token_account
        .to_account_info();

    let taker_account_info = context.accounts.taker.to_account_info();
    let taker_offer_token_account_info =
        context.accounts.taker_offer_token_account.to_account_info();
    let taker_desired_token_account_info = context
        .accounts
        .taker_desired_token_account
        .to_account_info();

    let vault = context.accounts.vault;
    let vault_account_info = context.accounts.vault.to_account_info();

    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"offer",
        maker_account_info.key.as_ref(),
        &offer.id.to_le_bytes()[..],
        &[offer.bump],
    ]];

    // Empty the vault back to the maker
    let transfer_accounts = Transfer {
        from: vault_account_info,
        to: maker_offered_token_account_info,
        authority: offer.to_account_info(),
    };

    let cpi_context =
        CpiContext::new_with_signer(vault_account_info, transfer_accounts, &signer_seeds);

    transfer(cpi_context, vault.amount)?;

    // Close the vault
    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"offer",
        maker_account_info.key.as_ref(),
        &offer.id.to_le_bytes()[..],
        &[offer.bump],
    ]];

    // Was 'close_accounts'
    let accounts_for_close_instruction = CloseAccount {
        account: vault_account_info,
        destination: maker_account_info,
        authority: offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        token_program_account_info,
        accounts_for_close_instruction,
        &signer_seeds,
    );
    close_account(cpi_context)
}
