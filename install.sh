#!/bin/bash
# AN (安装) Installer
# https://github.com/clearclown/AN
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash

set -e

REPO="clearclown/AN"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="an"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Check architecture
detect_arch() {
    local arch=$(uname -m)
    case "$arch" in
        x86_64|amd64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac
}

# Check OS
detect_os() {
    local os=$(uname -s)
    case "$os" in
        Linux)
            echo "linux"
            ;;
        *)
            error "Unsupported OS: $os (AN only supports Linux)"
            ;;
    esac
}

# Get latest release version
get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | \
        grep '"tag_name":' | \
        sed -E 's/.*"([^"]+)".*/\1/'
}

# Main installation
main() {
    echo ""
    echo "  ___   _  _ "
    echo " / _ \ | \| |"
    echo "| (_) || .\` |"
    echo " \___/ |_|\_|"
    echo ""
    echo "AN (安装) - Unified Package Manager for Linux"
    echo ""

    # Detect system
    local os=$(detect_os)
    local arch=$(detect_arch)
    info "Detected: ${os}-${arch}"

    # Create install directory
    if [ ! -d "$INSTALL_DIR" ]; then
        info "Creating directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR"
    fi

    # Get latest version
    info "Fetching latest version..."
    local version=$(get_latest_version)

    if [ -z "$version" ]; then
        warn "Could not fetch latest version, building from source..."
        install_from_source
        return
    fi

    info "Latest version: $version"

    # Download binary
    local download_url="https://github.com/${REPO}/releases/download/${version}/an-${os}-${arch}"
    info "Downloading from: $download_url"

    if curl -fsSL -o "/tmp/${BINARY_NAME}" "$download_url"; then
        # Install binary
        chmod +x "/tmp/${BINARY_NAME}"
        mv "/tmp/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
        info "Installed to: ${INSTALL_DIR}/${BINARY_NAME}"
    else
        warn "Binary not available, building from source..."
        install_from_source
        return
    fi

    # Verify installation
    if command -v an &> /dev/null; then
        echo ""
        info "Installation complete!"
        echo ""
        an --version
    else
        warn "AN installed but not in PATH"
        echo ""
        echo "Add this to your ~/.bashrc or ~/.zshrc:"
        echo ""
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
    fi

    # Initial setup
    echo ""
    info "Run 'an sync' to download the app database"
    echo ""
}

# Install from source (fallback)
install_from_source() {
    info "Building from source..."

    # Check for cargo
    if ! command -v cargo &> /dev/null; then
        error "Cargo not found. Please install Rust: https://rustup.rs/"
    fi

    # Clone and build
    local tmp_dir=$(mktemp -d)
    cd "$tmp_dir"

    info "Cloning repository..."
    git clone --depth 1 "https://github.com/${REPO}.git" .

    info "Building release..."
    cargo build --release

    # Install
    cp "target/release/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    # Cleanup
    cd -
    rm -rf "$tmp_dir"

    info "Installed to: ${INSTALL_DIR}/${BINARY_NAME}"

    # Verify
    if command -v an &> /dev/null; then
        echo ""
        info "Installation complete!"
        an --version
    else
        warn "AN installed but not in PATH"
        echo "Add ~/.local/bin to your PATH"
    fi
}

main "$@"
