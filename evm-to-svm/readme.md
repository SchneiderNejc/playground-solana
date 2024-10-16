Source: https://solana.com/developers/evm-to-svm/complete-guide

You do not have to redeploy another smart contract onto the blockchain when creating new tokens. Instead, you create a new account, known as the mint account, off of the Solana Token Program, where the account defines a set of values to give the number of tokens in circulation, decimal points, who can mint more tokens, and who can freeze tokens.

The Base Fee can be calculated based on the number of signatures in a transaction. Each signature costs 5000 lamports. Half of the base fees are burnt, and half are rewarded to the validators.

When a wallet wants to make a transaction, a recent blockhash will be pulled from the cluster to create a valid transaction. This recent blockhash only makes the transaction valid for 150 blocks after the recent blockhash was retrieved.

				Ethereum	Solana
Single Transaction Compute Cap	30,000,000	1,400,000 Compute Units
Block Compute Cap		30,000,000 Gas	48,000,000 Compute Units

Each account referenced may be at most 12,000,000 compute units used per block. This cap prevents people from write-locking a single account too many times in a single block, further preventing the local fee markets from being overrun by one account.
Another limit on transactions is the depth of instruction calls you can make in a single instruction. This limit is currently set to 4, meaning you can only call instructions at a depth of 4 before the transaction would revert. This makes re-entrancy issues nonexistent on Solana compared to something you’d have to worry about on Ethereum.