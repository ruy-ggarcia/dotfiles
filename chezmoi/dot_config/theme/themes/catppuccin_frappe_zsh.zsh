# Catppuccin Frappe theme for Zsh
# Palette: https://github.com/catppuccin/catppuccin
# Variant: Frappe (dark)
# Managed by Chezmoi. Do not edit directly.
# ============================================================================
# This file is sourced by ~/.config/theme/current.zsh
# It exports color variables for use by scripts and tools.
# The prompt is handled by Powerlevel10k — these colors affect LS_COLORS,
# FZF, and any custom scripts that read THEME_* variables.

# --- Theme metadata ---
export THEME_NAME="catppuccin-frappe"
export THEME_STYLE="dark"

# --- Catppuccin Frappe palette ---
export THEME_ROSEWATER="#f2d5cf"
export THEME_FLAMINGO="#eebebe"
export THEME_PINK="#f4b8e4"
export THEME_MAUVE="#ca9ee6"
export THEME_RED="#e78284"
export THEME_MAROON="#ea999c"
export THEME_PEACH="#ef9f76"
export THEME_YELLOW="#e5c890"
export THEME_GREEN="#a6d189"
export THEME_TEAL="#81c8be"
export THEME_SKY="#99d1db"
export THEME_SAPPHIRE="#85c1dc"
export THEME_BLUE="#8caaee"
export THEME_LAVENDER="#babbf1"
export THEME_TEXT="#c6d0f5"
export THEME_SUBTEXT1="#b5bfe2"
export THEME_SUBTEXT0="#a5adce"
export THEME_OVERLAY2="#949cbb"
export THEME_OVERLAY1="#838ba7"
export THEME_OVERLAY0="#737994"
export THEME_SURFACE2="#626880"
export THEME_SURFACE1="#51576d"
export THEME_SURFACE0="#414559"
export THEME_BASE="#303446"
export THEME_MANTLE="#292c3c"
export THEME_CRUST="#232634"

# --- LS_COLORS ---
export LS_COLORS="di=1;34:ln=36:so=35:pi=33:ex=1;32:bd=1;33;40:cd=1;33;40:su=37;41:sg=30;43:tw=30;42:ow=34;42"

# --- FZF ---
export FZF_DEFAULT_OPTS=" \
  --color=bg+:#414559,bg:#303446,spinner:#f2d5cf,hl:#e78284 \
  --color=fg:#c6d0f5,header:#e78284,info:#ca9ee6,pointer:#f2d5cf \
  --color=marker:#f2d5cf,fg+:#c6d0f5,prompt:#ca9ee6,hl+:#e78284"
