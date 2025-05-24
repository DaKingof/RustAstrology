# Development Guide

## Getting Started

### Prerequisites
- Rust (via rustup)
- Node.js (for Tauri)
- Nix (for development environment)


### Setup

```bash
# Enter development environment
nix-shell

# Install Rust toolchain (if not already installed)
rustup install stable
rustup default stable
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM bundler)
cargo install trunk wasm-bindgen-cli

# Install Tauri CLI
cargo install tauri-cli
```

## Development Workflow

### Running the Application

#### Web Version
```bash
trunk serve
```

#### Desktop Version
```bash
cargo tauri dev
```

### Building for Production

#### Web
```bash
trunk build --release
```

#### Desktop
```bash
cargo tauri build
```

## Testing

### Unit Tests
```bash
cargo test
```

### WebAssembly Tests
```bash
wasm-pack test --node
```

## Code Style
- Follow Rust's official style guide
- Use `rustfmt` for code formatting
- Run `cargo clippy` for linting

## Commit Guidelines
- Use [Conventional Commits](https://www.conventionalcommits.org/)
- Keep commits small and focused
- Write meaningful commit messages

## Debugging
- Use `log` crate for application logging
- Set `RUST_LOG=debug` for detailed logs
- Use browser developer tools for frontend debugging
