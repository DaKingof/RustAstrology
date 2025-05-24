# Project Context

## Project Vision
Build a high-performance, accurate, and user-friendly astrology application using Rust and WebAssembly, with both web and desktop interfaces.

## Key Components

### Core Engine
- **Swiss Ephemeris**: High-precision astronomical calculations
- **Calculations**: Planetary positions, aspects, house systems, and transits
- **Performance**: Optimized for real-time chart rendering

### Frontend
- **Framework**: Leptos for reactive UI
- **Visualization**: Canvas/WebGL for interactive charts
- **Responsive**: Works on desktop and mobile

### Desktop Integration (Tauri)
- Native system integration
- Offline functionality
- System tray support

## Technical Stack
- **Language**: Rust (2021 edition)
- **Frontend**: WebAssembly, WebGL, JavaScript/TypeScript
- **Build Tools**: Cargo, Trunk, wasm-pack
- **Testing**: wasm-bindgen-test, wasm-pack-test

## Project Status
- [ ] Core astronomical calculations
- [ ] Basic chart rendering
- [ ] User interface implementation
- [ ] Desktop application wrapper
- [ ] Testing and optimization

## Known Limitations
- WebAssembly memory constraints
- Cross-platform compatibility considerations
- Performance with complex calculations
