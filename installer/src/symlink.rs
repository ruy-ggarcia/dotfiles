use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Local;

use crate::models::SymlinkJob;

/// Processes a list of symlink jobs using a three-case idempotent algorithm:
///
/// 1. **Nothing exists at target** → create symlink
/// 2. **Correct symlink exists** (already points to `source_absolute`) → skip (idempotent)
/// 3. **Conflict** (file, dir, or symlink pointing elsewhere) → backup, then create symlink
pub fn process_symlinks(jobs: &[SymlinkJob]) -> Result<()> {
    for job in jobs {
        let source = &job.source_absolute;
        let target = &job.target_absolute;

        // Ensure parent directory of the target exists (T-023)
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create parent directories for: {}",
                    target.display()
                )
            })?;
        }

        // Check what exists at the target path using symlink_metadata
        // (does NOT follow symlinks, so we see the symlink itself if present)
        match fs::symlink_metadata(target) {
            Err(_) => {
                // Case 1: Nothing exists at target — create the symlink
                symlink(source, target).with_context(|| {
                    format!(
                        "failed to create symlink: {} -> {}",
                        target.display(),
                        source.display()
                    )
                })?;
            }
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    // Check where the existing symlink points
                    let current_target = fs::read_link(target).with_context(|| {
                        format!("failed to read symlink: {}", target.display())
                    })?;

                    if current_target == *source {
                        // Case 2: Correct symlink already exists — skip (idempotent)
                        continue;
                    }
                }

                // Case 3: Conflict — backup then create symlink
                let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
                let backup_path =
                    PathBuf::from(format!("{}.bak.{}", target.display(), timestamp));

                fs::rename(target, &backup_path).with_context(|| {
                    format!(
                        "failed to backup '{}' to '{}'",
                        target.display(),
                        backup_path.display()
                    )
                })?;

                eprintln!(
                    "  Backed up: {} -> {}",
                    target.display(),
                    backup_path.display()
                );

                symlink(source, target).with_context(|| {
                    format!(
                        "failed to create symlink: {} -> {}",
                        target.display(),
                        source.display()
                    )
                })?;
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (T-024)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink as create_symlink;
    use tempfile::TempDir;

    // -----------------------------------------------------------------------
    // T-024-1: creates a new symlink when nothing exists at target
    // -----------------------------------------------------------------------
    #[test]
    fn test_symlink_creates_new() {
        let tmp = TempDir::new().unwrap();

        // Create a source file
        let source = tmp.path().join("source.txt");
        fs::write(&source, "hello").unwrap();

        // Target does not exist yet
        let target = tmp.path().join("target.txt");

        let jobs = vec![SymlinkJob {
            source_absolute: source.clone(),
            target_absolute: target.clone(),
        }];

        process_symlinks(&jobs).unwrap();

        // Target should now be a symlink pointing to source
        assert!(
            target.exists(),
            "target symlink should exist"
        );
        let link_target = fs::read_link(&target).unwrap();
        assert_eq!(
            link_target, source,
            "symlink should point to source"
        );
    }

    // -----------------------------------------------------------------------
    // T-024-2: skips when correct symlink already exists (idempotent)
    // -----------------------------------------------------------------------
    #[test]
    fn test_symlink_idempotent_skip() {
        let tmp = TempDir::new().unwrap();

        // Create a source file
        let source = tmp.path().join("source.txt");
        fs::write(&source, "hello").unwrap();

        // Create a correct symlink at target already
        let target = tmp.path().join("target.txt");
        create_symlink(&source, &target).unwrap();

        let jobs = vec![SymlinkJob {
            source_absolute: source.clone(),
            target_absolute: target.clone(),
        }];

        process_symlinks(&jobs).unwrap();

        // No backup should exist
        let entries: Vec<_> = fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        let backup_exists = entries.iter().any(|e| {
            e.file_name()
                .to_string_lossy()
                .contains(".bak.")
        });
        assert!(
            !backup_exists,
            "no backup should be created when symlink is already correct"
        );

        // Symlink still points to correct source
        let link_target = fs::read_link(&target).unwrap();
        assert_eq!(link_target, source, "symlink should still point to source");
    }

    // -----------------------------------------------------------------------
    // T-024-3: backs up a conflicting regular file and creates the symlink
    // -----------------------------------------------------------------------
    #[test]
    fn test_symlink_conflict_backup() {
        let tmp = TempDir::new().unwrap();

        // Create a source file
        let source = tmp.path().join("source.txt");
        fs::write(&source, "source content").unwrap();

        // Create a regular file at the target path (conflict)
        let target = tmp.path().join("target.txt");
        fs::write(&target, "existing content").unwrap();

        let jobs = vec![SymlinkJob {
            source_absolute: source.clone(),
            target_absolute: target.clone(),
        }];

        process_symlinks(&jobs).unwrap();

        // A .bak.{timestamp} file should exist
        let entries: Vec<_> = fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        let backup_exists = entries.iter().any(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("target.txt.bak.")
        });
        assert!(backup_exists, "a .bak. backup file should exist");

        // Target should now be a symlink pointing to source
        let metadata = fs::symlink_metadata(&target).unwrap();
        assert!(
            metadata.file_type().is_symlink(),
            "target should now be a symlink"
        );
        let link_target = fs::read_link(&target).unwrap();
        assert_eq!(link_target, source, "symlink should point to source");
    }

    // -----------------------------------------------------------------------
    // T-024-4: backs up a symlink pointing elsewhere and redirects to new source
    // -----------------------------------------------------------------------
    #[test]
    fn test_symlink_wrong_symlink_backup() {
        let tmp = TempDir::new().unwrap();

        // Create two source files
        let source_a = tmp.path().join("source_a.txt");
        let source_b = tmp.path().join("source_b.txt");
        fs::write(&source_a, "content A").unwrap();
        fs::write(&source_b, "content B").unwrap();

        // Create a symlink at target pointing to source_a
        let target = tmp.path().join("target.txt");
        create_symlink(&source_a, &target).unwrap();

        // Job says target should point to source_b
        let jobs = vec![SymlinkJob {
            source_absolute: source_b.clone(),
            target_absolute: target.clone(),
        }];

        process_symlinks(&jobs).unwrap();

        // A .bak.{timestamp} should exist (was pointing to source_a)
        let entries: Vec<_> = fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        let backup_exists = entries.iter().any(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("target.txt.bak.")
        });
        assert!(backup_exists, "a .bak. backup of old symlink should exist");

        // Target should now point to source_b
        let link_target = fs::read_link(&target).unwrap();
        assert_eq!(link_target, source_b, "symlink should now point to source_b");
    }

    // -----------------------------------------------------------------------
    // T-024-5: creates parent directories if they don't exist
    // -----------------------------------------------------------------------
    #[test]
    fn test_symlink_creates_parent_dirs() {
        let tmp = TempDir::new().unwrap();

        // Create a source file
        let source = tmp.path().join("source.txt");
        fs::write(&source, "hello").unwrap();

        // Target with deeply nested non-existent parent dirs
        let target = tmp.path().join("deep").join("nested").join("target.txt");

        let jobs = vec![SymlinkJob {
            source_absolute: source.clone(),
            target_absolute: target.clone(),
        }];

        process_symlinks(&jobs).unwrap();

        // Parent directories should have been created
        assert!(
            target.parent().unwrap().exists(),
            "parent directories should have been created"
        );

        // Target should be a symlink pointing to source
        let link_target = fs::read_link(&target).unwrap();
        assert_eq!(link_target, source, "symlink should point to source");
    }
}
