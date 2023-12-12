import {
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";

// AKA program ID
const fakeKeypair = Keypair.generate();
const REVEAL_PROGRAM_ADDRESS = fakeKeypair.publicKey;

// See https://github.com/solana-labs/solana-pay/blob/master/SPEC.md#post-response
interface SerializedRevealTransactionRequest {
  // The name of an item being purchased, a discount applied to the purchase, or a thank you note.
  // The wallet should display the value to the user.
  message: string;
  // A base64-encoded serialized transaction.
  transaction: string;
}

const revealSchema = borsh.struct([
  borsh.u8("reveal"),
  borsh.u32("requesterPubkey"),
  borsh.u32("revealerPubkey"),
]);

const buffer = Buffer.alloc(1000);
revealSchema.encode({ variant: 2, playerId: 1435, itemId: 737498 }, buffer);

const instructionBuffer = buffer.slice(0, equipPlayerSchema.getSpan(buffer));

// See https://github.com/solana-labs/solana-pay/blob/master/SPEC.md#post-response
const makeRevealTransactionRequest = function (
  requesterPubkey: PublicKey,
  revealerPubkey: PublicKey
): SerializedRevealTransactionRequest {
  // https://solana-labs.github.io/solana-web3.js/classes/Transaction.html#serialize

  const transaction = new Transaction();

  const revealInstruction = new TransactionInstruction({
    keys: [
      // revealer must sign transaction
      { pubkey: revealerPubkey, isSigner: false, isWritable: false },
      // requester doesn't need to sign transaction
      { pubkey: requesterPubkey, isSigner: false, isWritable: false },
    ],
    programId: REVEAL_PROGRAM_ADDRESS,
    data: Buffer.from([]),
  });

  transaction.add(revealInstruction);

  const serializedTransaction = transaction.serialize();

  const base64EncodedTransaction = serializedTransaction.toString("base64");
  return {
    message:
      "Please sign the following transaction to reveal your real world identity.",
    transaction: base64EncodedTransaction,
  };
};

// Step 1: [deliberately implemented in a basic way for Capstone] a revealer POSTs their own pubkey and requester pubkey to a non-blockchain server. The server looks up stored identity information, and if found, sends the revealer a serialised TX using the same mechanism as Solana Pay (details of the TX in next step).

// https://github.com/solana-labs/solana-pay/blob/master/SPEC.md#post-response

// A Solana Pay transaction request URL describes an interactive request for any Solana transaction.
// Just want to clarify - I can create a transaction request for any solana TX, including one with instructions for a new on-chain program I make?
// Just respond to the POST with:
// {"transaction":"<transaction>"}
// Where transaction is base64 output of serialize() ?
// https://solana-labs.github.io/solana-web3.js/classes/Transaction.html#serialize

// The transaction includes an instruction to an on-chain program's 'reveal()' instruction handler. This instruction includes the pubkey of the requester, the pubkey of the revealer, and the revealers identity information encrypted with the pubkey of the requester.
