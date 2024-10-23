import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Cpi } from "../target/types/cpi";

describe("cpi", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Cpi as Program<Cpi>;

  const [PDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("pda"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Fund PDA with SOL", async () => {
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: PDA,
      lamports: transferAmount,
    });

    const transaction = new Transaction().add(transferInstruction);

    const transactionSignature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [wallet.payer] // signer
    );

    console.log(
      `\nTransaction Signature:` +
        `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
    );
  });
});
