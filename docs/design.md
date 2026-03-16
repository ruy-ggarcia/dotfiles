# Technical Design Document

## 1. Repository Structure

> The directory layout lives in [`ARCHITECTURE.md`](../ARCHITECTURE.md) at the repo root — that is the single source of truth. This section covers the **rationale and depth** behind each convention.

### Philosophy: Convention Over Configuration

The engine (`installer/`) auto-discovers modules and themes at runtime by scanning well-defined directory conventions. There is a strict decoupling between the engine and its configuration data. Adding a new tool module or theme palette requires **zero changes to the installer code** — create a directory or a TOML file in the right place, and the engine picks it up automatically.

This means the scanner (`scanner.rs`) is the only place that encodes directory conventions. If the conventions change, only the scanner changes — no other engine code is affected.

### Module Convention (Detailed)

Each tool lives in `modules/<tool-name>/`. The scanner reads every immediate subdirectory of `modules/` and treats it as a module.

**`packages.toml`** defines platform-specific package dependencies:

```toml
[packages]
macos = ["package-name"]
ubuntu = ["package-name"]
```

The `package.rs` module implements the `PackageManager` trait, with concrete implementations for Homebrew (macOS) and APT (Ubuntu/Linux). The correct implementation is selected at runtime based on the detected OS.

**`home/` subtree**: Every file under `home/` is symlinked relative to `~/`. For example, `modules/zsh/home/.zshrc` becomes `~/.zshrc`.

**`config/` subtree**: Every file under `config/` is symlinked relative to `~/.config/`. For example, `modules/kitty/config/kitty/kitty.conf` becomes `~/.config/kitty/kitty.conf`.

**`.tera` extension**: Files ending in `.tera` are Tera templates. They are NOT symlinked directly. Instead, they are rendered to `.rendered/` using the active theme palette, and the rendered file is what gets symlinked to the target location. The `.tera` extension is stripped in the rendered output (e.g., `kitty.conf.tera` → `.rendered/kitty/kitty.conf`).

**Static files**: Files without `.tera` extension are symlinked directly from their location in `modules/`.

### Theme Convention (Detailed)

Each theme variant is a single TOML file in `themes/palettes/<name>.toml`. The contract for required color keys is defined in `themes/schema.json`. Every palette must conform to that schema — the engine validates this at startup.

The scanner discovers all `.toml` files in `themes/palettes/` and presents them as choices in the TUI. The selected palette is loaded and passed as the Tera rendering context for all template files.

### Symlink Strategy (Detailed)

The symlink manager (`symlink.rs`) handles all three cases idempotently:

| Target state | Action |
|---|---|
| Does not exist | Create symlink |
| Is a symlink pointing to the correct source | Do nothing |
| Is a file, directory, or wrong symlink | Back up to `{target}.bak.{timestamp}`, then create correct symlink |

This guarantees that running the installer multiple times is safe. Pre-existing user configs are never silently destroyed — they are backed up with a timestamp suffix.

### Template Rendering Flow (Detailed)

1. User selects a theme in the TUI
2. Engine loads `themes/palettes/<selected>.toml` into a Tera context
3. Scanner finds all `.tera` files across all selected modules
4. `template.rs` renders each `.tera` file with the theme context into `.rendered/`
5. `symlink.rs` symlinks `~/.config/...` (or `~/...`) to the corresponding `.rendered/` path
6. `.rendered/` is gitignored — it is always regenerated fresh on each run

## 2. Maintainer Workflows

### Adding a New Tool

Step-by-step example for adding `starship`:

1. Create `modules/starship/`
2. Create `modules/starship/packages.toml` with package mappings for each OS
3. Create the config file at the correct path under `home/` or `config/`:
   - If static: e.g., `modules/starship/config/starship/starship.toml`
   - If themed: e.g., `modules/starship/config/starship/starship.toml.tera`
4. If themed, use Tera syntax to reference palette variables (e.g., `{{ palette.base }}`)
5. Commit — the engine auto-discovers the new module and shows it in the TUI on next run

No installer code changes required.

### Adding a New Theme

Step-by-step example for adding `dracula`:

1. Open `themes/schema.json` and note all required keys
2. Create `themes/palettes/dracula.toml` with values for every required key
3. Commit — the engine auto-discovers it and shows it in the TUI on next run

No installer code changes required.

## 3. Core Engine Architecture

### Overview

The Rust binary is organized into 8 source modules with clear separation of concerns. The engine operates in three sequential phases: Discovery, Interactive (TUI), and Execution. All shared data structures live in a centralized `models.rs` to prevent circular dependencies.

### Module Structure

| Module | Responsibility |
|--------|---------------|
| `main.rs` | CLI entrypoint — orchestrates the three phases |
| `models.rs` | Shared data structures (`Theme`, `Module`, `Plan`, `UserSelection`) |
| `scanner.rs` | Discovery phase — auto-discovers `modules/` and `themes/palettes/` |
| `tui.rs` | Interactive phase — `inquire`-based wizard returning `UserSelection` |
| `engine.rs` | Execution phase orchestrator — generates `Plan` from `UserSelection`, coordinates package/template/symlink operations |
| `package.rs` | `PackageManager` trait with `Brew` and `Apt` implementations |
| `template.rs` | Tera rendering — loads theme palette as context, renders `.tera` files to `.rendered/` |
| `symlink.rs` | Idempotent symlink manager with `.bak.{timestamp}` backups |

### Core Data Structures

```rust
use std::collections::HashMap;
use std::path::PathBuf;

/// A parsed theme from themes/palettes/*.toml
pub struct Theme {
    pub name: String,
    pub path: PathBuf,
    pub variables: HashMap<String, String>,
}

/// A discovered tool module from modules/<name>/
pub struct Module {
    pub name: String,
    pub path: PathBuf,
    pub packages_by_os: HashMap<String, Vec<String>>,
}

/// Output of the TUI phase
pub struct UserSelection {
    pub selected_modules: Vec<Module>,
    pub selected_theme: Theme,
}

/// Actionable execution plan generated by engine.rs
pub struct Plan {
    pub packages_to_install: Vec<String>,
    pub templates_to_render: Vec<TemplateJob>,
    pub symlinks_to_create: Vec<SymlinkJob>,
}

pub struct TemplateJob {
    pub source: PathBuf,      // e.g., modules/kitty/config/kitty/kitty.conf.tera
    pub destination: PathBuf, // e.g., .rendered/kitty/config/kitty/kitty.conf
}

pub struct SymlinkJob {
    pub source_absolute: PathBuf, // File in .rendered/ or modules/
    pub target_absolute: PathBuf, // e.g., ~/.config/kitty/kitty.conf
}
```

### Execution Flow

#### Phase 1: Discovery (`scanner.rs`)

1. Detect current OS via `std::env::consts::OS`.
2. Read `themes/schema.json` to load required palette keys.
3. Scan `themes/palettes/*.toml` — parse into `Theme` structs, validate against schema. Warn and skip invalid themes.
4. Scan `modules/*/` — for each directory:
   - Parse `packages.toml` into OS-specific package lists.
   - If current OS is not present in the mapping, mark the module as unsupported.
   - Build a `Module` struct.

#### Phase 2: Interactive TUI (`tui.rs`)

1. Present a multi-select prompt with discovered modules.
2. Present a single-select prompt with discovered themes.
3. Return a `UserSelection`.

#### Phase 3: Execution (`engine.rs`)

1. **Plan Generation:**
   - Flatten and deduplicate packages for the current OS across all selected modules.
   - Walk `home/` and `config/` subtrees of each selected module.
   - For `.tera` files: create a `TemplateJob` (source → `.rendered/`) and a `SymlinkJob` (`.rendered/` → target).
   - For static files: create a direct `SymlinkJob` (module source → target).
2. **Package Installation:** Call `PackageManager::install()` with the deduplicated list.
3. **Template Rendering:** Render all `TemplateJob`s using the selected theme.
4. **Symlink Creation:** Process all `SymlinkJob`s idempotently.

### PackageManager Trait

```rust
pub trait PackageManager {
    /// Check if the package manager binary exists on the system
    fn is_available(&self) -> bool;

    /// Update local package index
    fn update_index(&self) -> Result<()>;

    /// Install packages idempotently (skip already installed)
    fn install(&self, packages: &[String]) -> Result<()>;
}

/// Factory: detect OS and return the correct implementation
pub fn get_package_manager() -> Result<Box<dyn PackageManager>> {
    match std::env::consts::OS {
        "macos" => Ok(Box::new(Brew)),
        "linux" => Ok(Box::new(Apt)),
        os => anyhow::bail!("Unsupported OS: {}", os),
    }
}
```

### Symlink Algorithm

For each `SymlinkJob`:

1. Ensure parent directories of `target_absolute` exist (`create_dir_all`).
2. Check if something exists at `target_absolute`:
   - **Nothing exists** → create symlink.
   - **Correct symlink exists** (already points to `source_absolute`) → skip (idempotent).
   - **Conflict** (file, directory, or wrong symlink) → backup to `{target}.bak.{timestamp}`, then create symlink.

### Template Rendering Flow

1. Initialize a `Tera` instance.
2. Build a `tera::Context` from the selected `Theme.variables`.
3. For each `TemplateJob`:
   - Read raw template from `source`.
   - Render via `tera.render_str()`.
   - Ensure parent directories of `destination` exist.
   - Write rendered output to `destination` (in `.rendered/`).

All `.tera` files are re-rendered on every run regardless of changes (idempotent by design).

### Error Handling

The engine uses `anyhow` for error propagation with a **fail-fast** strategy:

| Failure | Behavior |
|---------|----------|
| Package installation fails | Abort immediately — continuing without dependencies produces broken environments |
| Template rendering fails | Abort immediately — missing variables or syntax errors indicate a broken theme/template contract |
| Symlink creation fails | Abort immediately — permission errors or path issues must be resolved |
| User-facing errors | Caught in `main.rs` and printed cleanly to stderr, no panic traces |

## 4. TUI Flow

### Overview

The TUI uses the `inquire` crate to present a linear, 5-step wizard. The flow is designed for both first-run provisioning and re-runs (theme changes, adding modules). Cancellation at any step (`Ctrl+C`) exits cleanly with no side effects.

### Wizard Steps

#### Step 1: Welcome

The binary boots, runs the Discovery phase, and displays a header:

```text
=========================================
  DOTFILES INSTALLER v1.0.0
  OS: MacOS (aarch64) | Package: brew
=========================================
```

If discovery finds an invalid state (e.g., no modules, no themes), the binary fails fast with a clear error before any prompts.

#### Step 2: Module Selection (`inquire::MultiSelect`)

A flat list of all discovered modules with category labels. Primary tools (Zsh, Kitty, Zellij, NeoVim, OpenCode) are pre-selected by default.

```text
? Select modules to install and configure:
  [x] zsh       (Shell)
  [ ] bash      (Shell)
  [x] kitty     (Terminal)
  [ ] alacritty (Terminal)
  [x] zellij    (Multiplexer)
  [ ] tmux      (Multiplexer)
  [x] neovim    (Editor)
  [x] opencode  (Tools)
```

#### Step 3: Theme Selection (`inquire::Select`)

A single-select list of all discovered theme palettes from `themes/palettes/`. Default: `catppuccin-mocha`.

```text
? Select global theme:
    catppuccin-frappe
    catppuccin-latte
    catppuccin-macchiato
  > catppuccin-mocha
    kanagawa-dragon
    kanagawa-lotus
    kanagawa-wave
    rose-pine
    rose-pine-dawn
    rose-pine-moon
```

#### Step 4: Plan Confirmation (`inquire::Confirm`)

Before mutating the system, display a summary of the generated `Plan` and ask for explicit confirmation:

```text
? Review Installation Plan:
  - Packages to install: zsh, kitty, zellij, neovim
  - Templates to render: 45 files
  - Symlinks to create: 60 files (~/ and ~/.config/)
  - Active Theme: catppuccin-mocha

  Do you want to proceed? [Y/n]
```

#### Step 5: Execution Feedback

Clean, line-by-line status output. No spinners or animations — just clear, parsable feedback:

```text
[✓] Installing packages via brew...
[✓] Rendering templates to .rendered/ with catppuccin-mocha...
[✓] Backing up existing configurations...
[✓] Creating symlinks...
    -> ~/.zshrc
    -> ~/.config/kitty/kitty.conf
    -> ~/.config/nvim/init.lua

=========================================
  SUCCESS: Environment provisioned.
  Please restart your terminal to apply changes.
=========================================
```

On failure, the engine aborts with a clear error message and the system is left in a safe state (partially provisioned, no data loss due to backups).

### Cancellation and Re-runs

- `Ctrl+C` at any prompt → clean exit, code `0`, message "Setup canceled by user."
- Re-running the installer presents the same wizard. Previously installed packages are skipped by the package manager (idempotent). Templates are always re-rendered. Symlinks are re-evaluated.

## 5. Distribution Strategy

### Overview

The installer is distributed as a pre-compiled binary attached to GitHub Releases. A bootstrap shell script (`install.sh`) handles cloning the repository and downloading the correct binary for the user's platform. The binary requires the repository (specifically `modules/` and `themes/`) at runtime, so the clone-and-run model is mandatory.

### Runtime Model: Clone-and-Run

The bootstrap script clones the repository to `~/.dotfiles`, downloads the binary into `~/.dotfiles/.bin/`, and launches the TUI from within the repo. This ensures:

1. The binary has access to `modules/` and `themes/` at runtime.
2. The user has the full repo locally to inspect, customize, or fork.
3. No Rust toolchain is required on the target machine.

### Bootstrap Script (`install.sh`)

Located at the repository root. Executed via `curl -sL https://raw.github.com/.../install.sh | bash`.

Logic:

1. **Clone or update:** If `~/.dotfiles` exists, `git pull`. Otherwise, `git clone` to `~/.dotfiles`.
2. **Detect platform:** OS via `uname -s` (darwin/linux), architecture via `uname -m` (x86_64/arm64/aarch64).
3. **Map to Rust target:**
   - MacOS Intel → `x86_64-apple-darwin`
   - MacOS Silicon → `aarch64-apple-darwin`
   - Linux x86_64 → `x86_64-unknown-linux-musl`
4. **Download binary:** Fetch `dotfiles-{target}.tar.gz` from the latest GitHub Release to `~/.dotfiles/.bin/`.
5. **Execute:** Launch the TUI immediately from `~/.dotfiles/.bin/dotfiles`.

### GitHub Actions CI/CD

#### `build.yml` (Continuous Integration)

- **Trigger:** Pull requests to `main`
- **Steps:** `cargo fmt --check`, `cargo clippy`, `cargo test`
- **Purpose:** Gate broken code from merging

#### `release.yml` (Continuous Delivery)

- **Trigger:** Push of a tag matching `v*` (e.g., `v1.0.0`)
- **Toolchain:** Stable Rust
- **Build matrix:**

| Target | OS | Notes |
|--------|----|-------|
| `x86_64-apple-darwin` | MacOS Intel | |
| `aarch64-apple-darwin` | MacOS Silicon | |
| `x86_64-unknown-linux-musl` | Linux | MUSL for static linking — avoids glibc version issues across distros |

- **Steps:**
  1. Cross-compile `--release` for each target
  2. Compress binaries into `dotfiles-{target}.tar.gz`
  3. Attach tarballs to the GitHub Release via `softprops/action-gh-release`

### Updates

- **Full update:** Re-run `curl -sL .../install.sh | bash` — pulls latest repo changes, downloads latest binary, launches TUI.
- **Quick re-run:** Execute `~/.dotfiles/.bin/dotfiles` directly — skips repo/binary update, useful for changing theme or adding modules.
- **Version display:** Binary prints version from `Cargo.toml` (compiled in at build time) in the welcome header.

## 6. Theme System

### Overview

The theme system implements a semantic translation layer between native theme color definitions and tool-specific configuration formats. Each theme variant is defined as a TOML palette file conforming to a strict JSON Schema contract. The Rust engine validates palettes at discovery time and passes them as Tera template context at render time. Templates consume the contract blindly — they never reference theme-specific color names.

### Palette Schema

Every palette TOML file must implement this exact structure:

```toml
# themes/palettes/catppuccin-mocha.toml

[meta]
name = "Catppuccin Mocha"
variant = "dark"                    # "dark" or "light"
nvim_theme = "catppuccin-mocha"     # String for vim.cmd.colorscheme()
nvim_plugin = "catppuccin"          # Lua require() target
nvim_variant = "mocha"              # Plugin-specific variant identifier

[colors.core]
background = "#1e1e2e"
foreground = "#cdd6f4"
cursor_bg = "#f5e0dc"
cursor_fg = "#1e1e2e"
selection_bg = "#585b70"
selection_fg = "#cdd6f4"
url = "#89b4fa"

[colors.ansi.normal]
black = "#45475a"
red = "#f38ba8"
green = "#a6e3a1"
yellow = "#f9e2af"
blue = "#89b4fa"
magenta = "#f5c2e7"
cyan = "#94e2d5"
white = "#bac2de"

[colors.ansi.bright]
black = "#585b70"
red = "#f38ba8"
green = "#a6e3a1"
yellow = "#f9e2af"
blue = "#89b4fa"
magenta = "#f5c2e7"
cyan = "#94e2d5"
white = "#a6adc8"

[colors.ui]
border_active = "#cba6f7"
border_inactive = "#6c7086"
status_bg = "#11111b"
status_fg = "#cdd6f4"
tab_active_bg = "#cba6f7"
tab_active_fg = "#11111b"
tab_inactive_bg = "#181825"
tab_inactive_fg = "#bac2de"
```

**Total: 31 color keys + 5 meta keys.** Structured in three semantic groups:

| Group | Keys | Purpose |
|-------|------|---------|
| `colors.core` | 7 | Terminal base colors: background, foreground, cursor, selection, URLs |
| `colors.ansi.normal` + `colors.ansi.bright` | 16 | Standard ANSI terminal colors (0-15) |
| `colors.ui` | 8 | Multiplexer/shell UI: borders, status bar, tabs |
| `meta` | 5 | Theme identity and NeoVim plugin configuration |

### Schema Validation (`themes/schema.json`)

All palette TOML files are validated against a JSON Schema during the Discovery phase. Invalid palettes are rejected with a clear error before any TUI prompts appear.

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["meta", "colors"],
  "properties": {
    "meta": {
      "type": "object",
      "required": ["name", "variant", "nvim_theme", "nvim_plugin", "nvim_variant"],
      "properties": {
        "name": { "type": "string" },
        "variant": { "type": "string", "enum": ["dark", "light"] },
        "nvim_theme": { "type": "string" },
        "nvim_plugin": { "type": "string" },
        "nvim_variant": { "type": "string" }
      }
    },
    "colors": {
      "type": "object",
      "required": ["core", "ansi", "ui"],
      "properties": {
        "core": {
          "type": "object",
          "required": ["background", "foreground", "cursor_bg", "cursor_fg", "selection_bg", "selection_fg", "url"]
        },
        "ansi": {
          "type": "object",
          "required": ["normal", "bright"],
          "properties": {
            "normal": { "$ref": "#/definitions/ansi_colors" },
            "bright": { "$ref": "#/definitions/ansi_colors" }
          }
        },
        "ui": {
          "type": "object",
          "required": ["border_active", "border_inactive", "status_bg", "status_fg", "tab_active_bg", "tab_active_fg", "tab_inactive_bg", "tab_inactive_fg"]
        }
      }
    }
  },
  "definitions": {
    "ansi_colors": {
      "type": "object",
      "required": ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"],
      "patternProperties": {
        "^.*$": { "type": "string", "pattern": "^#[0-9a-fA-F]{6}$" }
      }
    }
  }
}
```

### Tool Template Examples

Templates consume the palette contract via Tera variables. Each tool gets its own `.tera` template.

#### Kitty (`modules/kitty/config/kitty/kitty.conf.tera`)

```text
foreground {{ colors.core.foreground }}
background {{ colors.core.background }}
selection_foreground {{ colors.core.selection_fg }}
selection_background {{ colors.core.selection_bg }}
url_color {{ colors.core.url }}
cursor {{ colors.core.cursor_bg }}
cursor_text_color {{ colors.core.cursor_fg }}

color0  {{ colors.ansi.normal.black }}
color1  {{ colors.ansi.normal.red }}
color2  {{ colors.ansi.normal.green }}
color3  {{ colors.ansi.normal.yellow }}
color4  {{ colors.ansi.normal.blue }}
color5  {{ colors.ansi.normal.magenta }}
color6  {{ colors.ansi.normal.cyan }}
color7  {{ colors.ansi.normal.white }}
color8  {{ colors.ansi.bright.black }}
color9  {{ colors.ansi.bright.red }}
color10 {{ colors.ansi.bright.green }}
color11 {{ colors.ansi.bright.yellow }}
color12 {{ colors.ansi.bright.blue }}
color13 {{ colors.ansi.bright.magenta }}
color14 {{ colors.ansi.bright.cyan }}
color15 {{ colors.ansi.bright.white }}
```

#### Alacritty (`modules/alacritty/config/alacritty/alacritty.toml.tera`)

```toml
[colors.primary]
background = "{{ colors.core.background }}"
foreground = "{{ colors.core.foreground }}"

[colors.cursor]
cursor = "{{ colors.core.cursor_bg }}"
text = "{{ colors.core.cursor_fg }}"

[colors.selection]
background = "{{ colors.core.selection_bg }}"
text = "{{ colors.core.selection_fg }}"

[colors.normal]
black   = "{{ colors.ansi.normal.black }}"
red     = "{{ colors.ansi.normal.red }}"
green   = "{{ colors.ansi.normal.green }}"
yellow  = "{{ colors.ansi.normal.yellow }}"
blue    = "{{ colors.ansi.normal.blue }}"
magenta = "{{ colors.ansi.normal.magenta }}"
cyan    = "{{ colors.ansi.normal.cyan }}"
white   = "{{ colors.ansi.normal.white }}"

[colors.bright]
black   = "{{ colors.ansi.bright.black }}"
red     = "{{ colors.ansi.bright.red }}"
green   = "{{ colors.ansi.bright.green }}"
yellow  = "{{ colors.ansi.bright.yellow }}"
blue    = "{{ colors.ansi.bright.blue }}"
magenta = "{{ colors.ansi.bright.magenta }}"
cyan    = "{{ colors.ansi.bright.cyan }}"
white   = "{{ colors.ansi.bright.white }}"
```

#### Zellij (`modules/zellij/config/zellij/config.kdl.tera`)

```kdl
themes {
    dotfiles {
        fg "{{ colors.core.foreground }}"
        bg "{{ colors.ui.status_bg }}"
        black "{{ colors.ansi.normal.black }}"
        red "{{ colors.ansi.normal.red }}"
        green "{{ colors.ansi.normal.green }}"
        yellow "{{ colors.ansi.normal.yellow }}"
        blue "{{ colors.ansi.normal.blue }}"
        magenta "{{ colors.ansi.normal.magenta }}"
        cyan "{{ colors.ansi.normal.cyan }}"
        white "{{ colors.ansi.normal.white }}"
        orange "{{ colors.ansi.normal.yellow }}"
    }
}
theme "dotfiles"
```

#### tmux (`modules/tmux/home/.tmux.conf.tera`)

```text
set -g status-style "bg={{ colors.ui.status_bg }},fg={{ colors.ui.status_fg }}"
set -g pane-active-border-style "fg={{ colors.ui.border_active }}"
set -g pane-border-style "fg={{ colors.ui.border_inactive }}"
set -g window-status-current-style "bg={{ colors.ui.tab_active_bg }},fg={{ colors.ui.tab_active_fg }}"
set -g window-status-style "bg={{ colors.ui.tab_inactive_bg }},fg={{ colors.ui.tab_inactive_fg }}"
```

### NeoVim Strategy

NeoVim is handled differently from all other tools. Instead of injecting hex color values into the configuration, we leverage the NeoVim plugin ecosystem. Colorscheme plugins (catppuccin.nvim, rose-pine.nvim, kanagawa.nvim) define hundreds of highlight groups internally — syntax, tree-sitter, LSP diagnostics, git signs, telescope, and more. Replicating this via Tera templates would be impractical and fragile.

Instead, the Tera template generates a Lua bridge file that tells NeoVim which plugin to load and how to configure its variant:

#### NeoVim (`modules/nvim/config/nvim/lua/core/theme.lua.tera`)

```lua
local plugin = "{{ meta.nvim_plugin }}"
local variant = "{{ meta.nvim_variant }}"
local theme_name = "{{ meta.nvim_theme }}"

if plugin == "catppuccin" then
    require("catppuccin").setup({ flavour = variant })
    vim.cmd.colorscheme(theme_name)
elseif plugin == "rose-pine" then
    require("rose-pine").setup({ variant = variant })
    vim.cmd.colorscheme(theme_name)
elseif plugin == "kanagawa" then
    require("kanagawa").setup({ theme = variant })
    vim.cmd.colorscheme(theme_name)
else
    vim.cmd.colorscheme(theme_name)
end
```

The `[meta]` section of each palette TOML provides the three values needed:
- `nvim_plugin` — the Lua module to `require()` and configure
- `nvim_variant` — the plugin-specific variant identifier passed to `setup()`
- `nvim_theme` — the string passed to `vim.cmd.colorscheme()`

Each plugin has a different setup API, which is why the bridge file uses conditional logic. The fallback branch handles any future theme whose plugin uses the standard `vim.cmd.colorscheme()` call without a setup step.

### Workflow: Adding a New Theme

To add a theme (e.g., Dracula), a maintainer:

1. Look up the official hex color values from the theme's documentation.
2. Create `themes/palettes/dracula.toml`.
3. Fill in all 31 color keys mapping the theme's native colors to the semantic schema (e.g., Dracula's `#282a36` → `colors.core.background`).
4. Fill in the 5 `[meta]` fields including the NeoVim plugin identifiers.
5. Run the installer — the scanner validates the TOML against `schema.json`. If a key is missing, validation fails with a clear error: `Invalid palette dracula.toml: missing key 'colors.core.url'`.
6. If validation passes, "Dracula" appears in the TUI theme selector automatically.

No template changes required. No Rust code changes required.

