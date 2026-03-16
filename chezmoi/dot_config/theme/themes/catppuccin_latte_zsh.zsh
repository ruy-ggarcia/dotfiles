# Catppuccin Latte theme for Zsh
# Palette: https://github.com/catppuccin/catppuccin
# Variant: Latte (light)
# Managed by Chezmoi. Do not edit directly.
# ============================================================================
# This file is sourced by ~/.config/theme/current.zsh
# It exports color variables for use by scripts and tools.
# The prompt is handled by Powerlevel10k — these colors affect LS_COLORS,
# FZF, and any custom scripts that read THEME_* variables.

# --- Theme metadata ---
export THEME_NAME="catppuccin-latte"
export THEME_STYLE="light"

# --- Catppuccin Latte palette ---
export THEME_ROSEWATER="#dc8a78"
export THEME_FLAMINGO="#dd7878"
export THEME_PINK="#ea76cb"
export THEME_MAUVE="#8839ef"
export THEME_RED="#d20f39"
export THEME_MAROON="#e64553"
export THEME_PEACH="#fe640b"
export THEME_YELLOW="#df8e1d"
export THEME_GREEN="#40a02b"
export THEME_TEAL="#179299"
export THEME_SKY="#04a5e5"
export THEME_SAPPHIRE="#209fb5"
export THEME_BLUE="#1e66f5"
export THEME_LAVENDER="#7287fd"
export THEME_TEXT="#4c4f69"
export THEME_SUBTEXT1="#5c5f77"
export THEME_SUBTEXT0="#6c6f85"
export THEME_OVERLAY2="#7c7f93"
export THEME_OVERLAY1="#8c8fa1"
export THEME_OVERLAY0="#9ca0b0"
export THEME_SURFACE2="#acb0be"
export THEME_SURFACE1="#bcc0cc"
export THEME_SURFACE0="#ccd0da"
export THEME_BASE="#eff1f5"
export THEME_MANTLE="#e6e9ef"
export THEME_CRUST="#dce0e8"

# --- LS_COLORS ---
# Light theme: darker colors for visibility on light background
export LS_COLORS="di=1;34:ln=35:so=32:pi=33:ex=1;31:bd=1;33;40:cd=1;33;40:su=37;41:sg=30;43:tw=30;42:ow=34;42"

# --- FZF ---
export FZF_DEFAULT_OPTS=" \
  --color=bg+:#ccd0da,bg:#eff1f5,spinner:#dc8a78,hl:#d20f39 \
  --color=fg:#4c4f69,header:#d20f39,info:#8839ef,pointer:#dc8a78 \
  --color=marker:#dc8a78,fg+:#4c4f69,prompt:#8839ef,hl+:#d20f39"
