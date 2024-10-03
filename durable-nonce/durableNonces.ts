// @todo import solana-web3.js

// # Create Nonce Authority #

const nonceAuthKP = Keypair.generate();
// airdrop some SOL into this account from https://solfaucet.com/

// # Create Nonce Accounts #

const nonceKeypair = Keypair.generate();
const tx = new Transaction();

// the fee payer can be any account
tx.feePayer = nonceAuthKP.publicKey;

// to create the nonce account, you can use fetch the recent blockhash
// or use a nonce from a different, pre-existing nonce account
tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

tx.add(
  // create system account with the minimum amount needed for rent exemption.
  // NONCE_ACCOUNT_LENGTH is the space a nonce account takes
  SystemProgram.createAccount({
    fromPubkey: nonceAuthKP.publicKey,
    newAccountPubkey: nonceKeypair.publicKey,
    lamports: 0.0015 * LAMPORTS_PER_SOL,
    space: NONCE_ACCOUNT_LENGTH,
    programId: SystemProgram.programId,
  }),
  // initialise nonce with the created nonceKeypair's pubkey as the noncePubkey
  // also specify the authority of the nonce account
  SystemProgram.nonceInitialize({
    noncePubkey: nonceKeypair.publicKey,
    authorizedPubkey: nonceAuthKP.publicKey,
  })
);

// sign the transaction with both the nonce keypair and the authority keypair
tx.sign(nonceKeypair, nonceAuthKP);

// send the transaction
const sig = await sendAndConfirmRawTransaction(
  connection,
  tx.serialize({ requireAllSignatures: false })
);
console.log("Nonce initiated: ", sig);

// # Fetch Initialised Nonce Account #

const accountInfo = await connection.getAccountInfo(nonceKeypair.publicKey);
const nonceAccount = NonceAccount.fromAccountData(accountInfo.data);

// # Sign Transaction using Durable Nonce #

// make a system transfer instruction
const ix = SystemProgram.transfer({
  fromPubkey: publicKey,
  toPubkey: publicKey,
  lamports: 100,
});

// make a nonce advance instruction
const advanceIX = SystemProgram.nonceAdvance({
  authorizedPubkey: nonceAuthKP.publicKey,
  noncePubkey: noncePubKey,
});

// add them to a transaction
const tx = new Transaction();
tx.add(advanceIX);
tx.add(ix);

// use the nonceAccount's stored nonce as the recentBlockhash
tx.recentBlockhash = nonceAccount.nonce;
tx.feePayer = publicKey;

// sign the tx with the nonce authority's keypair
tx.sign(nonceAuthKP);

// make the owner of the publicKey sign the transaction
// this should open a wallet popup and let the user sign the tx
const signedTx = await signTransaction(tx);

// once you have the signed tx, you can serialize it and store it
// in a database, or send it to another device. You can submit it
// at a later point, without the tx having a mortality
const serialisedTx = bs58.encode(
  signedTx.serialize({ requireAllSignatures: false })
);
console.log("Signed Durable Transaction: ", serialisedTx);
