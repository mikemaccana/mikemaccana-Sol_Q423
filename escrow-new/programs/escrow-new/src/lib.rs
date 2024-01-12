pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::{make_offer, refund_offer, take_offer};

declare_id!("8atVt41JaLmrLmCUgP8N9jEkc3KHwk5EV4AUYRxnqcix");

#[program]
pub mod escrow_new {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOfferAccountConstraints>,
        id: u64,
        deposit_amount: u64,
        desired_amount: u64,
    ) -> Result<()> {
        make_offer::handler(context)
    }

    pub fn take_offer(context: Context<TakeOfferAccountConstraints>) -> Result<()> {
        take_offer::handler(context, amount)
    }

    pub fn refund_offer(context: Context<RefundOfferAccountConstraints>) -> Result<()> {
        refund_offer::handler(context)
    }
}
