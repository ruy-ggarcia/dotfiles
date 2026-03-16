#!/usr/bin/env bash
# set-theme.sh - Cross-platform dynamic theme switcher
# Managed by Chezmoi. Do not edit directly.
# ============================================================================
#
# Switches the active color theme across Zsh, Kitty, and Zellij.
# Themes are stored in ~/.config/theme/themes/ and referenced by pointer files.
#
# Usage:
#   set-theme <theme-name>       Switch to a theme
#   set-theme --list             List available themes
#   set-theme --help             Show this help
#
# Available themes:
#   catppuccin-latte, catppuccin-frappe, catppuccin-macchiato,
#   catppuccin-mocha, pine-rose
# ============================================================================

set -euo pipefail

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------
THEME_DIR="$HOME/.config/theme"
THEMES_DIR="$THEME_DIR/themes"
ZELLIJ_CONF="$HOME/.config/zellij/config.kdl"

# Map of theme names to file prefixes (theme-name -> file_prefix)
declare -A THEME_MAP=(
  [catppuccin-latte]="catppuccin_latte"
  [catppuccin-frappe]="catppuccin_frappe"
  [catppuccin-macchiato]="catppuccin_macchiato"
  [catppuccin-mocha]="catppuccin_mocha"
  [pine-rose]="pine_rose"
)

# ---------------------------------------------------------------------------
# Colors for output
# ---------------------------------------------------------------------------
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# ---------------------------------------------------------------------------
# Functions
# ---------------------------------------------------------------------------

info()    { echo -e "${BLUE}[info]${NC} $1"; }
ok()      { echo -e "${GREEN}[ok]${NC}   $1"; }
warn()    { echo -e "${YELLOW}[warn]${NC} $1"; }
err()     { echo -e "${RED}[err]${NC}  $1" >&2; }

usage() {
  cat <<EOF
${BOLD}Usage:${NC} $(basename "$0") <theme-name>
       $(basename "$0") --list
       $(basename "$0") --help

Switch the active color theme across Zsh, Kitty, and Zellij.

${BOLD}Options:${NC}
  --list, -l    List available themes
  --help, -h    Show this help message

${BOLD}Available themes:${NC}
$(list_themes)
EOF
}

list_themes() {
  for theme in "${!THEME_MAP[@]}"; do
    echo "  - $theme"
  done | sort
}

die() {
  err "$1"
  exit 1
}

# Cross-platform sed -i (macOS requires '' after -i, Linux does not)
sed_inplace() {
  if [[ "$(uname -s)" == "Darwin" ]]; then
    sed -i '' "$@"
  else
    sed -i "$@"
  fi
}

update_zsh_theme() {
  local prefix="$1"
  local theme_name="$2"
  local target="$THEMES_DIR/${prefix}_zsh.zsh"

  [[ -f "$target" ]] || die "Zsh theme file not found: $target"

  cat > "$THEME_DIR/current.zsh" <<EOF
# Dynamic Theme Pointer - Zsh
# Managed by the set-theme script. Do not edit directly.
# ============================================================================
# This file sources the active Zsh theme. The set-theme script rewrites
# this file to point at the selected theme under ~/.config/theme/themes/.
# Current: $theme_name

source "\$HOME/.config/theme/themes/${prefix}_zsh.zsh"
EOF

  ok "Zsh theme -> ${prefix}_zsh.zsh"
}

update_kitty_theme() {
  local prefix="$1"
  local theme_name="$2"
  local target="$THEMES_DIR/${prefix}_kitty.conf"

  [[ -f "$target" ]] || die "Kitty theme file not found: $target"

  cat > "$THEME_DIR/current_kitty.conf" <<EOF
# Dynamic Theme Pointer - Kitty
# Managed by the set-theme script. Do not edit directly.
# ============================================================================
# This file includes the active Kitty theme. The set-theme script rewrites
# this file to point at the selected theme under ~/.config/theme/themes/.
# Current: $theme_name

include ~/.config/theme/themes/${prefix}_kitty.conf
EOF

  ok "Kitty theme -> ${prefix}_kitty.conf"

  # Signal Kitty to reload config (SIGUSR1) if it's running
  if pgrep -x kitty &>/dev/null; then
    pkill -USR1 -x kitty
    ok "Sent SIGUSR1 to Kitty (config reloaded)"
  else
    warn "Kitty not running, skipping reload signal"
  fi
}

update_zellij_theme() {
  local theme_name="$1"

  if [[ -f "$ZELLIJ_CONF" ]]; then
    # Patch the theme "..." line in config.kdl
    if grep -q '^theme "' "$ZELLIJ_CONF"; then
      sed_inplace "s/^theme \".*\"/theme \"$theme_name\"/" "$ZELLIJ_CONF"
      ok "Zellij config.kdl -> theme \"$theme_name\""
    else
      warn "No 'theme \"...\"' line found in $ZELLIJ_CONF"
      warn "Add 'theme \"$theme_name\"' to your Zellij config manually."
    fi
  else
    warn "Zellij config not found at $ZELLIJ_CONF, skipping"
  fi
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

# Handle flags
case "${1:-}" in
  --list|-l)
    echo -e "${BOLD}Available themes:${NC}"
    list_themes
    exit 0
    ;;
  --help|-h|"")
    usage
    exit 0
    ;;
  -*)
    die "Unknown option: $1. Use --help for usage."
    ;;
esac

theme_name="$1"

# Validate theme
if [[ -z "${THEME_MAP[$theme_name]+x}" ]]; then
  err "Unknown theme '$theme_name'"
  echo "" >&2
  echo "Available themes:" >&2
  list_themes >&2
  exit 1
fi

file_prefix="${THEME_MAP[$theme_name]}"

echo ""
echo -e "${BOLD}Switching theme to: ${BLUE}$theme_name${NC}"
echo "─────────────────────────────────────"

update_zsh_theme "$file_prefix" "$theme_name"
update_kitty_theme "$file_prefix" "$theme_name"
update_zellij_theme "$theme_name"

echo "─────────────────────────────────────"
echo -e "${GREEN}${BOLD}Theme switched to '$theme_name' successfully.${NC}"
echo ""
echo "Note: Restart your shell or run 'source ~/.zshrc' to apply Zsh changes."
echo "      Zellij requires a restart to pick up theme changes."
