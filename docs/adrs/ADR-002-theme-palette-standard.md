# ADR-002: Theme Palette Standard

- **Status:** Accepted
- **Date:** 2026-03-16
- **Deciders:** Dotfiles project team

---

## Context

The dotfiles project needs a consistent, machine-readable way to define color palettes for supported themes (Catppuccin, Rosé Pine, Kanagawa) and apply them across all managed tool configurations: Kitty, Alacritty, Zellij, Tmux, NeoVim, and Zsh.

The core requirement is **visual fidelity** — the rendered configuration must match the original theme design exactly, without color compression or approximation.

Three approaches were evaluated:

---

## Options Considered

### Option A — Base16

**Base16** is a widely adopted architecture for building themes using a palette of exactly 16 colors (base00–base0F). A large ecosystem of pre-built schemes and templates exists for most terminal tools.

**Pros:**
- Large community with hundreds of pre-built schemes.
- Templates already exist for most tools (Kitty, Alacritty, Tmux, NeoVim, etc.).
- Simple, well-documented format.

**Cons:**
- Hard limit of 16 colors. Catppuccin uses 26 colors, Rosé Pine uses 15 (fits, but lossy), and Kanagawa uses 36+.
- Mapping a 26- or 36-color theme into 16 slots requires color approximation, visibly degrading the UI.
- Some Base16 NeoVim plugins remap colors in ways that break syntax highlighting for modern colorschemes.

**Verdict:** ❌ Rejected. The 16-color limit degrades Catppuccin and Kanagawa to the point of being unusable.

---

### Option B — Base24

**Base24** extends Base16 with 8 additional slots (base10–base17), providing 24 named color roles. It was designed specifically to address Base16's limitations for modern themes.

**Pros:**
- More color slots than Base16 (24 vs 16).
- Extends the existing Base16 ecosystem conceptually.

**Cons:**
- Minimal community adoption — very few pre-built schemes exist for our target themes.
- No pre-built Base24 schemes found for Catppuccin, Rosé Pine, or Kanagawa.
- Template ecosystem is nearly non-existent (we would still need to write all 6 tool templates).
- 24 slots still insufficient for Kanagawa (36+ colors).

**Verdict:** ❌ Rejected. Offers no practical advantage over Base16 for our use case — we would still write all templates and would still lack color fidelity for Kanagawa.

---

### Option C — Custom JSON/TOML Schema

Define a project-specific palette schema in JSON or TOML where each color key maps directly to the original theme's named color roles. Use **Tera** (the Rust templating engine already used by the installer) to render tool-specific configuration files from these palettes.

**Example palette structure (TOML):**

```toml
[palette]
# Catppuccin Mocha — native names preserved
rosewater  = "#f5e0dc"
flamingo   = "#f2cdcd"
pink       = "#f5c2e7"
mauve      = "#cba6f7"
red        = "#f38ba8"
maroon     = "#eba0ac"
peach      = "#fab387"
yellow     = "#f9e2af"
green      = "#a6e3a1"
teal       = "#94e2d5"
sky        = "#89dceb"
sapphire   = "#74c7ec"
blue       = "#89b4fa"
lavender   = "#b4befe"
text       = "#cdd6f4"
subtext1   = "#bac2de"
subtext0   = "#a6adc8"
overlay2   = "#9399b2"
overlay1   = "#7f849c"
overlay0   = "#6c7086"
surface2   = "#585b70"
surface1   = "#45475a"
surface0   = "#313244"
base       = "#1e1e2e"
mantle     = "#181825"
crust      = "#11111b"
```

**Pros:**
- **Full visual fidelity:** 1:1 mapping to the original theme palette. No color compression or approximation.
- **Supports any theme size:** Catppuccin (26), Rosé Pine (15), Kanagawa (36+) — all represented exactly.
- **Native integration with Tera:** Dotfiles already uses Tera for templating. Palette keys become template variables directly.
- **Explicit and readable:** Color roles are named as the theme author intended, making templates self-documenting.
- **Full control:** We own the schema and can extend it as needed (e.g., adding semantic aliases).

**Cons:**
- We must write all 6 tool templates ourselves (Kitty, Alacritty, Zellij, Tmux, NeoVim, Zsh). This is a one-time cost.
- No shared ecosystem to pull from — every template is maintained by this project.

**Verdict:** ✅ Recommended.

---

## Decision Outcome

**Chosen Option: C — Custom JSON/TOML Schema**

Decided on 2026-03-16. The team chose Custom Schema over Base16 and Base24 because:
- Catppuccin (26 colors), Rosé Pine (15 colors), and Kanagawa (36+ colors) cannot be faithfully represented in Base16's 16-color limit.
- Base24 lacks community adoption and has no pre-built schemes for our target themes.
- Writing 6 Tera templates (Kitty, Alacritty, Zellij, Tmux, NeoVim, Zsh) is a one-time cost in exchange for perfect visual fidelity and full control.
