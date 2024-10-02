#!/bin/bash

curl https://api.devnet.solana.com -X POST -H "Content-Type: application/json" -d '
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getRecentPrioritizationFees",
  "params": [
    ["CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY"]
  ]
}'
