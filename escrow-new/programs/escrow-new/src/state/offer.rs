use anchor_lang::prelude::*;

// Gives us all our serialization / deserialization traits
#[account]
// Was called Escrow
pub struct Offer {
    // Identifier
    // Was called Seed
    // No need for pub as we will be using methods in impl
    pub id: u64,

    // Was mint_a
    pub offer_token: Pubkey,
    // 'Amount' maker is offering is determined by what makers put in the vault
    // TODO: Why isn't offer token mint determined by what maker puts in the vault?

    // What the maker wants back
    // Was mint_b
    pub desired_token: Pubkey,

    // Was Receive
    pub desired_amount: u64,

    // Faster to not have to recalculate the bumps
    pub bump: u8,
    pub vault_bump: u8,
}
