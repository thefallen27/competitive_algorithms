# Benchmarking Code Snippets

This repository contains code snippets in Python, C++, and Rust to measure and benchmark the execution time of your code.
You can use these snippets to profile your code's performance.

## Benchmarking Script

Additionally, a benchmarking script (`benchmark.sh`) is provided. This script allows you to run and measure the execution time of a specified command. To use it:

```
./benchmark.sh <command> [args ...]
```
For example, to benchmark a Python script, you can run:
```
./benchmark.sh python script.py
```

If the command passed to the script requires input from a file, you can redirect the input using standard input redirection (<).
For example:
```
./benchmark.sh python script.py < input.txt
```

System Information

The system_info function in the benchmarking script displays system information, including OS version, CPU details, and available memory.

Please note that you may need to install additional software or libraries, depending on the code snippets and commands you want to benchmark.
