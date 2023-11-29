import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";
import { WbaVault, IDL } from "./programs/wba_vault.js";
import dotenv from "dotenv";
dotenv.config();

const log = console.log;

import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";

// Import our keypair from the wallet file
const keypair = getKeypairFromEnvironment("WBA_WALLET");
log(`Loaded keypair`);

// Commitment
const commitment: Commitment = "confirmed";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(IDL, "<address>" as Address, provider);

// Create a random keypair
const vaultState = new PublicKey("<address>");
// Create the PDA for our enrollment account
// Seeds are "auth", vaultState
log(`Create the PDA for our enrollment account`);
const vaultAuth = PublicKey.createProgramAddressSync(
  [Buffer.from("auth"), vaultState.toBuffer()],
  program.programId
);

// Create the vault key
// Seeds are "vault", vaultAuth
log(`Create the vault key`);
const vault = PublicKey.createProgramAddressSync(
  [Buffer.from("vault"), vaultAuth.toBuffer()],
  program.programId
);

// Execute our enrollment transaction
try {
  const signature = await program.methods
    .deposit(new BN(10 * LAMPORTS_PER_SOL))
    .accounts({
      owner: keypair.publicKey,
      vault,
      vaultAuth,
      vaultState,
      systemProgram: SystemProgram.programId,
    })
    .signers([keypair])
    .rpc();
  console.log(
    `Deposit success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
  );
} catch (error) {
  console.error(`Oops, something went wrong: ${error}`);
}
