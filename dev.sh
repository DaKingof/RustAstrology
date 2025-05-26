#!/usr/bin/env zsh
set -e

# Check if we're in a nix-shell, if not enter one
if [ -z "$IN_NIX_SHELL" ]; then
  echo "Not in nix-shell, entering one now..."
  exec nix-shell --run "$0 $@"
fi

echo "Starting Rust Astrology development environment..."

# Step 1: First just build the WebAssembly frontend binary
echo "Building WebAssembly frontend binary..."
cargo build --target wasm32-unknown-unknown --no-default-features --features wasm

# Step 2: Process the WebAssembly binary with wasm-bindgen
echo "Processing WebAssembly binary with wasm-bindgen..."
wasm-bindgen target/wasm32-unknown-unknown/debug/RustAstrology.wasm --out-dir ./pkg --target web --no-typescript

# Step 3: Run the Trunk server for WebAssembly frontend in the background
echo "Starting WebAssembly frontend server..."
(trunk serve) &
TRUNK_PID=$!

# Give the Trunk server time to start
sleep 3

# Step 4: Run the Tauri application
echo "Starting Tauri desktop application..."
cd src-tauri
cargo run

# Clean up the Trunk server when Tauri exits
kill $TRUNK_PID 2>/dev/null || true