// scripts/init_treasury.ts
import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Idl } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import fs from "fs";
import path from "path";

async function main() {
  console.log("SCRIPT START (init_treasury.ts)");

  // -------------------------------
  // Load IDL
  // -------------------------------
  const idlPath = path.join(__dirname, "idl.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8")) as Idl;

  console.log("IDL loaded:", idl.address);

  // -------------------------------
  // Provider (using Anchor env)
  // -------------------------------
  const provider = AnchorProvider.env();
  anchor.setProvider(provider);

  // -------------------------------
  // Program (CORRECT CONSTRUCTOR)
  // Anchor reads programId from IDL automatically
  // -------------------------------
  const program = new anchor.Program(idl as Idl, provider);

  console.log("Program ID:", program.programId.toBase58());

  // -------------------------------
  // Derive Treasury PDA
  // -------------------------------
  const [treasuryPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  );

  console.log("Treasury PDA:", treasuryPDA.toBase58());

  // -------------------------------
  // Detect correct method name
  // -------------------------------
  const hasCamel = program.methods.initializeTreasury !== undefined;
  const hasSnake = program.methods.initialize_treasury !== undefined;

  if (!hasCamel && !hasSnake) {
    console.error("ERROR: initialize_treasury not found in program.methods");
    console.log(
      "Available methods:",
      Object.keys(program.methods).join(", ")
    );
    process.exit(1);
  }

  const method = hasCamel
    ? program.methods.initializeTreasury()
    : program.methods.initialize_treasury();

  // -------------------------------
  // Send Transaction
  // -------------------------------
  const tx = await method
    .accounts({
      treasury: treasuryPDA,
      authority: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("Treasury initialized!");
  console.log("Transaction:", tx);
}

main().catch((err) => {
  console.error("INIT TREASURY FAILED:");
  console.error(err);
  process.exit(1);
});
