# Catppuccin Mocha theme for Zsh
# Palette: https://github.com/catppuccin/catppuccin
# Variant: Mocha (dark)
# Managed by Chezmoi. Do not edit directly.
# ============================================================================
# This file is sourced by ~/.config/theme/current.zsh
# It exports color variables for use by scripts and tools.
# The prompt is handled by Powerlevel10k — these colors affect LS_COLORS,
# FZF, and any custom scripts that read THEME_* variables.

# --- Theme metadata ---
export THEME_NAME="catppuccin-mocha"
export THEME_STYLE="dark"

# --- Catppuccin Mocha palette ---
export THEME_ROSEWATER="#f5e0dc"
export THEME_FLAMINGO="#f2cdcd"
export THEME_PINK="#f5c2e7"
export THEME_MAUVE="#cba6f7"
export THEME_RED="#f38ba8"
export THEME_MAROON="#eba0ac"
export THEME_PEACH="#fab387"
export THEME_YELLOW="#f9e2af"
export THEME_GREEN="#a6e3a1"
export THEME_TEAL="#94e2d5"
export THEME_SKY="#89dceb"
export THEME_SAPPHIRE="#74c7ec"
export THEME_BLUE="#89b4fa"
export THEME_LAVENDER="#b4befe"
export THEME_TEXT="#cdd6f4"
export THEME_SUBTEXT1="#bac2de"
export THEME_SUBTEXT0="#a6adc8"
export THEME_OVERLAY2="#9399b2"
export THEME_OVERLAY1="#7f849c"
export THEME_OVERLAY0="#6c7086"
export THEME_SURFACE2="#585b70"
export THEME_SURFACE1="#45475a"
export THEME_SURFACE0="#313244"
export THEME_BASE="#1e1e2e"
export THEME_MANTLE="#181825"
export THEME_CRUST="#11111b"

# --- LS_COLORS ---
export LS_COLORS="di=1;34:ln=36:so=35:pi=33:ex=1;32:bd=1;33;40:cd=1;33;40:su=37;41:sg=30;43:tw=30;42:ow=34;42"

# --- FZF ---
export FZF_DEFAULT_OPTS=" \
  --color=bg+:#313244,bg:#1e1e2e,spinner:#f5e0dc,hl:#f38ba8 \
  --color=fg:#cdd6f4,header:#f38ba8,info:#cba6f7,pointer:#f5e0dc \
  --color=marker:#f5e0dc,fg+:#cdd6f4,prompt:#cba6f7,hl+:#f38ba8"
