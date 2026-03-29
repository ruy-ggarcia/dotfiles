use std::collections::HashSet;
use std::path::Path;

use crate::models::{Module, Shell};

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

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
}
