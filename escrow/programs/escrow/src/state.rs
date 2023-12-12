use anchor_lang::prelude::*;

use crate::constants::{ANCHOR_DISCRIMINATOR_SIZE, PUBKEY_SIZE, U64_SIZE, U8_SIZE};

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

// Make space for the Offer/Escrow
impl Space for Offer {
    // ANCHOR DISCRUMINATOR is always 8 bytes
    // 8 bytes for the u64 = 64 bits is 8 bytes
    // 32 for every pubkey
    // u8s are 1 bytes
    // TODO: can    I use sizeOf() or similar?
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR_SIZE
        + 1 * U64_SIZE // id
        + 2 * PUBKEY_SIZE // offer_token, desired_token
        + 1 * U64_SIZE // desired_amount
        + 2 * U8_SIZE; // bump, vault_bump
}
