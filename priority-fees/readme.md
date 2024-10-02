base Transaction Fee is 5000 lamports per signature in your transaction.

A user can still have their transaction executed with no priority fee attached but with a lesser guarantee. When blocks are saturated with transactions with priority fees, validators will drop transactions without priority fees.

The higher the CU required for the transaction, the more fees you will pay when adding priority fees.

The getRecentPrioritizationFees JSON RPC API method will retrieve the lowest priority fees used recently to land a transaction in a block.

getRecentPrioritizationFees.ts
run: python3 recent_priority_fees.py
optional beautify: https://jsonlint.com/
interpret: 
if you notice that several slots have prioritization fees of 50,000 - 80,000 micro-lamports and occasional spikes up to 351,000, you might estimate that paying around 50,000 - 80,000 micro-lamports in priority fees would give you a decent chance of landing your transaction in the next block.
If the network is quiet: If most recent slots show prioritization fees of 0, you likely don't need to pay any additional fee to get your transaction processed quickly.
If the network is congested: If you see a lot of high prioritization fees in the recent slots (e.g., 50,000 micro-lamports or higher), you'll need to match or exceed those fees to get prioritized.

Or run the script directly, using ./get_recent_prioritization_fees.sh

---------------------------------


python virtual environment
install venv

create:
python3 -m venv myenv

activate:
source myenv/bin/activate

Install packages
pip install requests

Freeze dependencies into a file
pip freeze > requirements.txt


deactivate
deactivate

install the dependencies
pip install -r requirements.txt

