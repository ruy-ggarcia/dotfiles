# =============================================================================
# Dotfiles — Zsh Configuration
# Managed by the dotfiles installer. Symlinked to ~/.zshrc
# =============================================================================

# =============================================================================
# Oh-My-Zsh Bootstrap
# =============================================================================

export ZSH="$HOME/.oh-my-zsh"

# Auto-install Oh-My-Zsh if not present
if [ ! -d "$ZSH" ]; then
  echo "Oh-My-Zsh not found. Installing..."
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
fi

# Theme — robbyrussell is simple, fast, and universally reliable
ZSH_THEME="robbyrussell"

# Plugins
# External plugins (zsh-autosuggestions, zsh-syntax-highlighting) must be
# cloned into $ZSH_CUSTOM/plugins/ before they take effect:
#   git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
#   git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
plugins=(
  git
  docker
  kubectl
  z
  fzf
  zsh-autosuggestions
  zsh-syntax-highlighting
)

# Source Oh-My-Zsh (only if the install directory exists)
[ -f "$ZSH/oh-my-zsh.sh" ] && source "$ZSH/oh-my-zsh.sh"

# =============================================================================
# PATH Configuration
# =============================================================================

# Homebrew — Apple Silicon
if [ -d "/opt/homebrew/bin" ]; then
  export PATH="/opt/homebrew/bin:/opt/homebrew/sbin:$PATH"
fi

# Homebrew — Intel Mac
if [ -d "/usr/local/bin" ]; then
  export PATH="/usr/local/bin:/usr/local/sbin:$PATH"
fi

# User local binaries
export PATH="$HOME/.local/bin:$PATH"

# Rust / Cargo
export PATH="$HOME/.cargo/bin:$PATH"

# Go
export PATH="$HOME/go/bin:$PATH"

# =============================================================================
# Environment Variables
# =============================================================================

export EDITOR="nvim"
export VISUAL="nvim"
export LANG="en_US.UTF-8"
export LC_ALL="en_US.UTF-8"

# =============================================================================
# History Configuration
# =============================================================================

# Size — large enough to never worry about it
HISTSIZE=50000
SAVEHIST=50000
HISTFILE="$HOME/.zsh_history"

# Deduplicate and share history across all open sessions
setopt HIST_IGNORE_ALL_DUPS   # Remove duplicate entries
setopt HIST_IGNORE_SPACE      # Skip commands prefixed with a space
setopt HIST_VERIFY             # Show expanded history before executing
setopt SHARE_HISTORY           # Share history between all sessions in real-time
setopt INC_APPEND_HISTORY      # Write to history file immediately, not on exit

# =============================================================================
# Completion System
# =============================================================================

# Initialize the completion system
autoload -Uz compinit
compinit

# Case-insensitive completion
zstyle ':completion:*' matcher-list 'm:{a-z}={A-Za-z}'

# =============================================================================
# Aliases — Navigation
# =============================================================================

alias ll='ls -lhF'
alias la='ls -lahF'
alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'

# =============================================================================
# Aliases — Git
# =============================================================================

alias gs='git status'
alias gp='git push'
alias gl='git pull'
alias gd='git diff'
alias gc='git commit'
alias gco='git checkout'
alias gb='git branch'
alias glog='git log --oneline --graph --decorate'

# =============================================================================
# Aliases — Docker
# =============================================================================

alias dk='docker'
alias dkc='docker compose'
alias dkps='docker ps'
alias dki='docker images'

# =============================================================================
# Aliases — Editor
# =============================================================================

# Redirect vim to neovim
alias vim='nvim'
alias vi='nvim'
