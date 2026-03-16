# dotfiles

Cross-platform development environment managed with [Chezmoi](https://www.chezmoi.io/). A single command takes a fresh macOS or Ubuntu machine to a fully configured terminal environment — same tooling, same keybindings, same themes, everywhere.

> **Status:** Phase 1 (Proof of Concept) — Zsh · Kitty · Zellij · Catppuccin (all flavours) · Pine Rose

---

## What's included

| Category | Tools | Status |
|---|---|---|
| Shell | Zsh + Oh My Zsh + Powerlevel10k | ✅ Phase 1 |
| Terminal emulator | Kitty | ✅ Phase 1 |
| Multiplexer | Zellij | ✅ Phase 1 |
| Themes | Catppuccin (Latte, Frappé, Macchiato, Mocha) · Pine Rose | ✅ Phase 1 |
| Terminal emulators | Alacritty · Ghostty | 🔜 Phase 2 |
| Multiplexer | Tmux | 🔜 Phase 2 |
| Fonts | Meslo NF · IosevkaTerm NF · JetBrainsMono NF · FiraCode NF | 🔜 Phase 2 |
| Editors | NeoVim (LazyVim) · Emacs | 🔜 Phase 3 |
| Dev tools | LazyGit · OpenCode · Neofetch | 🔜 Phase 3 |
| Shells | Bash · Fish | 🔜 Phase 4 |

---

## Requirements

### macOS
- macOS Ventura 13+ (ARM or Intel)
- [Homebrew](https://brew.sh) — install it first if not present:
  ```bash
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ```

### Ubuntu
- Ubuntu 22.04 LTS or 24.04 LTS
- `curl` and `git` (usually pre-installed):
  ```bash
  sudo apt-get update && sudo apt-get install -y curl git
  ```

---

## Setup

### 1. Install Chezmoi and apply the dotfiles

**macOS:**
```bash
brew install chezmoi
chezmoi init --apply <your-repo-url>
```

**Ubuntu:**
```bash
sh -c "$(curl -fsLS get.chezmoi.io)" -- -b ~/.local/bin
chezmoi init --apply <your-repo-url>
```

That's it. Chezmoi clones the repo, processes all templates for your OS, and puts every config file in the right place.

### 2. Install a Nerd Font

Powerlevel10k requires a [Nerd Font](https://www.nerdfonts.com). The prompt is configured for **MesloLGS NF** (the font p10k recommends). Until font installation is automated (Phase 2), install it manually:

**macOS:**
```bash
brew install --cask font-meslo-lg-nerd-font
```

**Ubuntu:**
```bash
mkdir -p ~/.local/share/fonts
curl -fLo ~/.local/share/fonts/MesloLGS-NF-Regular.ttf \
  https://github.com/romkatv/powerlevel10k-media/raw/master/MesloLGS%20NF%20Regular.ttf
fc-cache -fv
```

Then set **MesloLGS NF** as the font in your terminal.

### 3. Install Oh My Zsh + Powerlevel10k

Until the install scripts are automated (Phase 2), run these manually:

```bash
# Oh My Zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Powerlevel10k
git clone --depth=1 https://github.com/romkatv/powerlevel10k.git \
  ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k
```

After installing, re-apply the dotfiles so the managed `.zshrc` and `.p10k.zsh` are put in place:
```bash
chezmoi apply
```

### 4. Apply a theme

The default theme is **Catppuccin Macchiato**. To switch:

```bash
set-theme catppuccin-mocha
set-theme catppuccin-latte
set-theme catppuccin-frappe
set-theme catppuccin-macchiato
set-theme pine-rose
```

To list all available themes:
```bash
set-theme --list
```

`set-theme` is a shell function defined in `.zshrc`. It updates Kitty, Zellij, and Zsh in one shot and reloads what it can live.

---

## Theme system

Each application loads its active theme through an include/source mechanism, never by hardcoding colours in the main config:

| App | Mechanism | Live reload |
|---|---|---|
| Kitty | `include ~/.config/theme/current_kitty.conf` | ✅ SIGUSR1 |
| Zellij | `theme` directive in `config.kdl` (patched by switcher) | ⚠️ New session |
| Zsh | `source ~/.config/theme/current.zsh` in `.zshrc` | ✅ Same session |

The switcher (`scripts/set-theme.sh`) updates the pointer files and triggers reloads. No manual editing required.

---

## Repository structure

```
dotfiles/
├── chezmoi/                        # Chezmoi source directory
│   ├── dot_zshrc.tmpl              # ~/.zshrc  (OS-aware template)
│   ├── dot_p10k.zsh.tmpl           # ~/.p10k.zsh  (Powerlevel10k config)
│   └── dot_config/
│       ├── kitty/
│       │   └── kitty.conf.tmpl     # ~/.config/kitty/kitty.conf
│       ├── zellij/
│       │   ├── config.kdl          # ~/.config/zellij/config.kdl
│       │   └── themes/
│       │       └── pine-rose.kdl   # Custom theme (Catppuccin is built-in)
│       └── theme/
│           ├── current.zsh         # Active Zsh theme pointer
│           ├── current_kitty.conf  # Active Kitty theme pointer
│           └── themes/             # Per-app theme definitions
│               ├── catppuccin_{latte,frappe,macchiato,mocha}_kitty.conf
│               ├── catppuccin_{latte,frappe,macchiato,mocha}_zsh.zsh
│               ├── pine_rose_kitty.conf
│               └── pine_rose_zsh.zsh
├── scripts/
│   └── set-theme.sh                # Theme switcher CLI
└── PRD.md                          # Product requirements and architecture
```

---

## Updating

To pull the latest changes and re-apply:

```bash
chezmoi update
```

This is equivalent to `git pull` + `chezmoi apply`. Chezmoi only touches files that actually changed.

---

## Troubleshooting

**Prompt looks broken / missing icons**
Make sure your terminal is using **MesloLGS NF** (or another Nerd Font). The p10k prompt uses powerline glyphs that only render correctly with a patched font.

**`set-theme` command not found**
The function is defined in `.zshrc`. Start a new shell session or run `source ~/.zshrc`.

**Zellij theme didn't change**
Zellij doesn't support live theme reloading yet. Open a new Zellij session to see the updated theme.

**Chezmoi reports conflicts**
If you have existing config files, Chezmoi will ask what to do. Run `chezmoi diff` first to see what would change, then `chezmoi apply` to proceed.
