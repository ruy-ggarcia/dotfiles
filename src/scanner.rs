use std::collections::HashSet;
use std::path::Path;

use crate::models::{Shell, TerminalEmulator, Theme};

pub fn scan_shells(entries: &[(Shell, &Path)]) -> Vec<Shell> {
    let mut seen: HashSet<Shell> = HashSet::new();
    let mut result: Vec<Shell> = Vec::new();

    for (shell, path) in entries {
        if path.exists() && seen.insert(shell.clone()) {
            result.push(shell.clone());
        }
    }

    result
}

pub fn scan_terminal_emulators(entries: &[(TerminalEmulator, &Path)]) -> Vec<TerminalEmulator> {
    let mut seen: std::collections::HashSet<TerminalEmulator> = std::collections::HashSet::new();
    let mut result: Vec<TerminalEmulator> = Vec::new();
    for (terminal_emulator, path) in entries {
        if path.exists() && seen.insert(terminal_emulator.clone()) {
            result.push(terminal_emulator.clone());
        }
    }
    result
}

pub fn validate_theme(toml_str: &str) -> Result<Theme, String> {
    let table: toml::Table = toml_str
        .parse()
        .map_err(|e| format!("TOML parse error: {e}"))?;

    let required_keys = [
        "name",
        // UI colors consumed by terminal emulator templates
        "base",
        "text",
        "cursor",
        "selection_bg",
        "selection_fg",
        // ANSI palette consumed by terminal emulator templates
        "color0",
        "color1",
        "color2",
        "color3",
        "color4",
        "color5",
        "color6",
        "color7",
        "color8",
        "color9",
        "color10",
        "color11",
        "color12",
        "color13",
        "color14",
        "color15",
    ];
    for key in &required_keys {
        match table.get(*key) {
            Some(toml::Value::String(_)) => {}
            Some(_) => return Err(format!("Key '{key}' is not a string")),
            None => return Err(format!("Missing required key: '{key}'")),
        }
    }

    let name = table["name"].as_str().unwrap().to_string();
    let colors: std::collections::HashMap<String, String> = table
        .iter()
        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
        .collect();

    Ok(Theme { name, colors })
}

pub fn scan_themes(dir: &Path) -> Vec<Theme> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Cannot read themes dir {}: {e}", dir.display());
            return vec![];
        }
    };

    let mut themes: Vec<Theme> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? != "toml" {
                return None;
            }
            let content = std::fs::read_to_string(&path).ok()?;
            match validate_theme(&content) {
                Ok(palette) => Some(palette),
                Err(e) => {
                    eprintln!("Skipping {}: {e}", path.display());
                    None
                }
            }
        })
        .collect();

    themes.sort_by(|a, b| a.name.cmp(&b.name));
    themes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const VALID_TOML: &str = concat!(
        "name = \"Test Theme\"\n",
        "base = \"#24273a\"\ntext = \"#cad3f5\"\n",
        "cursor = \"#f4dbd6\"\nselection_bg = \"#5b6078\"\nselection_fg = \"#cad3f5\"\n",
        "color0 = \"#494d64\"\ncolor1 = \"#ed8796\"\ncolor2 = \"#a6da95\"\ncolor3 = \"#eed49f\"\n",
        "color4 = \"#8aadf4\"\ncolor5 = \"#f5bde6\"\ncolor6 = \"#8bd5ca\"\ncolor7 = \"#b8c0e0\"\n",
        "color8 = \"#5b6078\"\ncolor9 = \"#ed8796\"\ncolor10 = \"#a6da95\"\ncolor11 = \"#eed49f\"\n",
        "color12 = \"#8aadf4\"\ncolor13 = \"#f5bde6\"\ncolor14 = \"#8bd5ca\"\ncolor15 = \"#a5adcb\"\n",
    );

    #[test]
    fn test_module_shell_debug_format() {
        let shell = Shell::Zsh;
        let debug_str = format!("{:?}", shell);
        assert_eq!(debug_str, "Zsh");
    }

    #[test]
    fn test_module_equality() {
        let a = Shell::Zsh;
        let b = Shell::Zsh;
        assert_eq!(a, b);
    }

    #[test]
    fn test_scan_shells_returns_zsh_when_binary_exists() {
        let entries = [(Shell::Zsh, Path::new("/bin/sh"))];
        let result = scan_shells(&entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Shell::Zsh);
    }

    #[test]
    fn test_scan_shells_returns_bash_when_binary_exists() {
        let entries = [(Shell::Bash, Path::new("/bin/sh"))];
        let result = scan_shells(&entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Shell::Bash);
    }

    #[test]
    fn test_scan_shells_skips_missing_binaries() {
        let entries = [(Shell::Zsh, Path::new("/tmp/does-not-exist-abc123"))];
        let result = scan_shells(&entries);
        assert!(result.is_empty());
    }

    #[test]
    fn test_scan_shells_returns_multiple_when_both_exist() {
        let entries = [
            (Shell::Bash, Path::new("/bin/sh")),
            (Shell::Zsh, Path::new("/bin/sh")),
        ];
        let result = scan_shells(&entries);
        assert_eq!(result.len(), 2);
        let shells: Vec<&Shell> = result.iter().collect();
        assert!(shells.contains(&&Shell::Bash));
        assert!(shells.contains(&&Shell::Zsh));
    }

    #[test]
    fn test_scan_shells_returns_empty_when_none_exist() {
        let entries = [
            (Shell::Bash, Path::new("/tmp/does-not-exist-abc123")),
            (Shell::Zsh, Path::new("/tmp/does-not-exist-abc123")),
        ];
        let result = scan_shells(&entries);
        assert!(result.is_empty());
    }

    #[test]
    fn test_scan_shells_deduplicates() {
        let entries = [
            (Shell::Zsh, Path::new("/bin/sh")),
            (Shell::Zsh, Path::new("/bin/sh")),
        ];
        let result = scan_shells(&entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Shell::Zsh);
    }

    #[test]
    fn test_scan_themes_finds_toml_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.toml"), VALID_TOML).unwrap();
        std::fs::write(dir.path().join("b.toml"), concat!(
            "name = \"Second Theme\"\n",
            "base = \"#181616\"\ntext = \"#c5c9c5\"\n",
            "cursor = \"#c5c9c5\"\nselection_bg = \"#393836\"\nselection_fg = \"#c5c9c5\"\n",
            "color0 = \"#282727\"\ncolor1 = \"#c4746e\"\ncolor2 = \"#8a9a7b\"\ncolor3 = \"#c4b28a\"\n",
            "color4 = \"#7fb4ca\"\ncolor5 = \"#a292a3\"\ncolor6 = \"#949fb5\"\ncolor7 = \"#c5c9c5\"\n",
            "color8 = \"#393836\"\ncolor9 = \"#c4746e\"\ncolor10 = \"#87a987\"\ncolor11 = \"#c4b28a\"\n",
            "color12 = \"#7fb4ca\"\ncolor13 = \"#a292a3\"\ncolor14 = \"#949fb5\"\ncolor15 = \"#9e9b93\"\n",
        )).unwrap();
        let themes = scan_themes(dir.path());
        assert_eq!(themes.len(), 2);
    }

    #[test]
    fn test_scan_themes_skips_invalid_themes() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("valid.toml"), VALID_TOML).unwrap();
        std::fs::write(
            dir.path().join("invalid.toml"),
            "base = \"#181616\"\ntext = \"#c5c9c5\"\n",
        )
        .unwrap();
        let themes = scan_themes(dir.path());
        assert_eq!(themes.len(), 1);
    }

    #[test]
    fn test_scan_themes_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let themes = scan_themes(dir.path());
        assert!(themes.is_empty());
    }

    #[test]
    fn test_validate_theme_passes_valid() {
        let result = validate_theme(VALID_TOML);
        assert!(result.is_ok());
        let palette = result.unwrap();
        assert_eq!(palette.name, "Test Theme");
    }

    #[test]
    fn test_validate_theme_fails_missing_key() {
        // Missing name, cursor, selection_*, and all color0-color15
        let toml_str = "base = \"#24273a\"\ntext = \"#cad3f5\"\n";
        let result = validate_theme(toml_str);
        assert!(result.is_err());
    }
}
