# Rust Astrology - Uranian Dial

A modern astrology application built with Rust and Qt, featuring an interactive Uranian Astrology Dial. This application provides accurate astrological calculations and presents them through a native Qt-based user interface with a focus on Uranian astrology techniques.

## Features

- **Interactive Uranian Dial**: Visualize planetary positions and aspects on a 360° dial
- **Harmonic Analysis**: Apply harmonics to the dial (1x to 90x) for advanced astrological techniques
- **Accurate Calculations**: Built with precise astronomical calculations
- **Real-time Interaction**: Rotate and zoom the dial with intuitive mouse and touch controls
- **Customizable Display**: Toggle zodiac signs, degree marks, planets, and midpoints
- **Cross-Platform**: Runs on Windows, macOS, and Linux
- **Native Performance**: Leverages Qt for smooth, responsive UI
- **Modern UI**: Built with Qt Quick and QML for a polished experience

## Prerequisites

- Rust (latest stable)
- Qt 5.x development libraries
- Nix (for development environment setup, recommended for all platforms)

## Development Setup

### Using the Development Script (Recommended)

This project includes a `dev.sh` script that sets up the development environment and runs the application:

```bash
# Make the script executable
chmod +x dev.sh

# Run the development environment
./dev.sh
```

This script will:
1. Enter a Nix shell with all required dependencies
2. Set up Qt environment variables
3. Build and run the application

### Manual Setup (Alternative)

If you prefer not to use the development script, you can set up the environment manually:

1. Install Rust using rustup:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Nix (if not already installed):
   ```bash
   curl -L https://nixos.org/nix/install | sh
   ```

3. Enter the Nix development shell:
   ```bash
   nix-shell
   ```

4. Build and run the application:
   ```bash
   cargo run
   ```

## Project Structure

- `src/` - Application source code
  - `main.rs` - Main application entry point
- `qml/` - QML UI files
  - `main.qml` - Main application window
- `shell.nix` - Nix development environment configuration
- `dev.sh` - Development script for easy setup and execution
- `Cargo.toml` - Rust project configuration and dependencies

## Development Workflow

### Building the Application

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Running the Application

```bash
# Run in debug mode (recommended for development)
./dev.sh

# Or run directly (if environment is already set up)
cargo run
```

### Dependencies

- `qmetaobject` - Rust bindings for Qt's meta-object system
- Qt 5.x - For the GUI framework
  - qtbase
  - qtdeclarative
  - qtquickcontrols2
  - qtgraphicaleffects

## Troubleshooting

### Common Issues

1. **Qt not found**
   - Ensure you're running the application through `dev.sh` or have set up the Qt environment variables correctly

2. **Missing libraries**
   - Run `nix-shell` to ensure all dependencies are available

3. **QML module not found**
   - Make sure the `QML2_IMPORT_PATH` environment variable is set correctly

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Desktop Build

```bash
cd src-tauri
cargo tauri build
```

## Project Structure

- `/src`: Frontend Rust code (Leptos components)
- `/src-tauri`: Tauri application code
- `/assets`: Static assets
- `/dist`: Production build output (web)
- `index.html`: Main HTML entry point
- `Trunk.toml`: Trunk configuration

## Dependencies

Key dependencies:
- Tauri v2: For building cross-platform desktop apps
- Leptos: For reactive UI
- wasm-bindgen: For JavaScript/Rust interop
- Trunk: WebAssembly web application bundler

## License

MIT

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