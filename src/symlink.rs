use std::path::Path;

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
