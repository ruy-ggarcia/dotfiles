#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::models::{Module, Theme};

/// Reads `schema_path` and parses it as JSON.
pub fn load_schema(schema_path: &Path) -> Result<serde_json::Value> {
    let content = fs::read_to_string(schema_path)
        .with_context(|| format!("failed to read schema file: {}", schema_path.display()))?;
    let value = serde_json::from_str(&content)
        .with_context(|| format!("failed to parse JSON schema: {}", schema_path.display()))?;
    Ok(value)
}

/// Scans `modules_dir` for tool modules.
///
/// For each immediate subdirectory that contains a `packages.toml`, a [`Module`]
/// is built.  Subdirectories missing `packages.toml` are skipped with a warning.
pub fn scan_modules(modules_dir: &Path) -> Result<Vec<Module>> {
    let mut modules = Vec::new();

    let read_dir = fs::read_dir(modules_dir)
        .with_context(|| format!("failed to read modules directory: {}", modules_dir.display()))?;

    for entry in read_dir {
        let entry = entry.with_context(|| "failed to read directory entry")?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let packages_path = path.join("packages.toml");

        if !packages_path.exists() {
            eprintln!(
                "[warn] scanner: no packages.toml in '{}', skipping",
                path.display()
            );
            continue;
        }

        let content = match fs::read_to_string(&packages_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "[warn] scanner: failed to read '{}': {e}, skipping",
                    packages_path.display()
                );
                continue;
            }
        };

        let parsed: toml::Value = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "[warn] scanner: failed to parse '{}': {e}, skipping",
                    packages_path.display()
                );
                continue;
            }
        };

        // Extract the [packages] table
        let packages_by_os = extract_packages_by_os(&parsed);

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        modules.push(Module {
            name,
            path: path.clone(),
            packages_by_os,
        });
    }

    Ok(modules)
}

/// Scans `palettes_dir` for `.toml` theme palette files.
///
/// Each file is parsed and its keys are flattened into a `HashMap<String, String>`
/// suitable for use as a Tera context.  Files that fail to parse are skipped with
/// a warning.
pub fn scan_themes(palettes_dir: &Path) -> Result<Vec<Theme>> {
    let mut themes = Vec::new();

    let read_dir = fs::read_dir(palettes_dir)
        .with_context(|| format!("failed to read palettes directory: {}", palettes_dir.display()))?;

    for entry in read_dir {
        let entry = entry.with_context(|| "failed to read directory entry")?;
        let path = entry.path();

        // Only process .toml files
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "[warn] scanner: failed to read '{}': {e}, skipping",
                    path.display()
                );
                continue;
            }
        };

        let parsed: toml::Value = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "[warn] scanner: failed to parse '{}': {e}, skipping",
                    path.display()
                );
                continue;
            }
        };

        let mut variables: HashMap<String, String> = HashMap::new();
        flatten_toml(&parsed, "", &mut variables);

        // Theme name: meta.name key if present, otherwise filename without extension
        let name = variables
            .get("meta.name")
            .cloned()
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string()
            });

        themes.push(Theme {
            name,
            path: path.clone(),
            variables,
        });
    }

    Ok(themes)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extracts `[packages]` table from a parsed `packages.toml` value.
///
/// Returns a map from OS identifier (e.g. `"macos"`) to list of package names.
fn extract_packages_by_os(value: &toml::Value) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    let packages_table = match value.get("packages").and_then(|v| v.as_table()) {
        Some(t) => t,
        None => return map,
    };

    for (os, pkg_list) in packages_table {
        if let Some(arr) = pkg_list.as_array() {
            let pkgs: Vec<String> = arr
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
            map.insert(os.clone(), pkgs);
        }
    }

    map
}

/// Recursively flatten a [`toml::Value`] into dot-separated key/value pairs.
///
/// Only string leaf values are included; non-string primitives are converted
/// via `to_string()`.  Arrays are skipped (not representable as a single string).
fn flatten_toml(value: &toml::Value, prefix: &str, out: &mut HashMap<String, String>) {
    match value {
        toml::Value::Table(table) => {
            for (key, child) in table {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_toml(child, &new_prefix, out);
            }
        }
        toml::Value::Array(_) => {
            // Arrays are intentionally not flattened into the Tera string context
        }
        toml::Value::String(s) => {
            out.insert(prefix.to_string(), s.clone());
        }
        toml::Value::Integer(i) => {
            out.insert(prefix.to_string(), i.to_string());
        }
        toml::Value::Float(f) => {
            out.insert(prefix.to_string(), f.to_string());
        }
        toml::Value::Boolean(b) => {
            out.insert(prefix.to_string(), b.to_string());
        }
        toml::Value::Datetime(dt) => {
            out.insert(prefix.to_string(), dt.to_string());
        }
    }
}

// ---------------------------------------------------------------------------
// Tests (T-013)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // -----------------------------------------------------------------------
    // T-013-1: scan_modules happy path
    // -----------------------------------------------------------------------
    #[test]
    fn test_scan_modules_happy_path() {
        let tmp = TempDir::new().unwrap();
        let module_dir = tmp.path().join("testmod");
        fs::create_dir_all(&module_dir).unwrap();
        fs::write(
            module_dir.join("packages.toml"),
            "[packages]\nmacos = [\"zsh\"]\nubuntu = [\"zsh\"]\n",
        )
        .unwrap();

        let modules = scan_modules(tmp.path()).unwrap();

        assert_eq!(modules.len(), 1);
        let m = &modules[0];
        assert_eq!(m.name, "testmod");
        assert_eq!(m.path, module_dir);
        assert_eq!(m.packages_by_os["macos"], vec!["zsh"]);
        assert_eq!(m.packages_by_os["ubuntu"], vec!["zsh"]);
    }

    // -----------------------------------------------------------------------
    // T-013-2: scan_modules skips dirs without packages.toml
    // -----------------------------------------------------------------------
    #[test]
    fn test_scan_modules_missing_packages_toml() {
        let tmp = TempDir::new().unwrap();
        let module_dir = tmp.path().join("nopackages");
        fs::create_dir_all(&module_dir).unwrap();
        // No packages.toml written

        let modules = scan_modules(tmp.path()).unwrap();

        assert_eq!(modules.len(), 0, "should skip dirs without packages.toml");
    }

    // -----------------------------------------------------------------------
    // T-013-3: scan_themes happy path
    // -----------------------------------------------------------------------
    #[test]
    fn test_scan_themes_happy_path() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("test-theme.toml"),
            "[meta]\nname = \"Test\"\n\n[colors.core]\nbackground = \"#000000\"\n",
        )
        .unwrap();

        let themes = scan_themes(tmp.path()).unwrap();

        assert_eq!(themes.len(), 1);
        let t = &themes[0];
        assert_eq!(t.name, "Test");
        assert_eq!(t.variables["meta.name"], "Test");
        assert_eq!(t.variables["colors.core.background"], "#000000");
    }

    // -----------------------------------------------------------------------
    // T-013-4: scan_themes skips invalid TOML files
    // -----------------------------------------------------------------------
    #[test]
    fn test_scan_themes_invalid_toml() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("bad.toml"), "this is ][[ not valid toml").unwrap();

        let themes = scan_themes(tmp.path()).unwrap();

        assert_eq!(themes.len(), 0, "should skip files that fail to parse");
    }

    // -----------------------------------------------------------------------
    // T-012: load_schema
    // -----------------------------------------------------------------------
    #[test]
    fn test_load_schema_valid() {
        let tmp = TempDir::new().unwrap();
        let schema_file = tmp.path().join("schema.json");
        fs::write(&schema_file, r#"{"type": "object"}"#).unwrap();

        let schema = load_schema(&schema_file).unwrap();

        assert_eq!(schema["type"], "object");
    }

    #[test]
    fn test_load_schema_missing_file() {
        let tmp = TempDir::new().unwrap();
        let schema_file = tmp.path().join("nonexistent.json");

        assert!(load_schema(&schema_file).is_err());
    }
}
