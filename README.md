# Rust Astrology

RustAstrology project published on GitHub from MCP

A modern astrology application built with Rust, WebAssembly, and Tauri. This application provides accurate astrological calculations using the Swiss Ephemeris and presents them through a beautiful, responsive web interface.

## Features

- Accurate Astrological Calculations: Powered by Swiss Ephemeris
- Cross-Platform: Runs on Windows, macOS, and Linux
- Web & Desktop: Build for both web and desktop with shared Rust code
- High Performance: Leverages WebAssembly for near-native performance
- Responsive UI: Built with Leptos for a reactive user interface

## Prerequisites

- Rust (latest stable)
- Node.js (for Tauri)
- Nix (for NixOS users)
- System dependencies (handled by Nix)

## Getting Started

### NixOS Setup

If you're on NixOS, you can enter the development environment with:

  nix-shell

This will:
1. Set up the Rust toolchain
2. Install necessary system dependencies
3. Configure the development environment

### Manual Setup

1. Install Rust using rustup:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Install WebAssembly targets:
   rustup target add wasm32-unknown-unknown
   rustup target add wasm32-wasi

3. Install Trunk:
   cargo install trunk

4. Install wasm-opt:
   cargo install wasm-opt

## Running the Application

### Web Version

  trunk serve --open

### Desktop Version (Tauri)

  cd src-tauri
  cargo tauri dev

## Project Structure

.
├── Cargo.toml           # Root Cargo.toml
├── shell.nix            # Nix development environment
├── .env                 # Environment variables
├── src/                 # Main application code
│   ├── lib.rs           # Library root
│   ├── main.rs          # Web entry point
│   └── ...
├── src-tauri/           # Tauri desktop application
│   ├── Cargo.toml
│   └── ...
└── index.html           # Web entry point

## License

This project is licensed under either of

  Apache License, Version 2.0
  (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

  MIT license
  (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.