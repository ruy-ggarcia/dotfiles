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
