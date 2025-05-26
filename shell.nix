{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Trunk for Rust WASM development
    trunk
    
    # Basic build tools
    pkg-config
    cmake
    gnumake
    
    # System libraries for Tauri
    openssl
    
    # Webkit and GTK dependencies
    webkitgtk_4_1  # Specific version required by Tauri
    gtk3
    glib
    cairo
    pango
    atk
    gdk-pixbuf
    
    # For AppIndicator support
    libayatana-appindicator
    
    # Dependencies required by Tauri on Linux
    librsvg
    libsoup_2_4  # Renamed from libsoup
    
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