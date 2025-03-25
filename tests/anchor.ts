import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { SimpleStorage } from "../target/types/simple_storage";

// Setup provider
const provider = anchor.AnchorProvider.local();
anchor.setProvider(provider);

// Fetch IDL biar program dikenal
async function getProgram() {
  const idl = await anchor.Program.fetchIdl("11111111111111111111111111111111");
  return new anchor.Program(idl, "11111111111111111111111111111111", provider);
}

describe("simple_storage", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SimpleStorage as anchor.Program<SimpleStorage>;
  
  let program;
  let storageAccount = anchor.web3.Keypair.generate();

  before(async () => {
    program = await getProgram();
  });

  it("Initialize storage", async () => {
    const tx = await program.methods
      .initialize(new anchor.BN(42))
      .accounts({
        storageAccount: storageAccount.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([storageAccount])
      .rpc();

    console.log("Transaction Signature", tx);
  });

  it("Update storage value", async () => {
    const newValue = new anchor.BN(100);

    const tx = await program.methods
      .update(newValue)
      .accounts({
        storageAccount: storageAccount.publicKey,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Updated storage value:", newValue.toString());
  });

  it("Read storage value", async () => {
    const account = await program.account.storageAccount.fetch(storageAccount.publicKey);
    console.log("Stored Value:", account.storedValue.toString());

    // Manual assertion tanpa chai
    if (account.storedValue.toString() !== "100") {
      throw new Error("Stored value mismatch!");
    }
  });
});