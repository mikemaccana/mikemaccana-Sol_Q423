use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer, CloseAccount, close_account},
};

use crate::state::Offer;
// Was 'Take'
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

    // Was called 'Escrow'
    #[account(
        mut,
        // Send rent from Offer back to maker
        close = maker, 
        seeds = [
            b"offer", 
            maker.key().as_ref(),
            offer.id.to_le_bytes().as_ref(),
        ],
        bump = offer.bump,
        // https://www.anchor-lang.com/docs/account-constraints
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
 
impl<'info> TakeOfferAccountConstraints<'info> {

    // Send money from taker to maker
    // Was 'deposit'
    pub fn send_desired_tokens_to_maker(&mut self) -> Result<()> {
        // Send some tokens to our vault account
        let transfer_accounts = Transfer {
            from: self.taker_desired_token_account.to_account_info(),
            to: self.maker_desired_token_account.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer(cpi_context, self.offer.desired_amount)
    }

    // Send money from vault to taker
    // Was 'withdraw'
    pub fn empty_vault(&mut self) -> Result<()> {
        // See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
        // 55m42s
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"offer",
            self.maker.to_account_info().key.as_ref(),
            &self.offer.id.to_le_bytes()[..],
            &[self.offer.bump],
        ]];

        // Release the tokens to the taker's account
        let transfer_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.taker_offer_token_account.to_account_info(),
            authority: self.offer.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            &signer_seeds,
        );

        transfer(cpi_context, self.vault.amount)
    }

    // Close the vault
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
            // Send vault rent back to taker
            destination: self.taker.to_account_info(),
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