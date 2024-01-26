use anchor_lang::prelude::{account, borsh, AnchorDeserialize, AnchorSerialize, Pubkey};

// The actual secret data that is revealed.
// Along with some fairly standard account metadata.
//
// TODO: maybe use zero_copy?
// See https://github.com/solana-developers/anchor-zero-copy-example/tree/main
// for more information on the #[account(zero_copy)] attribute.
#[account()]
pub struct Revelation {
    pub id: u64,
    pub requester: Pubkey,
    pub revealer: Pubkey,
    // Ie a 1KB buffer.
    pub data: [u8; 1024],
}
