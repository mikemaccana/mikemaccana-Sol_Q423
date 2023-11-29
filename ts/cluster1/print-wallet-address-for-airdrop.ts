import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
import dotenv from "dotenv";
dotenv.config();
const log = console.log;

const keypair = await getKeypairFromEnvironment("DEV_WALLET");

log(`Wallet address is:`, keypair.publicKey.toBase58());
