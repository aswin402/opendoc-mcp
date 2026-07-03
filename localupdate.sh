#!/bin/bash
set -e

echo "=== Updating opendoc-mcp global installation ==="

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed. Please install Rust first."
    exit 1
fi

# Rebuild release binary
echo "Rebuilding release binary..."
cargo +stable build --release --all-features

# Copy binary to global location
INSTALL_DIR="$HOME/.local/bin"
if [ ! -d "$INSTALL_DIR" ]; then
    echo "Error: Installation directory $INSTALL_DIR does not exist. Please run localinstall.sh first."
    exit 1
fi

echo "Copying updated binary to $INSTALL_DIR/opendoc-mcp..."
cp target/release/opendoc-mcp "$INSTALL_DIR/opendoc-mcp"

echo ""
echo "=== Update complete! ==="
