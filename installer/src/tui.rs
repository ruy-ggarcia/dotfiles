use anyhow::{anyhow, Result};
use inquire::{Confirm, InquireError, MultiSelect, Select};

use crate::models::{Module, Plan, Theme};

// ---------------------------------------------------------------------------
// Welcome banner
// ---------------------------------------------------------------------------

/// Prints the installer welcome banner to stdout.
///
/// Reads OS and architecture from compile-time/runtime constants and embeds
/// the binary version via `env!("CARGO_PKG_VERSION")`.
///
/// The package manager label is inferred from the current OS:
/// `brew` on macOS, `apt` on Linux, `unknown` otherwise.
pub fn display_welcome() {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let version = env!("CARGO_PKG_VERSION");
    let pkg_mgr = match os {
        "macos" => "brew",
        "linux" => "apt",
        _ => "unknown",
    };

    println!("=========================================");
    println!("  DOTFILES INSTALLER v{}", version);
    println!("  OS: {} ({}) | Package: {}", os, arch, pkg_mgr);
    println!("=========================================");
    println!();
}

// ---------------------------------------------------------------------------
// Module selection
// ---------------------------------------------------------------------------

/// Module names that are pre-selected by default in the TUI.
///
/// These are the "primary tools" as defined in the design spec.
const PRIMARY_MODULES: &[&str] = &["zsh", "kitty", "zellij", "neovim", "opencode"];

/// Presents an interactive multi-select prompt for the user to choose modules.
///
/// Primary tools (zsh, kitty, zellij, neovim, opencode) are pre-selected.
///
/// Returns the selected [`Module`]s.  An **empty `Vec`** signals that the user
/// cancelled the prompt (`Ctrl+C`) — callers should treat this as a clean exit
/// rather than an error.
pub fn select_modules(modules: Vec<Module>) -> Result<Vec<Module>> {
    if modules.is_empty() {
        anyhow::bail!("No modules found — cannot continue.");
    }

    // Use module names as display labels (lowercase, matching directory names)
    let labels: Vec<String> = modules.iter().map(|m| m.name.clone()).collect();

    // Determine default pre-selected indices for primary tools
    let defaults: Vec<usize> = modules
        .iter()
        .enumerate()
        .filter(|(_, m)| PRIMARY_MODULES.contains(&m.name.to_lowercase().as_str()))
        .map(|(i, _)| i)
        .collect();

    let result = MultiSelect::new("Select modules to install and configure:", labels.clone())
        .with_default(&defaults)
        .prompt();

    match result {
        Ok(selected_labels) => {
            // Map selected labels back to the original Module values
            let selected = modules
                .into_iter()
                .zip(labels.iter())
                .filter(|(_, label)| selected_labels.contains(label))
                .map(|(module, _)| module)
                .collect();
            Ok(selected)
        }
        // cancellation → return empty vec so the caller can exit cleanly
        Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
            Ok(vec![])
        }
        Err(e) => Err(anyhow!("Module selection failed: {}", e)),
    }
}

// ---------------------------------------------------------------------------
// Theme selection
// ---------------------------------------------------------------------------

/// Presents an interactive single-select prompt for the user to choose a theme.
///
/// Defaults to `catppuccin-mocha` if present (matched by path file-stem),
/// otherwise falls back to the first item.
///
/// Returns the selected [`Theme`], or an error if the user cancels (`Ctrl+C`).
pub fn select_theme(themes: Vec<Theme>) -> Result<Theme> {
    if themes.is_empty() {
        anyhow::bail!("No themes found — cannot continue.");
    }

    let labels: Vec<String> = themes.iter().map(|t| t.name.clone()).collect();

    // Prefer catppuccin-mocha by file-stem (e.g. "catppuccin-mocha.toml"),
    // also accept if the display name itself is "catppuccin-mocha".
    // Fall back to index 0 when not found.
    let default_index = themes
        .iter()
        .position(|t| {
            let stem = t
                .path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            stem == "catppuccin-mocha" || t.name == "catppuccin-mocha"
        })
        .unwrap_or(0);

    let result = Select::new("Select global theme:", labels.clone())
        .with_starting_cursor(default_index)
        .prompt();

    match result {
        Ok(selected_label) => {
            let theme = themes
                .into_iter()
                .find(|t| t.name == selected_label)
                .expect("selected label must match a theme");
            Ok(theme)
        }
        // cancellation → propagate as error (caller handles clean exit)
        Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
            Err(anyhow!("Setup canceled by user."))
        }
        Err(e) => Err(anyhow!("Theme selection failed: {}", e)),
    }
}

// ---------------------------------------------------------------------------
// Confirmation prompt
// ---------------------------------------------------------------------------

/// Displays a summary of the installation plan and asks for confirmation.
///
/// Shows package count, template count, symlink count, and the active theme name.
/// Uses `[Y/n]` (default-yes) semantics via `inquire::Confirm`.
///
/// Returns `true` if the user confirms, `false` if they decline.
/// Returns an error if the user cancels with `Ctrl+C`.
pub fn confirm_plan(plan: &Plan, theme_name: &str) -> Result<bool> {
    println!();
    println!("Installation Plan:");
    println!("  Packages to install: {}", plan.packages_to_install.len());
    println!("  Templates to render: {}", plan.templates_to_render.len());
    println!("  Symlinks to create:  {}", plan.symlinks_to_create.len());
    println!("  Active theme:        {}", theme_name);
    println!();

    let result = Confirm::new("Do you want to proceed?")
        .with_default(true)
        .prompt();

    match result {
        Ok(answer) => Ok(answer),
        // cancellation → propagate as error (caller checks and exits cleanly)
        Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
            Err(anyhow!("Setup canceled by user."))
        }
        Err(e) => Err(anyhow!("Confirmation prompt failed: {}", e)),
    }
}

// ---------------------------------------------------------------------------
// Execution feedback helpers
// ---------------------------------------------------------------------------

/// Prints the post-installation success banner to stdout.
pub fn display_success() {
    println!();
    println!("=========================================");
    println!("  SUCCESS: Environment provisioned.");
    println!("  Please restart your terminal to apply changes.");
    println!("=========================================");
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_module(name: &str) -> Module {
        Module {
            name: name.to_string(),
            path: PathBuf::from(format!("modules/{}", name)),
            packages_by_os: HashMap::new(),
        }
    }

    fn make_theme(name: &str, stem: &str) -> Theme {
        Theme {
            name: name.to_string(),
            path: PathBuf::from(format!("themes/palettes/{}.toml", stem)),
            variables: HashMap::new(),
        }
    }

    // -------------------------------------------------------------------------
    // primary module pre-selection indices
    // -------------------------------------------------------------------------

    #[test]
    fn test_primary_module_indices_are_preselected() {
        let modules = vec![
            make_module("bash"),
            make_module("zsh"),   // primary
            make_module("kitty"), // primary
            make_module("alacritty"),
            make_module("zellij"), // primary
            make_module("tmux"),
            make_module("neovim"),   // primary
            make_module("opencode"), // primary
        ];

        let defaults: Vec<usize> = modules
            .iter()
            .enumerate()
            .filter(|(_, m)| PRIMARY_MODULES.contains(&m.name.to_lowercase().as_str()))
            .map(|(i, _)| i)
            .collect();

        // zsh=1, kitty=2, zellij=4, neovim=6, opencode=7
        assert_eq!(defaults, vec![1, 2, 4, 6, 7]);
    }

    #[test]
    fn test_primary_modules_constant_matches_spec() {
        // Ensure the constant aligns exactly with the design spec (no extras, no typos)
        assert!(PRIMARY_MODULES.contains(&"zsh"));
        assert!(PRIMARY_MODULES.contains(&"kitty"));
        assert!(PRIMARY_MODULES.contains(&"zellij"));
        assert!(PRIMARY_MODULES.contains(&"neovim"));
        assert!(PRIMARY_MODULES.contains(&"opencode"));
        // "nvim" is NOT a primary tool — directory should be "neovim"
        assert!(!PRIMARY_MODULES.contains(&"nvim"));
        assert_eq!(PRIMARY_MODULES.len(), 5);
    }

    #[test]
    fn test_no_primary_modules_in_empty_list() {
        let modules: Vec<Module> = vec![];
        let defaults: Vec<usize> = modules
            .iter()
            .enumerate()
            .filter(|(_, m)| PRIMARY_MODULES.contains(&m.name.to_lowercase().as_str()))
            .map(|(i, _)| i)
            .collect();
        assert!(defaults.is_empty());
    }

    // -------------------------------------------------------------------------
    // catppuccin-mocha default cursor position
    // -------------------------------------------------------------------------

    #[test]
    fn test_catppuccin_mocha_default_by_stem() {
        let themes = vec![
            make_theme("Catppuccin Frappe", "catppuccin-frappe"),
            make_theme("Catppuccin Latte", "catppuccin-latte"),
            make_theme("Catppuccin Mocha", "catppuccin-mocha"),
            make_theme("Rose Pine", "rose-pine"),
        ];

        let default_index = themes
            .iter()
            .position(|t| {
                let stem = t
                    .path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                stem == "catppuccin-mocha" || t.name == "catppuccin-mocha"
            })
            .unwrap_or(0);

        assert_eq!(default_index, 2, "catppuccin-mocha should be at index 2");
    }

    #[test]
    fn test_theme_default_falls_back_to_zero_when_mocha_absent() {
        let themes = vec![
            make_theme("Rose Pine", "rose-pine"),
            make_theme("Kanagawa Wave", "kanagawa-wave"),
        ];

        let default_index = themes
            .iter()
            .position(|t| {
                let stem = t
                    .path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                stem == "catppuccin-mocha" || t.name == "catppuccin-mocha"
            })
            .unwrap_or(0);

        assert_eq!(default_index, 0);
    }

    // -------------------------------------------------------------------------
    // display_welcome uses env!("CARGO_PKG_VERSION")
    // -------------------------------------------------------------------------

    #[test]
    fn test_display_welcome_does_not_panic() {
        // Verifies the function is callable and doesn't panic.
        // stdout content is not easily captured in unit tests; we rely on
        // the fact that env!("CARGO_PKG_VERSION") is always available at compile time.
        display_welcome();
    }

    #[test]
    fn test_cargo_pkg_version_is_available() {
        let version = env!("CARGO_PKG_VERSION");
        assert!(!version.is_empty(), "version must be a non-empty string");
    }

    // -------------------------------------------------------------------------
    // cancellation handling for select_modules (empty vec = cancelled)
    // -------------------------------------------------------------------------

    #[test]
    fn test_select_modules_empty_input_bails() {
        let result = select_modules(vec![]);
        assert!(result.is_err(), "empty module list should return an error");
    }

    // -------------------------------------------------------------------------
    // select_theme empty input bails
    // -------------------------------------------------------------------------

    #[test]
    fn test_select_theme_empty_input_bails() {
        let result = select_theme(vec![]);
        assert!(result.is_err(), "empty theme list should return an error");
    }
}
