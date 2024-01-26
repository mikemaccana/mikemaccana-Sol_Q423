use crate::state::Revelation;
use anchor_lang::prelude::*;

use crate::ANCHOR_DISCRIMINATOR_SIZE;

// The account contraints also has access to the parameters of the instruction
// In this case, that's where 'id' is set.
#[derive(Accounts)]
#[instruction(id: u64)]
#[derive(Debug)]
pub struct RevealAccountConstraints<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    // #[account(
    //     init,
    //     payer = sender,
    //     seeds = [b"relevation", sender.key().as_ref(), recipient.key().as_ref(), &id.to_le_bytes().as_ref()],
    //     bump,
    //     // Per https://github.com/coral-xyz/anchor/pull/2346
    //     space = ANCHOR_DISCRIMINATOR_SIZE + Form8300RevealedIdentity::INIT_SPACE,
    // )]
    // pub form8300_revealed_identity: Account<'info, Revelation>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    // Account context
    ctx: Context<RevealAccountConstraints>,
    // ID for the revelation
    id: u64,
) -> Result<()> {
    // Now shove the data into the account.

    // log the ctx
    msg!("Context: {:?}", ctx);

    msg!("Revealing identity for {}", id);
    Ok(())
}
