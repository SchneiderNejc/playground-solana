base Transaction Fee is 5000 lamports per signature in your transaction.

A user can still have their transaction executed with no priority fee attached but with a lesser guarantee. When blocks are saturated with transactions with priority fees, validators will drop transactions without priority fees.

The higher the CU required for the transaction, the more fees you will pay when adding priority fees.

The getRecentPrioritizationFees JSON RPC API method will retrieve the lowest priority fees used recently to land a transaction in a block.