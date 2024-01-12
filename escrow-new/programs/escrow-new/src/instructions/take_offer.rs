use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {}

pub fn handler(context: Context<RefundOfferAccountConstraints>) -> Result<()> {

    let offer = context.accounts.offer;    

    // TODO: understand what account_info is
    let token_program_account_info = context.accounts.token_program.to_account_info();
    
    let maker_account_info = context.accounts.maker.to_account_info();
    let maker_offered_token_account_info = context.accounts.maker_offered_token_account.to_account_info();
    let maker_desired_token_account_info = context.accounts.maker_desired_token_account.to_account_info();

    let taker_account_info = context.accounts.taker.to_account_info();
    let taker_offer_token_account_info = context.accounts.taker_offer_token_account.to_account_info();
    let taker_desired_token_account_info = context.accounts.taker_desired_token_account.to_account_info();

    let vault_account_info = context.accounts.vault.to_account_info();

    // Transfer tokens from taker to maker
    // Was the 'deposit' function in Dean's code
    let transfer_accounts = Transfer {
        from: taker_desired_token_account_info,
        to: maker_desired_token_account_info,
        authority: taker_account_info,
    };
    let cpi_context = CpiContext::new(token_program_account_info, transfer_accounts);
    transfer(cpi_context, self.offer.desired_amount)

    // Transfer from vault to taker
    // Was 'withdraw'
    // See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
    // 55m42s
    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"offer",
        maker_account_info.key.as_ref(),
        &offer.id.to_le_bytes()[..],
        &[offer.bump],
    ]];

    // Release the tokens to the taker's account
    let transfer_accounts = Transfer {
        from: vault_account_info,
        to: taker_offer_token_account_info,
        authority: offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        token_program_account_info,
        transfer_accounts,
        &signer_seeds,
    );

    transfer(cpi_context, self.vault.amount)

    // Close the vault
    // Was 'withdraw'
    // See https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
    // 55m42s
    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"offer",
        self.maker.to_account_info().key.as_ref(),
        &self.offer.id.to_le_bytes()[..],
        &[self.offer.bump],
    ]];

    // Was 'close_accounts'
    let accounts_for_close_instruction = CloseAccount {
        account: vault_account_info,
        // Send vault rent back to taker
        destination: taker_account_info,
        authority: offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        token_program_account_info,
        accounts_for_close_instruction,
        &signer_seeds,
    );
    close_account(cpi_context)


    Ok(())
}
