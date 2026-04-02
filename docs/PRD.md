# Dotfiles — Product Requirements Document

## 1. Executive Summary

The Dotfiles project provides an interactive, zero-side-effect way to configure developer environments across MacOS and Linux. Through a single command, users can apply a consistent theme, font, and configuration across all their installed terminal tools — without dotfiles touching their package manager, installing software, or requiring elevated privileges.

Dotfiles detects which tools are present on the system and offers to configure only those. This makes the tool safe to run on any machine at any time, regardless of what is or isn't installed.

## 2. Goals & Non-Goals

### Goals

- **Interactive TUI:** Guide the user through configuration choices via a terminal-based interface.
- **Cross-platform — MacOS & Linux:** Deliver a consistent experience on both supported operating systems.
- **Zero Side-Effects:** Never install packages, never require elevated privileges, never modify the system beyond placing configuration files in the user's home directory.
- **Tool Detection:** Discover what is actually installed on the machine and offer to configure only that.
- **Global Theme Switching:** Apply a chosen color palette across all supported tools simultaneously via a shared theme system.
- **Nerd Font Selection:** Surface Nerd Fonts already installed on the system for use in terminal configurations.

### Non-Goals

- Installing tools, packages, or fonts — the user is expected to have their stack installed.
- Managing graphical user interface (GUI) applications.
- Supporting the Windows operating system.
- Handling or storing secrets, credentials, or sensitive API keys.

## 3. User Personas

- **User:** Someone who uses dotfiles to configure their existing environment. They have their tools installed; they want consistent, opinionated dotfiles with a theme they choose. They run dotfiles, answer a few questions, and get productive.
- **Maintainer:** Someone who uses dotfiles AND contributes by adding new tool modules, themes, or configuration templates to the repository.

## 4. User Stories

### Story 1: Guided Configuration

- **As a** User,
- **I want** to run a single command that detects my installed tools and opens a configuration menu,
- **So that** I can apply opinionated dotfiles to my existing setup without reading documentation or editing config files by hand.
- _Acceptance Criteria:_
  - Running the bootstrap command opens a TUI.
  - The TUI presents only the tools detected on the current machine.
  - Selections are applied automatically upon confirmation.

### Story 2: Environment Consistency Across Machines

- **As a** User,
- **I want** my terminal environment to feel identical whether I am on my MacOS laptop or a Linux server,
- **So that** my workflow and muscle memory remain uninterrupted regardless of which machine I am on.
- _Acceptance Criteria:_
  - The same tool configurations (shell, terminal, multiplexer, editor) are applied consistently on both platforms when those tools are present.
  - Keybindings and aliases are consistent across platforms.
  - The theme renders identically on both platforms.

### Story 3: Global Theme Management

- **As a** User,
- **I want** to change my color theme globally across all my terminal tools at once,
- **So that** I don't have to manually update individual configuration files for my shell, multiplexer, and editor.
- _Acceptance Criteria:_
  - The TUI offers a selection of themes (Catppuccin, Rosé Pine, Kanagawa variants).
  - Applying a theme updates configurations for all installed and selected tools simultaneously.
  - Theme colors are consistent across tools — the same palette key maps to the same visual role everywhere.

### Story 5: Custom Theme

- **As a** User,
- **I want** to add my own custom theme palette to my local dotfiles setup,
- **So that** I can use a color scheme not included in the defaults without forking the repository or opening a pull request.
- _Acceptance Criteria:_
  - The user can place a valid TOML palette file in `~/.config/dotfiles/themes/custom/`
  - The TUI picks it up and presents it alongside the built-in themes
  - Custom themes are never modified or deleted by dotfiles updates

### Story 4: Safe Re-execution (Idempotency)

- **As a** User,
- **I want** to be able to run dotfiles multiple times — after changing my theme or installing new tools — without breaking my existing setup,
- **So that** dotfiles is a tool I can reach for freely, not one I run once and never touch again.
- _Acceptance Criteria:_
  - Running dotfiles twice results in the same system state as running it once.
  - Existing configuration files that are already correctly applied are not touched.
  - Conflicting files are backed up before being replaced, never silently overwritten.

## 5. Functional Requirements

### P0 (Critical)

- **Single-Command Bootstrap:** The entire setup process must be triggerable via a single initial command.
- **Interactive TUI:** Dotfiles must present a TUI for the user to select which detected tools to configure, which theme to apply, and which Nerd Font to use.
- **Cross-Platform Support:** The system must officially support MacOS and Ubuntu Linux.
- **Tool Detection:** Dotfiles must detect which supported tools are installed on the system and offer to configure only those. Tools that are not installed are silently excluded from the selection.
- **Shell Configuration:** Dotfiles must support configuring Bash and Zsh, including themed prompt and fzf color integration. Shell rc files (`~/.zshrc`, `~/.bashrc`) must never be replaced or overwritten — the installer injects a single guarded `source` line and manages the prompt in a separate fragment file.
- **Idempotent Execution:** Dotfiles must be safely repeatable at any time without destructive consequences.

### P1 (High)

- **Extended Tool Suite:** Dotfiles must support configuring the full terminal stack when those tools are detected: Kitty, Alacritty, Zellij, Tmux, NeoVim, and OpenCode.
- **Global Theme Switching:** Dotfiles must apply the selected theme palette across all configured tools via a shared template system.
- **Nerd Font Selection:** Dotfiles must detect Nerd Fonts already installed on the system and present them for selection. Font selection is optional — if no Nerd Fonts are found, the step is skipped gracefully.

## 6. Non-Functional Requirements

- **Zero Side-Effects:** Dotfiles must not install packages, modify system-level configuration, or require elevated privileges. Its only effect on the system is placing configuration files in the user's home directory.
- **No Toolchain Required:** The dotfiles binary must be distributed pre-compiled. The user must not need Rust, Node, or any other build toolchain on the target machine.
- **Performance:** The TUI must load promptly (under 2 seconds) and remain responsive during user interaction.
- **Reliability:** Failures must be reported clearly with enough context to diagnose the problem. A failed run must never leave the system in a partially-configured state.

## 7. Open Questions

1. **Unsupported Tool Versions:** Some tools (e.g., NeoVim, Zellij) may be installed but at a version too old to support the provided configuration. Should dotfiles validate minimum versions before offering to configure a tool, or configure it and surface compatibility warnings at the end?

2. **Tools Installed Outside Standard Locations:** On some systems, tools installed via version managers (e.g., `asdf`, `mise`) may not be immediately visible to standard detection. Should dotfiles support additional detection strategies beyond checking the standard executable search path?

## 8. Risks & Mitigations

- **Conflicting Existing Dotfiles:** Users with existing configurations may have files that conflict with dotfiles' targets. For shell rc files (`~/.zshrc`, `~/.bashrc`): dotfiles never replaces them — it appends a single source line (idempotent). For all other tool config files (Kitty, Alacritty, Zellij, etc.): dotfiles backs up any conflicting file before replacing it, and the backup path is printed so the user can recover.
- **Invalid Theme Configurations:** A palette file missing required color keys would cause template rendering to fail. Mitigation: palette schema validation runs at startup and rejects malformed palettes before any user interaction begins.
