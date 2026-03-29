# Dotfiles — Agent Instructions

## Project Overview

Dotfiles is a cross-platform (MacOS + Linux) developer environment configuration system. It consists of a Rust-based TUI installer that auto-discovers tool modules and theme palettes, renders templated configurations via Tera, and manages symlinks idempotently.

## Tech Stack

- **Language:** Rust
- **TUI:** `inquire`
- **Template Engine:** `tera`
- **Supported Platforms:** MacOS (Homebrew), Linux/Ubuntu (APT)

## Repository Structure

> **Source of truth:** [`ARCHITECTURE.md`](ARCHITECTURE.md) at the repo root is the canonical reference for the directory layout. The tree below is an inline copy kept here so agents have it directly in context.

```
.
├── .github/
│   └── workflows/
│       ├── build.yml              # CI: Cross-compiles the Rust installer
│       └── release.yml            # CD: Attaches binaries to GitHub Releases
├── docs/
│   ├── adr/                       # Architecture Decision Records
│   │   ├── ADR-001-programming-language.md
│   │   └── ADR-002-theme-palette-standard.md
│   ├── PRD.md                     # Product Requirements Document
│   ├── decisions.md               # Architecture decisions log
│   └── design.md                  # Technical Design Document
├── installer/                     # Rust Cargo project — the engine
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs                # CLI entrypoint
│       ├── tui.rs                 # inquire-based wizard logic
│       ├── scanner.rs             # Auto-discovers modules/ and themes/
│       ├── package.rs             # PackageManager trait (apt, brew)
│       ├── template.rs            # Tera rendering engine
│       └── symlink.rs             # Symlink manager with .bak backups
├── modules/                       # Tool configurations (one dir per tool)
│   ├── zsh/
│   │   ├── packages.toml          # Package names per distro
│   │   └── home/                  # Files symlinked to ~/
│   │       └── .zshrc
│   ├── kitty/
│   │   ├── packages.toml
│   │   └── config/                # Files symlinked to ~/.config/
│   │       └── kitty/
│   │           └── kitty.conf.tera
│   ├── alacritty/
│   │   ├── packages.toml
│   │   └── config/
│   │       └── alacritty/
│   │           └── alacritty.toml.tera
│   ├── zellij/
│   │   ├── packages.toml
│   │   └── config/
│   │       └── zellij/
│   │           └── config.kdl.tera
│   ├── tmux/
│   │   ├── packages.toml
│   │   └── home/
│   │       └── .tmux.conf.tera
│   ├── nvim/
│   │   ├── packages.toml
│   │   └── config/
│   │       └── nvim/
│   │           └── ...            # Full NeoVim config tree
│   └── opencode/
│       ├── packages.toml
│       └── config/
│           └── opencode/
│               └── ...
├── themes/
│   ├── schema.json                # Contract: required keys for all palettes
│   └── palettes/                  # One TOML file per theme variant
│       ├── catppuccin-latte.toml
│       ├── catppuccin-frappe.toml
│       ├── catppuccin-macchiato.toml
│       ├── catppuccin-mocha.toml
│       ├── rose-pine.toml
│       ├── rose-pine-moon.toml
│       ├── rose-pine-dawn.toml
│       ├── kanagawa-wave.toml
│       ├── kanagawa-dragon.toml
│       └── kanagawa-lotus.toml
├── fonts/
│   └── manifest.toml              # Font names + GitHub Release URLs
├── .rendered/                     # GITIGNORED — compiled Tera output
├── install.sh                     # Bootstrap: curl | bash entrypoint
├── AGENTS.md                      # AI agent instructions
├── ARCHITECTURE.md                # Human-facing architecture overview
└── README.md
```

## Key Conventions

### Modules (`modules/`)

- One directory per tool: `modules/<tool-name>/`
- `packages.toml` — package dependencies per OS:
  ```toml
  [packages]
  macos = ["package-name"]
  ubuntu = ["package-name"]
  ```
- `home/` — files symlinked relative to `~/`
- `config/` — files symlinked relative to `~/.config/`
- `.tera` extension — file is a Tera template, rendered with the active theme palette before symlinking

### Themes (`themes/`)

- `themes/schema.json` — defines the required color keys for all palettes
- `themes/palettes/<name>.toml` — one file per theme variant
- All palettes must conform to `schema.json`

### Symlinks

- Static files → symlinked directly from `modules/`
- Template files → rendered to `.rendered/`, then symlinked from there
- Existing targets are backed up to `{name}.bak.{timestamp}` before overwriting

### Rendered Output (`.rendered/`)

- Gitignored directory containing compiled Tera template output
- Regenerated on every installer run
- Symlinks from `~/.config/` point here for templated files

## Development

### Installer (Rust)

- Located in `installer/`
- Standard Cargo project: `cargo build`, `cargo run`, `cargo test`
- Source modules:
  - `main.rs` — CLI entrypoint
  - `tui.rs` — inquire wizard flow
  - `scanner.rs` — auto-discovers modules and themes
  - `package.rs` — PackageManager trait
  - `template.rs` — Tera rendering
  - `symlink.rs` — symlink management with backups

## Documentation

- `ARCHITECTURE.md` — High-level architecture overview (repo structure, conventions summary)
- `docs/PRD.md` — Product Requirements Document
- `docs/design.md` — Technical Design Document (detailed rationale, workflows)
- `docs/decisions.md` — Architecture Decisions Log
- `docs/adr/` — Architecture Decision Records
