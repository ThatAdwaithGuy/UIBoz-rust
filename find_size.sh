#!/bin/bash

# Check if src directory exists
if [ ! -d "src" ]; then
    echo "Error: src directory not found"
    exit 1
fi

# Count lines recursively and sum them up
total_lines=$(find src -type f | xargs wc -l | tail -n 1 | awk '{print $1}')

# Output the result
echo "Total lines in src directory: $total_lines"
