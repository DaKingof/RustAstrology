{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Trunk for Rust WASM development
    trunk
    
    # Basic build tools
    pkg-config
    cmake
    gnumake
    atkmm

    # System libraries for Tauri
    openssl
    
    # GTK and related dependencies
    gdk-pixbuf
    
    # QT dependencies
    qt6.full
    qt6.qtwebengine
    qt6.qtwebsockets
    qt6.qttools
    qt6.qtbase
    qt6.qtdeclarative
    qt6.qtsvg
    
    # WebEngine dependencies
    nss
    nspr
    libxkbcommon
    
    # Node.js (for Tauri CLI)
    nodejs
    
    # Rust toolchain
    rustup
    
    # Tools for icon generation
    imagemagick
  ];

  shellHook = ''
    # Set up Rust if not already installed
    if ! command -v rustc &> /dev/null; then
      echo "Installing Rust..."
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
      rustup default stable
      rustup target add wasm32-unknown-unknown
    fi
    
    export PATH="$HOME/.cargo/bin:$PATH"
    echo "Rust Astrology development environment ready!"
  '';
}