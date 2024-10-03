source: https://solana.com/developers/guides/advanced/introduction-to-durable-nonces

durable nonces provide an opportunity to create and sign a transaction that can be submitted at any point in the future.

Durable Nonce Applications
Scheduled Transactions - Users can pre-sign a transaction and then submit it at a later date, allowing for planned transfers
Multisig Wallets - one party signs a transaction, and others may confirm at a later time. 
Programs Requiring Future Interaction - ensures the contract interaction happens at the correct time without necessitating the presence of the transaction creator.
Cross-chain Interactions - sign the transaction with a Durable Nonce and then execute it once the required confirmations are received
Decentralized Derivatives Platforms

Double Spend #
Solution: Crosscheck signatures within only a set period of recent time, and discard the transaction if it gets "too" old.

 A blockhash contains a 32-byte SHA-256 hash. It is used to indicate when a client last observed the ledger. 
Using recent blockhashes, transactions are checked in the last 150 blocks. If they are found, they are rejected. They are also rejected if they get older than 150 blocks.

Durable Nonces #
Durable Transaction Nonces, which are 32-byte in length (usually represented as base58 encoded strings), are used in place of recent blockhashes to make every transaction unique (to avoid double-spending) while removing the mortality on the unexecuted transaction.

Nonce Account #
The Nonce Account is the account that stores the value of the nonce. This account is owned by the SystemProgram and is rent-exempt;

Nonce Authority #
Nonce authority is the account that controls the Nonce Account.  By default, the account that creates the Nonce Account is delegated as the Nonce Authority, but it's possible to transfer the authority onto a different keypair account.

Create Nonce Authority 
solana-keygen new -o nonce-authority.json
solana config set -k ~/<path>/nonce-authority.json
solana airdrop 2

Create Nonce Account #
solana-keygen new -o nonce-account.json
solana create-nonce-account nonce-account.json 0.0015

Fetch Nonce #
solana nonce nonce-account.json

Displace Nonce Account #
solana nonce-account nonce-account.json

Advancing Nonce #
As discussed before, advancing the Nonce or changing the value of the nonce is an important step for making subsequent transactions unique. The Nonce Authority needs to sign the transaction with the nonceAdvance instruction.
solana new-nonce nonce-account.json

Withdraw from Nonce Account #
solana withdraw-from-nonce-account nonce-account.json nonce-authority.json 0.0000001


--------------------

To sign an offline transaction, we need to use:

--sign-only: which prevents clients from sending the transaction.
--blockhash: which lets us specify a recent blockhash so that the client does not try to fetch for it in an offline setting.

We can get a recent blockhash from https://solscan.io/blocks?cluster=devnet