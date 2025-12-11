console.log("TEST FILE START");

// Can we load fs?
import * as fs from "fs";
console.log("FS OK");

// Can we load anchor?
import * as anchor from "@coral-xyz/anchor";
console.log("ANCHOR OK");

// Can we load web3?
import { PublicKey } from "@solana/web3.js";
console.log("WEB3 OK");

// Can we load JSON?
const idl = require("../target/idl/candle_markets.json");
console.log("IDL OK:", idl.address);

// Now final test:
async function main() {
  console.log("MAIN REACHED");

  const [pda] = await PublicKey.findProgramAddress(
    [Buffer.from("treasury")],
    new PublicKey(idl.address)
  );

  console.log("PDA:", pda.toBase58());
}

main();
