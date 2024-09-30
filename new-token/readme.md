Course url:
https://solana.com/developers/guides/getstarted/how-to-create-a-token

IPFS links:
https://bafybeigg4dwv5aftpciagaxmldjg4nagpes57drk6dbe5vcdgyoryezrly.ipfs.w3s.link/token.png
https://bafybeigwwv5qgrjma73hzge5o4hvbsbq3wacybp36gic5d2pryfttyqhb4.ipfs.w3s.link/metadata.json

Addresses:
bosNMKY5WJxUKDFgZdk9aWrXGnCznWpmwYnWhC9WYds - token owner
mntx98uQ71yUdNKvvQ2hCwDFnMiEFR7dknASU9bvjLY - token 
frenBLRGGkDaYZkbTh66uxVEfx8qrPwdn5C1XZUeKt8 - friend

CLI commands:
solana-keygen grind --starts-with bos:1
solana config set --keypair bosNMKY5WJxUKDFgZdk9aWrXGnCznWpmwYnWhC9WYds.json
solana config set --url devnet
solana config get
spl-token create-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb --enable-metadata mntx98uQ71yUdNKvvQ2hCwDFnMiEFR7dknASU9bvjLY.json --decimals 2
spl-token initialize-metadata mntx98uQ71yUdNKvvQ2hCwDFnMiEFR7dknASU9bvjLY 'Extra token' 'XTRA' https://bafybeigwwv5qgrjma73hzge5o4hvbsbq3wacybp36gic5d2pryfttyqhb4.ipfs.w3s.link/metadata.json
spl-token create-account mntx98uQ71yUdNKvvQ2hCwDFnMiEFR7dknASU9bvjLY
spl-token mint mntx98uQ71yUdNKvvQ2hCwDFnMiEFR7dknASU9bvjLY 100
spl-token transfer mntx98uQ71yUdNKvvQ2hCwDFnMiEFR7dknASU9bvjLY 15 frenBLRGGkDaYZkbTh66uxVEfx8qrPwdn5C1XZUeKt8 --fund-recipient