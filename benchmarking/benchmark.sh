#!/bin/bash

# Function to run and benchmark a command
benchmark_command() {
    local command=("$@")
    local start_time=$(date +%s.%N)
    "${command[@]}"
    local end_time=$(date +%s.%N)
    local execution_time=$(echo "$end_time - $start_time" | bc)
    echo "Execution Time: $execution_time seconds"
}

# Function to display system information
system_info() {
    echo "System Information:"
    echo "-------------------"
    uname -a
    echo ""
    lscpu
    echo ""
    free -h
}

# Main script
if [ $# -eq 0 ]; then
    echo "Usage: $0 <command> [args ...]"
    exit 1
fi

command_to_run=("$@")

echo "Benchmarking command: ${command_to_run[*]}"
echo "------------------------"

benchmark_command "${command_to_run[@]}"

system_info
