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

        // TODO: this data should come from a PDA provided by an oracle
        // hacking this for the purposes of MVP
        let hackStoredIdentity: BTreeMap<Pubkey, VerifiedCredentials> = BTreeMap::new();
        let steve = VerifiedCredentials {
            given_name: "Steve".to_string(),
            family_name: "Smith".to_string(),
        };
        hackStoredIdentity.insert(revealer_pubkey, steve);

        let found_identity = hackStoredIdentity
            .get(&revealer_pubkey)
            .ok_or(RevealError::IdentityUnknownError)?;

        // Make a PDA based on the timestamp
        // TODO: Save the bump
        let (pda, _nonce) = Pubkey::find_program_address(
            &[
                &requester_pubkey.to_bytes(),
                &revealer_pubkey.to_bytes(),
                &timestamp.to_le_bytes(),
            ],
            &id(),
        );

        // TODO: can't set data on an address.
        // set data to what's inside the address
        let individually_revealed_identity = &mut context.accounts.individually_revealed_identity;
        individually_revealed_identity.given_name = found_identity.given_name;
        individually_revealed_identity.family_name = found_identity.family_name;

        Ok(())
    }
}

// Validate incoming accounts for instructions
#[derive(Accounts)]
pub struct RevealAccountConstraints<'info> {
    #[account(
        init,
        seeds = [requester_pubkey.as_bytes(), revealer_pubkey.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 1 + 4 + title.len() + 4 + description.len()
    )]
    #[account()]
    pub requester: AccountInfo<'info>,

    #[account()]
    pub revealer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct VerifiedCredentials {
    pub given_name: String,
    pub family_name: String,
}
