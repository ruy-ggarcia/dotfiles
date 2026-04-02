# ADR-006: Cross-Compilation Tooling

- **Status:** Accepted
- **Date:** 2026-03-30
- **Deciders:** Dotfiles project team

---

## Context

The release pipeline must produce binaries for three targets from GitHub Actions runners:

| Target | Runner | Challenge |
|--------|--------|-----------|
| `aarch64-apple-darwin` | `macos-latest` | Native — no cross-compilation needed |
| `x86_64-apple-darwin` | `macos-latest` | Cross-compile on M-series runner |
| `x86_64-unknown-linux-musl` | `ubuntu-latest` | MUSL static binary — requires toolchain |

The Linux MUSL target is the critical case. It requires a cross-compilation toolchain since the standard `cargo build` uses glibc by default.

Two main options were evaluated:

### Option 1: `cross`

Docker-based cross-compilation wrapper. Runs cargo inside a pre-configured container with the musl toolchain.

**Pros:** Mature, well-tested, handles complex C FFI dependencies automatically.
**Cons:** Requires Docker daemon in CI. GitHub Actions ubuntu runners do not have Docker by default on all configurations. Slower cold start due to image pull. Overkill for a pure-Rust project with no C dependencies.

### Option 2: `cargo-zigbuild` (CHOSEN)

Uses Zig as a cross-linker. No Docker required. Zig installs in seconds via `pip install ziglang`.

**Pros:** No Docker dependency. Fast install in CI. Works cleanly in GitHub Actions ubuntu runners. The project has no C FFI — zigbuild's main limitation (complex C dependencies) does not apply.
**Cons:** Less battle-tested than `cross` for complex projects. Requires `pip` to be available (it is on all GitHub Actions runners).

---

## Decision

**`cargo-zigbuild`** for the `x86_64-unknown-linux-musl` target. macOS targets use native `cargo build` with `rustup target add`.

---

## Consequences

- No Docker dependency in CI — simpler, faster builds.
- If C dependencies are added in the future, this choice should be re-evaluated.
- Zig version is pinned via `mlugg/setup-zig@v2` with `version: 0.15.2` in the release workflow.
