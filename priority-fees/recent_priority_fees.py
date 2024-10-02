import requests
import matplotlib.pyplot as plt

# Define the URL for the Solana devnet endpoint
url = "https://api.devnet.solana.com"

# Prepare the request payload
payload = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getRecentPrioritizationFees",
    "params": [["CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY"]]
}

# Send the POST request
response = requests.post(url, json=payload)

# Parse the response
data = response.json()

# Extract prioritization fees and slots from the result
prioritization_fees = [item['prioritizationFee'] for item in data['result']]
slots = [item['slot'] for item in data['result']]

# Plot the prioritization fees over the slots
plt.figure(figsize=(10, 6))
plt.plot(slots, prioritization_fees, marker='o')
plt.title("Solana Prioritization Fees Over Slots")
plt.xlabel("Slot")
plt.ylabel("Prioritization Fee (micro-lamports)")
plt.grid(True)

# Display the plot
plt.show()

# Print out the prioritization fees and slots in a readable format
for slot, fee in zip(slots, prioritization_fees):
    print(f"Slot: {slot}, Prioritization Fee: {fee} micro-lamports")
