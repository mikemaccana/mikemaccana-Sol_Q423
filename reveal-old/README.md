how does wallet adapter relate to wallet connect?

4 bytes (to store size) plus length of the string

https://www.anchor-lang.com/docs/space

anchor init projectname --template=multiple

zed IDE

q. nate token 22 idea for my capstone?

- just to have this as a token standard
- but transfer hooks

q. attack mention at end of https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/#entrypoint-rs-programs-and-accounts
q. authority?

## Reveal - Capstone project

- Sender wants to send someone 10,000
- Recipient wants to be able to collect Form 8300 data

how does wallet adapter relate to wallet connect?

4 bytes (to store size) plus length of the string

https://www.anchor-lang.com/docs/space

anchor init projectname --template=multiple

https://whimsical.com/8300-reveal-architecture-diagram-GfD5WCPooLam6WZzb1YFAf

- Sender app (likely a wallet) encrypts senders private data with recipient oubkey
- Sender makes transaction:
  - TokenProgram.transfer() for the 10K+ transaction
  - RevealProgram.reveal() for the relevent 8300 details  
    This instruction includes the pubkey of the sender, the pubkey of the recipient, and the senders identity information encrypted with the pubkey of the recipient.

Step 4: Anyone on Solana can look up the data in the PDA, however only the recipient can decrypt the data inside. This ensures the sender can prove their identity to the recipient but not publicly.

## Submission notes

JAN 26TH SHIP CODE
PROGRAM ID IS IN REPO
DECK or VIDEO walkthrough
No need for frontend
Mathi and Richard for making my own Oracle

References
https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view
44m37s

SSN Validator https://github.com/uphold/ssn-validator/blob/master/src/index.js

## Nate chat 20240104

nate@web3builders.dev

form-8300-version-reveal-instruction

different instructions for different compliance

- Enhanced due diligence kyc - high-risk customers and large transactions
- fatca

can you tell me more about the airtable / hubspot [ ]
(I guess pull data needed to complete instruction from hubspot etc, or pull data into hubspot)

How does the note program match notes to transactions?

pda for businesses

keep it modular

chief compliance officer can create a pda

have them individual

70-90% are institional or whales

zk not needed

[*] jordan / kaylan solana pay (have asked on slack, answer was no support for incomplete transactions at present)

[ ] on chain kyc eth - zks vitalik worked on ProofOfConcept, zk.me on polygon, legend trading customisable automated kyc but not encrypted, most off chain,
canton network

See https://medium.com/@zkMe/zkme-becomes-first-decentralized-issuer-to-launch-on-polygonid-548cbe57ebee

[ ] lookup tables?

---

Financial Privacy and Regulation Can Coexist with ZK Proofs

[*] fully Homomorphic encryption
Processing data whle it's encrypted.

get mvp out

can't build a broker book on chain

[ ] zk questions on bard / copilot

bring into artisan

##

// 'instruction introspection'
// start transaction
// verify off chain
// continue
