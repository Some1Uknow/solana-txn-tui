#!/bin/bash

set -e

REPO="Some1UKnow/solana-txn-tui"
BINARY_NAME="solana-txn-tui"

# Detect OS and Arch
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        OS_TYPE="linux"
        ;;
    Darwin)
        OS_TYPE="darwin"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        ARCH_TYPE="amd64"
        ;;
    arm64|aarch64)
        if [ "$OS" = "Darwin" ]; then
            ARCH_TYPE="arm64"
        else
            ARCH_TYPE="amd64" # Fallback for now, or add specific linux-arm support later
        fi
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

ASSET_NAME="${BINARY_NAME}-${OS_TYPE}-${ARCH_TYPE}"
INSTALL_DIR="/usr/local/bin"
DEST_FILE="$INSTALL_DIR/$BINARY_NAME"

# Determine latest tag
LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo "Could not fetch latest release tag."
    exit 1
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$ASSET_NAME"

echo "Detected $OS $ARCH"
echo "Downloading $ASSET_NAME from $LATEST_TAG..."

# Check if we have write access to /usr/local/bin, otherwise use sudo
if [ -w "$INSTALL_DIR" ]; then
    curl -fL -o "$DEST_FILE" "$DOWNLOAD_URL"
    chmod +x "$DEST_FILE"
else
    echo "Need sudo access to install to $INSTALL_DIR"
    sudo curl -fL -o "$DEST_FILE" "$DOWNLOAD_URL"
    sudo chmod +x "$DEST_FILE"
fi

echo "Installation complete! You can now run '$BINARY_NAME' from anywhere."
