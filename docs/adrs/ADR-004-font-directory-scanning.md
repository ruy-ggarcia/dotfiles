# ADR-004: Font Directory Scanning Strategy

- **Status:** Accepted
- **Date:** 2026-03-30
- **Deciders:** Dotfiles project team

---

## Context

`font::scan_fonts(dirs)` uses `std::fs::read_dir` which lists only the immediate children of a directory. On macOS this is sufficient — Nerd Fonts installed by the user land directly in `~/Library/Fonts/`:

```
~/Library/Fonts/
  JetBrainsMonoNerdFont-Regular.ttf   ← depth 1, detected ✓
  JetBrainsMonoNerdFont-Bold.ttf      ← depth 1, detected ✓
```

On Linux, system fonts are organized in subdirectory trees by family or source:

```
/usr/share/fonts/
  truetype/
    dejavu/
      DejaVuSans.ttf                  ← depth 3, NOT detected ✗
  opentype/
    ...
~/.local/share/fonts/
  JetBrainsMonoNerdFont-Regular.ttf   ← depth 1, detected ✓ (manual install)
```

In practice, users who install Nerd Fonts manually (the target audience of this tool) will typically copy files directly into `~/.local/share/fonts`, where they ARE detected. However, fonts installed via `apt` or another package manager land under `/usr/share/fonts/` in nested subdirectories, and are currently invisible to the scanner.

The `walkdir` crate is already available in the project and is the standard solution for recursive directory traversal in Rust.

---

## Decision Drivers

- **Correctness:** Users with apt-installed fonts should see them in the TUI selection.
- **Simplicity:** The current flat scan is simple and correct for the primary use case (manual install).
- **Zero side-effects:** Font scanning must remain read-only.
- **Performance:** Font directories can contain thousands of files; recursive scan must not be noticeably slow.

---

## Options Considered

### Option 1: Keep flat scan (current behavior)

`read_dir` on each configured directory. Depth 1 only.

**Pros:** Simple, fast, correct for manually installed fonts (`~/.local/share/fonts`).
**Cons:** Misses fonts installed via `apt` in `/usr/share/fonts/**/*.ttf`.

### Option 2: Recursive scan via `walkdir`

Replace `read_dir` with `WalkDir::new(dir)` in `font::scan_fonts`.

**Pros:** Detects all installed fonts regardless of depth. Correct on both platforms.
**Cons:** Slightly more complex. Scanning `/usr/share/fonts` recursively on a well-provisioned system could touch hundreds of files, but is still fast in practice.

### Option 3: Configurable depth

Parametrize scan depth per directory (depth 1 for macOS paths, recursive for Linux paths).

**Pros:** Optimal performance per platform.
**Cons:** Over-engineered. Adds complexity without meaningful benefit — recursive scan on font directories is fast enough.

---

## Decision

**Option 2: Recursive scan via `walkdir`.**

`scan_fonts` now uses `WalkDir::new(dir)` instead of `std::fs::read_dir`, detecting Nerd Fonts at any depth under each configured directory. `walkdir` was already a project dependency.

---

## Consequences

All Nerd Fonts installed on the system — whether manually placed or installed via a package manager — are detected regardless of subdirectory depth. The scan implementation is marginally more complex but `walkdir` handles missing directories and permission errors gracefully via `.into_iter().flatten()`.
