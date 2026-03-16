# Dotfiles — Product Requirements Document

## 1. Executive Summary

The Dotfiles project aims to provide a reliable, automated, and interactive way to configure developer environments across MacOS and Linux. Through a single command, users will be able to provision a fully functional terminal-based development environment featuring curated tools, consistent typography, and synchronized global theming. The installation process will be guided by an interactive Terminal User Interface (TUI), ensuring a smooth onboarding experience.

## 2. Goals & Non-Goals

### Goals

- **Interactive TUI:** Provide a terminal-based user interface to guide the user through configuration and installation choices.
- **Cross-platform — MacOS & Linux:** Ensure consistent configuration and setup across supported operating systems.
- **Global Theme Switching:** Allow users to switch themes across all supported terminal applications and tools seamlessly.

### Non-Goals

- Managing graphical user interface (GUI) applications.
- Supporting the Windows operating system.
- Handling or storing secrets, credentials, or sensitive API keys.

## 3. User Personas

- **User:** Someone who uses the installer to set up their environment. They run it, configure via TUI, and get productive. They don't contribute to the project.
- **Maintainer:** Someone who uses the installer AND contributes by adding new tools, themes, or configurations to the repository.

## 4. User Stories

### Story 1: Guided Installation

- **As a** User,
- **I want** to run a single command that launches an interactive configuration menu,
- **So that** I can easily choose which tools to install without needing to read extensive documentation.
- _Acceptance Criteria:_
  - Running the bootstrap command opens a TUI.
  - The TUI presents clear options for shells, terminal emulators, multiplexers, and themes.
  - Selections are saved and applied automatically upon confirmation.

### Story 2: Environment Consistency Across OS

- **As a** User,
- **I want** my development environment to feel identical whether I am on my MacOS laptop or a Linux server,
- **So that** my workflow and muscle memory remain uninterrupted.
- _Acceptance Criteria:_
  - The same primary tools (Zsh, Kitty, Zellij, NeoVim) are installed and configured on both OS platforms.
  - Keybindings and aliases are consistent across platforms.

### Story 3: Global Theme Management

- **As a** User,
- **I want** to change my color theme globally across all my terminal tools at once,
- **So that** I don't have to manually update individual configuration files for my shell, multiplexer, and editor.
- _Acceptance Criteria:_
  - A command or interface is provided to select a theme (e.g., Catppuccin, Rosé Pine, Kanagawa).
  - Applying a theme updates the configuration for Kitty/Alacritty, Zellij/tmux, and NeoVim simultaneously.

### Story 4: Safe Re-execution (Idempotency)

- **As a** User,
- **I want** to be able to run the setup script multiple times after tweaking my configurations,
- **So that** missing components are installed without corrupting or duplicating existing working setups.
- _Acceptance Criteria:_
  - Executing the installer twice results in the same system state as executing it once.
  - Existing valid configurations are not destructively overwritten without prompt or backup.

## 5. Functional Requirements

### P0 (Critical)

- **Single-Command Bootstrap:** The entire setup process must be triggerable via a single initial command.
- **Interactive TUI:** The installer must present a TUI for users to select components (Shell, Terminal, Multiplexer, Editor, Theme).
- **Cross-Platform Support:** The system must officially support MacOS and Linux.
- **Native Package Managers:** Each OS distribution must use its native package manager for installations.
- **Core Tooling Provisioning:** The system must install and configure the primary stack: Zsh, Kitty, Zellij, NeoVim, and OpenCode.
- **Idempotent Execution:** The setup scripts must be safely repeatable.

### P1 (High)

- **Secondary Tooling Support:** The system must support installing alternative tools based on user choice: Bash, Alacritty, and tmux.
- **Font Installation:** The system must install specified Nerd Fonts (Meslo NF, Iosevka NF, JetBrains Mono NF) on both supported operating systems.
- **Global Theme Switching:** The system must provide a mechanism to apply selected themes (Catppuccin, Rosé Pine, Kanagawa) across all relevant installed applications simultaneously.

## 6. Non-Functional Requirements

- **Ease of Installation:** The installer must not require the user to install development toolchains or compilers on the target machine prior to bootstrapping.
- **Performance:** The interactive TUI must load promptly (under 2 seconds) and remain responsive during user interaction.
- **Reliability:** The setup process must handle network interruptions gracefully, failing clearly and allowing the user to retry without leaving the system in a broken state.

## 7. Open Questions

1. **Outdated Packages:** How should we handle situations where native package managers on Linux provide significantly outdated versions of critical tools (e.g., NeoVim, Zellij) compared to what is required for the configurations to work correctly? Do we strictly stick to the native manager, or do we allow fallbacks (like downloading release binaries)?

## 8. Risks & Mitigations

- To be identified during the design phase.
