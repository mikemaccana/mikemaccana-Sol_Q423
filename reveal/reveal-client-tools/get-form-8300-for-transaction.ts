import { Keypair, ParsedTransactionWithMeta } from "@solana/web3.js";
const REVEAL_PROGRAM_ADDRESS = "fill_me_in";

// See constants.ts for details re: notes vs memos
const getNoteOrMemo = (
  keypair: Keypair,
  rawTransaction: ParsedTransactionWithMeta
): string | null => {
  const instructions = rawTransaction.transaction.message.instructions;

  const revealInstruction = instructions.find((instruction) => {
    return instruction.programId.toBase58() === REVEAL_PROGRAM_ADDRESS;
  });

  if (revealInstruction) {
    const encryptedForm8300Details = revealInstruction.parsed;
    const decryptedForm8300Details = keypair.secretKey;
    return memo;
  }

  return null;
};
