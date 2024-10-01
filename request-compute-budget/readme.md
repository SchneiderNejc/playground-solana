Source: https://solana.com/developers/guides/advanced/how-to-request-optimal-compute

When you pay priority fees on your transactions, you must specify the exact amount of compute units you expect to use; otherwise, you will overpay for your transaction. 

For precise control over your transaction's computational resources, use the setComputeUnitLimit instruction from the Compute Budget program. This instruction allocates a specific number of compute units for your transaction, ensuring you only pay for what you need.

The Solana helpers npm package includes getSimulationComputeUnits, a small function that uses simulateTransaction to calculate the compute units.
npm i @solana-developers/helpers

Special Considerations
The compute usage can change if the transaction you are executing has a call to find_program_address.

If you have a variable compute usage on your transactions, you can do one of two things:

Run a test over your transactions over some time to find out the ceiling compute unit usage and use that number.

Take the compute units returned from simulateTransaction and add a percentage to the total. For example, if you chose to add 10% more CU and the result you received from simulateTransaction was 1000 CU, you would set 1100 CU on your transaction.