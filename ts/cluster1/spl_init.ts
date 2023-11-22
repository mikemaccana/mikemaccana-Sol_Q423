import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
import dotenv from "dotenv";
dotenv.config();

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const keypair = await getKeypairFromEnvironment("DEV_WALLET");
try {
  // Start here
  console.log(`Loaded keypair:`, keypair.publicKey.toBase58());

  const mint = await createMint(
    connection,
    keypair,
    keypair.publicKey,
    null,
    6
  );
  console.log(`Made mint:`, mint.toBase58());
} catch (error) {
  console.log(`Oops, something went wrong: ${error}`);
}
