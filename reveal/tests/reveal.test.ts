import { before, describe, test } from "node:test";
import assert from "node:assert/strict";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Reveal } from "../target/types/reveal";
import {
  getExplorerLink,
  confirmTransaction,
} from "@solana-developers/helpers";
const log = console.log;

// See https://medium.com/solana-fun/how-to-switch-test-runner-to-jest-instead-of-mocha-in-anchor-99e6035a3940
// except don't use jest because everyone hates jest.
describe("reveal", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Reveal as Program<Reveal>;

  test("reveal() function writes correct data to the chain", async () => {
    // encrypt the data
    // await program.methods
    //   .make(seed, new BN(1e6), new BN(1e6))
    //   .accounts({ ...accounts })
    //   .signers([maker])
    //   .rpc()
    //   .then(confirm)
    //   .then(log);

    const thing = await program.methods.initialize();

    log(`❤️❤️❤️❤️❤️❤️❤️❤️`);
    log(thing);
    // .accounts({ ...accounts })
    //   .signers([maker])
    //   .rpc();

    // await confirmTransaction(provider.connection, transactionSignature);
    // const explorerLink = getExplorerLink("transaction", transactionSignature);

    // Find the account
    // const pda = findProgramAddress();
    // Check the data inside
    // ensure it matches

    assert(true);
  });

  // console.log("Your transaction signature", transactionSignature);
  assert(true);
});
