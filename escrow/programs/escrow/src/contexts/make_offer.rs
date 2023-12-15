use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::state::Offer;

// GO BACK TO MAKEBUMOS

// 110 when done

// Was called "Initialize"
// The constraints for the Initialize instruction
// info is just a name we're giving for the lifetime
// but info is the convention to name the lifetime for Anchor
// under the hood we have 'account info' and 'account meta'
// https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
// TODO: 27:44 in vieo - why are these Structs named like they are functions?
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOfferAccountConstraints<'info> {
    // We can use the Signer macro or the Signer type
    // both do the validation - the #[account] macros are the constraint
    // #[account(mut Signer)]
    //
    // Signer type allows any account to be able to sign (including program accounts)
    #[account(mut)]
    pub maker: Signer<'info>,

    // The currencies we're swapping
    // Was called 'mint_a'
    pub offer_token: Account<'info, Mint>,
    // Was called 'mint_b'
    pub desired_token: Account<'info, Mint>,

    // MakeOffer() will transfer the balance out of the Makers ATA account
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

impl<'info> MakeOfferAccountConstraints<'info> {
    // Make an Offer instance

    // Whoever signs this is the 'maker'
    // Deposits an asset (of whatever token mint)
    // Requests an amount of a particular token mint

    // Was called Initialize
    pub fn deposit(&mut self, deposit_amount: u64) -> Result<()> {
        // Send some tokens to our vault account
        let transfer_accounts = Transfer {
            from: self.maker_token_account.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer(cpi_context, deposit_amount)
    }

    // id was 'seed'
    // deposit_amount was 'Deposit'
    // desired_amount was 'Receive'
    // was 'saveEscrow'
    pub fn saveOffer(
        &mut self,
        id: u64,
        desired_amount: u64,
        // Created automatically based on the name 'MakeOfferAccountConstraints'
        // to access all
        bumps: &MakeOfferAccountConstraintsBumps,
    ) -> Result<()> {
        // make a variable 'id' set from the unix timestamp
        // We'll use this to identify the offer
        // TODO: same account makes an offer at the same time
        let id: u64 = Clock::get()?.unix_timestamp as u64;

        // Set the values for an Offer account (which will hold details of this offer)
        // set_inner() saves us from having to do context.accounts.offer.a = a, b = b etc. since we can just use a Struct
        self.offer.set_inner(Offer {
            id,
            offer_token: self.offer_token.key(),
            desired_token: self.desired_token.key(),
            desired_amount,
            bump: bumps.offer,
            vault_bump: bumps.vault,
        });

        /// HACK TODO remove
        return Ok(());
    }
}
