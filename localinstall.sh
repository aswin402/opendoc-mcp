#!/bin/bash
set -e

echo "=== Installing opendoc-mcp globally (local user space) ==="

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed. Please install Rust first (https://rustup.rs/)."
    exit 1
fi

# Build the release binary
echo "Building release binary..."
cargo +stable build --release --all-features

# Create user bin directory if it doesn't exist
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "Installing binary to $INSTALL_DIR/opendoc-mcp..."
cp target/release/opendoc-mcp "$INSTALL_DIR/opendoc-mcp"

# Check if INSTALL_DIR is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "Warning: $INSTALL_DIR is not in your PATH."
    echo "To run it globally, please add it to your PATH by adding the following line to your ~/.bashrc, ~/.zshrc, or profile:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo ""
echo "=== Installation complete! ==="
echo "Binary is installed at: $INSTALL_DIR/opendoc-mcp"
echo ""
echo "You can now use it in your MCP clients (e.g. Claude Desktop, Cursor, or Cline) with:"
echo "Command: $INSTALL_DIR/opendoc-mcp"
