// import { ... } from "@solana/web3.js"

const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
    units: 300,
  });

  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 20000,
  });

  const transaction = new Transaction()
    .add(modifyComputeUnits)
    .add(addPriorityFee)
    .add(
      SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: toAccount,
        lamports: 10000000,
      }),
    );