// No imports needed: web3, borsh, pg and more are globally available

describe("Test", () => {
  it("Initialize", async () => {
    // Generate keypair for the new account
    const newAccountKp = new web3.Keypair();

    const instructionIndex = 0;
    const data = 42;

    // Create instruction data buffer
    const instructionData = Buffer.alloc(1 + 8);
    instructionData.writeUInt8(instructionIndex, 0);
    instructionData.writeBigUInt64LE(BigInt(data), 1);

    const instruction = new web3.TransactionInstruction({
      keys: [
        {
          pubkey: newAccountKp.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: pg.wallet.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: web3.SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: pg.PROGRAM_ID,
      data: instructionData,
    });

    const transaction = new web3.Transaction().add(instruction);

    const txHash = await web3.sendAndConfirmTransaction(
      pg.connection,
      transaction,
      [pg.wallet.keypair, newAccountKp]
    );
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Fetch Account
    const newAccount = await pg.connection.getAccountInfo(
      newAccountKp.publicKey
    );

    // Deserialize Account Data
    const deserializedAccountData = borsh.deserialize(
      AccountDataSchema,
      AccountData,
      newAccount.data
    );

    console.log(Number(deserializedAccountData.data));
  });
});

class AccountData {
  data = 0;
  constructor(fields: { data: number }) {
    if (fields) {
      this.data = fields.data;
    }
  }
}

const AccountDataSchema = new Map([
  [AccountData, { kind: "struct", fields: [["data", "u64"]] }],
]);
