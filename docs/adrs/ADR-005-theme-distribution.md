# ADR-005: Theme Distribution Strategy

- **Status:** Accepted
- **Date:** 2026-03-30
- **Deciders:** Dotfiles project team

---

## Context

The dotfiles installer distributes theme palettes as TOML files in the source repository. A pre-compiled binary has no access to the source tree at runtime. Theme TOML files must be available when the binary runs.

Two options were considered:

### Option 1: Pure in-memory embedding

Embed theme TOML strings via `include_str!` and parse them entirely in memory. The binary never writes theme files to disk.

**Pros:** Binary is fully self-contained. No disk writes for themes.
**Cons:** Users cannot extend the theme system without contributing to the repository and cutting a new release. The theme directory becomes an internal detail invisible to users.

### Option 2: Seed to disk with defaults/custom split (CHOSEN)

Embed theme TOML strings via `include_str!`. On each startup, write them to `~/.config/dotfiles/themes/defaults/`. Also scan `~/.config/dotfiles/themes/custom/` for user-provided palettes. Custom themes take precedence over defaults when names collide.

**Pros:**
- Users can add personal themes by dropping a TOML file in `~/.config/dotfiles/themes/custom/` — no fork, no PR, no new release required.
- Default themes are always up to date (re-seeded on each run).
- The `custom/` directory is clearly separated — never touched by the tool.
- Enables PRD Story 5 (Custom Theme) with no additional complexity.

**Cons:** The binary writes files to disk on startup (beyond config files). This is a minor relaxation of the zero-side-effects model, but the writes are to the user's own home directory and are deterministic.

---

## Decision

**Option 2.** Seed default themes to `~/.config/dotfiles/themes/defaults/` on each run. Scan both `defaults/` and `custom/`. Custom takes precedence on name collision.

---

## Consequences

- Users can author custom themes by placing valid TOML palette files in `~/.config/dotfiles/themes/custom/`.
- Default themes are always in sync with the installed binary version — no stale palette files.
- The `~/.config/dotfiles/themes/` directory is documented as the theme extension point.
