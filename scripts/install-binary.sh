#!/usr/bin/env bash
set -euo pipefail

# Script to download and install pre-built totui-mcp binary from GitHub releases
# This runs after the plugin is installed from the marketplace

REPO="grimurjonsson/to-tui"
BINARY_NAME="totui-mcp"

echo "Installing totui-mcp binary..."
echo ""

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Map OS names and set binary extension
BINARY_EXT=""
case "$OS" in
    darwin)
        PLATFORM="apple-darwin"
        ;;
    linux)
        PLATFORM="unknown-linux-gnu"
        ;;
    mingw*|msys*|cygwin*)
        PLATFORM="pc-windows-gnu"
        BINARY_EXT=".exe"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        echo "   Supported: macOS (darwin), Linux, Windows"
        exit 1
        ;;
esac

TARGET="${ARCH}-${PLATFORM}"
echo "Detected platform: $TARGET"
echo ""

# Get the latest release tag
echo "Fetching latest release..."
API_RESPONSE=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest")
LATEST_TAG=$(echo "$API_RESPONSE" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' || true)

if [ -z "$LATEST_TAG" ]; then
    echo "❌ Could not fetch latest release"
    echo ""

    # Check if it's a 404 (no releases yet)
    if echo "$API_RESPONSE" | grep -q '"message": "Not Found"'; then
        echo "   No releases found for this repository."
        echo "   The maintainer needs to create a release first."
        echo ""
        echo "   Build from source instead:"
        SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
        PLUGIN_ROOT="$(dirname "$SCRIPT_DIR")"
        echo "   cd $PLUGIN_ROOT && cargo build --release --bin totui-mcp"
    else
        # Show the API response for debugging
        echo "   API Response:"
        echo "$API_RESPONSE" | head -5
        echo ""
        echo "   Please check https://github.com/${REPO}/releases"
    fi
    exit 1
fi

echo "Latest version: $LATEST_TAG"
echo ""

# Construct download URL
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${BINARY_NAME}-${TARGET}${BINARY_EXT}"

echo "Downloading from: $DOWNLOAD_URL"

# Determine installation directory (where this script is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_ROOT="$(dirname "$SCRIPT_DIR")"
INSTALL_DIR="$PLUGIN_ROOT/target/release"

mkdir -p "$INSTALL_DIR"

# Download binary
if ! curl -L -f -o "$INSTALL_DIR/$BINARY_NAME${BINARY_EXT}" "$DOWNLOAD_URL"; then
    echo ""
    echo "❌ Download failed"
    echo "   URL: $DOWNLOAD_URL"
    echo "   This might mean:"
    echo "   1. No binary exists for your platform ($TARGET)"
    echo "   2. The release doesn't include pre-built binaries yet"
    echo "   3. Network connection issue"
    echo ""
    echo "   You can build from source instead:"
    echo "   cd $PLUGIN_ROOT && cargo build --release --bin totui-mcp"
    exit 1
fi

# Make it executable (not needed on Windows but doesn't hurt)
chmod +x "$INSTALL_DIR/$BINARY_NAME${BINARY_EXT}" 2>/dev/null || true

echo ""
echo "✓ Binary installed successfully"
echo "  Location: $INSTALL_DIR/$BINARY_NAME${BINARY_EXT}"
echo ""
echo "Restart Claude Code to activate the MCP server."
