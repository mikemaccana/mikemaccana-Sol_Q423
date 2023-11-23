import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr";
import dotenv from "dotenv";
import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
import { readFile } from "fs/promises";
const log = console.log;
dotenv.config();

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");
const bundlrUploader = createBundlrUploader(umi);

// Import our keypair from the wallet file
const keypairSolanaWeb3 = await getKeypairFromEnvironment("DEV_WALLET");
let keypair = umi.eddsa.createKeypairFromSecretKey(
  new Uint8Array(keypairSolanaWeb3.secretKey)
);

// Set up UMI to sign things with our wallet
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

try {
  // Upload a file to Arweave
  // Get files from https://deanmlittle.github.io/generug/
  const content = await readFile("./cluster1/rug.png");
  const imageFile = await createGenericFile(content, "rug.png", {
    contentType: "image/png",
  });

  const [imageURI] = await bundlrUploader.upload([imageFile]);
  log("Image uploaded to URI: ", imageURI);
} catch (error) {
  log("Oops.. Something went wrong", error);
}
