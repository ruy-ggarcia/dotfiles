use std::collections::HashSet;
use std::path::Path;

pub fn scan_fonts(dirs: &[&Path]) -> Vec<String> {
    let mut families: HashSet<String> = HashSet::new();
    for dir in dirs {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext != "ttf" && ext != "otf" {
                continue;
            }
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if let Some(idx) = stem.find("NerdFont") {
                let family = format!("{} Nerd Font", &stem[..idx]);
                families.insert(family);
            }
        }
    }
    let mut result: Vec<String> = families.into_iter().collect();
    result.sort();
    result
}

pub fn font_dirs(home: &str) -> Vec<std::path::PathBuf> {
    if cfg!(target_os = "linux") {
        vec![
            std::path::PathBuf::from(home).join(".local/share/fonts"),
            std::path::PathBuf::from("/usr/share/fonts"),
            std::path::PathBuf::from("/usr/local/share/fonts"),
        ]
    } else {
        vec![
            std::path::PathBuf::from(home).join("Library/Fonts"),
            std::path::PathBuf::from("/Library/Fonts"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_file(dir: &TempDir, name: &str) {
        fs::write(dir.path().join(name), b"").unwrap();
    }

    #[test]
    fn test_scan_fonts_finds_nerd_font_ttf() {
        let dir = TempDir::new().unwrap();
        make_file(&dir, "FooNerdFont-Regular.ttf");
        let result = scan_fonts(&[dir.path()]);
        assert_eq!(result, vec!["Foo Nerd Font"]);
    }

    #[test]
    fn test_scan_fonts_ignores_non_nerd_fonts() {
        let dir = TempDir::new().unwrap();
        make_file(&dir, "Arial.ttf");
        let result = scan_fonts(&[dir.path()]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_scan_fonts_empty_dir() {
        let dir = TempDir::new().unwrap();
        let result = scan_fonts(&[dir.path()]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_scan_fonts_missing_dir() {
        let result = scan_fonts(&[Path::new("/tmp/does-not-exist-nerd-fonts-xyz")]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_scan_fonts_deduplicates_variants() {
        let dir = TempDir::new().unwrap();
        make_file(&dir, "FooNerdFont-Regular.ttf");
        make_file(&dir, "FooNerdFont-Bold.ttf");
        let result = scan_fonts(&[dir.path()]);
        assert_eq!(result, vec!["Foo Nerd Font"]);
    }

    #[test]
    fn test_scan_fonts_multiple_dirs() {
        let dir1 = TempDir::new().unwrap();
        let dir2 = TempDir::new().unwrap();
        make_file(&dir1, "FooNerdFont-Regular.ttf");
        make_file(&dir2, "BarNerdFont-Regular.ttf");
        let result = scan_fonts(&[dir1.path(), dir2.path()]);
        assert_eq!(result, vec!["Bar Nerd Font", "Foo Nerd Font"]);
    }

    #[test]
    fn test_scan_fonts_handles_otf() {
        let dir = TempDir::new().unwrap();
        make_file(&dir, "FooNerdFont-Regular.otf");
        let result = scan_fonts(&[dir.path()]);
        assert_eq!(result, vec!["Foo Nerd Font"]);
    }

    #[test]
    fn test_font_dirs_never_empty() {
        let dirs = font_dirs("/home/user");
        assert!(!dirs.is_empty());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_font_dirs_macos_contains_library_fonts() {
        let dirs = font_dirs("/Users/testuser");
        let paths: Vec<String> = dirs.iter().map(|p| p.to_string_lossy().to_string()).collect();
        assert!(paths.iter().any(|p| p.contains("Library/Fonts")));
        assert!(!paths.iter().any(|p| p.contains(".local/share/fonts")));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_font_dirs_linux_contains_local_share_fonts() {
        let dirs = font_dirs("/home/testuser");
        let paths: Vec<String> = dirs.iter().map(|p| p.to_string_lossy().to_string()).collect();
        assert!(paths.iter().any(|p| p.contains(".local/share/fonts")));
        assert!(paths.iter().any(|p| p == "/usr/share/fonts"));
        assert!(!paths.iter().any(|p| p.contains("Library/Fonts")));
    }
}
