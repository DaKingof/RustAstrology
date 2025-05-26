{ pkgs ? import /home/mend/nixpkgs {} }:

let
  # Use the standard Qt 5 package set
  qt5 = pkgs.libsForQt5;
  
  # Create a comprehensive Qt environment with all the necessary components
  qtEnv = with qt5; [
    qtbase
    qtdeclarative
    qtquickcontrols
    qtquickcontrols2
    qtgraphicaleffects
    qtsvg
    qtmultimedia
    qtwayland
    qttools
    qtwebengine
    qtwebchannel
    qtlocation
    qtgraphicaleffects
    qt3d
    qtimageformats
    qtcharts
    qtconnectivity
    qtscript
    qtsensors
    qtserialport
    qtspeech
    qtxmlpatterns
  ];
  
in
pkgs.mkShell {
  # Native build inputs (tools needed during build)
  nativeBuildInputs = with pkgs; [
    pkg-config
    cmake
    gnumake
    ninja
    clang
    lld
    rustc
    cargo
    rustup
    rustfmt
    rust-analyzer
    qt5.qttools
    qt5.qmake
    python3
    git
  ];

  # Build inputs (libraries and headers)
  buildInputs = with pkgs; [
    # Qt 5 with all required modules
  ] ++ qtEnv
    
    # System libraries
    ++ [
      openssl
      zlib
      libxml2
      libxslt
      
      # For qmlimportscanner
      qt5.qttools.bin
      
      # For qmake
      qt5.qtbase.dev
      
      # For bindgen
      llvmPackages.libclang
    ];

  # Environment variables
  QT_PLUGIN_PATH = "${qt5.qtbase.bin}/${qt5.qtbase.qtPluginPrefix}";
  QML2_IMPORT_PATH = with qt5; "${qtquickcontrols2}/${qtbase.qtQmlPrefix}";
  
  # Library paths for runtime linking
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath ([
    stdenv.cc.cc.lib
  ] ++ qtEnv);
  
  # Set QTDIR for qmake
  QTDIR = "${qt5.qtbase.dev}";

  # Shell hook for additional setup
  shellHook = ''
    # Set up Rust if not already installed
    if ! command -v rustc &> /dev/null; then
      echo "Installing Rust..."
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
      rustup default stable
    fi
    
    # Add Cargo binaries to PATH
    export PATH="$HOME/.cargo/bin:$PATH"
    
    # Set up Qt environment variables
    export QT_QPA_PLATFORM_PLUGIN_PATH="${qt5.qtbase.bin}/${qt5.qtbase.qtPluginPrefix}/platforms"
    
    # Helpful environment variables for development
    export QT_QUICK_CONTROLS_STYLE=Material
    export QT_QUICK_CONTROLS_MATERIAL_THEME=Light
    
    echo "Qt 5 development environment ready for Rust Astrology!"
    echo "Run './dev.sh' to start the application."
  '';
}