#!/bin/bash
# Quick run script for GYeet

set -e

# Build if binary doesn't exist
if [ ! -f "target/release/gyeet" ]; then
    echo "Binary not found. Building..."
    ./build.sh
fi

# Run the application
echo "🎮 Launching GYeet..."
./target/release/gyeet

