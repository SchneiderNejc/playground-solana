from solana.rpc.api import Client
from solana.publickey import PublicKey

# Connect to Solana Devnet
client = Client("https://api.devnet.solana.com")  # Using Devnet endpoint

# Replace with the wallet's public key
wallet_address = PublicKey("nejctPbWZauU1Zae5BsunZLEWAsDX1v5GiaBYrxW6rY")

# Query for all accounts owned by the wallet
response = client.get_program_accounts(wallet_address)

if response['result']:
    print(f"Accounts owned by wallet {wallet_address}:")
    for account in response['result']:
        print(f"Account: {account['pubkey']}")
        print(f"  - Owner: {account['account']['owner']}")
        print(f"  - Lamports: {account['account']['lamports']}")
        print(f"  - Executable: {account['account']['executable']}")
else:
    print(f"No accounts found for wallet {wallet_address}.")