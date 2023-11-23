import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr";
import dotenv from "dotenv";
import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
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

const UPLOADED_BUNDLR_IMAGE =
  "https://arweave.net/r3gMe7nnW9Fo_b8K9WcWyMjpOSLzyzptnZNaAnGYguY";

try {
  // Upload metadata
  // Follow this JSON structure
  // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
  const metadata = {
    name: "Mike's amazing rug",
    symbol: "RUG",
    description:
      "A run made by the rug generator for Mike's WBA NFT workshop entry",
    image: UPLOADED_BUNDLR_IMAGE,
    attributes: [{ trait_type: "?", value: "?" }],
    properties: {
      files: [
        {
          type: "image/png",
          uri: UPLOADED_BUNDLR_IMAGE,
        },
      ],
    },
    creators: [],
  };

  // Actually upload the metadata
  const metadataUri = await bundlrUploader.uploadJson(metadata);

  console.log("The metadata URI: ", metadata);
} catch (error) {
  log("Oops.. Something went wrong", error);
}
