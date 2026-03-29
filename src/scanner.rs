use std::collections::HashSet;
use std::path::Path;

use crate::models::{Module, Theme, Shell};

pub fn scan_shells(entries: &[(Shell, &Path)]) -> Vec<Module> {
    let mut seen: HashSet<Shell> = HashSet::new();
    let mut result: Vec<Module> = Vec::new();

    for (shell, path) in entries {
        if path.exists() && seen.insert(shell.clone()) {
            result.push(Module { shell: shell.clone() });
        }
    }

    result
}

pub fn validate_theme(toml_str: &str) -> Result<Theme, String> {
    let table: toml::Table = toml_str
        .parse()
        .map_err(|e| format!("TOML parse error: {e}"))?;

    let required_keys = ["name", "base", "text", "accent", "surface", "overlay"];
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

    const VALID_TOML: &str = "name = \"Test Theme\"\nbase = \"#24273a\"\ntext = \"#cad3f5\"\naccent = \"#8aadf4\"\nsurface = \"#363a4f\"\noverlay = \"#6e738d\"\n";

    #[test]
    fn test_module_shell_debug_format() {
        let shell = Shell::Zsh;
        let debug_str = format!("{:?}", shell);
        assert_eq!(debug_str, "Zsh");
    }

    #[test]
    fn test_module_equality() {
        let a = Module { shell: Shell::Zsh };
        let b = Module { shell: Shell::Zsh };
        assert_eq!(a, b);
    }

    #[test]
    fn test_scan_shells_returns_zsh_when_binary_exists() {
        let entries = [(Shell::Zsh, Path::new("/bin/sh"))];
        let result = scan_shells(&entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].shell, Shell::Zsh);
    }

    #[test]
    fn test_scan_shells_returns_bash_when_binary_exists() {
        let entries = [(Shell::Bash, Path::new("/bin/sh"))];
        let result = scan_shells(&entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].shell, Shell::Bash);
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
        let shells: Vec<&Shell> = result.iter().map(|m| &m.shell).collect();
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
        assert_eq!(result[0].shell, Shell::Zsh);
    }

    #[test]
    fn test_scan_themes_finds_toml_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.toml"), VALID_TOML).unwrap();
        std::fs::write(dir.path().join("b.toml"), "name = \"Second Theme\"\nbase = \"#181616\"\ntext = \"#c5c9c5\"\naccent = \"#7fb4ca\"\nsurface = \"#282727\"\noverlay = \"#8a9a7b\"\n").unwrap();
        let themes = scan_themes(dir.path());
        assert_eq!(themes.len(), 2);
    }

    #[test]
    fn test_scan_themes_skips_invalid_themes() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("valid.toml"), VALID_TOML).unwrap();
        std::fs::write(dir.path().join("invalid.toml"), "base = \"#181616\"\ntext = \"#c5c9c5\"\n").unwrap();
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
        let toml_str = "base = \"#24273a\"\ntext = \"#cad3f5\"\naccent = \"#8aadf4\"\nsurface = \"#363a4f\"\noverlay = \"#6e738d\"\n";
        let result = validate_theme(toml_str);
        assert!(result.is_err());
    }
}
