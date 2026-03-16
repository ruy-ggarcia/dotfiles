# Product Requirements Document: Cross-Platform Dotfiles Repository

**Status:** Draft
**Last updated:** 2026-03-16
**Owner:** Ruy Garcia
**Audience:** Anyone setting up a Unix/macOS development environment

---

## 1. Overview

This document defines the requirements for a cross-platform dotfiles repository that automates provisioning of development environments across macOS and Ubuntu Linux, ensuring consistent tooling, keybindings, themes, and shell behavior on any machine.

## 2. Objectives

1. **Portable environment** -- The user can go from a fresh macOS or Ubuntu install to a fully configured dev environment with a single command, enabling effortless movement between machines with zero friction.
2. **Cross-platform parity** -- Configurations produce equivalent developer experiences on macOS (Homebrew) and Ubuntu (APT), without forcing Homebrew onto Linux.
3. **Centralized theming** -- A single CLI command switches the active color theme across every configured application simultaneously.
4. **Incremental adoption** -- The repository ships in phases. Each phase is self-contained, useful on its own, and does not break previous phases.
5. **User-agnostic design** -- Nothing is hardcoded to a specific username, home directory, or machine. Machine-specific overrides and secrets stay outside version control. The user can set up the repo on any machine and get a working environment.

## 3. Non-Goals

- **Window managers / desktop environments** -- No i3, Sway, Yabai, Aerospace, or similar. Out of scope.
- **Server provisioning** -- This is a developer workstation tool, not an Ansible replacement.
- **Secrets management** -- Chezmoi supports secret backends (1Password, Bitwarden, etc.), but integrating a specific provider is deferred. Templates will be secret-ready but not secret-dependent.
- **NixOS / Nix Home Manager** -- The chosen tooling approach (see §6.1) uses Chezmoi, not Nix. If the dotfiles manager changes in the future, Nix is still out of scope.
- **Windows / WSL** -- Not a target platform.

## 4. Target Platforms

| Attribute | macOS | Ubuntu Linux |
|-----------|-------|--------------|
| Versions | Ventura 13+ (ARM & Intel) | 22.04 LTS, 24.04 LTS |
| Package manager | Homebrew | APT (with binary download fallback for tools absent from APT) |
| Shell default | Zsh (system) | Bash (system), Zsh installed via APT |
| Font path | `~/Library/Fonts/` | `~/.local/share/fonts/` |
| Config root | `~/.config/` | `~/.config/` |

## 5. Tool Inventory

### 5.1 Shells

| Tool | Role | Phase |
|------|------|-------|
| Zsh | Primary shell | 1 |
| Bash | Secondary shell / CI compatibility | 4 |
| Fish | Future expansion (optional) | 4 |

### 5.2 Terminal Emulators

| Tool | Config format | Phase |
|------|---------------|-------|
| Kitty | `kitty.conf` (INI-like) | 1 |
| Alacritty | `alacritty.toml` | 2 |
| Ghostty | `config` (key-value) | 2 |

### 5.3 Terminal Multiplexers

| Tool | Config format | Phase |
|------|---------------|-------|
| Zellij | `config.kdl` (KDL) | 1 |
| Tmux | `tmux.conf` | 2 |

### 5.4 Editors

| Tool | Config approach | Phase |
|------|-----------------|-------|
| NeoVim | LazyVim distribution (Lua) | 3 |
| Emacs | Custom Emacs config (Elisp) | 3 |

### 5.5 Dev Tools

| Tool | Purpose | Phase |
|------|---------|-------|
| OpenCode | AI-assisted coding CLI | 3 |
| LazyGit | Terminal Git UI | 3 |
| Neofetch | System info display | 3 |

### 5.6 Fonts

| Font | Phase |
|------|-------|
| Meslo Nerd Font | 2 |
| IosevkaTerm Nerd Font | 2 |
| JetBrainsMono Nerd Font | 2 |
| FiraCode Nerd Font | 2 |

All fonts are Nerd Font patched variants, installed from GitHub releases or Homebrew cask.

### 5.7 Themes

| Theme | Phase |
|-------|-------|
| Catppuccin (Mocha, Latte, Frappe, Macchiato) | 1 |
| Pine Rose (custom / Rosé Pine variant) | 1 |
| Nakagawa | 2 |
| Additional themes | 3+ |

## 6. Architecture

### 6.1 Dotfiles Manager -- Technology Decision: Chezmoi

The product requirements for the dotfiles manager are:

- **Version control** -- All configuration lives in a Git repository.
- **Cross-platform support** -- Must handle macOS and Ubuntu differences (paths, packages, config formats) from a single source.
- **Automated deployment** -- A single command provisions a fresh machine end-to-end.
- **Native package managers** -- Uses Homebrew on macOS and APT on Ubuntu; no Homebrew on Linux.
- **User-agnostic** -- No hardcoded usernames, home directories, or machine-specific values in committed files.

**Chezmoi** was selected as the implementation tool because it satisfies all of the above:

| Requirement | How Chezmoi addresses it |
|-------------|--------------------------|
| Version control | Source directory is a plain Git repo |
| Cross-platform | Go templates with built-in OS/arch variables (`.chezmoi.os`, `.chezmoi.arch`, `.chezmoi.osRelease.id`) |
| Automated deployment | `chezmoi init --apply <repo>` does everything in one shot |
| Native package managers | `run_once_` scripts with OS-conditional logic |
| User-agnostic | Template data layer separates machine-local values from committed config |

Additional Chezmoi capabilities that influenced the decision: single static binary (easy to bootstrap), `run_once_`/`run_onchange_` script lifecycle, `.chezmoiexternal.toml` for pulling upstream dependencies, and a mature secret-backend integration point for future use.

> **Note:** Chezmoi is an implementation choice, not a product constraint. If a better tool emerges, the requirements above remain unchanged -- only the tooling layer would be swapped.

**How Chezmoi is used in this project:**

- **Templating** -- `.tmpl` files with Go template syntax for OS/machine branching.
- **Scripts** -- `run_once_*` and `run_onchange_*` scripts for package installation and post-apply hooks.
- **Data** -- `~/.config/chezmoi/chezmoi.toml` stores machine-local data (hostname, preferred theme, personal overrides). This file is not committed; a `.chezmoidata.toml` in the repo provides defaults.
- **Externals** -- `chezmoi.toml` `[externals]` or `.chezmoiexternal.toml` for pulling Nerd Fonts, theme repos, or plugin managers from upstream.

### 6.2 OS-Specific Templating Strategy

All config files that differ between macOS and Ubuntu are Chezmoi templates. The branching uses Chezmoi's built-in variables:

```go-template
{{ if eq .chezmoi.os "darwin" -}}
# macOS-specific config
{{ else if eq .chezmoi.os "linux" -}}
# Ubuntu-specific config
{{ end -}}
```

For cases where the OS distinction is insufficient (e.g., Ubuntu vs. Fedora), `.chezmoi.osRelease.id` is used.

Shared config (the majority) lives outside conditionals. OS-specific blocks are kept minimal and documented with inline comments explaining why the branch exists.

### 6.3 Package Installation Strategy

Package installation is handled by `run_once_` scripts, one per category:

```
run_once_010_install-packages.sh.tmpl
run_once_020_install-fonts.sh.tmpl
run_once_030_install-extras.sh.tmpl
```

**macOS (Homebrew):**
- CLI tools via `brew install`.
- Cask apps and fonts via `brew install --cask`.
- A `Brewfile` (generated or static) can be used for declarative installs.

**Ubuntu (APT):**
- Core packages via `sudo apt-get install -y`.
- Tools unavailable in APT repos (e.g., Zellij, LazyGit, Neofetch replacements) are installed via:
  1. Official APT repositories / PPAs if available.
  2. Direct `.deb` package download from GitHub releases.
  3. Precompiled binary download to `~/.local/bin/` as last resort.
- The install scripts are idempotent: they check `command -v <tool>` before installing.

### 6.4 Font Installation Strategy

| | macOS | Ubuntu |
|-|-------|--------|
| Install method | `brew install --cask font-<name>-nerd-font` | Download `.zip` from [nerdfonts.com](https://www.nerdfonts.com/font-downloads) or GitHub releases, extract to `~/.local/share/fonts/` |
| Cache refresh | Automatic | `fc-cache -fv` |
| Path | `~/Library/Fonts/` (managed by Homebrew cask) | `~/.local/share/fonts/` |

The font install script (`run_once_020_install-fonts.sh.tmpl`) handles both paths with OS branching.

### 6.5 Dynamic Theme Switcher Architecture

The theme system has three layers: **theme definitions**, **a switcher CLI**, and **per-app adapters**.

#### 6.5.1 Theme Definitions

Each theme is defined as a single canonical TOML file under `~/.config/theme/themes/`:

```
~/.config/theme/
  current              # plain text file containing the active theme name, e.g. "catppuccin-mocha"
  themes/
    catppuccin-mocha.toml
    catppuccin-latte.toml
    pine-rose.toml
    nakagawa.toml
```

A theme TOML file defines a normalized color palette:

```toml
[meta]
name = "Catppuccin Mocha"
variant = "dark"

[colors]
background = "#1e1e2e"
foreground = "#cdd6f4"
cursor     = "#f5e0dc"
selection_bg = "#585b70"
selection_fg = "#cdd6f4"

# ANSI 0-15
black   = "#45475a"
red     = "#f38ba8"
green   = "#a6e3a1"
yellow  = "#f9e2af"
blue    = "#89b4fa"
magenta = "#f5c2e7"
cyan    = "#94e2d5"
white   = "#bac2de"

bright_black   = "#585b70"
bright_red     = "#f38ba8"
bright_green   = "#a6e3a1"
bright_yellow  = "#f9e2af"
bright_blue    = "#89b4fa"
bright_magenta = "#f5c2e7"
bright_cyan    = "#94e2d5"
bright_white   = "#a6adc8"

# Extended / semantic (optional)
accent      = "#89b4fa"
warning     = "#f9e2af"
error       = "#f38ba8"
success     = "#a6e3a1"
surface0    = "#313244"
surface1    = "#45475a"
surface2    = "#585b70"
```

This normalized schema means adding a new theme is one file. No app-specific knowledge required.

#### 6.5.2 The Switcher CLI

A shell script (or compiled binary in later phases) at `~/.local/bin/theme-switch`:

```
Usage: theme-switch <theme-name>
       theme-switch --list
       theme-switch --current
```

When invoked:

1. Validates the theme name against `~/.config/theme/themes/`.
2. Writes the theme name to `~/.config/theme/current`.
3. Iterates over registered **adapters** and invokes each one.
4. Prints confirmation and any app-specific reload instructions.

#### 6.5.3 Per-App Adapters

Each supported application has an adapter script in `~/.config/theme/adapters/`:

```
~/.config/theme/adapters/
  kitty.sh
  alacritty.sh
  ghostty.sh
  zellij.sh
  tmux.sh
  zsh.sh
  neovim.sh
  emacs.sh
```

An adapter:

1. Reads the canonical theme TOML (parsed via a lightweight TOML reader -- a small Python/shell helper or `dasel`/`yq`).
2. Generates the app-specific theme config file (e.g., `~/.config/kitty/current-theme.conf`).
3. Triggers a live reload if the app supports it (e.g., `kitty @ set-colors`, `tmux source-file`, NeoVim via RPC).

**App integration points:**

| App | Generated file | Include mechanism | Live reload |
|-----|----------------|-------------------|-------------|
| Kitty | `~/.config/kitty/current-theme.conf` | `include current-theme.conf` in `kitty.conf` | `kitty @ set-colors --all` |
| Alacritty | `~/.config/alacritty/current-theme.toml` | `import` in `alacritty.toml` | Automatic (watches file) |
| Ghostty | `~/.config/ghostty/current-theme` | `config-file` directive | Automatic (watches file) |
| Zellij | `~/.config/zellij/themes/current.kdl` | `theme` directive in `config.kdl` | Requires session restart |
| Tmux | `~/.config/tmux/current-theme.conf` | `source-file` in `tmux.conf` | `tmux source-file` |
| Zsh | `~/.config/zsh/current-theme.zsh` | `source` in `.zshrc` | New shell sessions |
| NeoVim | Sets via Lua global / colorscheme name | Lua autocommand reads theme file | `:colorscheme` via RPC or on focus |
| Emacs | Sets via Elisp theme load | Elisp reads theme file on focus | `load-theme` via emacsclient |

#### 6.5.4 Theme Lifecycle During `chezmoi apply`

Chezmoi templates for each app reference the _include mechanism_ (e.g., Kitty's `include` directive), not the theme colors directly. The actual theme file is generated by the switcher, not by Chezmoi. This separation means:

- `chezmoi apply` sets up the _structure_ (configs with include directives, adapter scripts, theme definitions).
- `theme-switch <name>` sets up the _content_ (the actual colors).
- A `run_after_` Chezmoi script invokes `theme-switch $(cat ~/.config/theme/current)` to ensure consistency after apply.

## 7. Repository Structure

```
dotfiles/                              # chezmoi source directory root
  .chezmoi.toml.tmpl                   # chezmoi config template (sets data defaults)
  .chezmoidata.toml                    # shared default data (default theme, etc.)
  .chezmoiexternal.toml                # external deps (fonts, plugin managers)
  .chezmoiignore                       # OS-conditional ignores

  # -- Install scripts (ordered by numeric prefix) --
  run_once_010_install-packages.sh.tmpl
  run_once_020_install-fonts.sh.tmpl
  run_once_030_install-extras.sh.tmpl
  run_after_090_apply-theme.sh.tmpl

  # -- Shell configs --
  dot_zshrc.tmpl
  dot_zshenv.tmpl
  dot_bashrc.tmpl
  dot_bash_profile.tmpl

  # -- XDG config directory --
  private_dot_config/

    # Theme system
    theme/
      current                          # active theme name (not templated, managed by switcher)
      themes/
        catppuccin-mocha.toml
        catppuccin-latte.toml
        pine-rose.toml
        nakagawa.toml
      adapters/
        kitty.sh
        alacritty.sh
        ghostty.sh
        zellij.sh
        tmux.sh
        zsh.sh
        neovim.sh
        emacs.sh

    # Kitty
    kitty/
      kitty.conf.tmpl
      current-theme.conf               # generated by theme-switch, not committed

    # Alacritty
    alacritty/
      alacritty.toml.tmpl
      current-theme.toml               # generated by theme-switch

    # Ghostty
    ghostty/
      config.tmpl
      current-theme                    # generated by theme-switch

    # Zellij
    zellij/
      config.kdl.tmpl
      themes/
        current.kdl                    # generated by theme-switch

    # Tmux
    tmux/
      tmux.conf.tmpl
      current-theme.conf               # generated by theme-switch

    # NeoVim (LazyVim)
    nvim/
      init.lua
      lua/
        config/
          lazy.lua
        plugins/
          ...

    # Emacs
    emacs/
      init.el
      ...

    # Zsh (XDG-compliant extras)
    zsh/
      aliases.zsh
      functions.zsh
      current-theme.zsh                # generated by theme-switch
      completions/

    # Dev tools
    lazygit/
      config.yml.tmpl
    opencode/
      config.yaml.tmpl
    neofetch/
      config.conf.tmpl

  # -- Local bin --
  private_dot_local/
    bin/
      executable_theme-switch          # the switcher CLI script

  # -- Documentation --
  docs/
    ONBOARDING.md
    THEME_AUTHORING.md
```

**Notes on Chezmoi naming conventions:**
- `dot_` prefix maps to `.` in the target.
- `private_` sets `0700` permissions on directories.
- `executable_` sets `0755` on files.
- `.tmpl` suffix marks files for template processing.
- `run_once_` / `run_after_` scripts execute during `chezmoi apply`.

## 8. Phased Rollout

### Phase 1 -- Proof of Concept

**Scope:** Zsh + Kitty + Zellij + Catppuccin + Pine Rose

**Deliverables:**
- Chezmoi source directory initialized and pushed to Git.
- `.chezmoi.toml.tmpl` with OS detection and default data.
- `run_once_010_install-packages.sh.tmpl` installs Zsh, Kitty, Zellij via `brew` (macOS) or `apt` + binary fallback (Ubuntu).
- Zsh config (`.zshrc`, `.zshenv`) with prompt, aliases, PATH setup, and theme sourcing.
- Kitty config with `include current-theme.conf`.
- Zellij config with theme directive pointing to generated KDL.
- Theme definitions for Catppuccin Mocha and Pine Rose.
- Adapters for Kitty, Zellij, Zsh.
- `theme-switch` script with `--list`, `--current`, and positional theme name.
- `run_after_090_apply-theme.sh.tmpl` to apply theme post-chezmoi-apply.
- Tested on macOS (ARM) and Ubuntu 24.04.

**Success criteria:**
1. `chezmoi init --apply <repo>` on a fresh macOS or Ubuntu machine installs packages and produces a working Zsh + Kitty + Zellij environment.
2. `theme-switch catppuccin-mocha` changes colors in Kitty, Zellij, and Zsh prompt.
3. `theme-switch pine-rose` does the same with Pine Rose colors.
4. Running `chezmoi apply` a second time is idempotent (no errors, no changes if nothing changed).

### Phase 2 -- Terminal Ecosystem Expansion

**Scope:** Alacritty, Ghostty, Tmux, Nerd Fonts, Nakagawa theme

**Deliverables:**
- Alacritty config with theme import mechanism.
- Ghostty config with `config-file` theme include.
- Tmux config with `source-file` theme include.
- Adapters for Alacritty, Ghostty, Tmux.
- Font installation script covering all target Nerd Fonts.
- Nakagawa theme definition TOML.
- Install scripts updated for new tools.

**Success criteria:**
1. All five terminal apps (Kitty, Alacritty, Ghostty, Zellij, Tmux) switch themes simultaneously with one `theme-switch` call.
2. Nerd Font glyphs render correctly in all terminal emulators on both platforms.
3. No Homebrew is installed or invoked on Ubuntu at any point.

### Phase 3 -- Editors and Dev Tools

**Scope:** NeoVim (LazyVim), Emacs, OpenCode, LazyGit, Neofetch

**Deliverables:**
- NeoVim config: LazyVim base with custom plugin specs, LSP configs, and theme integration.
- Emacs config with theme loading from the central theme system.
- Adapters for NeoVim and Emacs.
- LazyGit, OpenCode, Neofetch configs (templated for OS differences).
- Install scripts updated.

**Success criteria:**
1. NeoVim and Emacs colorschemes change when `theme-switch` is invoked.
2. LazyVim plugins install without manual intervention on both platforms.
3. All dev tools are installed and configured via `chezmoi apply`.

### Phase 4 -- Shell Expansion and Documentation Polish

**Scope:** Bash config, Fish config, documentation, workflow polish

**Deliverables:**
- Bash config (`.bashrc`, `.bash_profile`) with same aliases, PATH, and theme sourcing as Zsh.
- Fish config (`config.fish`) with equivalent setup.
- `ONBOARDING.md`: step-by-step guide for setting up a new machine.
- `THEME_AUTHORING.md`: how to add a new theme.
- `.chezmoiignore` refined so Fish/Bash configs are opt-in via chezmoi data flags.
- Chezmoi data flags for opting into/out of specific tools (e.g., `use_fish = false`).
- CI job (GitHub Actions) that validates `chezmoi apply` on macOS and Ubuntu runners.

**Success criteria:**
1. Following `ONBOARDING.md` produces a working environment on a fresh machine in under 15 minutes.
2. Adding a new theme requires only creating one TOML file -- no other changes.
3. CI passes on both macOS and Ubuntu with a clean `chezmoi init --apply`.

## 9. Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| **APT packages lag behind Homebrew versions** | High | Medium | Use binary downloads from GitHub releases for tools where version matters (Zellij, LazyGit). Pin minimum versions in install scripts. |
| **Theme TOML parsing in shell is fragile** | Medium | High | Keep the TOML schema flat (no nested tables beyond `[meta]` and `[colors]`). Provide a small Python fallback parser. Test with every theme on every adapter. |
| **Chezmoi `run_once_` scripts re-run unintentionally** | Medium | Low | Scripts are idempotent by design. Use `command -v` checks. Chezmoi tracks script hashes so they only re-run when content changes. |
| **Live reload not supported by all apps** | Certain | Low | Document which apps require restart. Zellij and new shell sessions are known cases. The switcher prints reload instructions per app. |
| **NeoVim plugin ecosystem churn** | Medium | Medium | Pin LazyVim to a known-good commit via `.chezmoiexternal.toml`. Avoid bleeding-edge plugins. |
| **Font download URLs break** | Low | Medium | Use Chezmoi externals with checksums. Fall back to Homebrew cask on macOS. Document manual install as last resort. |
| **Defaults don't suit every machine** | Medium | Low | All preferences (default theme, preferred shell, enabled tools) are in `~/.config/chezmoi/chezmoi.toml` which is per-machine and not committed. Repo provides sane defaults in `.chezmoidata.toml`. The user can override defaults on any machine. |
| **Secrets accidentally committed** | Low | High | `.chezmoiignore` excludes sensitive paths. No secret values in templates -- only references to secret backends. Pre-commit hook with `git-secrets` or `gitleaks`. |

## 10. Open Questions

1. **TOML parser for adapters** -- Use a vendored Python script, `dasel`, or `yq`? Decision needed before Phase 1 implementation.
2. **Zsh plugin manager** -- Use `zinit`, `antidote`, `sheldon`, or none (manual sourcing)? Affects `.zshrc` structure.
3. **NeoVim theme integration** -- Should the adapter set a Lua global that LazyVim reads, or write a small Lua file that gets sourced? Decision needed before Phase 3.
4. **CI matrix** -- GitHub Actions macOS runners are expensive. Run Ubuntu on every push, macOS only on PRs to `main`?

---

*This document is the source of truth for the dotfiles project scope. Update it as decisions are made on open questions and as phases are completed.*
