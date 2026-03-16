# Architecture Decisions Log

Summary of key architecture decisions made for the Dotfiles project.

## Decisions

### D1: Programming Language — Rust

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated Rust, Go, Python, Bash, and TypeScript (Deno) for the CLI installer. Python and Bash were eliminated early (Python requires a runtime on target; Bash cannot deliver the TUI requirement). TypeScript/Deno produces oversized binaries. The final evaluation was between Rust and Go. See [ADR-001](adr/ADR-001-programming-language.md) for the full analysis.
- **Decision:** Use Rust as the programming language for the entire project.
- **Rationale:** The team has established Rust experience but no Go experience. While Go offers advantages in cross-compilation simplicity and TUI ecosystem (Charm), learning a new language during project development introduces unacceptable timeline and code quality risk. Rust satisfies all PRD requirements and ensures idiomatic code from day one.

### D2: Custom Engine (No Existing Dotfiles Manager)

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated chezmoi, GNU Stow, yadm, bare git repo, and custom scripts against PRD requirements (single-command bootstrap, interactive TUI, global theme switching, zero external dependencies).
- **Decision:** Build a custom installer from scratch.
- **Rationale:** Wrapping an existing tool like chezmoi introduces an external dependency, complicating the "single-command, no-toolchain" bootstrap requirement. A standalone Rust binary can natively handle symlinking, templating, and TUI in one zero-dependency package.

### D3: TUI Framework — `inquire`

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated ratatui, cursive, inquire, and dialoguer for building the installation wizard.
- **Decision:** Use `inquire` for the interactive TUI.
- **Rationale:** The installer is a linear, multi-step wizard — not a persistent dashboard. `inquire` provides multi-select, single-select, and confirmation prompts with minimal complexity. `ratatui` would be overkill for this use case and significantly increase maintenance burden.

### D4: Theme System — Custom Schema + Tera Templates

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated Base16, Base24, and a custom schema for defining theme palettes. See [ADR-002](adr/ADR-002-theme-palette-standard.md) for the full analysis.
- **Decision:** Define a custom JSON/TOML palette schema and render tool configurations via Tera templates.
- **Rationale:** Base16's 16-color limit cannot represent Catppuccin (26 colors) or Kanagawa (36+ colors) without visible degradation. Custom schema maps 1:1 to native theme definitions and integrates naturally with Tera template rendering.

### D5: Font Installation — Direct Download

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated native package managers vs. direct download for Nerd Fonts installation on MacOS and Linux.
- **Decision:** Download fonts directly from the Nerd Fonts GitHub Releases API.
- **Rationale:** Linux package managers are inconsistent with font availability. Direct download to `~/Library/Fonts` (MacOS) or `~/.local/share/fonts` (Linux) with `fc-cache` guarantees version consistency across all platforms.

### D6: Package Manager Abstraction — Trait-based

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Needed a way to abstract package installation across MacOS (Homebrew) and Linux (APT for Ubuntu as P0).
- **Decision:** Implement a Rust `PackageManager` trait with per-distro implementations and a TOML/YAML mapping of abstract tool names to distro-specific package names.
- **Rationale:** No widely adopted Rust crate exists for this. A simple trait abstraction with a static mapping file is lightweight, extensible to new distros, and keeps the dependency count at zero.

### D7: Distribution — Pre-compiled Binary + Bootstrap Script

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** PRD requires "no development toolchains on target machine." Evaluated pre-compiled binaries, source compilation, and container-based approaches.
- **Decision:** Distribute pre-compiled binaries via GitHub Releases. Provide a `curl | bash` bootstrap script that detects OS/arch and downloads the correct binary.
- **Rationale:** Industry standard pattern (used by Rustup, Starship, Bun). Zero dependencies on the target machine. GitHub Actions handles cross-compilation for MacOS (x86_64, aarch64) and Linux (x86_64).

### D8: Configuration Drift — Backup Strategy

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** When re-running the installer, existing config files may have been manually edited by the user.
- **Decision:** Back up existing files to `{filename}.bak.{timestamp}` before overwriting.
- **Rationale:** Non-destructive approach. Users retain their modifications and can manually merge changes back if needed.

### D9: Linux Distribution Scope — Ubuntu (P0)

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** The PRD targets "Linux" broadly. Needed to scope which distributions are P0 for v1.
- **Decision:** Ubuntu is the only P0 Linux distribution. Other distributions (Arch, Fedora) may be added later.
- **Rationale:** Pragmatic scoping. Ubuntu covers the most common server and desktop Linux use case. The trait-based package manager abstraction makes adding new distros straightforward in future versions.
