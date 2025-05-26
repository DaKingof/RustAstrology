#!/usr/bin/env bash
set -e

# Remove the existing Cargo.lock file
rm -f Cargo.lock

# Generate a new Cargo.lock file
cargo update

# Make the dev.sh script executable
chmod +x dev.sh

# Print success message
echo "Cargo.lock has been regenerated successfully."