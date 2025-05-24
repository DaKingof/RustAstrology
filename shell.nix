{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [

    trunk
    # Basic build tools
    pkg-config
    cmake
    clang
    
    # System libraries
    zlib
    openssl
    
    # Node.js (for Tauri and npm packages)
    nodejs
    
    # Rust (minimal installation, rest via rustup)
    rustup
  ];

  # Simple shell hook to set up Rust
  shellHook = ''
    export PATH="$HOME/.cargo/bin:$PATH"
    
    # Install stable Rust if not present
    if ! command -v rustc &> /dev/null; then
      echo "Installing Rust..."
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
      rustup default stable
      rustup target add wasm32-unknown-unknown
    fi
    
    echo "Rust development environment ready!"
    echo "Run 'cargo install' to install Rust tools"
  '';

  # Basic environment variables
  RUST_BACKTRACE = "1";
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  
  # SSL configuration
  SSL_CERT_FILE = "${cacert}/etc/ssl/certs/ca-bundle.crt";
}