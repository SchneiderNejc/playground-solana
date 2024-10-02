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

# Sort all fees, including zero fees
sorted_fees = sorted(prioritization_fees)

# Plot the prioritization fees over the slots
plt.figure(figsize=(10, 6))
plt.plot(slots, prioritization_fees, marker='o')
plt.title("Solana Prioritization Fees Over Slots")
plt.xlabel("Slot")
plt.ylabel("Prioritization Fee (micro-lamports)")
plt.grid(True)

# Save the plot as a PNG image instead of showing it
plt.savefig("prioritization_fees_plot.png")

# Print out the prioritization fees and slots in a readable format
for slot, fee in zip(slots, prioritization_fees):
    print(f"Slot: {slot}, Prioritization Fee: {fee} micro-lamports")

# Calculate the average of the top 10% highest prioritization fees (including zeros)
top_10_percent_index = int(len(sorted_fees) * 0.9)
top_10_percent_fees = sorted_fees[top_10_percent_index:]
average_top_10_percent = sum(top_10_percent_fees) / len(top_10_percent_fees) if top_10_percent_fees else 0

# Calculate the average of the bottom 75% prioritization fees (including zeros)
half_index = int(len(sorted_fees) * 0.75)
bottom_75_percent_fees = sorted_fees[:half_index]
average_bottom_75_percent = sum(bottom_75_percent_fees) / len(bottom_75_percent_fees) if bottom_75_percent_fees else 0

# Print the averages after fetching all the values
print(f"\nAverage of top 10% highest fees: {average_top_10_percent} micro-lamports")
print(f"Average of bottom 75% lowest fees: {average_bottom_75_percent} micro-lamports")
