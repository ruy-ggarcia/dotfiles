# Contributing

This project follows **convention over configuration**: the Rust engine auto-discovers modules and themes at runtime. The two most common contributions — adding a tool module and adding a theme palette — require **zero changes to Rust code**.

## Adding a new module

A module is a directory under `modules/` that bundles a tool's package name and its configuration files.

### 1. Create the directory

```
modules/<tool-name>/
```

### 2. Add `packages.toml`

Declare the package name for each supported OS:

```toml
[packages]
macos = ["tool-name"]
ubuntu = ["tool-name"]
```

Both keys are required. Use the exact package name as the OS package manager expects it (Homebrew on macOS, APT on Ubuntu).

### 3. Add configuration files

Place files under `home/` or `config/` depending on where they should be symlinked:

| Directory | Symlinked to |
|-----------|-------------|
| `modules/<tool>/home/` | `~/` |
| `modules/<tool>/config/` | `~/.config/` |

The path structure inside these directories mirrors the target. For example:

- `modules/starship/config/starship/starship.toml` → `~/.config/starship/starship.toml`
- `modules/foo/home/.foorc` → `~/.foorc`

### 4. Add theme support (optional)

If the tool supports colors, add a `.tera` extension to the config file:

```
modules/starship/config/starship/starship.toml.tera
```

The `.tera` extension tells the engine to render the file using the active theme palette before symlinking. The `.tera` extension is stripped in the rendered output.

Reference palette variables using Tera syntax:

```toml
# example: starship.toml.tera
[palette.starship]
red = "{{ colors.ansi.normal.red }}"
green = "{{ colors.ansi.normal.green }}"
```

Available palette variables are defined in `themes/schema.json`. The full structure:

- `{{ meta.name }}`, `{{ meta.variant }}`, `{{ meta.nvim_theme }}`, `{{ meta.nvim_plugin }}`, `{{ meta.nvim_variant }}`
- `{{ colors.core.background }}`, `{{ colors.core.foreground }}`, `{{ colors.core.cursor_bg }}`, `{{ colors.core.cursor_fg }}`, `{{ colors.core.selection_bg }}`, `{{ colors.core.selection_fg }}`, `{{ colors.core.url }}`
- `{{ colors.ansi.normal.{black,red,green,yellow,blue,magenta,cyan,white} }}`
- `{{ colors.ansi.bright.{black,red,green,yellow,blue,magenta,cyan,white} }}`
- `{{ colors.ui.border_active }}`, `{{ colors.ui.border_inactive }}`, `{{ colors.ui.status_bg }}`, `{{ colors.ui.status_fg }}`, `{{ colors.ui.tab_active_bg }}`, `{{ colors.ui.tab_active_fg }}`, `{{ colors.ui.tab_inactive_bg }}`, `{{ colors.ui.tab_inactive_fg }}`

### 5. Done

Commit and the engine will auto-discover the module on the next run. No Rust code changes required.

---

## Adding a new theme palette

A theme palette is a single TOML file in `themes/palettes/` that maps a theme's native colors to the semantic schema.

### 1. Review the schema

Open `themes/schema.json` and note all required keys. There are 36 total: 5 `[meta]` keys and 31 color keys.

### 2. Create the palette file

```
themes/palettes/<name>.toml
```

Use lowercase and hyphens for the filename — it becomes the display name in the TUI (e.g., `dracula.toml` → "dracula").

### 3. Fill in all required keys

```toml
[meta]
name = "Dracula"
variant = "dark"            # "dark" or "light"
nvim_theme = "dracula"      # passed to vim.cmd.colorscheme()
nvim_plugin = "dracula"     # Lua require() target
nvim_variant = ""           # plugin-specific variant (empty string if unused)

[colors.core]
background   = "#282a36"
foreground   = "#f8f8f2"
cursor_bg    = "#f8f8f2"
cursor_fg    = "#282a36"
selection_bg = "#44475a"
selection_fg = "#f8f8f2"
url          = "#8be9fd"

[colors.ansi.normal]
black   = "#21222c"
red     = "#ff5555"
green   = "#50fa7b"
yellow  = "#f1fa8c"
blue    = "#bd93f9"
magenta = "#ff79c6"
cyan    = "#8be9fd"
white   = "#f8f8f2"

[colors.ansi.bright]
black   = "#6272a4"
red     = "#ff6e6e"
green   = "#69ff94"
yellow  = "#ffffa5"
blue    = "#d6acff"
magenta = "#ff92df"
cyan    = "#a4ffff"
white   = "#ffffff"

[colors.ui]
border_active    = "#bd93f9"
border_inactive  = "#6272a4"
status_bg        = "#21222c"
status_fg        = "#f8f8f2"
tab_active_bg    = "#bd93f9"
tab_active_fg    = "#282a36"
tab_inactive_bg  = "#282a36"
tab_inactive_fg  = "#6272a4"
```

All color values must be 6-digit hex strings (`#rrggbb`).

### 4. Validate

```
cd installer
cargo test
```

The test suite validates all palettes in `themes/palettes/` against the schema. If a key is missing or a color value is malformed, the test will fail with a clear error.

### 5. Done

Commit and the engine will auto-discover the palette on the next run. No Rust code changes required.

---

## Development setup

```bash
git clone git@github.com:<you>/dotfiles.git
cd dotfiles/installer
cargo build
```

### Run tests

```bash
cargo test
```

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy -- -D warnings
```

---

## Commit conventions

This project uses [Conventional Commits](https://www.conventionalcommits.org/).

```
type: short description in imperative mood

Explain WHY this change is needed. The diff shows what changed —
the commit body explains the reasoning behind it.
```

Valid types: `feat`, `fix`, `docs`, `test`, `chore`, `refactor`, `ci`, `cd`, `build`

Rules:
- No scope in parentheses — use `feat:` not `feat(tui):`
- Body is not optional for anything non-trivial — explain the intent

---

## CI

All pull requests must pass three checks, run against the `installer/` directory:

| Check | Command |
|-------|---------|
| Formatting | `cargo fmt --check` |
| Linting | `cargo clippy -- -D warnings` |
| Tests | `cargo test` |

See `.github/workflows/build.yml` for the full workflow definition.
