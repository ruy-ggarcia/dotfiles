# ADR-001: Programming Language for the Dotfiles CLI

- **Status:** Accepted
- **Date:** 2026-03-16
- **Deciders:** Dotfiles project team

---

## Context

The dotfiles project requires a CLI installer that:

1. Distributes as a **single static binary** — no runtime or toolchain required on the target machine.
2. Provides an **interactive TUI** (multi-select, single-select, confirmation prompts) for the installation wizard.
3. Renders **tool configuration files** from Tera templates and palette definitions.
4. Runs on **MacOS (x86_64, aarch64) and Linux (x86_64)** without modification.

Five languages were evaluated against these requirements.

---

## Options Considered

### Option A — Rust

Rust compiles to a fully static native binary. The ecosystem includes `inquire` for interactive TUI prompts and `tera` for Jinja2-style template rendering — both requirements are first-class.

**Pros:**
- Produces a single static binary with zero runtime dependencies (fulfills P0 requirement).
- `inquire` crate: multi-select, single-select, and confirmation prompts — exactly the wizard pattern needed.
- `tera` crate: full Jinja2-compatible templating already used in the design.
- Strong type system and memory safety reduce runtime errors in a tool that writes to the user's `$HOME`.
- The team has established Rust experience — idiomatic code from day one.

**Cons:**
- Cross-compilation (e.g., targeting `x86_64-unknown-linux-musl` from MacOS) requires additional tooling (`cross`, Docker, or GitHub Actions matrix).
- Longer compile times compared to Go — relevant for CI pipelines.
- Higher barrier to entry for external contributors unfamiliar with Rust.

**Verdict:** ✅ Recommended.

---

### Option B — Go

Go compiles to a single binary and has excellent built-in cross-compilation via `GOOS`/`GOARCH` environment variables. The Charm ecosystem (`bubbletea`, `lipgloss`) provides rich TUI capabilities.

**Pros:**
- Trivial cross-compilation: `GOOS=linux GOARCH=amd64 go build` with no external tools.
- Simple, readable syntax — lower barrier for contributors.
- Charm ecosystem (`bubbletea`, `huh`) well-suited for interactive CLIs.
- Fast compile times.

**Cons:**
- **No team Go experience.** Learning a new language while building the project introduces unacceptable timeline and code quality risk.
- Binaries include the Go runtime — typically 8–15 MB vs. Rust's leaner output (though still acceptable for distribution).
- Garbage collector introduces non-deterministic latency (minor concern for a CLI installer).
- Template rendering (`text/template`, `raymond`) less expressive than Tera.

**Verdict:** ❌ Rejected. Strong language, but zero team experience makes it a liability at this project stage.

---

### Option C — Python

Python is widely available and has excellent CLI libraries (`click`, `rich`, `questionary`).

**Pros:**
- Rapid development cycle.
- Rich ecosystem for CLI and TUI.
- High contributor familiarity.

**Cons:**
- **Requires a Python runtime on the target machine.** Violates the P0 requirement of zero external dependencies.
- Packaging as a self-contained binary (`PyInstaller`, `Nuitka`) produces large, fragile artifacts.
- Version incompatibilities between Python 3.x releases introduce portability issues.

**Verdict:** ❌ Rejected. Cannot satisfy the single static binary requirement without significant packaging complexity and binary size overhead.

---

### Option D — Bash

Bash scripts are universally available on target systems and require no compilation.

**Pros:**
- No binary to distribute or compile — scripts run directly.
- Zero dependencies on any target Unix-like system.

**Cons:**
- **Cannot deliver the TUI requirement.** Interactive multi-select prompts in pure Bash require `dialog` or `whiptail`, which are not universally available and produce inconsistent UIs.
- No type system — large Bash programs become unmaintainable quickly.
- Complex string manipulation and JSON/TOML parsing are painful and error-prone.
- No path to cross-platform Windows support.

**Verdict:** ❌ Rejected. TUI requirement alone eliminates Bash as a viable option.

---

### Option E — TypeScript (Deno)

Deno bundles TypeScript scripts into a single executable and provides a built-in standard library.

**Pros:**
- Familiar syntax for frontend-oriented contributors.
- `deno compile` produces a standalone binary.
- Good async I/O primitives.

**Cons:**
- `deno compile` bundles the V8 engine — binary sizes typically exceed 80 MB. Unacceptable for a bootstrap script download.
- The Deno TUI ecosystem is immature compared to Rust or Go.
- Less ergonomic for file system operations and process management than native languages.

**Verdict:** ❌ Rejected. Binary size alone makes distribution via `curl | bash` impractical.

---

## Decision Outcome

**Chosen Option: A — Rust**

Decided on 2026-03-16. The team chose Rust over Go because:
- The team has established Rust experience but no Go experience. Learning a new language while building a core project introduces unacceptable timeline risk.
- Rust fully satisfies all PRD technical requirements: static binary distribution, interactive TUI (`inquire`), template rendering (`tera`), and cross-platform support.
- Cross-compilation friction (compared to Go's native support) will be mitigated through a well-configured GitHub Actions CI pipeline.
- Higher barrier for external contributors will be mitigated with contributing documentation and labeled good-first-issue tickets.
