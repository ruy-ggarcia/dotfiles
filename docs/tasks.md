# Dotfiles — Implementation Task Breakdown

This document tracks all implementation tasks organized by milestone. Each milestone builds on the previous and represents a shippable increment of the dotfiles installer.

**Conventions:**
- Tasks are tagged `T-NNN` and grouped by milestone.
- Complexity labels: **S** (≤ 1 hr), **M** (1–3 hrs), **L** (3–8 hrs), **XL** (> 8 hrs or multiple interconnected pieces).
- Check off tasks with `- [x]` as they are completed.
- Dependencies reference other task IDs; all listed deps must be complete before starting.

---

## Summary

| Milestone | Tasks | Focus |
|-----------|-------|-------|
| M1: Project Scaffolding | T-001 – T-007 | Repo skeleton, Cargo project, directory structure |
| M2: Core Engine (no TUI) | T-008 – T-029 | Data models, scanner, package manager, template, symlink, orchestrator |
| M3: TUI | T-030 – T-037 | `inquire`-based wizard, feedback, Ctrl+C handling |
| M4: Distribution | T-038 – T-042 | `install.sh`, GitHub Actions CI/CD, cross-compilation |
| M5: Theme System | T-043 – T-060 | Schema, 10 palette files, tool Tera templates |
| M6: Tool Configurations | T-061 – T-070 | Full tool configs, font installer, NeoVim tree |

---

## Milestone 1: Project Scaffolding

**Goal:** Bootstrap the repository skeleton so `cargo build` succeeds and the directory tree matches `ARCHITECTURE.md`.

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [x] | T-001 | Initialize Cargo project | Run `cargo init --name dotfiles` in `installer/`; set `edition = "2021"` in `Cargo.toml` | — | S |
| - [x] | T-002 | Add Cargo dependencies | Add `anyhow`, `inquire`, `tera`, `toml`, `serde`, `serde_json`, `walkdir`, `chrono` to `Cargo.toml` | T-001 | S |
| - [x] | T-003 | Create top-level directories | Create `modules/`, `themes/palettes/`, `fonts/`, `.rendered/` placeholder, `.github/workflows/` | T-001 | S |
| - [x] | T-004 | Create `.gitignore` | Add entries: `.rendered/`, `target/`, `*.bak.*` | T-003 | S |
| - [x] | T-005 | Create sample `zsh` module | Create `modules/zsh/` with `packages.toml` (`macos = ["zsh"]`, `ubuntu = ["zsh"]`) and stub `home/.zshrc` | T-003 | S |
| - [ ] | T-006 | Create placeholder schema and manifest | Create empty `themes/schema.json` and `fonts/manifest.toml` so the directory structure is complete | T-003 | S |
| - [ ] | T-007 | Stub `main.rs` | Write stub `installer/src/main.rs` that prints "Dotfiles Installer" and exits — verifies `cargo build` is clean | T-002 | S |

**Acceptance Criteria (M1):** `cargo build` succeeds; directory tree matches `ARCHITECTURE.md`; `modules/zsh/` is discoverable.

---

## Milestone 2: Core Engine (no TUI)

**Goal:** Implement all engine components end-to-end with hardcoded selections, so the zsh symlink is created idempotently without any TUI.

### 2A — Data Models

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-008 | Create `models.rs` | Define structs: `Theme`, `Module`, `UserSelection`, `Plan`, `TemplateJob`, `SymlinkJob` as specified in `design.md §3`; all fields `pub`, derive `Debug` | T-007 | M |
| - [ ] | T-009 | Wire models into `main.rs` | Declare `mod models;` in `main.rs`; verify it compiles with no unused-import warnings | T-008 | S |

### 2B — Scanner

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-010 | Implement `scan_modules` | Create `installer/src/scanner.rs` with `scan_modules(repo_root: &Path) -> Result<Vec<Module>>` — reads every immediate subdirectory of `modules/`, parses `packages.toml` via `toml` crate, builds `Module` structs | T-008 | M |
| - [ ] | T-011 | Implement `scan_themes` | Add `scan_themes(repo_root: &Path, schema: &serde_json::Value) -> Result<Vec<Theme>>` — reads all `.toml` files in `themes/palettes/`, parses palette variables as flat `HashMap<String, String>`, validates required keys against schema; warn + skip invalid palettes | T-008, T-012 | M |
| - [ ] | T-012 | Implement `load_schema` | Add `load_schema(repo_root: &Path) -> Result<serde_json::Value>` to `scanner.rs` — reads `themes/schema.json` | T-008 | S |
| - [ ] | T-013 | Unit tests for `scan_modules` | Happy path (zsh module), missing `packages.toml` (error or skip), unknown OS key (mark unsupported) | T-010 | M |

### 2C — Package Manager

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-014 | Define `PackageManager` trait | Create `installer/src/package.rs` with `PackageManager` trait (`is_available`, `update_index`, `install`) and `get_package_manager() -> Result<Box<dyn PackageManager>>` factory | T-007 | M |
| - [ ] | T-015 | Implement `Brew` | `is_available` checks for `brew` binary via `Command`; `install` runs `brew install <packages>` idempotently | T-014 | M |
| - [ ] | T-016 | Implement `Apt` | `is_available` checks for `apt-get`; `update_index` runs `apt-get update`; `install` runs `apt-get install -y <packages>` | T-014 | M |
| - [ ] | T-017 | Unit tests for OS detection | Test `get_package_manager` OS detection logic (mock `std::env::consts::OS`) | T-015, T-016 | M |

### 2D — Template Rendering

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-018 | Implement `render_templates` | Create `installer/src/template.rs` with `render_templates(jobs: &[TemplateJob], theme: &Theme, rendered_dir: &Path) -> Result<()>` — initializes `Tera`, builds context from `theme.variables`, renders each job, writes output | T-008 | M |
| - [ ] | T-019 | Parent directory creation | Add `create_dir_all` for each render destination in `template.rs` | T-018 | S |
| - [ ] | T-020 | Unit tests for `render_templates` | Valid Tera template renders correctly; missing variable returns descriptive error | T-018 | M |

### 2E — Symlink Manager

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-021 | Implement `process_symlinks` | Create `installer/src/symlink.rs` with `process_symlinks(jobs: &[SymlinkJob]) -> Result<()>` implementing the three-case idempotent algorithm (missing → create, correct → skip, conflict → backup + create) | T-008 | L |
| - [ ] | T-022 | Implement backup logic | Generate `{target}.bak.{chrono_timestamp}` path; rename existing target before creating symlink | T-021 | M |
| - [ ] | T-023 | Parent directory creation | Add `create_dir_all` for parent directories of each symlink target | T-021 | S |
| - [ ] | T-024 | Unit tests for `process_symlinks` | Missing target (creates symlink), correct symlink (no-op), conflicting file (backup created, new symlink correct) | T-021, T-022 | M |

### 2F — Engine Orchestrator

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-025 | Implement `generate_plan` | Create `installer/src/engine.rs` with `generate_plan(selection: &UserSelection, repo_root: &Path) -> Result<Plan>` — walks `home/` and `config/` subtrees of each selected module, produces `TemplateJob` for `.tera` files and `SymlinkJob` for static files; deduplicates packages | T-008, T-018, T-021 | L |
| - [ ] | T-026 | Implement `execute_plan` | Add `execute_plan(plan: &Plan, theme: &Theme, pkg_mgr: &dyn PackageManager, repo_root: &Path) -> Result<()>` — calls package install → template render → symlink creation in sequence | T-025, T-014, T-018, T-021 | M |
| - [ ] | T-027 | Unit test for `generate_plan` | Given a module with one static file and one `.tera` file, verify plan has 1 template job and 2 symlink jobs | T-025 | M |

### 2G — Wire in `main.rs` (hardcoded, no TUI)

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-028 | Integrate engine into `main.rs` | Load schema → scan modules → scan themes → hardcode `UserSelection` (first module, first theme) → generate plan → execute plan; wrap in `anyhow::Result` and print errors to stderr cleanly | T-010, T-011, T-025, T-026 | M |
| - [ ] | T-029 | Smoke test engine end-to-end | Run `cargo run` in the repo — verify it scans `modules/zsh/`, renders nothing (no `.tera` files yet), creates `~/.zshrc` symlink successfully | T-028 | S |

**Acceptance Criteria (M2):** Engine runs end-to-end with hardcoded selections; zsh symlink created; re-run is idempotent (no backup created second time).

---

## Milestone 3: TUI

**Goal:** Replace hardcoded selections with an interactive `inquire`-based wizard; handle cancellation gracefully.

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-030 | Implement `display_welcome` | Create `installer/src/tui.rs` with `display_welcome(os: &str, pkg_mgr: &str, version: &str)` — prints ASCII header banner with OS, arch, package manager, and version string | T-028 | S |
| - [ ] | T-031 | Implement `select_modules` | Add `select_modules(modules: Vec<Module>) -> Result<Vec<Module>>` — uses `inquire::MultiSelect`; pre-selects primary tools (zsh, kitty, zellij, neovim, opencode) | T-030 | M |
| - [ ] | T-032 | Implement `select_theme` | Add `select_theme(themes: Vec<Theme>) -> Result<Theme>` — uses `inquire::Select`; default selection `catppuccin-mocha` | T-030 | M |
| - [ ] | T-033 | Implement `confirm_plan` | Add `confirm_plan(plan: &Plan, theme: &Theme) -> Result<bool>` — shows plan summary (package count, template count, symlink count, theme name) and uses `inquire::Confirm` | T-030, T-025 | M |
| - [ ] | T-034 | Handle Ctrl+C / cancellation | Bubble up all `inquire` errors; `main.rs` catches `InquireError::OperationCanceled` and exits cleanly with message "Setup canceled by user." | T-031, T-032, T-033 | S |
| - [ ] | T-035 | Add execution feedback | Print `[✓] Installing packages...`, `[✓] Rendering templates...`, `[✓] Creating symlinks...` and per-symlink `  -> {target}` lines | T-026 | S |
| - [ ] | T-036 | Wire TUI into `main.rs` | Replace hardcoded selections with `tui::select_modules()` and `tui::select_theme()`; confirm plan before execution; print success or failure footer | T-031, T-032, T-033, T-034, T-035 | M |
| - [ ] | T-037 | Print version from `Cargo.toml` | Embed version string at compile time via `env!("CARGO_PKG_VERSION")` in the welcome header | T-030 | S |

**Acceptance Criteria (M3):** Full wizard runs; selecting zsh + catppuccin-mocha + confirming → symlink created; Ctrl+C exits cleanly; re-run is idempotent.

---

## Milestone 4: Distribution

**Goal:** Automate CI checks and produce cross-compiled release binaries downloadable via `install.sh`.

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-038 | Write `install.sh` | Clone or pull `~/.dotfiles`; detect OS via `uname -s` / `uname -m`; map to Rust target triple; download `dotfiles-{target}.tar.gz` from latest GitHub Release to `~/.dotfiles/.bin/`; extract and execute | T-036 | L |
| - [ ] | T-039 | Create `build.yml` CI workflow | Trigger on PRs to `main`; steps: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` | T-007 | M |
| - [ ] | T-040 | Create `release.yml` CD workflow | Trigger on `v*` tags; build matrix for 3 targets; cross-compile `--release`; compress each binary as `dotfiles-{target}.tar.gz`; attach to GitHub Release via `softprops/action-gh-release` | T-039 | L |
| - [ ] | T-041 | Configure cross-compilation | Add `cross` crate or configure `cargo-zigbuild` in release workflow for Linux MUSL cross-compilation from macOS runner | T-040 | M |
| - [ ] | T-042 | Test `install.sh` locally | Run script on macOS; verify `~/.dotfiles/.bin/dotfiles` is present and executable | T-038, T-040 | S |

**Acceptance Criteria (M4):** `build.yml` passes on a test PR; `release.yml` triggers on `v0.1.0` tag and attaches 3 tarballs to the release; `install.sh` downloads and runs the correct binary.

---

## Milestone 5: Theme System

**Goal:** Implement the full schema, author all 10 palette files, and create Tera templates for every supported tool.

### 5A — Schema

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-043 | Write `themes/schema.json` | Full JSON Schema as specified in `design.md §6`: required sections `meta` (5 keys), `colors.core` (7), `colors.ansi.normal` (8), `colors.ansi.bright` (8), `colors.ui` (8); hex color pattern validation | T-006 | M |
| - [ ] | T-044 | Update scanner schema validation | Validate against the full `schema.json` structure (use `jsonschema` crate or manual checks); reject palette with clear per-field error message | T-043, T-011 | M |

### 5B — Palette Files

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-045 | `catppuccin-mocha.toml` | All 36 keys (31 colors + 5 meta); source from official Catppuccin color spec | T-043 | M |
| - [ ] | T-046 | `catppuccin-macchiato.toml` | Catppuccin Macchiato variant | T-043 | M |
| - [ ] | T-047 | `catppuccin-frappe.toml` | Catppuccin Frappé variant | T-043 | M |
| - [ ] | T-048 | `catppuccin-latte.toml` | Catppuccin Latte (light) variant; `meta.variant = "light"` | T-043 | M |
| - [ ] | T-049 | `rose-pine.toml` | Rosé Pine Main variant | T-043 | M |
| - [ ] | T-050 | `rose-pine-moon.toml` | Rosé Pine Moon variant | T-043 | M |
| - [ ] | T-051 | `rose-pine-dawn.toml` | Rosé Pine Dawn (light) variant | T-043 | M |
| - [ ] | T-052 | `kanagawa-wave.toml` | Kanagawa Wave variant | T-043 | M |
| - [ ] | T-053 | `kanagawa-dragon.toml` | Kanagawa Dragon variant | T-043 | M |
| - [ ] | T-054 | `kanagawa-lotus.toml` | Kanagawa Lotus (light) variant | T-043 | M |
| - [ ] | T-055 | Verify all palettes pass validation | Run installer scanner in isolation (unit test or manual run) against all 10 palettes | T-044, T-045, T-046, T-047, T-048, T-049, T-050, T-051, T-052, T-053, T-054 | S |

### 5C — Tool Templates

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-056 | `kitty.conf.tera` | Full Kitty color config using `{{ colors.core.* }}` and `{{ colors.ansi.* }}` variables as specified in `design.md §6` | T-008 | M |
| - [ ] | T-057 | `alacritty.toml.tera` | Alacritty TOML color config using theme variables | T-008 | M |
| - [ ] | T-058 | `config.kdl.tera` (Zellij) | Zellij KDL theme block using `colors.ansi.*` and `colors.ui.*` | T-008 | M |
| - [ ] | T-059 | `.tmux.conf.tera` | tmux status bar and pane border styles using `colors.ui.*` | T-008 | M |
| - [ ] | T-060 | `theme.lua.tera` (Neovim) | Conditional Lua bridge for catppuccin/rose-pine/kanagawa plugin setup using `{{ meta.nvim_plugin }}`, `{{ meta.nvim_variant }}`, `{{ meta.nvim_theme }}` | T-008 | L |

**Acceptance Criteria (M5):** All 10 palettes pass validation; running the installer with the kitty module + any palette renders a valid `kitty.conf` in `.rendered/`; switching themes re-renders correctly.

---

## Milestone 6: Tool Configurations

**Goal:** Author all full, production-quality tool configs so a fresh macOS install is fully provisioned in a single run.

| Status | ID | Title | Description | Dependencies | Complexity |
|--------|----|-------|-------------|--------------|------------|
| - [ ] | T-061 | Complete `modules/zsh/home/.zshrc` | Full Zsh config: Oh-My-Zsh or Prezto setup, aliases, PATH exports, plugin list; static file (no Tera) | T-005 | L |
| - [ ] | T-062 | Complete Kitty module | `modules/kitty/packages.toml` and any additional static Kitty config files (font size, keybindings) alongside `kitty.conf.tera` | T-056, T-062 | M |
| - [ ] | T-063 | Complete Alacritty module | `modules/alacritty/packages.toml` and static Alacritty config sections (font, window, scrolling) alongside the color template | T-057 | M |
| - [ ] | T-064 | Complete Zellij module | `modules/zellij/packages.toml` and full `config.kdl.tera` with keybindings, default layout, plugin settings beyond just the theme block | T-058 | L |
| - [ ] | T-065 | Complete tmux module | `modules/tmux/packages.toml` and full `.tmux.conf.tera` with keybindings, TPM plugin manager, status bar format beyond just colors | T-059 | L |
| - [ ] | T-066 | Create full NeoVim config tree | `modules/nvim/config/nvim/`: `init.lua`, `lua/core/options.lua`, `lua/core/keymaps.lua`, `lua/core/theme.lua.tera`, `lua/plugins/` with lazy.nvim specs (LSP, treesitter, telescope, colorscheme plugins) | T-060 | XL |
| - [ ] | T-067 | Create opencode module | `modules/opencode/packages.toml` and config files under `modules/opencode/config/opencode/` (skills, `AGENTS.md`, static OpenCode config) | T-025 | M |
| - [ ] | T-068 | Create `fonts/manifest.toml` | Font entries: `MesloLGS NF`, `Iosevka Nerd Font`, `JetBrainsMono NF` — each with name and GitHub Release download URL | T-006 | S |
| - [ ] | T-069 | Implement font installer | Read `fonts/manifest.toml`; download and install fonts to `~/Library/Fonts` (macOS) or `~/.local/share/fonts` (Linux) | T-068, T-026 | L |
| - [ ] | T-070 | Add font selection to TUI | `inquire::MultiSelect` for fonts after module selection in `tui.rs` | T-069, T-031 | M |

**Acceptance Criteria (M6):** Full end-to-end install on macOS provisions zsh + kitty + zellij + neovim + opencode with catppuccin-mocha; all symlinks in `~/` and `~/.config/` are correct; switching to rose-pine re-renders all templates correctly.

---

## Dependency Graph

```
T-001 → T-002 → T-003 → T-004 → T-005 → T-006 → T-007
                                                     ↓
T-008 → T-009 (models — must precede all engine modules)
                ↓
T-010 → T-013 (scanner: modules)
T-011 → T-012 (scanner: themes + schema)
T-014 → T-015 → T-016 → T-017 (package manager)
T-018 → T-019 → T-020 (template rendering)
T-021 → T-022 → T-023 → T-024 (symlink manager)
                                     ↓
T-025 → T-026 → T-027 (engine — depends on all above)
                            ↓
T-028 → T-029 (wire main.rs — M2 complete)
                    ↓
T-030 → T-031 → T-032 → T-033 → T-034 → T-035 → T-036 → T-037 (TUI — M3)
                                                                      ↓
T-038 → T-039 → T-040 → T-041 → T-042 (distribution — M4)
                                              ↓
T-043 → T-044 (schema — M5A)
T-045..T-055 (palettes — depend on T-043)
T-056..T-060 (templates — depend on T-008 models)
                                              ↓
T-061..T-070 (tool configs — depend on M5 complete)
```

**Critical path:** T-007 → T-008 → T-009 → T-025 → T-026 → T-028 → T-030 → T-036 → T-038 → T-043 → T-056 → T-061

All Milestone 2 engine tasks (T-010 through T-027) can be developed in parallel once T-008 (models) is complete. Palette files (T-045–T-054) can be authored in parallel with Rust code since they are pure data files.

---

## Legend

| Label | Complexity | Estimated Effort |
|-------|-----------|-----------------|
| **S** | Small | ≤ 1 hour — configuration, stubs, minor wiring |
| **M** | Medium | 1–3 hours — single-function implementation with tests |
| **L** | Large | 3–8 hours — multi-part feature, non-trivial logic |
| **XL** | Extra-Large | > 8 hours — multiple interconnected pieces or full subsystems |
