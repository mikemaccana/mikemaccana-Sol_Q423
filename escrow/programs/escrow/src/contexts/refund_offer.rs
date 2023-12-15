use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        close_account, spl_token::instruction::transfer, CloseAccount, Mint, Token, TokenAccount,
        Transfer,
    },
};

use crate::state::Offer;

#[derive(Accounts)]
pub struct RefundOfferAccountConstraints<'info> {
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
    maker_token_account: Account<'info, TokenAccount>,

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

impl<'info> RefundOfferAccountConstraints<'info> {
    pub fn refund(&mut self) -> Result<()> {
        // See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
        // 55m42s
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"offer",
            self.maker.to_account_info().key.as_ref(),
            &self.offer.id.to_le_bytes()[..],
            &[self.offer.bump],
        ]];

        // Release the tokens to our vault account
        let transfer_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.maker_token_account.to_account_info(),
            authority: self.offer.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            &signer_seeds,
        );

        transfer(cpi_context, self.vault.amount);
    }

    pub fn close_vault(&mut self) -> Result<()> {
        // See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
        // 55m42s
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"offer",
            self.maker.to_account_info().key.as_ref(),
            &self.offer.id.to_le_bytes()[..],
            &[self.offer.bump],
        ]];

        // Was 'close_accounts'
        let accounts_for_close_instruction = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.offer.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts_for_close_instruction,
            &signer_seeds,
        );
        close_account(cpi_context)
    }
}
