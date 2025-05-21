#!/bin/bash
set -e

# Constants - These should match with the constants in src/main.rs
REPO_OWNER=${REPO_OWNER:-"your-org"}
REPO_NAME=${REPO_NAME:-"mcdp-binaries"}
BINARY_NAME=${BINARY_NAME:-"mcdp-tool"}
REPO="$REPO_OWNER/$REPO_NAME"

# Installation options
VERSION=${VERSION:-"latest"}
INSTALL_DIR=${INSTALL_DIR:-"$HOME/.local/bin"}

echo "Installing $BINARY_NAME version $VERSION to $INSTALL_DIR"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Detect platform
PLATFORM="unknown"
ARCH="unknown"

case "$(uname -s)" in
    Linux*)     PLATFORM="linux";;
    Darwin*)    PLATFORM="macos";;
    *)          echo "Unsupported platform: $(uname -s)"; exit 1;;
esac

case "$(uname -m)" in
    x86_64*)    ARCH="amd64";;
    amd64*)     ARCH="amd64";;
    arm64*)     ARCH="arm64";;
    aarch64*)   ARCH="arm64";;
    *)          echo "Unsupported architecture: $(uname -m)"; exit 1;;
esac

echo "Detected platform: $PLATFORM/$ARCH"

# Get download URL
if [ "$VERSION" = "latest" ]; then
    RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"
    DOWNLOAD_URL=$(curl -s $RELEASE_URL | grep "browser_download_url.*$BINARY_NAME-.*-$PLATFORM-$ARCH\"" | cut -d : -f 2,3 | tr -d \")
    VERSION=$(echo $DOWNLOAD_URL | sed -E "s/.*\/v([^\/]+)\/.*/\1/")
    echo "Latest version is $VERSION"
else
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$VERSION/$BINARY_NAME-$VERSION-$PLATFORM-$ARCH"
fi

# Download binary
echo "Downloading from $DOWNLOAD_URL"
curl -L -o "$INSTALL_DIR/$BINARY_NAME" "$DOWNLOAD_URL"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Verify installation
if command -v "$INSTALL_DIR/$BINARY_NAME" &> /dev/null; then
    echo "$BINARY_NAME $VERSION installed successfully to $INSTALL_DIR"
    $INSTALL_DIR/$BINARY_NAME --version
else
    echo "Installation successful, but $BINARY_NAME is not in PATH"
    echo "You may need to add $INSTALL_DIR to your PATH"
    echo "For example, add this to your shell configuration:"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi 