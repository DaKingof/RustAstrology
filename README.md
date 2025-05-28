# Natal Astrology Chart Dials

A modern astrology application built with Rust, WebAssembly, and Leptos. This application provides interactive natal astrology chart dials with precise harmonic calculations and a beautiful, responsive interface.

## Features

- **Dual Chart Dials**: Left 360° dial and right harmonic dial working in tandem
- **Counterclockwise Rotation**: Traditional natal astrology chart orientation
- **Harmonic Arms**: Dynamic arms based on selected harmonic (3rd, 4th, 8th, 12th, 16th, 32nd, 64th)
- **Interactive Controls**: Click-to-select harmonic picker with real-time updates
- **Planet Symbols**: All major planets positioned with accurate colors
- **Aries Symbol**: Special blue Aries ♈ symbol as requested
- **High Performance**: Built with Rust and WebAssembly for near-native speed
- **Responsive Design**: Works on desktop and mobile devices

## Technical Implementation

### Left Dial (360°)
- Fixed degree labels every 10° counterclockwise (0°-180° and back to 0°)
- Harmonic arms = selected harmonic × 4
- Static 360° scale with traditional natal chart rotation
- All planets positioned on outer circumference

### Right Dial (Harmonic)
- Dynamic scale: 0° to (360/harmonic/2)°
- Planets positioned using harmonic modulus calculation
- Real-time updates when harmonic selection changes
- Simplified degree labeling for clarity

### Interactive Features
- Harmonic picker with 7 options (3rd through 64th)
- Click-to-select interface for easy interaction
- Real-time dial updates and calculations
- Professional dark theme with smooth animations

## Prerequisites

- Rust (latest stable)
- Node.js (for development tools)
- Nix (for NixOS users - optional)

## Getting Started

### Quick Start (using the development script)

```bash
# Clone and navigate to the project
git clone <repository-url>
cd rust-astrology

# Run the development environment
./dev.sh
```

### Manual Setup

1. **Install Rust and WebAssembly targets:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. **Install Trunk (WebAssembly bundler):**
   ```bash
   cargo install trunk
   ```

3. **Start development server:**
   ```bash
   trunk serve --port 3000
   ```

### NixOS Setup

If you're on NixOS, you can enter the development environment with:

```bash
nix-shell
```

This will automatically set up the Rust toolchain and necessary system dependencies.

## Running the Application

### Development Mode
```bash
trunk serve --port 3000
```

The application will be available at `http://localhost:3000`

### Production Build
```bash
trunk build --release
```

Output will be in the `dist/` directory.

## Project Structure

```
rust-astrology/
├── src/
│   └── main.rs           # Main application code with all components
├── Cargo.toml           # Rust dependencies and configuration
├── Trunk.toml           # WebAssembly build configuration
├── index.html           # HTML entry point with styling
├── shell.nix            # Nix development environment
├── dev.sh               # Development script
└── README.md            # This file
```

## Key Components

### Chart Renderer
- `ChartRenderer` struct with static methods for drawing
- Counterclockwise coordinate conversion from degrees
- Circle structures, harmonic arms, and planet symbols
- Optimized canvas rendering for smooth performance

### Harmonic System
- `HarmonicType` enum with all supported harmonics
- Dynamic arm calculation (harmonic × 4)
- Real-time harmonic scale adjustment
- Planet position modulus calculations

### Interactive UI
- Leptos reactive components for real-time updates
- Click-to-select harmonic picker
- Professional dark theme styling
- Responsive layout for different screen sizes

## Supported Harmonics

- **3rd Harmonic** (12 arms, 0°-60° scale)
- **4th Harmonic** (16 arms, 0°-45° scale) - Default
- **8th Harmonic** (32 arms, 0°-22.5° scale)
- **12th Harmonic** (48 arms, 0°-15° scale)
- **16th Harmonic** (64 arms, 0°-11.25° scale)
- **32nd Harmonic** (128 arms, 0°-5.625° scale)
- **64th Harmonic** (256 arms, 0°-2.8125° scale)

## Planet Symbols and Colors

- ☉ Sun (Gold)
- ☽ Moon (Silver)
- ☿ Mercury (Orange)
- ♀ Venus (Light Green)
- ♂ Mars (Tomato)
- ♃ Jupiter (Dodger Blue)
- ♄ Saturn (Saddle Brown)
- ♅ Uranus (Dark Turquoise)
- ♆ Neptune (Royal Blue)
- ♇ Pluto (Dark Red)
- **♈ Aries (Blue)** - Special blue coloring as requested

## Browser Compatibility

- Modern browsers with WebAssembly support
- Chrome 57+, Firefox 52+, Safari 11+, Edge 16+
- Mobile browsers on iOS and Android

## Development

### Building
```bash
# Development build
cargo build

# Release build  
cargo build --release

# WebAssembly build
trunk build --release
```

### Testing
```bash
# Run Rust tests
cargo test

# Run WebAssembly tests
wasm-pack test --node
```

## Performance

- WebAssembly compilation for near-native performance
- Optimized canvas rendering with minimal redraws
- Efficient harmonic calculations
- Responsive design with smooth animations

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
