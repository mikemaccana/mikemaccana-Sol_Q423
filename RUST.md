# Rust notes

Owners and borrowers (&)

Immutable by default

One mutable reference at once

Always add 8 bytes for an Anchor account discriminator.

Signer Account Program UncheckedAccount

every tx has 3 things:

- program id
- acconts struct
- instruction

---

Every folder with modules has mod.rs

unimplemented!()
richard uses 'seed' for 'id'

people often use mint_x to mean token_x, since a token is identified by the mint

once you add mut you need 'info
because Solana has AccountInfo

## nearly every new anchor project needs anchor spl and init-if-needed

What is InterfaceAccount again?

The Token / Token22 future proofing

init will make token22 accounts as needed
