#!/usr/bin/env bash
set -e

# Check if we're in a nix-shell, if not enter one
if [ -z "$IN_NIX_SHELL" ]; then
  echo "Not in nix-shell, entering one now..."
  exec nix-shell --run "$0 $@"
fi

echo "Starting Rust Astrology Qt development environment..."

# Set Qt environment variables
export QT_QUICK_CONTROLS_STYLE=Material
export QT_QUICK_CONTROLS_MATERIAL_THEME=Light
export QT_LOGGING_RULES="*.debug=true;qt.*=false"
export RUST_LOG=debug

# Set up library paths
if [ -n "$LD_LIBRARY_PATH" ]; then
    export LD_LIBRARY_PATH
fi

# Find Qt base directory
echo "Setting up Qt paths..."
QT_DIRS=$(find /nix/store -name "qt-5*" -type d 2>/dev/null)
if [ -z "$QT_DIRS" ]; then
    QT_DIRS=$(find /nix/store -path "*-qt-5*" -type d 2>/dev/null)
fi

# Set QML2_IMPORT_PATH with a comprehensive search
echo "Finding QML modules..."
QML_PATHS=""
for dir in $(find /nix/store -path "*/qt5/*" -name "qml" -type d 2>/dev/null); do
    QML_PATHS="$QML_PATHS:$dir"
done

for dir in $(find /nix/store -name "qtquickcontrols*" -type d 2>/dev/null | grep -v "doc"); do
    for qml_dir in $(find "$dir" -name "qml" -type d 2>/dev/null); do
        QML_PATHS="$QML_PATHS:$qml_dir"
    done
done

for dir in $(find /nix/store -name "qtdeclarative*" -type d 2>/dev/null | grep -v "doc"); do
    for qml_dir in $(find "$dir" -name "qml" -type d 2>/dev/null); do
        QML_PATHS="$QML_PATHS:$qml_dir"
    done
done

# Remove leading colon and set QML2_IMPORT_PATH
QML_PATHS=${QML_PATHS#:}
export QML2_IMPORT_PATH="$QML_PATHS"
echo "QML2_IMPORT_PATH set to: $QML2_IMPORT_PATH"

# Set QT_PLUGIN_PATH with a comprehensive search
echo "Finding Qt plugins..."
PLUGIN_PATHS=""
for dir in $(find /nix/store -path "*/qt5/*" -name "plugins" -type d 2>/dev/null); do
    PLUGIN_PATHS="$PLUGIN_PATHS:$dir"
done

for dir in $(find /nix/store -name "qtbase*" -type d 2>/dev/null | grep -v "doc"); do
    for plugin_dir in $(find "$dir" -name "plugins" -type d 2>/dev/null); do
        PLUGIN_PATHS="$PLUGIN_PATHS:$plugin_dir"
    done
done

# Remove leading colon and set QT_PLUGIN_PATH
PLUGIN_PATHS=${PLUGIN_PATHS#:}
export QT_PLUGIN_PATH="$PLUGIN_PATHS"
echo "QT_PLUGIN_PATH set to: $QT_PLUGIN_PATH"

# Set QT_QPA_PLATFORM_PLUGIN_PATH to help find platform plugins
for dir in $(echo "$QT_PLUGIN_PATH" | tr ':' '\n'); do
    PLATFORM_DIR=$(find "$dir" -name "platforms" -type d 2>/dev/null | head -n 1)
    if [ -n "$PLATFORM_DIR" ]; then
        export QT_QPA_PLATFORM_PLUGIN_PATH="$PLATFORM_DIR"
        echo "QT_QPA_PLATFORM_PLUGIN_PATH set to: $QT_QPA_PLATFORM_PLUGIN_PATH"
        break
    fi
done

# Function to clean up on script exit
cleanup() {
  echo "Cleaning up..."
  if [ -n "$APP_PID" ]; then
    kill $APP_PID 2>/dev/null || true
  fi
  exit 0
}

# Set up trap to clean up on script exit
trap cleanup EXIT

# Build the application first to catch any compilation errors
echo "Building application..."
if ! cargo build; then
  echo "Build failed. Please fix the errors above and try again."
  exit 1
fi

# Run the application
echo "Starting application..."
cargo run &
APP_PID=$!

# Wait for the application to exit
wait $APP_PID