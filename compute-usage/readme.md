Source: https://solana.com/developers/guides/advanced/how-to-optimize-compute

Solana programs have a few compute limitations to be aware of:
Max Compute per block: 48 million CU
Max Compute per account per block: 12 million CU
Max Compute per transaction: 1.4 million CU

Logging 
Logging is also very expensive. You should avoid logging non-essential information in your programs to keep your program usage down.
If you do want to log a pubkey, you can use .key() and .log() to efficiently log it with lower compute usage.

Data Types
Larger data types use more Compute Units overall. Make sure you actually need a larger data type such as a u64.

Serialization 
Serialization and deserialization are both expensive operations depending on the account struct. If possible, use zero copy and directly interact with the account data

Program Derived Addresses
If find_program_address has to take a long time to find a valid address, meaning it has a high bump, the overall compute unit usage will be higher. You can optimize finding the PDAs after initialization by saving the bump into an account and using it in the future. 
