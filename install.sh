#!/usr/bin/env bash
set -euo pipefail

REPO="izyuumi/xcode-discord-rpc"
BINARY="xcode-discord-rpc"
INSTALL_DIR="${HOME}/.local/bin"

echo "Installing ${BINARY}..."

# Get latest release tag
TAG=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$TAG" ]; then
  echo "Error: Could not determine latest release." >&2
  exit 1
fi

echo "Latest release: ${TAG}"

# Determine architecture
ARCH=$(uname -m)
case "$ARCH" in
  arm64|aarch64) ASSET="${BINARY}-aarch64-apple-darwin" ;;
  x86_64)        ASSET="${BINARY}-x86_64-apple-darwin" ;;
  *)
    echo "Error: Unsupported architecture: ${ARCH}" >&2
    exit 1
    ;;
esac

URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"
echo "Downloading ${URL}..."

mkdir -p "${INSTALL_DIR}"
curl -sL "${URL}" -o "${INSTALL_DIR}/${BINARY}"
chmod +x "${INSTALL_DIR}/${BINARY}"

echo "Installed to ${INSTALL_DIR}/${BINARY}"
echo ""
echo "Make sure ${INSTALL_DIR} is in your PATH:"
echo "  export PATH=\"\${HOME}/.local/bin:\${PATH}\""
