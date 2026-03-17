#!/usr/bin/env bash
# Bootstrap installer — downloads the dotfiles binary for the current platform
# and launches the interactive setup wizard.
#
# Intended usage: curl -fsSL https://raw.githubusercontent.com/ruygarcia/dotfiles/main/install.sh | bash

set -euo pipefail

# ─── Configuration ────────────────────────────────────────────────────────────

REPO="ruygarcia/dotfiles"
DOTFILES_DIR="${HOME}/.dotfiles"
BIN_DIR="${DOTFILES_DIR}/.bin"
BINARY="${BIN_DIR}/dotfiles"

# ─── Cleanup on exit ──────────────────────────────────────────────────────────

TMPDIR_WORK=""
cleanup() {
  if [[ -n "${TMPDIR_WORK}" && -d "${TMPDIR_WORK}" ]]; then
    rm -rf "${TMPDIR_WORK}"
  fi
}
trap cleanup EXIT

# ─── Helpers ──────────────────────────────────────────────────────────────────

info()  { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
ok()    { printf '\033[1;32m  ✓\033[0m %s\n' "$*"; }
err()   { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

# ─── Step 1: Clone or update the repository ───────────────────────────────────

info "Setting up ~/.dotfiles"

if [[ ! -d "${DOTFILES_DIR}" ]]; then
  info "Cloning repository to ${DOTFILES_DIR}"
  git clone "git@github.com:${REPO}.git" "${DOTFILES_DIR}" \
    || err "Failed to clone repository. Check your internet connection and try again."
  ok "Repository cloned"
else
  info "Pulling latest changes in ${DOTFILES_DIR}"
  git -C "${DOTFILES_DIR}" pull \
    || err "Failed to pull latest changes. Resolve any conflicts in ${DOTFILES_DIR} and retry."
  ok "Repository up to date"
fi

# ─── Step 2: Detect platform and select the correct binary ────────────────────

info "Detecting platform"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}-${ARCH}" in
  Darwin-x86_64)  TARGET="x86_64-apple-darwin"       ;;
  Darwin-arm64)   TARGET="aarch64-apple-darwin"       ;;
  Linux-x86_64)   TARGET="x86_64-unknown-linux-musl"  ;;
  *)
    err "Unsupported platform: ${OS} / ${ARCH}.
Supported platforms:
  • macOS  (x86_64, arm64)
  • Linux  (x86_64)

Please open an issue at https://github.com/${REPO}/issues"
    ;;
esac

ok "Target: ${TARGET}"

# ─── Step 3: Download and extract the binary ──────────────────────────────────

TARBALL="dotfiles-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${TARBALL}"

info "Downloading ${TARBALL}"

TMPDIR_WORK="$(mktemp -d)"

curl -fsSL "${DOWNLOAD_URL}" -o "${TMPDIR_WORK}/${TARBALL}" \
  || err "Failed to download ${DOWNLOAD_URL}.
Make sure a release exists for target '${TARGET}' at https://github.com/${REPO}/releases"

info "Extracting binary"

mkdir -p "${BIN_DIR}"
tar -xzf "${TMPDIR_WORK}/${TARBALL}" -C "${TMPDIR_WORK}" \
  || err "Failed to extract ${TARBALL}. The archive may be corrupt."

# The tarball is expected to contain a single 'dotfiles' binary at its root.
if [[ ! -f "${TMPDIR_WORK}/dotfiles" ]]; then
  err "Expected binary 'dotfiles' not found inside ${TARBALL}.
Check the release assets at https://github.com/${REPO}/releases"
fi

mv "${TMPDIR_WORK}/dotfiles" "${BINARY}"
chmod +x "${BINARY}"

ok "Binary installed to ${BINARY}"

# ─── Step 4: Launch the installer wizard ──────────────────────────────────────

info "Launching dotfiles wizard"
echo ""
exec "${BINARY}"
