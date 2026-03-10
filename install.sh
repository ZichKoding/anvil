#!/bin/sh
set -eu

REPO="ZichKoding/anvil"
BINARY="anvil"

main() {
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)  os_part="unknown-linux-gnu" ;;
        Darwin) os_part="apple-darwin" ;;
        *)      echo "Error: unsupported OS: $os" >&2; exit 1 ;;
    esac

    case "$arch" in
        x86_64|amd64)   arch_part="x86_64" ;;
        aarch64|arm64)   arch_part="aarch64" ;;
        *)               echo "Error: unsupported architecture: $arch" >&2; exit 1 ;;
    esac

    target="${arch_part}-${os_part}"

    echo "Detected platform: ${target}"

    # Fetch latest release tag
    tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | cut -d'"' -f4)"

    if [ -z "$tag" ]; then
        echo "Error: could not determine latest release" >&2
        exit 1
    fi

    echo "Latest release: ${tag}"

    archive="${BINARY}-${tag}-${target}.tar.gz"
    url="https://github.com/${REPO}/releases/download/${tag}/${archive}"

    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    echo "Downloading ${url}..."
    curl -fsSL "$url" -o "${tmpdir}/${archive}"

    echo "Extracting..."
    tar xzf "${tmpdir}/${archive}" -C "$tmpdir"

    # Install binary
    if [ -w "/usr/local/bin" ]; then
        install_dir="/usr/local/bin"
        cp "${tmpdir}/${BINARY}" "${install_dir}/${BINARY}"
        chmod +x "${install_dir}/${BINARY}"
    elif command -v sudo >/dev/null 2>&1; then
        install_dir="/usr/local/bin"
        echo "Installing to ${install_dir} (requires sudo)..."
        sudo cp "${tmpdir}/${BINARY}" "${install_dir}/${BINARY}"
        sudo chmod +x "${install_dir}/${BINARY}"
    else
        install_dir="${HOME}/.local/bin"
        mkdir -p "$install_dir"
        cp "${tmpdir}/${BINARY}" "${install_dir}/${BINARY}"
        chmod +x "${install_dir}/${BINARY}"
        case ":$PATH:" in
            *":${install_dir}:"*) ;;
            *) echo "Note: add ${install_dir} to your PATH" ;;
        esac
    fi

    echo "Installed ${BINARY} to ${install_dir}/${BINARY}"

    # Verify
    if command -v "$BINARY" >/dev/null 2>&1; then
        echo "Success! Run '${BINARY}' to get started."
    else
        echo "Installed, but '${BINARY}' is not on your PATH yet."
        echo "Add ${install_dir} to your PATH, then run '${BINARY}'."
    fi
}

main
