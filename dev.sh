#!/usr/bin/env bash
set -e # Exit immediately if a command exits with a non-zero status.

# Rust Astrology Development Environment Script
# This script sets up and runs the development environment

# Check if we are already in a nix-shell. The IN_NIX_SHELL variable is set by nix-shell.
if [ -z "$IN_NIX_SHELL" ]; then
    # If not, re-execute this script within nix-shell
    echo "Not in nix-shell, re-executing with nix-shell..."
    # Note: Ensure your shell.nix or default.nix provides all necessary tools like trunk.
    exec nix-shell --run "$0 $@"
    exit $? # Exit with the status of nix-shell if exec fails for some reason (though it shouldn't)
fi

# --- We are now inside the nix-shell environment --- 

echo "Starting Rust Astrology development environment..."

echo "Rust Astrology development environment ready!"

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf dist/ target/

# Check for existing processes using port 3000
echo "Checking for existing processes using port 3000..."
if command -v lsof >/dev/null 2>&1 && lsof -Pi :3000 -sTCP:LISTEN -t >/dev/null ; then
    echo "Port 3000 is in use. Killing existing processes..."
    lsof -ti:3000 | xargs kill -9 2>/dev/null || true
    sleep 2
fi

echo "Starting development server from within nix-shell..."
# Clean previous builds
if trunk clean; then
    echo "Trunk clean successful."
else
    echo "Trunk clean failed. Proceeding anyway..."
fi

# Serve the application
echo "Building and serving with Trunk on port 3000..."
if command -v trunk >/dev/null 2>&1; then
    trunk serve --port 3000
else
    echo "Trunk not found. Installing..."
    cargo install trunk
    trunk serve --port 3000
fi
