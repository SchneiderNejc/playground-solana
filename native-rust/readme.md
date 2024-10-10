Source: https://solana.com/developers/guides/getstarted/intro-to-native-rust
Github: https://github.com/solana-playground/solana-playground

Entrypoint #
Every Solana program includes a single entrypoint used to invoke the program.  The process_instruction function is then used to process the data passed into the entrypoint.

Instructions #
While there is only one entrypoint, program execution can follow different paths depending on the instruction_data. It is common to define instructions as variants within an enum, where each variant represents a distinct instruction on the program.

To access the accounts provided to the program, use an iterator to iterate over the list of accounts passed into the entrypoint through the accounts argument. The next_account_info function is used to access the next item in the iterator.

All Solana accounts include a data field that can be used to store any arbitrary data as a byte array. 

Client #
Interacting with Solana programs written in native Rust involves directly building the TransactionInstruction.

Similarly, fetching and deserializing account data requires creating a schema compatible with the on-chain program's data structures.

commands:
cargo new native-rust --lib