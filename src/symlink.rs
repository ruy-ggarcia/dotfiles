use std::path::Path;

/// Appends a guarded `source` line to an rc file (e.g. ~/.zshrc).
/// Idempotent: does nothing if the prompt path already appears in the file.
/// Creates the rc file if it does not exist.
pub fn inject_source_line(rc_path: &Path, prompt_path: &Path) -> std::io::Result<()> {
    let prompt_str = prompt_path.to_string_lossy();
    let existing = std::fs::read_to_string(rc_path).unwrap_or_default();
    if existing.contains(prompt_str.as_ref()) {
        return Ok(());
    }
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(rc_path)?;
    writeln!(file, "\n# dotfiles managed — do not remove")?;
    writeln!(file, r#"[[ -f "{prompt_str}" ]] && source "{prompt_str}""#)?;
    Ok(())
}

// FIXME: Used by non-shell modules (Kitty, Alacritty, Zellij, etc). Remove at M4.
#[allow(dead_code)]
pub fn create_symlink(src: &Path, dest: &Path) -> std::io::Result<()> {
    if dest.symlink_metadata().is_ok() {
        if dest.is_symlink() {
            std::fs::remove_file(dest)?;
        } else {
            let ts = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let dest_filename = dest
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let backup = dest
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join(format!("{dest_filename}.{ts}.bak"));
            std::fs::rename(dest, &backup)?;
        }
    }
    std::os::unix::fs::symlink(src, dest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_inject_source_line_appends_when_absent() {
        let dir = TempDir::new().unwrap();
        let rc = dir.path().join(".zshrc");
        std::fs::write(&rc, "export FOO=bar\n").unwrap();
        let prompt = dir.path().join("prompt.zsh");

        inject_source_line(&rc, &prompt).unwrap();

        let content = std::fs::read_to_string(&rc).unwrap();
        assert!(content.contains(&*prompt.to_string_lossy()));
    }

    #[test]
    fn test_inject_source_line_idempotent() {
        let dir = TempDir::new().unwrap();
        let rc = dir.path().join(".zshrc");
        let prompt = dir.path().join("prompt.zsh");

        inject_source_line(&rc, &prompt).unwrap();
        let content_after_first = std::fs::read_to_string(&rc).unwrap();

        inject_source_line(&rc, &prompt).unwrap();
        let content_after_second = std::fs::read_to_string(&rc).unwrap();

        assert_eq!(content_after_first, content_after_second);
    }

    #[test]
    fn test_inject_source_line_creates_rc_if_missing() {
        let dir = TempDir::new().unwrap();
        let rc = dir.path().join(".zshrc");
        let prompt = dir.path().join("prompt.zsh");

        inject_source_line(&rc, &prompt).unwrap();

        assert!(rc.exists());
    }

    #[test]
    fn test_create_symlink_creates_link() {
        let dir = TempDir::new().unwrap();
        let src = dir.path().join("source.txt");
        std::fs::write(&src, "content").unwrap();
        let dest = dir.path().join("link.txt");

        create_symlink(&src, &dest).unwrap();

        assert!(dest.exists());
    }

    #[test]
    fn test_create_symlink_points_to_source() {
        let dir = TempDir::new().unwrap();
        let src = dir.path().join("source.txt");
        std::fs::write(&src, "content").unwrap();
        let dest = dir.path().join("link.txt");

        create_symlink(&src, &dest).unwrap();

        let target = std::fs::read_link(&dest).unwrap();
        assert_eq!(target, src);
    }

    #[test]
    fn test_backup_created_for_existing_regular_file() {
        let dir = TempDir::new().unwrap();
        let src = dir.path().join("source.txt");
        std::fs::write(&src, "content").unwrap();
        let dest = dir.path().join("dest.txt");
        std::fs::write(&dest, "original").unwrap();

        create_symlink(&src, &dest).unwrap();

        let dest_filename = dest.file_name().unwrap().to_string_lossy();
        let backup_exists = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .any(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                name.starts_with(dest_filename.as_ref()) && name.contains("bak")
            });
        assert!(backup_exists);
    }

    #[test]
    fn test_backup_contains_original_content() {
        let dir = TempDir::new().unwrap();
        let src = dir.path().join("source.txt");
        std::fs::write(&src, "content").unwrap();
        let dest = dir.path().join("dest.txt");
        std::fs::write(&dest, "original").unwrap();

        create_symlink(&src, &dest).unwrap();

        let dest_filename = dest.file_name().unwrap().to_string_lossy();
        let backup_entry = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .find(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                name.starts_with(dest_filename.as_ref()) && name.contains("bak")
            })
            .expect("backup file not found");
        let content = std::fs::read_to_string(backup_entry.path()).unwrap();
        assert_eq!(content, "original");
    }

    #[test]
    fn test_create_symlink_replaces_existing() {
        let dir = TempDir::new().unwrap();
        let src1 = dir.path().join("source1.txt");
        let src2 = dir.path().join("source2.txt");
        std::fs::write(&src1, "first").unwrap();
        std::fs::write(&src2, "second").unwrap();
        let dest = dir.path().join("link.txt");

        create_symlink(&src1, &dest).unwrap();
        create_symlink(&src2, &dest).unwrap();

        let target = std::fs::read_link(&dest).unwrap();
        assert_eq!(target, src2);
    }
}
