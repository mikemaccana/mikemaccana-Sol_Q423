// From https://gist.githubusercontent.com/L0STE/98ee58ffa17300d9692d6005fcc7551d/raw/d848d06edec0df01b2b68284aea54d1c3687511f/spl_metadata.ts
import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";
import { createMetadataAccountV3 } from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  publicKey,
  signerIdentity,
  createSignerFromKeypair,
} from "@metaplex-foundation/umi";
import {
  publicKey as publicKeySerializer,
  string,
} from "@metaplex-foundation/umi/serializers";
import { base58 } from "@metaplex-foundation/umi/serializers";
import dotenv from "dotenv";
dotenv.config();

//Create a Umi instance
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

// Umi cheatsheet
// https://github.com/metaplex-foundation/umi/blob/main/README.md - Umi Documentation

// Whilst Umi only relies on the Signer interface to request signatures from a wallet, it also defines a Keypair type and a KeypairSigner type that are explicitly aware of their secret key.
// We can generate new Keypair object with EdDSA interface with this function:
// From a secret key [our case]
const keypairSolanaWeb3 = await getKeypairFromEnvironment("DEV_WALLET");
let keypair = umi.eddsa.createKeypairFromSecretKey(
  new Uint8Array(keypairSolanaWeb3.secretKey)
);
// From a seed -> const myKeypair = umi.eddsa.createKeypairFromSeed(mySeed);
// Create a new random keypair -> const myKeypair = umi.eddsa.generateKeypair();

// In order to use these keypairs as signers throughout your application, you can use the createSignerFromKeypair helper method:
const signerKeypair = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signerKeypair));

// We can create a new valid public key from a variety of inputs using the publicKey helper method
// [NB this are just publickey and are NOT signers]:
// From a base58 string [Our case]
const mint = publicKey("BvrY7utmPBGFwQQhdJkKHfziorchVm82GYTrSxoVjfjz");
const tokenMetadataProgramId = publicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
// From a 32-byte buffer -> publicKey(new Uint8Array(32));
// From a PublicKey or Signer type -> publicKey(someWallet as PublicKey | Signer);

// Each seed must be serialized as a Uint8Array to be utilized as seeds.
const seeds = [
  string({ size: "variable" }).serialize("metadata"),
  publicKeySerializer().serialize(tokenMetadataProgramId),
  publicKeySerializer().serialize(mint),
];

let tx = createMetadataAccountV3(umi, {
  mint: mint,
  mintAuthority: signerKeypair,
  updateAuthority: keypair.publicKey,
  data: {
    name: "L0STE - Test Token #1",
    symbol: "TST",
    uri: "https://arweave.net/euAlBrhc3NQJ5Q-oJnP10vsQFjTV7E9CgHZcVm8cogo",
    sellerFeeBasisPoints: 1000,
    creators: [{ address: keypair.publicKey, verified: true, share: 100 }],
    collection: null,
    uses: null,
  },
  isMutable: true,
  collectionDetails: null,
});

let result = await tx.sendAndConfirm(umi);
const signature = base58.deserialize(result.signature);

console.log(
  `Succesfully Minted!. Transaction Here: https://explorer.solana.com/tx/${tx}?cluster=devnet`
);
