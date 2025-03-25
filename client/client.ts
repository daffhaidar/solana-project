import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import type { SimpleStorage } from "../target/types/simple_storage";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SimpleStorage as anchor.Program<SimpleStorage>;

console.log("🚀 Solana Client Running...");

// Cek Wallet Address
console.log("📌 My address:", program.provider.publicKey.toString());

// Cek Balance Wallet
const balance = await program.provider.connection.getBalance(program.provider.publicKey);
console.log(`💰 My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);