# dotfiles

`dotfiles` is a small interactive Rust CLI for applying a consistent terminal setup on macOS and Linux — without touching your package manager, requiring sudo, or replacing your whole shell config.

It is built for people who already have their tools installed and want a safer, guided way to apply a shared theme and generated config fragments.

## What the repo supports today

Implemented in the current codebase:

- **Platforms:** macOS and Linux
- **Shells:** Bash and Zsh
- **Terminal emulators:** Kitty and Alacritty
- **Built-in themes:** Catppuccin Macchiato and Kanagawa Dragon
- **Configured release targets:**
  - `aarch64-apple-darwin`
  - `x86_64-apple-darwin`
  - `x86_64-unknown-linux-musl`

What is **not** implemented yet, even if it appears in planning docs:

- Zellij
- Tmux
- NeoVim
- OpenCode
- Windows support

## What it does

When you run `dotfiles`, it:

1. detects supported shells and terminal emulators already present on your machine
2. scans standard font directories for installed Nerd Fonts
3. seeds the default themes into `~/.config/dotfiles/themes/defaults/`
4. walks you through shell, terminal, font, font-size, and theme selection
5. renders managed files into `~/.config/dotfiles/rendered/`
6. appends a guarded `source` line to `~/.zshrc` and/or `~/.bashrc`
7. symlinks generated terminal config into Kitty and/or Alacritty's standard config path

If Kitty or Alacritty already have regular config files, `dotfiles` backs them up to timestamped `*.bak` files before replacing them with symlinks.

## Requirements

Before running the tool, you need:

- macOS or Linux
- at least one supported shell and/or terminal emulator already installed
- **at least one Nerd Font already installed**

> **Current limitation:** the implementation still expects at least one detectable Nerd Font. If none are installed, the font prompt is not skipped gracefully yet.

## Installation

### Run locally from the repository

If you're working from source:

```bash
cargo run
```

### Use the bootstrap installer

The repo includes `install.sh`, which:

- detects OS and architecture
- downloads a release tarball from GitHub Releases
- installs `dotfiles` into `~/.local/bin`
- removes macOS quarantine attributes when needed
- launches the binary after installation

```bash
curl -fsSL https://raw.githubusercontent.com/ruy-ggarcia/dotfiles/master/install.sh | bash
```

At the moment, the release workflow exists in the repo, but there is **not currently a published GitHub Release** to install from. If the installer cannot find a release asset, use the local source-based flow instead.

## Usage

Show help:

```bash
dotfiles --help
```

Run the interactive setup:

```bash
dotfiles
```

Typical prompt flow:

- select shells to configure
- select terminal emulators to configure
- choose a Nerd Font
- choose a font size
- choose a theme
- review the generated summary

## Files managed by `dotfiles`

Generated output lives under:

- `~/.config/dotfiles/rendered/prompt.zsh`
- `~/.config/dotfiles/rendered/prompt.bash`
- `~/.config/dotfiles/rendered/kitty.conf`
- `~/.config/dotfiles/rendered/alacritty.toml`

Integration points:

- `~/.zshrc` — appends a guarded source line; does not replace the file
- `~/.bashrc` — appends a guarded source line; does not replace the file
- `~/.config/kitty/kitty.conf` — symlink to generated config
- `~/.config/alacritty/alacritty.toml` — symlink to generated config

## Themes

Default themes are embedded in the project and seeded into:

- `~/.config/dotfiles/themes/defaults/`

You can add your own TOML palette files in:

- `~/.config/dotfiles/themes/custom/`

Custom themes override default themes when names collide.

## CI and release automation

GitHub Actions currently runs:

- `cargo test --locked` on `ubuntu-latest` and `macos-latest`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo build --release --locked` as a release smoke check

On `v*` tags, the release workflow is configured to:

- create a GitHub Release
- run tests
- build release tarballs for the three configured targets
- upload `dotfiles-<target>.tar.gz` assets

## More docs

- Product intent: `docs/PRD.md`
- Milestones and delivery status: `docs/ROADMAP.md`
- Architecture decisions: `docs/decisions.md`
- ADRs: `docs/adrs/`

## Contributing

Want to help? Start with [CONTRIB.md](./CONTRIB.md).
