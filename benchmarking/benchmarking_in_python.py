import time

# Start measuring time
start_time = time.perf_counter()

# Place your code here that you want to measure

# End measuring time
end_time = time.perf_counter()

# Calculate the elapsed time
elapsed_time = (end_time - start_time) * 1000.0

# Print the elapsed time in milliseconds
print(f"Elapsed time: {elapsed_time} milliseconds")
