---
description: This guide provides a comprehensive context layer for agents working on the Rust Astrology project, a modern astrology application built with Rust and WebAssembly, featuring Tauri for desktop application support.
---

Core Technologies
Language: Rust (edition 2021)
Frontend Framework: Leptos (v0.5)
Build System: Cargo
Development Environment: Nix (shell.nix)
Astrology Engine: Swiss Ephemeris (via swisseph crate)
Target Platforms: Web (WASM) and Desktop (via Tauri)
Key Dependencies
Core: leptos, wasm-bindgen, web-sys
Math & Astrology: nalgebra, swisseph
Date/Time: chrono
Serialization: serde, serde_json
Error Handling: thiserror, anyhow
Logging: log, env_logger
Development Environment
The project uses Nix for reproducible development environments. Key components:

Rust toolchain (installed via rustup)
Node.js (for Tauri)
System libraries: pkg-config, cmake, clang, zlib, openssl
Setup Commands
bash
CopyInsert
# Enter the development environment
nix-shell

# Install Rust if not present (handled by shell hook)
# Install additional Rust components
rustup target add wasm32-unknown-unknown
Project Structure
CopyInsert
rust_astrology/
├── Cargo.toml       # Rust project configuration
├── shell.nix        # Nix development environment
├── Trunk.toml       # Trunk (WASM bundler) config
├── index.html       # Main HTML entry point
├── src/             # Core application code
│   └── main.rs      # Main application entry point
└── src-tauri/       # Tauri desktop application
    ├── Cargo.toml   # Tauri-specific dependencies
    ├── assets/      # Static assets
    └── src/
        └── main.rs  # Tauri entry point
Key Components
1. Astrology Engine
Swiss Ephemeris: High-precision astronomical calculations
Features: Planetary positions, aspects, house systems
Integration: Accessed via the swisseph Rust crate
2. Frontend (Leptos)
Framework: Leptos for reactive UI
Rendering: Canvas/WebGL for chart rendering
State Management: Built-in reactivity system
3. Desktop Application (Tauri)
Wrapper: Provides native desktop experience
Integration: Bridges web frontend with system APIs
Development Workflow
Building
bash
CopyInsert
# Build for web (WASM)
cargo build --target wasm32-unknown-unknown

# Build for desktop (Tauri)
cargo tauri build
Running
bash
CopyInsert
# Development server (web)
trunk serve

# Desktop development
cargo tauri dev
Common Tasks
Adding Dependencies
Add to Cargo.toml under [dependencies]
Run cargo build to update
Debugging
Set RUST_BACKTRACE=1 for detailed error traces
Use log macros for application logging
Browser developer tools for frontend debugging
Best Practices
Code Organization
Keep UI components modular
Separate business logic from presentation
Use Rust's type system for domain modeling
Performance
Leverage Rust's zero-cost abstractions
Be mindful of WASM binary size
Use asynchronous operations for I/O
Next Steps
Set up CI/CD pipeline
Add comprehensive testing
Implement core astrology features
Design responsive UI components