## Capstone project

Short description: provides certainty about the real-world identity of a wallet address before sending crypto, by allowing the recipient to reveal their proven real-world identity to the sender.

Rust, PDAs, a little JS/TS for a basic frontend.

Longer description: this is based on a concept implemented previously in Portal (see https://x.com/portalpayments/status/1574764615635976194), however implemented in a significantly different way using an on-chain app, with the benefit that identity is revealed on-demand only to specific addresses.

There are two roles in this transaction:

- Requester - wishes to know revealers identity
- Revealer - may wish to reveal their identity, but if so, only to the requester

Step 0: requester sends the URL for the reveal server to the revealer

## Submission

JAN 26TH SHIP CODE
PROGRAM ID IS IN REPO
DECK or VIDEO walkthrough
No need for frontend
Mathi and Richard for making my own Oracle

## !!! How do I store the data BEFORE I encrypt it with the requester pubkey

## Chat to Dean

## Chat to colleagues

Step 1: [deliberately implemented in a basic way for Capstone] a revealer POSTs their own pubkey and requester pubkey to a non-blockchain server. The server looks up stored identity information, and if found, sends the revealer a serialised TX using the same mechanism as Solana Pay (details of the TX in next step).

https://github.com/solana-labs/solana-pay/blob/master/SPEC.md#post-response

A Solana Pay transaction request URL describes an interactive request for any Solana transaction.

https://solana-labs.github.io/solana-web3.js/classes/Transaction.html#serialize

The transaction includes an instruction to an on-chain program's 'reveal()' instruction handler. This instruction includes the pubkey of the requester, the pubkey of the revealer, and the revealers identity information encrypted with the pubkey of the requester.

Step 2: the revealer signs and sends the TX if they want to reveal their identity.

Step 3: This instruction handler will store the encrypted identity data in a PDA, with the seeds being the revealers's pubkey, the requester's pubkey, the unix timestamp as an integer, and the string 'reveal'.

Step 4: Anyone on Solana can look up the data in the PDA, however only the requester can decrypt the data inside. This ensures the revealer can prove their identity to the requester but not publicly.
