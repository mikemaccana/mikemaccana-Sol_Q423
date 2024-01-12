use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

// TODO: Why are these not built in?
// Also what would happen if I didn't use the #[constant] macro?
#[constant]
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[constant]
pub const U8_SIZE: usize = 1;

#[constant]
pub const U64_SIZE: usize = 8;

#[constant]
pub const PUBKEY_SIZE: usize = 32;
