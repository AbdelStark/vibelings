#!/usr/bin/env bash
# vibelings install script
# Usage: curl -sSL https://raw.githubusercontent.com/AbdelStark/vibelings/main/install.sh | bash

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

REPO="AbdelStark/vibelings"
BINARY_NAME="vibelings"
INSTALL_DIR="${VIBELINGS_INSTALL_DIR:-$HOME/.local/bin}"

info() {
    printf "${BLUE}info:${NC} %s\n" "$1"
}

success() {
    printf "${GREEN}success:${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}warning:${NC} %s\n" "$1"
}

error() {
    printf "${RED}error:${NC} %s\n" "$1" >&2
    exit 1
}

detect_platform() {
    local os arch

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            os="linux"
            ;;
        Darwin)
            os="macos"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            error "Please use install.ps1 for Windows"
            ;;
        *)
            error "Unsupported operating system: $os"
            ;;
    esac

    case "$arch" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac

    echo "${BINARY_NAME}-${os}-${arch}"
}

get_latest_version() {
    local version
    version=$(curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [[ -z "$version" ]]; then
        error "Could not determine latest version"
    fi

    echo "$version"
}

download_binary() {
    local platform="$1"
    local version="$2"
    local url="https://github.com/${REPO}/releases/download/${version}/${platform}.tar.gz"
    local temp_dir

    temp_dir="$(mktemp -d)"
    trap 'rm -rf "$temp_dir"' EXIT

    info "Downloading vibelings ${version} for ${platform}..."

    if ! curl -sSL -o "${temp_dir}/vibelings.tar.gz" "$url"; then
        return 1
    fi

    info "Extracting..."
    tar -xzf "${temp_dir}/vibelings.tar.gz" -C "$temp_dir"

    # Find the binary
    local binary_path="${temp_dir}/${platform}"
    if [[ ! -f "$binary_path" ]]; then
        # Try without extension
        binary_path="${temp_dir}/${BINARY_NAME}"
    fi

    if [[ ! -f "$binary_path" ]]; then
        return 1
    fi

    # Install
    mkdir -p "$INSTALL_DIR"
    cp "$binary_path" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    return 0
}

install_from_cargo() {
    info "Installing from source using cargo..."

    if ! command -v cargo &> /dev/null; then
        error "cargo not found. Please install Rust: https://rustup.rs"
    fi

    cargo install --git "https://github.com/${REPO}"
}

check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR is not in your PATH"
        echo ""
        echo "Add this to your shell profile (.bashrc, .zshrc, etc.):"
        echo ""
        echo "    export PATH=\"\$PATH:$INSTALL_DIR\""
        echo ""
    fi
}

main() {
    echo ""
    echo "  vibelings installer"
    echo "  Rustlings for agentic programming"
    echo ""

    local platform version
    platform="$(detect_platform)"

    info "Detected platform: $platform"

    # Try to get latest version and download binary
    if version="$(get_latest_version 2>/dev/null)"; then
        info "Latest version: $version"

        if download_binary "$platform" "$version"; then
            success "vibelings installed to ${INSTALL_DIR}/${BINARY_NAME}"
            check_path

            echo ""
            success "Installation complete!"
            echo ""
            echo "Get started:"
            echo "    vibelings init"
            echo "    vibelings list"
            echo "    vibelings"
            echo ""
            exit 0
        else
            warn "Could not download pre-built binary, falling back to cargo install"
        fi
    else
        warn "Could not fetch latest release, falling back to cargo install"
    fi

    # Fall back to cargo install
    install_from_cargo

    success "vibelings installed via cargo"
    echo ""
    echo "Get started:"
    echo "    vibelings init"
    echo "    vibelings list"
    echo "    vibelings"
    echo ""
}

main "$@"
