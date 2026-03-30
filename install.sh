#!/usr/bin/env bash
set -euo pipefail

REPO="ruy-ggarcia/dotfiles"
BIN_NAME="dotfiles"
INSTALL_DIR="${HOME}/.local/bin"

# ── Detect OS and architecture ────────────────────────────────────────────────

OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
  Darwin)
    case "${ARCH}" in
      arm64|aarch64) TARGET="aarch64-apple-darwin" ;;
      x86_64)        TARGET="x86_64-apple-darwin" ;;
      *) echo "Unsupported architecture: ${ARCH}. Supported: arm64, x86_64." >&2; exit 1 ;;
    esac
    ;;
  Linux)
    case "${ARCH}" in
      x86_64) TARGET="x86_64-unknown-linux-musl" ;;
      *) echo "Unsupported architecture: ${ARCH}. Supported: x86_64." >&2; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: ${OS}. Supported: macOS, Linux." >&2
    exit 1
    ;;
esac

ASSET="${BIN_NAME}-${TARGET}.tar.gz"

# ── Detect download tool ──────────────────────────────────────────────────────

if command -v curl &>/dev/null; then
  DOWNLOAD="curl -fsSL"
elif command -v wget &>/dev/null; then
  DOWNLOAD="wget -qO-"
else
  echo "curl or wget is required to install dotfiles." >&2
  exit 1
fi

# ── Download ──────────────────────────────────────────────────────────────────

LATEST_URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"
TMPDIR="$(mktemp -d)"
trap 'rm -rf "${TMPDIR}"' EXIT

echo "Downloading ${ASSET}..."
${DOWNLOAD} "${LATEST_URL}" > "${TMPDIR}/${ASSET}"

# ── Extract ───────────────────────────────────────────────────────────────────

tar xzf "${TMPDIR}/${ASSET}" -C "${TMPDIR}"

# ── Install ───────────────────────────────────────────────────────────────────

mkdir -p "${INSTALL_DIR}"
mv "${TMPDIR}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
chmod +x "${INSTALL_DIR}/${BIN_NAME}"

# Remove macOS quarantine attribute set on downloaded files
if [ "${OS}" = "Darwin" ]; then
  xattr -d com.apple.quarantine "${INSTALL_DIR}/${BIN_NAME}" 2>/dev/null || true
fi

# ── PATH check ────────────────────────────────────────────────────────────────

echo "dotfiles installed to ${INSTALL_DIR}/${BIN_NAME}"

if ! echo "${PATH}" | tr ':' '\n' | grep -qx "${INSTALL_DIR}"; then
  echo ""
  echo "NOTE: ${INSTALL_DIR} is not in your PATH."
  echo "Add the following line to your shell rc file:"
  echo ""
  echo "  export PATH=\"\${HOME}/.local/bin:\${PATH}\""
  echo ""
fi

echo "Run 'dotfiles' to start the configuration wizard."
