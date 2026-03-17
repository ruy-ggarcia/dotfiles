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

    let read_dir = fs::read_dir(modules_dir).with_context(|| {
        format!(
            "failed to read modules directory: {}",
            modules_dir.display()
        )
    })?;

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
/// Each file is parsed, validated against the palette schema contract (36 required
/// keys: 5 meta + 31 hex colors), and its keys are flattened into a
/// `HashMap<String, String>` suitable for use as a Tera context.  Files that fail
/// to parse or do not satisfy the schema are skipped with a warning.
pub fn scan_themes(palettes_dir: &Path) -> Result<Vec<Theme>> {
    let mut themes = Vec::new();

    let read_dir = fs::read_dir(palettes_dir).with_context(|| {
        format!(
            "failed to read palettes directory: {}",
            palettes_dir.display()
        )
    })?;

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

        // Validate the palette structure before accepting it
        if let Err(reason) = validate_palette(&parsed) {
            eprintln!(
                "[warn] scanner: '{}' failed schema validation: {reason}, skipping",
                path.display()
            );
            continue;
        }

        let mut variables: HashMap<String, String> = HashMap::new();
        flatten_toml(&parsed, "", &mut variables);

        // Theme name: meta.name key if present, otherwise filename without extension
        let name = variables.get("meta.name").cloned().unwrap_or_else(|| {
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
// Palette schema validation
// ---------------------------------------------------------------------------

/// Required keys under `[meta]`.
const META_REQUIRED: &[&str] = &[
    "name",
    "variant",
    "nvim_theme",
    "nvim_plugin",
    "nvim_variant",
];

/// Required keys under `[colors.core]`.
const COLORS_CORE_REQUIRED: &[&str] = &[
    "background",
    "foreground",
    "cursor_bg",
    "cursor_fg",
    "selection_bg",
    "selection_fg",
    "url",
];

/// Required keys under `[colors.ansi.normal]` and `[colors.ansi.bright]`.
const COLORS_ANSI_REQUIRED: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
];

/// Required keys under `[colors.ui]`.
const COLORS_UI_REQUIRED: &[&str] = &[
    "border_active",
    "border_inactive",
    "status_bg",
    "status_fg",
    "tab_active_bg",
    "tab_active_fg",
    "tab_inactive_bg",
    "tab_inactive_fg",
];

/// Returns `true` if `s` matches the hex-color pattern `^#[0-9a-fA-F]{6}$`.
fn is_hex_color(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.len() == 7 && bytes[0] == b'#' && bytes[1..].iter().all(|b| b.is_ascii_hexdigit())
}

/// Validates a parsed palette `toml::Value` against the schema contract.
///
/// Returns `Ok(())` if the palette is valid, or `Err(reason)` with a human-readable
/// description of the first violation found.
fn validate_palette(value: &toml::Value) -> Result<(), String> {
    // --- [meta] section ---
    let meta = value
        .get("meta")
        .and_then(|v| v.as_table())
        .ok_or_else(|| "missing [meta] section".to_string())?;

    for key in META_REQUIRED {
        if !meta.contains_key(*key) {
            return Err(format!("missing meta.{key}"));
        }
    }

    // meta.variant must be "dark" or "light"
    match meta.get("variant").and_then(|v| v.as_str()) {
        Some("dark") | Some("light") => {}
        Some(other) => {
            return Err(format!(
                "meta.variant must be \"dark\" or \"light\", got \"{other}\""
            ));
        }
        None => return Err("meta.variant is not a string".to_string()),
    }

    // --- [colors] section ---
    let colors = value
        .get("colors")
        .and_then(|v| v.as_table())
        .ok_or_else(|| "missing [colors] section".to_string())?;

    // colors.core
    let core = colors
        .get("core")
        .and_then(|v| v.as_table())
        .ok_or_else(|| "missing [colors.core] section".to_string())?;

    for key in COLORS_CORE_REQUIRED {
        match core.get(*key).and_then(|v| v.as_str()) {
            None => return Err(format!("missing colors.core.{key}")),
            Some(val) if !is_hex_color(val) => {
                return Err(format!("colors.core.{key} has invalid hex color \"{val}\""));
            }
            _ => {}
        }
    }

    // colors.ansi.normal and colors.ansi.bright
    let ansi = colors
        .get("ansi")
        .and_then(|v| v.as_table())
        .ok_or_else(|| "missing [colors.ansi] section".to_string())?;

    for sub in &["normal", "bright"] {
        let table = ansi
            .get(*sub)
            .and_then(|v| v.as_table())
            .ok_or_else(|| format!("missing [colors.ansi.{sub}] section"))?;

        for key in COLORS_ANSI_REQUIRED {
            match table.get(*key).and_then(|v| v.as_str()) {
                None => return Err(format!("missing colors.ansi.{sub}.{key}")),
                Some(val) if !is_hex_color(val) => {
                    return Err(format!(
                        "colors.ansi.{sub}.{key} has invalid hex color \"{val}\""
                    ));
                }
                _ => {}
            }
        }
    }

    // colors.ui
    let ui = colors
        .get("ui")
        .and_then(|v| v.as_table())
        .ok_or_else(|| "missing [colors.ui] section".to_string())?;

    for key in COLORS_UI_REQUIRED {
        match ui.get(*key).and_then(|v| v.as_str()) {
            None => return Err(format!("missing colors.ui.{key}")),
            Some(val) if !is_hex_color(val) => {
                return Err(format!("colors.ui.{key} has invalid hex color \"{val}\""));
            }
            _ => {}
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // -----------------------------------------------------------------------
    // scan_modules happy path
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
    // scan_modules skips dirs without packages.toml
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
    // Helpers shared by multiple tests
    // -----------------------------------------------------------------------

    /// Returns a TOML string that satisfies every required key in the schema.
    fn full_valid_palette_toml() -> &'static str {
        r##"
[meta]
name        = "Test Dark"
variant     = "dark"
nvim_theme  = "test"
nvim_plugin = "test.nvim"
nvim_variant = "dark"

[colors.core]
background   = "#1e1e2e"
foreground   = "#cdd6f4"
cursor_bg    = "#f5e0dc"
cursor_fg    = "#1e1e2e"
selection_bg = "#585b70"
selection_fg = "#cdd6f4"
url          = "#89b4fa"

[colors.ansi.normal]
black   = "#45475a"
red     = "#f38ba8"
green   = "#a6e3a1"
yellow  = "#f9e2af"
blue    = "#89b4fa"
magenta = "#f5c2e7"
cyan    = "#94e2d5"
white   = "#bac2de"

[colors.ansi.bright]
black   = "#585b70"
red     = "#f38ba8"
green   = "#a6e3a1"
yellow  = "#f9e2af"
blue    = "#89b4fa"
magenta = "#f5c2e7"
cyan    = "#94e2d5"
white   = "#a6adc8"

[colors.ui]
border_active    = "#89b4fa"
border_inactive  = "#45475a"
status_bg        = "#181825"
status_fg        = "#cdd6f4"
tab_active_bg    = "#89b4fa"
tab_active_fg    = "#1e1e2e"
tab_inactive_bg  = "#181825"
tab_inactive_fg  = "#585b70"
"##
    }

    // -----------------------------------------------------------------------
    // scan_themes happy path — full valid palette
    // -----------------------------------------------------------------------
    #[test]
    fn test_scan_themes_happy_path() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("test-theme.toml"),
            full_valid_palette_toml(),
        )
        .unwrap();

        let themes = scan_themes(tmp.path()).unwrap();

        assert_eq!(themes.len(), 1);
        let t = &themes[0];
        assert_eq!(t.name, "Test Dark");
        assert_eq!(t.variables["meta.name"], "Test Dark");
        assert_eq!(t.variables["colors.core.background"], "#1e1e2e");
        assert_eq!(t.variables["meta.variant"], "dark");
    }

    // -----------------------------------------------------------------------
    // scan_themes skips invalid TOML files
    // -----------------------------------------------------------------------
    #[test]
    fn test_scan_themes_invalid_toml() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("bad.toml"), "this is ][[ not valid toml").unwrap();

        let themes = scan_themes(tmp.path()).unwrap();

        assert_eq!(themes.len(), 0, "should skip files that fail to parse");
    }

    // -----------------------------------------------------------------------
    // scan_themes skips palettes that fail schema validation
    // -----------------------------------------------------------------------
    #[test]
    fn test_scan_themes_skips_invalid_palette() {
        let tmp = TempDir::new().unwrap();
        // Valid TOML but missing most required keys
        fs::write(
            tmp.path().join("incomplete.toml"),
            "[meta]\nname = \"Incomplete\"\n\n[colors.core]\nbackground = \"#000000\"\n",
        )
        .unwrap();

        let themes = scan_themes(tmp.path()).unwrap();

        assert_eq!(themes.len(), 0, "should skip palettes that fail validation");
    }

    // -----------------------------------------------------------------------
    // validate_palette — full valid palette passes
    // -----------------------------------------------------------------------
    #[test]
    fn test_validate_palette_valid() {
        let parsed: toml::Value = toml::from_str(full_valid_palette_toml()).unwrap();
        assert!(validate_palette(&parsed).is_ok());
    }

    // -----------------------------------------------------------------------
    // validate_palette — missing meta section
    // -----------------------------------------------------------------------
    #[test]
    fn test_validate_palette_missing_meta() {
        let parsed: toml::Value =
            toml::from_str("[colors.core]\nbackground = \"#000000\"\n").unwrap();
        let err = validate_palette(&parsed).unwrap_err();
        assert!(err.contains("meta"), "error should mention meta: {err}");
    }

    // -----------------------------------------------------------------------
    // validate_palette — missing individual meta key
    // -----------------------------------------------------------------------
    #[test]
    fn test_validate_palette_missing_meta_key() {
        // Has meta but missing nvim_theme
        let toml_str = r##"
[meta]
name     = "X"
variant  = "dark"
nvim_plugin  = "x.nvim"
nvim_variant = "dark"
# nvim_theme intentionally omitted
[colors.core]
background = "#000000"
"##;
        let parsed: toml::Value = toml::from_str(toml_str).unwrap();
        let err = validate_palette(&parsed).unwrap_err();
        assert!(
            err.contains("nvim_theme"),
            "error should name missing key: {err}"
        );
    }

    // -----------------------------------------------------------------------
    // validate_palette — invalid meta.variant value
    // -----------------------------------------------------------------------
    #[test]
    fn test_validate_palette_invalid_variant() {
        let mut toml_str = full_valid_palette_toml().to_string();
        toml_str = toml_str.replace("variant     = \"dark\"", "variant     = \"solarized\"");
        let parsed: toml::Value = toml::from_str(&toml_str).unwrap();
        let err = validate_palette(&parsed).unwrap_err();
        assert!(
            err.contains("variant"),
            "error should mention variant: {err}"
        );
        assert!(
            err.contains("solarized"),
            "error should echo the bad value: {err}"
        );
    }

    // -----------------------------------------------------------------------
    // validate_palette — invalid hex color format
    // -----------------------------------------------------------------------
    #[test]
    fn test_validate_palette_invalid_hex_color() {
        let mut toml_str = full_valid_palette_toml().to_string();
        // Replace a valid hex with a bad value (no leading #)
        toml_str = toml_str.replace("background   = \"#1e1e2e\"", "background   = \"1e1e2e\"");
        let parsed: toml::Value = toml::from_str(&toml_str).unwrap();
        let err = validate_palette(&parsed).unwrap_err();
        assert!(
            err.contains("background"),
            "error should name the offending key: {err}"
        );
        assert!(
            err.contains("1e1e2e"),
            "error should echo the bad value: {err}"
        );
    }

    // -----------------------------------------------------------------------
    // validate_palette — missing colors section
    // -----------------------------------------------------------------------
    #[test]
    fn test_validate_palette_missing_colors() {
        let toml_str = r##"
[meta]
name         = "X"
variant      = "dark"
nvim_theme   = "x"
nvim_plugin  = "x.nvim"
nvim_variant = "dark"
"##;
        let parsed: toml::Value = toml::from_str(toml_str).unwrap();
        let err = validate_palette(&parsed).unwrap_err();
        assert!(err.contains("colors"), "error should mention colors: {err}");
    }

    // -----------------------------------------------------------------------
    // validate_palette — missing color key in colors.ansi.bright
    // -----------------------------------------------------------------------
    #[test]
    fn test_validate_palette_missing_ansi_bright_key() {
        let mut toml_str = full_valid_palette_toml().to_string();
        // Remove the magenta line from bright section
        toml_str = toml_str.replace(
            "[colors.ansi.bright]\nblack   = \"#585b70\"\nred     = \"#f38ba8\"\ngreen   = \"#a6e3a1\"\nyellow  = \"#f9e2af\"\nblue    = \"#89b4fa\"\nmagenta = \"#f5c2e7\"\ncyan    = \"#94e2d5\"\nwhite   = \"#a6adc8\"",
            "[colors.ansi.bright]\nblack   = \"#585b70\"\nred     = \"#f38ba8\"\ngreen   = \"#a6e3a1\"\nyellow  = \"#f9e2af\"\nblue    = \"#89b4fa\"\ncyan    = \"#94e2d5\"\nwhite   = \"#a6adc8\"",
        );
        let parsed: toml::Value = toml::from_str(&toml_str).unwrap();
        let err = validate_palette(&parsed).unwrap_err();
        assert!(
            err.contains("bright") && err.contains("magenta"),
            "error should name bright.magenta: {err}"
        );
    }

    // -----------------------------------------------------------------------
    // T-055 — all 10 real palette files pass validate_palette()
    // -----------------------------------------------------------------------
    #[test]
    fn test_all_palettes_pass_validation() {
        // CARGO_MANIFEST_DIR = installer/ at test time; palettes live one level up.
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set by cargo test");
        let palettes_dir = std::path::Path::new(&manifest_dir).join("../themes/palettes");

        let entries: Vec<_> = std::fs::read_dir(&palettes_dir)
            .unwrap_or_else(|e| panic!("failed to read palettes dir {:?}: {e}", palettes_dir))
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("toml"))
            .collect();

        assert_eq!(
            entries.len(),
            10,
            "expected exactly 10 palette TOML files, found {}",
            entries.len()
        );

        let mut passed = Vec::new();
        let mut failed = Vec::new();

        for entry in &entries {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed to read {filename}: {e}"));
            let parsed: toml::Value = toml::from_str(&content)
                .unwrap_or_else(|e| panic!("{filename} is not valid TOML: {e}"));

            match validate_palette(&parsed) {
                Ok(()) => {
                    println!("[PASS] {filename}");
                    passed.push(filename);
                }
                Err(reason) => {
                    eprintln!("[FAIL] {filename}: {reason}");
                    failed.push((filename, reason));
                }
            }
        }

        if !failed.is_empty() {
            let report = failed
                .iter()
                .map(|(f, r)| format!("  {f}: {r}"))
                .collect::<Vec<_>>()
                .join("\n");
            panic!("{} palette(s) failed validation:\n{report}", failed.len());
        }

        println!("All {} palettes passed validation:", passed.len());
        for name in &passed {
            println!("  ✓ {name}");
        }
    }

    // -----------------------------------------------------------------------
    // is_hex_color helper
    // -----------------------------------------------------------------------
    #[test]
    fn test_is_hex_color_valid() {
        assert!(is_hex_color("#000000"));
        assert!(is_hex_color("#FFFFFF"));
        assert!(is_hex_color("#1e1e2e"));
        assert!(is_hex_color("#aAbBcC"));
    }

    #[test]
    fn test_is_hex_color_invalid() {
        assert!(!is_hex_color("000000")); // missing #
        assert!(!is_hex_color("#00000")); // too short
        assert!(!is_hex_color("#0000000")); // too long
        assert!(!is_hex_color("#gggggg")); // invalid chars
        assert!(!is_hex_color("#12345g")); // trailing invalid char
        assert!(!is_hex_color(""));
    }

    // -----------------------------------------------------------------------
    // load_schema
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
