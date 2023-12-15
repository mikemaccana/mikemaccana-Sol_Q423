// See https://www.anchor-lang.com/docs/account-constraints#spl-constraints
// See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod constants;

pub mod state;
// 'anchor sync' to update
declare_id!("Hh2kzENRayrRsGJz2eUumxtATkBCTgAu3N5R7SrCcmvG");

#[program]
pub mod mikes_cool_escrow {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOfferAccountConstraints>,
        // Was 'seed'
        id: u64,
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
        context.accounts.refund()?;
        context.accounts.close_vault()
    }

    // Whoever signs this is the 'taker'
    // Taker deposits assets (is that in the same transaction)
    pub fn take_offer(context: Context<TakeOfferAccountConstraints>) -> Result<()> {
        Ok(())
    }
}
