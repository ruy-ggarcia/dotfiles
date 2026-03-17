#![allow(dead_code)]

use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// A single font entry from fonts/manifest.toml.
#[derive(Debug, Clone, Deserialize)]
pub struct FontEntry {
    pub name: String,
    /// Filename of the .ttf to extract from the archive (e.g. "MesloLGSNerdFont-Regular.ttf")
    pub file: String,
    /// URL of the .tar.xz archive on GitHub Releases
    pub url: String,
}

/// Internal wrapper that matches the TOML top-level table `[[fonts]]`.
#[derive(Debug, Deserialize)]
struct FontManifest {
    fonts: Vec<FontEntry>,
}

// ---------------------------------------------------------------------------
// Manifest loader
// ---------------------------------------------------------------------------

/// Reads and parses `fonts/manifest.toml` from the given repo root.
///
/// Returns the list of [`FontEntry`] items declared in the manifest.
pub fn load_font_manifest(repo_root: &Path) -> Result<Vec<FontEntry>> {
    let manifest_path = repo_root.join("fonts").join("manifest.toml");

    let raw = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("Cannot read font manifest at {}", manifest_path.display()))?;

    let manifest: FontManifest =
        toml::from_str(&raw).context("Failed to parse fonts/manifest.toml")?;

    Ok(manifest.fonts)
}

// ---------------------------------------------------------------------------
// Installation
// ---------------------------------------------------------------------------

/// Returns the OS-specific directory where fonts should be installed.
///
/// - macOS  → `~/Library/Fonts/`
/// - Linux  → `~/.local/share/fonts/`
fn font_dir() -> Result<std::path::PathBuf> {
    let home = std::env::var("HOME").context("HOME environment variable is not set")?;
    let path = match std::env::consts::OS {
        "macos" => std::path::PathBuf::from(&home)
            .join("Library")
            .join("Fonts"),
        _ => std::path::PathBuf::from(&home)
            .join(".local")
            .join("share")
            .join("fonts"),
    };
    Ok(path)
}

/// Downloads and installs each font in `fonts`.
///
/// For each entry:
/// 1. Download the `.tar.xz` archive via `curl -fsSL`.
/// 2. Extract the specific `.ttf` file with `tar -xf`.
/// 3. Copy the `.ttf` to the OS font directory.
/// 4. Clean up temporary files.
///
/// On Linux, `fc-cache -f` is run once after all fonts are installed.
pub fn install_fonts(fonts: &[FontEntry]) -> Result<()> {
    if fonts.is_empty() {
        return Ok(());
    }

    let dest_dir = font_dir()?;
    std::fs::create_dir_all(&dest_dir)
        .with_context(|| format!("Failed to create font directory {}", dest_dir.display()))?;

    let tmp_dir = std::env::temp_dir().join("dotfiles-fonts");
    std::fs::create_dir_all(&tmp_dir)
        .context("Failed to create temporary directory for font downloads")?;

    for font in fonts {
        install_single_font(font, &tmp_dir, &dest_dir)?;
    }

    // Clean up temp dir
    let _ = std::fs::remove_dir_all(&tmp_dir);

    // Refresh font cache on Linux
    if std::env::consts::OS == "linux" {
        let status = std::process::Command::new("fc-cache")
            .arg("-f")
            .status()
            .context("Failed to run fc-cache")?;

        if !status.success() {
            eprintln!("Warning: fc-cache -f exited with non-zero status");
        }
    }

    Ok(())
}

/// Downloads, extracts, and copies a single font to the destination directory.
fn install_single_font(font: &FontEntry, tmp_dir: &Path, dest_dir: &Path) -> Result<()> {
    let archive_path = tmp_dir.join(format!("{}.tar.xz", &font.name.replace(' ', "_")));

    // --- Download ---
    let dl_status = std::process::Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(&archive_path)
        .arg(&font.url)
        .status()
        .with_context(|| format!("Failed to run curl for font '{}'", font.name))?;

    if !dl_status.success() {
        anyhow::bail!(
            "curl failed (exit {}) while downloading font '{}' from {}",
            dl_status,
            font.name,
            font.url
        );
    }

    // --- Extract the specific .ttf ---
    let extract_status = std::process::Command::new("tar")
        .args(["-xf"])
        .arg(&archive_path)
        .arg(&font.file)
        .current_dir(tmp_dir)
        .status()
        .with_context(|| {
            format!(
                "Failed to run tar for font '{}' (file: {})",
                font.name, font.file
            )
        })?;

    if !extract_status.success() {
        anyhow::bail!(
            "tar failed (exit {}) while extracting '{}' from archive for font '{}'",
            extract_status,
            font.file,
            font.name
        );
    }

    // --- Copy .ttf to font directory ---
    let extracted_ttf = tmp_dir.join(&font.file);
    let target_ttf = dest_dir.join(&font.file);

    std::fs::copy(&extracted_ttf, &target_ttf).with_context(|| {
        format!(
            "Failed to copy '{}' to '{}'",
            extracted_ttf.display(),
            target_ttf.display()
        )
    })?;

    // --- Clean up per-font temps ---
    let _ = std::fs::remove_file(&archive_path);
    let _ = std::fs::remove_file(&extracted_ttf);

    println!("[✓] Installed {}", font.name);

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Finds the repo root by walking up from the current directory until we
    /// find a directory that contains `fonts/manifest.toml`.
    fn find_repo_root() -> std::path::PathBuf {
        let mut dir = std::env::current_dir().expect("cannot get cwd");
        loop {
            if dir.join("fonts").join("manifest.toml").exists() {
                return dir;
            }
            match dir.parent() {
                Some(parent) => dir = parent.to_path_buf(),
                None => panic!("Could not find repo root containing fonts/manifest.toml"),
            }
        }
    }

    #[test]
    fn test_load_font_manifest_returns_entries() {
        let repo_root = find_repo_root();
        let fonts = load_font_manifest(&repo_root)
            .expect("load_font_manifest should succeed with the real manifest");

        assert!(!fonts.is_empty(), "manifest must contain at least one font");

        for font in &fonts {
            assert!(!font.name.is_empty(), "font name must not be empty");
            assert!(
                font.file.ends_with(".ttf"),
                "font file '{}' must end with .ttf",
                font.file
            );
            assert!(
                font.url.starts_with("https://"),
                "font url '{}' must start with https://",
                font.url
            );
        }
    }

    #[test]
    fn test_load_font_manifest_known_entries() {
        let repo_root = find_repo_root();
        let fonts = load_font_manifest(&repo_root).expect("load_font_manifest should succeed");

        let names: Vec<&str> = fonts.iter().map(|f| f.name.as_str()).collect();

        assert!(
            names.contains(&"MesloLGS Nerd Font"),
            "manifest should contain MesloLGS Nerd Font"
        );
        assert!(
            names.contains(&"JetBrainsMono Nerd Font"),
            "manifest should contain JetBrainsMono Nerd Font"
        );
        assert!(
            names.contains(&"Iosevka Nerd Font"),
            "manifest should contain Iosevka Nerd Font"
        );
    }

    #[test]
    fn test_load_font_manifest_missing_file() {
        let result = load_font_manifest(Path::new("/nonexistent/path"));
        assert!(
            result.is_err(),
            "should return an error for a missing manifest file"
        );
    }
}
