#!/usr/bin/env bash
set -e

# Check if we're in a nix-shell, if not enter one
if [ -z "$IN_NIX_SHELL" ]; then
  echo "Not in nix-shell, entering one now..."
  exec nix-shell --run "$0 $@"
fi

echo "Starting Rust Astrology development environment with QT..."

# Set QT environment variables
export QT_QUICK_BACKEND=software
export QT_QUICK_CONTROLS_STYLE=Basic

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf ./pkg ./target/wasm32-unknown-unknown/debug/RustAstrology.wasm

# Step 1: Build the WebAssembly frontend binary
echo "Building WebAssembly frontend binary..."
cargo build --target wasm32-unknown-unknown --no-default-features --features wasm

# Step 2: Process the WebAssembly binary with wasm-bindgen
echo "Processing WebAssembly binary with wasm-bindgen..."
wasm-bindgen target/wasm32-unknown-unknown/debug/RustAstrology.wasm --out-dir ./pkg --target web --no-typescript

# Step 3: Run the Trunk server for WebAssembly frontend in the background
echo "Starting WebAssembly frontend server..."
(trunk serve) &
TRUNK_PID=$!

# Function to clean up background processes
cleanup() {
  echo "Cleaning up..."
  kill $TRUNK_PID 2>/dev/null || true
  exit 0
}

# Set up trap to clean up on script exit
trap cleanup EXIT

# Give the Trunk server time to start
echo "Waiting for WebAssembly server to start..."
for i in {1..10}; do
  if curl -s http://localhost:8080 >/dev/null; then
    break
  fi
  sleep 1
done

# Step 4: Run the Tauri application with QT
echo "Starting Tauri desktop application with QT..."
cd src-tauri
cargo run --no-default-features --features desktop

# If we get here, the Tauri app has exited
cleanup