#!/bin/bash

# Rust Astrology Development Environment Script
# This script sets up and runs the development environment

echo "Starting Rust Astrology development environment..."

# Check if we're in nix-shell
if [ -z "$IN_NIX_SHELL" ]; then
    echo "Not in nix-shell, entering one now..."
    exec nix-shell --run "bash $0"
fi

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

# Build and serve with Trunk
echo "Building and serving with Trunk on port 3000..."
if command -v trunk >/dev/null 2>&1; then
    trunk serve --port 3000
else
    echo "Trunk not found. Installing..."
    cargo install trunk
    trunk serve --port 3000
fi
