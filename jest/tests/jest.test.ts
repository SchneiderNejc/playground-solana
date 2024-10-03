import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Jest } from "../target/types/jest";

describe("jest", () => {
  test("Is initialized!", async () => {
    // Start Bankrun with Anchor support
    const context = await startAnchor(".", [], []);
    const provider = new BankrunProvider(context);
    anchor.setProvider(provider);

    // Access your program
    const program = anchor.workspace.Jest as Program<Jest>;

    // Add your test logic
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
