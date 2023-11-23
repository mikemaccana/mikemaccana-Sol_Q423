import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
import dotenv from "dotenv";
dotenv.config();

// Import our keypair from the wallet file
const keypair = await getKeypairFromEnvironment("DEV_WALLET");

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// TODO: Replace this with the mint address you got from spl_init.ts
// Mint address (double check this)
const mint = new PublicKey("DkyW171gPJ6owwCLgebCFvAZWGuihze2REbjfqgHKNud");

try {
  // Create an ATA
  const ata = await getOrCreateAssociatedTokenAccount(
    connection,
    keypair,
    mint,
    keypair.publicKey
  );
  console.log(`Your ata is: ${ata.address.toBase58()}`);

  // Mint to ATA
  console.log(`Running mintTo...`);

  const mintTransaction = await mintTo(
    connection,
    keypair,
    mint,
    ata.address,
    keypair.publicKey,
    1_000_000
  );

  console.log("finished mintTo");
  console.log(`Your mint transaction id: ${mintTransaction}`);
} catch (error) {
  console.log(`Oops, something went wrong: ${(error as Error).message}`);
  console.log((error as Error).stack);
}
