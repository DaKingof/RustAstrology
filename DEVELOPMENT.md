# Development Guide

## Table of Contents
- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Style](#code-style)
- [Debugging](#debugging)
- [Performance](#performance)
- [Deployment](#deployment)

## Prerequisites
- Rust (latest stable)
- Node.js (LTS version)
- Nix (for development environment)
- Git

## Setup

### Clone the Repository
```bash
git clone https://github.com/yourusername/rust-astrology.git
cd rust-astrology
```

### Install Dependencies
```bash
nix-shell
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

## Development Workflow

### Running Locally
```bash
# Start development server
trunk serve

# Or for desktop development
cargo tauri dev
```

### Building
```bash
# Development build
cargo build

# Release build
cargo build --release
```

## Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run WASM tests
wasm-pack test --node
```

## Code Style
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Use `cargo clippy` for linting

## Debugging
- Use `log` crate for application logging
- Set `RUST_LOG=debug` for detailed logs
- Use browser developer tools for frontend debugging

## Performance
- Profile with `cargo flamegraph`
- Check WASM binary size with `wasm-bindgen --target web`
- Use `#[inline]` strategically

## Deployment

### Web
```bash
trunk build --release
# Output will be in the `dist` directory
```

### Desktop
```bash
cargo tauri build
# Output will be in `src-tauri/target/release`
```
