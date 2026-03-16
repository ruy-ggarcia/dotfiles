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
