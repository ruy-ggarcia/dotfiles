# Catppuccin Macchiato theme for Zsh
# Palette: https://github.com/catppuccin/catppuccin
# Variant: Macchiato (dark)
# Managed by Chezmoi. Do not edit directly.
# ============================================================================
# This file is sourced by ~/.config/theme/current.zsh
# It exports color variables for use by scripts and tools.
# The prompt is handled by Powerlevel10k — these colors affect LS_COLORS,
# FZF, and any custom scripts that read THEME_* variables.

# --- Theme metadata ---
export THEME_NAME="catppuccin-macchiato"
export THEME_STYLE="dark"

# --- Catppuccin Macchiato palette ---
export THEME_ROSEWATER="#f4dbd6"
export THEME_FLAMINGO="#f0c6c6"
export THEME_PINK="#f5bde6"
export THEME_MAUVE="#c6a0f6"
export THEME_RED="#ed8796"
export THEME_MAROON="#ee99a0"
export THEME_PEACH="#f5a97f"
export THEME_YELLOW="#eed49f"
export THEME_GREEN="#a6da95"
export THEME_TEAL="#8bd5ca"
export THEME_SKY="#91d7e3"
export THEME_SAPPHIRE="#7dc4e4"
export THEME_BLUE="#8aadf4"
export THEME_LAVENDER="#b7bdf8"
export THEME_TEXT="#cad3f5"
export THEME_SUBTEXT1="#b8c0e0"
export THEME_SUBTEXT0="#a5adcb"
export THEME_OVERLAY2="#939ab7"
export THEME_OVERLAY1="#8087a2"
export THEME_OVERLAY0="#6e738d"
export THEME_SURFACE2="#5b6078"
export THEME_SURFACE1="#494d64"
export THEME_SURFACE0="#363a4f"
export THEME_BASE="#24273a"
export THEME_MANTLE="#1e2030"
export THEME_CRUST="#181926"

# --- LS_COLORS ---
export LS_COLORS="di=1;34:ln=36:so=35:pi=33:ex=1;32:bd=1;33;40:cd=1;33;40:su=37;41:sg=30;43:tw=30;42:ow=34;42"

# --- FZF ---
export FZF_DEFAULT_OPTS=" \
  --color=bg+:#363a4f,bg:#24273a,spinner:#f4dbd6,hl:#ed8796 \
  --color=fg:#cad3f5,header:#ed8796,info:#c6a0f6,pointer:#f4dbd6 \
  --color=marker:#f4dbd6,fg+:#cad3f5,prompt:#c6a0f6,hl+:#ed8796"
