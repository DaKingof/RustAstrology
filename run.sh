#!/bin/bash

# Set environment variables for Qt
if [ -z "$QTDIR" ]; then
    if command -v qmake &> /dev/null; then
        QTDIR=$(qmake -query QT_INSTALL_PREFIX)
        export QTDIR
    else
        echo "Error: qmake not found. Please make sure Qt is installed and in your PATH."
        exit 1
    fi
fi

# Set up library paths
export LD_LIBRARY_PATH="$QTDIR/lib:${LD_LIBRARY_PATH:-}"

# Run the application
cargo run --release "$@"
