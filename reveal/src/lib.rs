mod errors;
use anchor_lang::prelude::*;
pub use errors::RevealError;
use std::collections::BTreeMap;

declare_id!("Hh2kzENRayrRsGJz2eUumxtATkBCTgAu3N5R7SrCcmvG");

#[program]
mod certainty {

    use super::*;
    pub fn reveal(
        context: Context<RevealAccountConstraints>,
        requester_pubkey: Pubkey,
        revealer_pubkey: Pubkey,
    ) -> Result<()> {
        // TODO: check that the revealer_pubkey is the signer

        // TODO: look up revealer's account
        // TODO: encrypt revealer's identity information with ther requested pubkey

        // TOOD: make a PDA for the reveal
        // uni timetsamp

        // Get a unix timestamp
        let timestamp = Clock::get()?.unix_timestamp;

        // Make a PDA based on the timestamp
        let (pda, _nonce) = Pubkey::find_program_address(
            &[
                &requester_pubkey.to_bytes(),
                &revealer_pubkey.to_bytes(),
                &timestamp.to_le_bytes(),
            ],
            &id(),
        );

        // TODO: this data should come from a PDA
        // provided by an oracle
        let mut identity_data = BTreeMap::new();
        identity_data.insert(revealer_pubkey, "Steve");

        let identity = identity_data
            .get(&revealer_pubkey)
            .ok_or(RevealError::IdentityUnknownError)?;

        // TODO: Save the bump

        unimplemented!()
    }
}

// Validate incoming accounts for instructions
#[derive(Accounts)]
pub struct RevealAccountConstraints<'info> {
    #[account()]
    pub requester: AccountInfo<'info>,

    #[account()]
    pub revealer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
