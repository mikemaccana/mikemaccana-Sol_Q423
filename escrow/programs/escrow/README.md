# Escrow Program

I followed [Dean's video](https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view) but rewrote the code.

- Calling the struct 'Take' or 'Make' etc is unintuitive to me. A struct is not a verb. It does not do things. So I name my structs TakeOfferAccountConstraints or MakeOfferAccountConstraints

- Likewise `recieve` is `desired_amount` - lots of the explanations used in the videos made better names for the variables used.

## Make offer

Maker sends token to vault
Says what amount and what token they would like

## Take offer

Send the desired tokens from taker to maker
Send money from vault to taker
Close the vault

## Refund

Empties the vault
Closes the vault
