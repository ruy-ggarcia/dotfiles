# Architecture

> Inspired by [matklad's ARCHITECTURE.md convention](https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html).
> This file is the **single source of truth** for the repository structure.
> It is intentionally short and stable — it describes the shape of the project, not implementation details.
> For detailed technical design, see [`docs/design.md`](docs/design.md).
> For product requirements, see [`docs/PRD.md`](docs/PRD.md).

## Philosophy

This project follows **convention over configuration**: the Rust engine in `installer/` auto-discovers modules and themes at runtime by scanning well-defined directory conventions. Adding a new tool or theme requires **zero changes to the installer code** — create the right directory or file and the engine picks it up automatically.

## Repository Layout

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
├── ARCHITECTURE.md                # This file
└── README.md
```

## How It Works

### Modules

Each tool lives in `modules/<tool-name>/`. The engine discovers all subdirectories there automatically. A module contains:

- `packages.toml` — package names per OS (`macos`, `ubuntu`)
- `home/` — files to symlink into `~/`
- `config/` — files to symlink into `~/.config/`
- Files with a `.tera` extension are Tera templates: they get rendered to `.rendered/` using the active theme palette before symlinking

### Themes

Each theme variant is a single TOML file in `themes/palettes/`. The contract for required color keys is defined in `themes/schema.json`. All palettes must conform to that schema. The engine discovers all `.toml` files in that directory automatically.

### Symlinks

The symlink manager handles three cases idempotently:

- Target does not exist → create symlink
- Target is already a correct symlink → do nothing
- Target is a file, directory, or wrong symlink → back up to `{target}.bak.{timestamp}` → create correct symlink

Static files are symlinked directly from the repo. Template files are symlinked from `.rendered/` (which is gitignored and regenerated on every run).
