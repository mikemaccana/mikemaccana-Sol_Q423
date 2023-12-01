import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
import dotenv from "dotenv";
dotenv.config();
const log = console.log;

const devWallet = await getKeypairFromEnvironment("DEV_WALLET");
const wbaWallet = await getKeypairFromEnvironment("WBA_WALLET");

log(`Dev wallet address is:`, devWallet.publicKey.toBase58());
log(`WBA wallet address is:`, wbaWallet.publicKey.toBase58());
