#!/bin/bash

# Check if a file path was provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <path-to-file>"
    exit 1
fi

file="$1"

# Check if the file exists
if [ ! -f "$file" ]; then
    echo "Error: File '$file' not found!"
    exit 1
fi

# Count occurrences
live_count=$(grep -c "StorageLive(" "$file")
dead_count=$(grep -c "StorageDead(" "$file")

# Print results
echo "StorageLive( count: $live_count"
echo "StorageDead( count: $dead_count"

