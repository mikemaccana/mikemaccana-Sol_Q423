import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
import dotenv from "dotenv";
dotenv.config();

import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = await getKeypairFromEnvironment("DEV_WALLET");

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("<mint address>");

// Recipient address
const to = new PublicKey("<receiver address>");

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    // Get the token account of the toWallet address, and if it does not exist, create it
    // Transfer the new token to the "toTokenAccount" we just created
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
