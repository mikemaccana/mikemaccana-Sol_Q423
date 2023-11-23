import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  percentAmount,
} from "@metaplex-foundation/umi";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import dotenv from "dotenv";
import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
const log = console.log;
dotenv.config();

import base58 from "bs58";

// Create out UMI instance
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

// Import our keypair from the wallet file
const keypairSolanaWeb3 = await getKeypairFromEnvironment("DEV_WALLET");
let keypair = umi.eddsa.createKeypairFromSecretKey(
  new Uint8Array(keypairSolanaWeb3.secretKey)
);

// Make our UMI instance sign everything with our wallet
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

const METADATA_URI =
  "https://arweave.net/z6ynCdJp1QZWn83N8oXLHnHMS220GNhhnRFB_-FniS0";

try {
  // https://mpl-token-metadata-js-docs.vercel.app/functions/createNft.html
  let tx = createNft(umi, {
    mint,
    name: "Mike's amazing rug",
    uri: METADATA_URI,
    sellerFeeBasisPoints: percentAmount(10),
  });
  let result = await tx.sendAndConfirm(umi);
  const signature = base58.encode(result.signature);

  console.log(
    `Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
  );

  console.log("Mint Address: ", mint.publicKey);
} catch (error) {
  log("Oops.. Something went wrong", error);
}
