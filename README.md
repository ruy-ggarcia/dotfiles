# dotfiles

A single-command dev environment setup for macOS and Linux. Interactive TUI, global theming across all tools, idempotent.

## Quick Start

```sh
curl -fsSL https://raw.githubusercontent.com/ruygarcia/dotfiles/main/install.sh | bash
```

No Rust toolchain required. The script clones this repo to `~/.dotfiles`, downloads the right pre-compiled binary for your platform, and launches the setup wizard.

## What it does

- Installs and configures your selected tools via the native package manager (Homebrew or APT)
- Applies a global color theme across all configured tools simultaneously
- Renders Tera templates with the chosen palette and creates symlinks into `~/` and `~/.config/`
- Backs up any existing config files before overwriting (`.bak.{timestamp}`)
- Safe to re-run — idempotent by design

## Supported tools

- Zsh
- Kitty
- Alacritty
- Zellij
- tmux
- NeoVim
- OpenCode

## Themes

| Family | Variants |
|--------|----------|
| Catppuccin | 4 (Latte, Frappé, Macchiato, Mocha) |
| Rosé Pine | 3 (Rose Pine, Moon, Dawn) |
| Kanagawa | 3 (Wave, Dragon, Lotus) |

## Requirements

- macOS (Intel or Apple Silicon) or Ubuntu/Linux (x86_64)
- `git`
- `curl`

## Manual usage

If you already have the repo cloned:

```sh
cd installer
cargo build --release
../target/release/dotfiles
```

## License

MIT
