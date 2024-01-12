use anchor_lang::prelude::*;

declare_id!("Hh2kzENRayrRsGJz2eUumxtATkBCTgAu3N5R7SrCcmvG");

#[program]
mod reveal {

    use super::*;
    pub fn reveal(
        context: Context<Form8300SenderAccountConstraints>,
        sender_pubkey: Pubkey,

        first_name: String,
        last_name: String,
        middle_initial: String,

        taxpayer_id: String,

        address: String,
        city: String,
        state: String,
        country: String,

        occupation_profession_or_business: String,
        date_of_birth: String,

        identifying_document: String,
        identifying_document_number: String,
        identifying_document_issued_by: String,
    ) -> Result<()> {
        // We need to make the account first

        // TODO: when does dean make the escrow accoint in make?
        // Get a unix timestamp
        let timestamp = Clock::get()?.unix_timestamp;

        // Make a PDA based on the timestamp
        // TODO: Save the bump
        let (pda, _nonce) = Pubkey::find_program_address(
            &[
                &sender_pubkey.to_bytes(),
                &sender_pubkey.to_bytes(),
                &timestamp.to_le_bytes(),
            ],
            &id(),
        );

        // Shove the data into the Account
        context
            .accounts
            .form8300_revealed_identity
            .set_inner(Form8300RevealedIdentity {
                first_name: first_name,
                last_name: last_name,
                middle_initial: middle_initial,

                taxpayer_id: taxpayer_id,

                address: address,
                city: city,
                state: state,
                country: country,

                occupation_profession_or_business: occupation_profession_or_business,
                date_of_birth: date_of_birth,

                identifying_document: identifying_document,
                identifying_document_number: identifying_document_number,
                identifying_document_issued_by: identifying_document_issued_by,
            });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Form8300RevealedIdentity {
    pub first_name: string,
    pub last_name: string,
    pub middle_initial: string,

    pub taxpayer_id: string,

    pub address: string,
    pub city: string,
    pub state: string,
    pub country: string,

    pub occupation_profession_or_business: string,
    pub date_of_birth: string,

    pub identifying_document: string,
    pub identifying_document_number: string,
    pub identifying_document_issued_by: string,
}

// Validate incoming accounts for instructions
// Really, Accounts are just account constraints
#[derive(Accounts)]
pub struct Form8300SenderAccountConstraints<'info> {
    #[account(
        init,
        payer = sender,
        seeds = [sender.key().as_bytes(), recipient.key().as_bytes(), &timestamp.to_le_bytes()],
        bump,
        // Per https://github.com/coral-xyz/anchor/pull/2346
        space = ANCHOR_DISCRIMINATOR_SIZE + Form8300RevealedIdentity::INIT_SPACE,
    )]
    pub form8300_revealed_identity: Account<'info, Offer>,

    #[account()]
    pub sender: Account<'info>,

    #[account()]
    pub recipient: Account<'info>,

    pub system_program: Program<'info, System>,
}
