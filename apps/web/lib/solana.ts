import { Connection, PublicKey } from "@solana/web3.js";

export const connection = new Connection("https://api.devnet.solana.com");

export const PROGRAM_ID = new PublicKey(
  "Pred1xion1111111111111111111111111111111"
);
