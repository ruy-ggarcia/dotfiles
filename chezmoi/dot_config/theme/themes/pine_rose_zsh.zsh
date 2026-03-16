# Pine Rose (Rose Pine) theme for Zsh
# Palette: https://rosepinetheme.com/palette
# Variant: Main (dark)
# Managed by Chezmoi. Do not edit directly.
# ============================================================================
# This file is sourced by ~/.config/theme/current.zsh
# It exports color variables for use by scripts and tools.
# The prompt is handled by Powerlevel10k — these colors affect LS_COLORS,
# FZF, and any custom scripts that read THEME_* variables.

# --- Theme metadata ---
export THEME_NAME="pine-rose"
export THEME_STYLE="dark"

# --- Rose Pine palette ---
export THEME_BASE="#191724"
export THEME_SURFACE="#1f1d2e"
export THEME_OVERLAY="#26233a"
export THEME_MUTED="#6e6a86"
export THEME_SUBTLE="#908caa"
export THEME_TEXT="#e0def4"
export THEME_LOVE="#eb6f92"
export THEME_GOLD="#f6c177"
export THEME_ROSE="#ebbcba"
export THEME_PINE="#31748f"
export THEME_FOAM="#9ccfd8"
export THEME_IRIS="#c4a7e7"
export THEME_HIGHLIGHT_LOW="#21202e"
export THEME_HIGHLIGHT_MED="#403d52"
export THEME_HIGHLIGHT_HIGH="#524f67"

# --- LS_COLORS ---
export LS_COLORS="di=1;34:ln=36:so=35:pi=33:ex=1;32:bd=1;33;40:cd=1;33;40:su=37;41:sg=30;43:tw=30;42:ow=34;42"

# --- FZF ---
export FZF_DEFAULT_OPTS=" \
  --color=bg+:#26233a,bg:#191724,spinner:#ebbcba,hl:#eb6f92 \
  --color=fg:#e0def4,header:#eb6f92,info:#c4a7e7,pointer:#ebbcba \
  --color=marker:#ebbcba,fg+:#e0def4,prompt:#c4a7e7,hl+:#eb6f92"
