# Architecture Decisions Log

Summary of key architecture decisions made for the Dotfiles project.

## Decisions

### D1: Programming Language — Rust

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated Rust, Go, Python, Bash, and TypeScript (Deno) for the CLI tool. Python and Bash were eliminated early (Python requires a runtime on target; Bash cannot deliver the TUI requirement). TypeScript/Deno produces oversized binaries. The final evaluation was between Rust and Go. See [ADR-001](adrs/ADR-001-programming-language.md) for the full analysis.
- **Decision:** Use Rust as the programming language for the entire project.
- **Rationale:** The team has established Rust experience but no Go experience. While Go offers advantages in cross-compilation simplicity and TUI ecosystem (Charm), learning a new language during project development introduces unacceptable timeline and code quality risk. Rust satisfies all PRD requirements and ensures idiomatic code from day one.

### D2: Custom Engine (No Existing Dotfiles Manager)

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated chezmoi, GNU Stow, yadm, bare git repo, and custom scripts against PRD requirements (single-command bootstrap, interactive TUI, global theme switching, zero external dependencies).
- **Decision:** Build a custom dotfiles tool from scratch.
- **Rationale:** Wrapping an existing tool like chezmoi introduces an external dependency, complicating the "single-command, no-toolchain" bootstrap requirement. A standalone Rust binary can natively handle symlinking, templating, and TUI in one zero-dependency package.

### D3: TUI Framework — `inquire`

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated ratatui, cursive, inquire, and dialoguer for building the configuration wizard.
- **Decision:** Use `inquire` for the interactive TUI.
- **Rationale:** Dotfiles is a linear, multi-step wizard — not a persistent dashboard. `inquire` provides multi-select, single-select, and confirmation prompts with minimal complexity. `ratatui` would be overkill for this use case and significantly increase maintenance burden.

### D4: Theme System — Custom Schema + Tera Templates

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** Evaluated Base16, Base24, and a custom schema for defining theme palettes. See [ADR-002](adrs/ADR-002-theme-palette-standard.md) for the full analysis.
- **Decision:** Define a custom JSON/TOML palette schema and render tool configurations via Tera templates.
- **Rationale:** Base16's 16-color limit cannot represent Catppuccin (26 colors) or Kanagawa (36+ colors) without visible degradation. Custom schema maps 1:1 to native theme definitions and integrates naturally with Tera template rendering.

### D5: Nerd Font Selection — Detect from System

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** The PRD requires surfacing Nerd Fonts already present on the system for use in terminal configurations. Installing fonts is explicitly a Non-Goal — the user is expected to have their stack installed. Two approaches were considered: (1) download fonts directly from Nerd Fonts GitHub Releases, (2) scan OS font directories for already-installed fonts.
- **Decision:** Scan OS font directories for Nerd Fonts already installed by the user and present them for selection.
- **Rationale:** Aligns with the zero-side-effects model. Dotfiles never modifies the system beyond placing configuration files in the user's home directory. Detection avoids any download logic or elevated privileges. If no Nerd Fonts are found, the font selection step is skipped gracefully.

### D6: Distribution — Pre-compiled Binary + Bootstrap Script

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** PRD requires "no development toolchains on target machine." Evaluated pre-compiled binaries, source compilation, and container-based approaches.
- **Decision:** Distribute pre-compiled binaries via GitHub Releases. Provide a `curl | bash` bootstrap script that detects OS/arch and downloads the correct binary.
- **Rationale:** Industry standard pattern (used by Rustup, Starship, Bun). Zero dependencies on the target machine. GitHub Actions handles cross-compilation for MacOS (x86_64, aarch64) and Linux (x86_64).

### D7: Configuration Drift — Backup Strategy

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** When re-running dotfiles, existing config files may have been manually edited by the user.
- **Decision:** Back up existing files to `{filename}.bak.{timestamp}` before overwriting.
- **Rationale:** Non-destructive approach. Users retain their modifications and can manually merge changes back if needed.

### D8: Linux Distribution Scope — Ubuntu (P0)

- **Date:** 2026-03-16
- **Status:** Accepted
- **Context:** The PRD targets "Linux" broadly. Needed to scope which distributions are P0 for v1.
- **Decision:** Ubuntu is the only P0 Linux distribution. Other distributions (Arch, Fedora) may be added later.
- **Rationale:** Pragmatic scoping. Ubuntu covers the most common server and desktop Linux use case. The trait-based detection approach makes adding new distros straightforward in future versions.

### D9: Shell RC Injection — Source Guard

- **Date:** 2026-03-29
- **Status:** Accepted
- **Context:** The installer previously replaced `~/.zshrc` and `~/.bashrc` entirely via symlinks, destroying any existing user content (aliases, exports, tool initialisers). This violates the zero-side-effects requirement. Three alternatives were evaluated: source guard (append one idempotent source line), managed block (delimited replace), and full ownership with escape hatch. See [ADR-003](adrs/ADR-003-shell-rc-injection.md) for full analysis.
- **Decision:** Use a source guard: render the prompt to `~/.config/dotfiles/rendered/prompt.{zsh,bash}` and append a single guarded `source` line to the user's existing rc file.
- **Rationale:** Non-destructive, idempotent, and consistent with industry-standard patterns (nvm, conda, Starship). The user's rc file is never overwritten — only a single line is appended, and only if it isn't already present.

### D10: Platform Detection — Compile-time `cfg!(target_os)`

- **Date:** 2026-03-30
- **Status:** Accepted
- **Context:** M2 requires platform-specific font directory resolution (macOS vs Linux). Two approaches were considered: (1) `std::env::consts::OS` — runtime string comparison, same binary on all platforms; (2) `cfg!(target_os)` — compile-time boolean, dead branches eliminated by the compiler.
- **Decision:** Use `cfg!(target_os = "linux")` (and the implicit macOS fallback) in `font::font_dirs()`.
- **Rationale:** Compile-time guards are idiomatic Rust for platform branching. Dead code is eliminated at compile time. A typo in the OS string produces a compiler error rather than a silent runtime bug. The cost — platform-specific branches cannot be tested from the other platform — is acceptable since font directory paths are not testable across platforms anyway.
