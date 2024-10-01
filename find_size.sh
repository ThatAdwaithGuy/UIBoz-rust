#!/bin/bash

# Check if src directory exists
if [ ! -d "src" ]; then
    echo "Error: src directory not found"
    exit 1
fi

# Count lines recursively and sum them up
total_lines_src=$(find src -type f | xargs wc -l | tail -n 1 | awk '{print $1}')

total_lines_creates=$(find crates -type f | xargs wc -l | tail -n 1 | awk '{print $1}')
# Output the result
total_lines=$(($total_lines_src+$total_lines_creates))
echo "Total lines in src directory: $total_lines"
