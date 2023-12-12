// See https://www.anchor-lang.com/docs/account-constraints#spl-constraints
// See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub mod contexts;
pub use contexts::*;

pub mod state;
use state::*;

// 'anchor sync' to update
declare_id!("Hh2kzENRayrRsGJz2eUumxtATkBCTgAu3N5R7SrCcmvG");

#[program]
pub mod mikes_cool_escrow {
    use super::*;
    use anchor_spl::token::{close_account, transfer, CloseAccount, Transfer};

    pub fn make_offer(
        context: Context<MakeOfferAccountConstraints>,
        // Was 'Deposit'
        deposit_amount: u64,
        // Was 'Receive'
        desired_amount: u64,
    ) -> Result<()> {
        context.accounts.deposit(deposit_amount)?;
        context
            .accounts
            .saveOffer(id, desired_amount, &context.bumps)
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

        // Was 'close_accounts'
        let accounts_for_close_instruction = CloseAccount {
            account: context.accounts.vault.to_account_info(),
            destination: context.accounts.maker.to_account_info(),
            authority: context.accounts.offer.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            context.accounts.token_program.to_account_info(),
            accounts_for_close_instruction,
            &signer_seeds,
        );
        close_account(cpi_context)
    }

    // Whoever signs this is the 'taker'
    // Taker deposits assets (is that in the same transaction)
}
