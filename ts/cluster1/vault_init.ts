import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
} from "@solana/web3.js";
import { Program, Wallet, AnchorProvider, Address } from "@coral-xyz/anchor";
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

const VAULT_PROGRAM = "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address;

// Create our program
const program = new Program<WbaVault>(IDL, VAULT_PROGRAM, provider);

// Create a random keypair
const vaultState = Keypair.generate();
console.log(`Vault public key: ${vaultState.publicKey.toBase58()}`);

// Create the PDA for our enrollment account
// Seeds are "auth", vaultState public key
const vaultAuth = PublicKey.createProgramAddressSync(
  [Buffer.from("auth"), vaultState.publicKey.toBuffer()],
  program.programId
);

// Create the vault key
// Seeds are "vault", vaultAuth public key
const vault = PublicKey.createProgramAddressSync(
  [Buffer.from("vault"), vaultAuth.toBuffer()],
  program.programId
);

// Execute our enrollment transaction
try {
  const signature = await program.methods
    .initialize()
    .accounts({
      owner: keypair.publicKey,
      vaultState: vaultState.publicKey,
      vaultAuth,
      vault,
      systemProgram: SystemProgram.programId,
    })
    .signers([keypair, vaultState])
    .rpc();
  console.log(
    `Init success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
  );
} catch (error) {
  console.error(`Oops, something went wrong: ${error}`);
}
