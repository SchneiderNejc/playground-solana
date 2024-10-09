n this example, a Program Derived Address (PDA) is used for the GameDataAccount address. This enables us to deterministically locate the address later on. It is important to note that the PDA in this example is generated with a single fixed value as the seed (level1), limiting our program to creating only one GameDataAccount. The init_if_needed constraint then ensures that the GameDataAccount is initialized only if it doesn't already exist.

Alternatively, you can use the signer's address as an extra seed in the initialize instruction, which would enable each player to create their own GameDataAccount.

For deployment and interaction Solana Playground was used.
https://beta.solpg.io/