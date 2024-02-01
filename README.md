# Mike's WBA repo

program private keys are generated during deploy
but aren't used after deploy since the update authority.

## `YourName_Sol_Q423`

Per [GitHub instructions from WBA](https://docs.google.com/presentation/d/17wqMRVF1NWpKI2emLDb7a2ImPUHdUbbpIJ2VaeRsels/edit#slide=id.g29bf01f6ef0_0_45)

Cloned from https://github.com/Web3-Builders-Alliance/solana-starter

## Capstone Architectural Diagram

[Capstone Architectural Diagram](https://whimsical.com/8300-reveal-architecture-diagram-GfD5WCPooLam6WZzb1YFAf)

## Capstone is in Seperate Repo

See [Reveal Repo](https://github.com/mikemaccana/reveal)

## Evolution of Programs on Solana

### What's consistent

- Tokens are still distinguished by their mint.
- ATAS are still used to store tokens.
- Accounts are still closed when we are done with them. We zero the data beforehand so nobody else can add SOL and revive them
- Program design remains the same, eg,
  - Eg Alice and Bob have ATAs for each token (the provided token and the desired token)
  - The instrictions - make, take, refund - are the same

### What changed since PaulX (https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/#entrypoint-rs-programs-and-accounts)

- PaulX - manual serialisation and deserialization
- Alice makes a temporary token account and changes authority of that account, instead of just signing the transaction with the make() isntruction.
- Lots of sysvars. I didn't spend too much time on these as they're not relevant anymore but I can see they used to be more prominent.
- ProgramError() and coersion from regular errors into program errors.
- We have is_initialised in the PDA - we just init things if we need now
- lots of role checking now handled by account constraints, eg checking if an account is a signer
- lots of account_info_iter - looks like accounts used to be an array, whereas now we name accounts in a struct
- closing accounts

### What changed between PaulX and IronAddictedDog (https://hackmd.io/@ironaddicteddog/solana-anchor-escrow)

- We now have Anchor, and auto serialisation, discriminators, IDLs, etc.
- We have a vault account rather than the temporary token account. Actually feel like this may have been possible previously, it's just that PaulX didn't think of it.
- We don't have ATAs - we are manually recording token account addresses, rather than just using mint to determine currencies involved, and sending to whatever the ATA is for that mint at that wallet address.
- We add JS tests

### What changed in Based Anchor - (https://github.com/deanmlittle/anchor-escrow-2023)

Lol it's hard to tell with this opme because it's my .... default idea of what an Anchor project looks like.

- Using ATAs now

What I think would be good additions, that I tried in my own version:

- Still focusing on contexts over instruction handlers
- Not using the InitSpace macro
