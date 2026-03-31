# Dotfiles — Roadmap

## MVP

The MVP is reached when all six milestones are complete. Each milestone is independently shippeable — no milestone delivers infrastructure without user-facing behavior.

---

## M1 — Shell Configuration (MacOS)

The first working product. A user on MacOS can run the binary, answer a few questions, and walk away with a consistently themed shell.

**Status:** 🟢 Completed

**Scope:**
- Detect Bash and Zsh installations on the system
- Interactive TUI: tool selection, theme selection, Nerd Font selection
- Apply selected theme to shell configuration via Tera templates
- Detect Nerd Fonts already installed on the system
- Idempotent execution — re-running produces the same result; shell rc files are never replaced (source guard injection); other conflicting config files are backed up before replacement
- MacOS support (x86_64 and aarch64)

---

## M2 — Cross-Platform: Linux

The same experience on Ubuntu Linux. Closes all P0 requirements from the PRD.

**Status:** ⚪ Pending

**Scope:**
- All M1 behavior on Ubuntu Linux (x86_64)
- Nerd Font detection from Linux system font directories

---

## M3 — Distribution

Any user can install dotfiles with a single command, without a Rust toolchain on their machine.

**Status:** ⚪ Pending

**Scope:**
- Pre-compiled binaries published to GitHub Releases for all supported platforms
- Bootstrap script: detects OS and architecture, downloads and runs the correct binary
- Automated release pipeline via GitHub Actions

---

## M4 — Terminal Emulators

The selected theme is reflected in the terminal emulator, not just the shell prompt.

**Status:** ⚪ Pending

**Scope:**
- Kitty configuration and theme integration
- Alacritty configuration and theme integration

---

## M5 — Multiplexers

Theme and configuration consistency extended to terminal multiplexers.

**Status:** ⚪ Pending

**Scope:**
- Zellij configuration and theme integration
- Tmux configuration and theme integration

---

## M6 — Editors

Full-stack terminal theming. Completes all P1 requirements from the PRD and marks the MVP.

**Status:** ⚪ Pending

**Scope:**
- NeoVim configuration and theme integration
- OpenCode configuration and theme integration
