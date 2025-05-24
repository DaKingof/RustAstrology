# Contributing to Rust Astrology

Thank you for your interest in contributing! We welcome all forms of contributions, including bug reports, feature requests, documentation improvements, and code contributions.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Reporting Issues](#reporting-issues)
- [Feature Requests](#feature-requests)
- [License](#license)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/rust-astrology.git`
3. Create a new branch: `git checkout -b my-feature-branch`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin my-feature-branch`
8. Open a pull request

## Development Workflow

### Prerequisites
- Rust (latest stable)
- Node.js (LTS version)
- Nix (for development environment)

### Setup

```bash
# Install development dependencies
nix-shell

# Install Rust toolchain
rustup target add wasm32-unknown-unknown

# Install additional tools
cargo install trunk wasm-bindgen-cli
```

### Running the Application

```bash
# Web version
trunk serve

# Desktop version
cargo tauri dev
```

## Pull Request Process

1. Ensure any install or build dependencies are removed before the build step
2. Update the README.md with details of changes if needed
3. Increase the version number in any examples and the README.md to the new version that this Pull Request would represent
4. The PR will be reviewed by maintainers

## Coding Standards

### Rust
- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `rustfmt` for code formatting
- Run `cargo clippy` for additional linting

### Web
- Follow web accessibility guidelines
- Ensure responsive design
- Optimize for performance

## Testing

### Writing Tests
- Write unit tests for new features
- Add integration tests for critical paths
- Include wasm-bindgen tests for web components

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run WASM tests
wasm-pack test --node
```

## Documentation

### Code Documentation
- Document all public APIs
- Include examples for complex functions
- Keep documentation up-to-date with code changes

### Project Documentation
- Update README.md for major changes
- Add or update documentation in the `docs` directory
- Include usage examples

## Reporting Issues

When reporting issues, please include:
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Environment details (OS, browser, Rust version, etc.)
- Any relevant error messages

## Feature Requests

For feature requests, please:
1. Check if the feature already exists
2. Explain why this feature would be valuable
3. Include any relevant use cases
4. Consider opening a discussion first for feedback

## License

By contributing, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
