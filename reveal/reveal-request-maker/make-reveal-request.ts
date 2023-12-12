// Step 1: A revealer POSTs their own pubkey and requester pubkey to a non-blockchain server. The server looks up stored identity information, and if found, sends the revealer a serialised TX using the same mechanism as Solana Pay (details of the TX in next step).
// The transaction includes an instruction to an on-chain program's 'reveal()' instruction handler.

// Question
// Should should this include the identity information?

// This instruction includes the pubkey of the requester, the pubkey of the revealer, and the revealers identity information encrypted with the pubkey of the requester.

import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  clusterApiUrl,
} from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";

const log = console.log;
const stringify = (object) => JSON.stringify(object, null, 2);

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
  // TODO: should I use unsigned integer or str for a pubkey?
  borsh.u8("variant"),
  borsh.u32("requesterPubkey"),
  borsh.u32("revealerPubkey"),
]);

// See https://github.com/solana-labs/solana-pay/blob/master/SPEC.md#post-response
const makeRevealTransactionRequest = async function (
  requesterPubkey: PublicKey,
  revealerPubkey: PublicKey
): Promise<SerializedRevealTransactionRequest> {
  // https://solana-labs.github.io/solana-web3.js/classes/Transaction.html#serialize

  const network = clusterApiUrl("devnet");
  const connection = new Connection(network);

  // All this BS goes away due to anchor
  const buffer = Buffer.alloc(1000);
  revealSchema.encode(
    {
      variant: 0,
      requesterPubkey: requesterPubkey.toBase58(),
      revealerPubkey: requesterPubkey.toBase58(),
    },
    buffer
  );

  // Ignore deprecation warning for now
  const instructionData = buffer.slice(0, revealSchema.getSpan(buffer));

  const transaction = new Transaction();

  const revealInstruction = new TransactionInstruction({
    keys: [
      // revealer must sign transaction
      { pubkey: revealerPubkey, isSigner: false, isWritable: false },
      // requester doesn't need to sign transaction
      { pubkey: requesterPubkey, isSigner: false, isWritable: false },
    ],
    programId: REVEAL_PROGRAM_ADDRESS,
    data: instructionData,
  });

  transaction.add(revealInstruction);

  const recentBlockHashAndHeight = await connection.getLatestBlockhash();

  transaction.recentBlockhash = recentBlockHashAndHeight.blockhash;

  transaction.feePayer = revealerPubkey;

  log(`About to serialise`);
  const serializedTransaction = transaction.serialize({
    // See https://github.com/solana-labs/solana-pay/blob/master/examples/point-of-sale/src/server/api/index.ts
    verifySignatures: false,
    requireAllSignatures: false,
  });

  const base64EncodedTransaction = serializedTransaction.toString("base64");
  return {
    message:
      "Please sign the following transaction to reveal your real world identity.",
    transaction: base64EncodedTransaction,
  };
};

const requesterKeypair = Keypair.generate();
const revealerKeypair = Keypair.generate();

const transactionRequest = await makeRevealTransactionRequest(
  requesterKeypair.publicKey,
  revealerKeypair.publicKey
);

log(stringify(transactionRequest));
