#![allow(dead_code)]

use std::collections::HashSet;
use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

use crate::models::{Plan, SymlinkJob, TemplateJob, Theme, UserSelection};
use crate::{package, symlink, template};

/// Determines the OS identifier used to look up packages.
///
/// Returns `"macos"` on macOS, `"ubuntu"` on Linux.
fn os_identifier() -> &'static str {
    match std::env::consts::OS {
        "macos" => "macos",
        "linux" => "ubuntu",
        other => other,
    }
}

/// Generates an execution `Plan` from a `UserSelection`.
///
/// # Steps
/// 1. Determine the current OS identifier.
/// 2. For each selected module, collect (and deduplicate) packages for the current OS.
/// 3. Walk the `home/` and `config/` subtrees of each selected module:
///    - `.tera` files → `TemplateJob` + `SymlinkJob` (via `.rendered/`)
///    - Static files  → direct `SymlinkJob`
pub fn generate_plan(selection: &UserSelection, repo_root: &Path) -> Result<Plan> {
    let os = os_identifier();
    let home_dir = std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("/tmp"));

    let rendered_root = repo_root.join(".rendered");

    let mut packages_seen: HashSet<String> = HashSet::new();
    let mut packages_to_install: Vec<String> = Vec::new();
    let mut templates_to_render: Vec<TemplateJob> = Vec::new();
    let mut symlinks_to_create: Vec<SymlinkJob> = Vec::new();

    for module in &selection.selected_modules {
        // Collect deduplicated packages for current OS
        if let Some(pkgs) = module.packages_by_os.get(os) {
            for pkg in pkgs {
                if packages_seen.insert(pkg.clone()) {
                    packages_to_install.push(pkg.clone());
                }
            }
        }

        // Walk home/ subtree → target: $HOME/<relative>
        let home_subtree = module.path.join("home");
        if home_subtree.is_dir() {
            for entry in WalkDir::new(&home_subtree)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                let src = entry.path().to_path_buf();
                let relative = src
                    .strip_prefix(&home_subtree)
                    .expect("walkdir entry must be under home_subtree")
                    .to_path_buf();
                let target = home_dir.join(&relative);

                plan_file(
                    src,
                    target,
                    &relative,
                    &rendered_root,
                    "home",
                    &module.name,
                    &mut templates_to_render,
                    &mut symlinks_to_create,
                );
            }
        }

        // Walk config/ subtree → target: $HOME/.config/<relative>
        let config_subtree = module.path.join("config");
        if config_subtree.is_dir() {
            for entry in WalkDir::new(&config_subtree)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                let src = entry.path().to_path_buf();
                let relative = src
                    .strip_prefix(&config_subtree)
                    .expect("walkdir entry must be under config_subtree")
                    .to_path_buf();
                let target = home_dir.join(".config").join(&relative);

                plan_file(
                    src,
                    target,
                    &relative,
                    &rendered_root,
                    "config",
                    &module.name,
                    &mut templates_to_render,
                    &mut symlinks_to_create,
                );
            }
        }
    }

    Ok(Plan {
        packages_to_install,
        templates_to_render,
        symlinks_to_create,
    })
}

/// Decides whether a file is a Tera template or static, and pushes the
/// appropriate jobs into the mutable vecs.
#[allow(clippy::too_many_arguments)]
fn plan_file(
    src: std::path::PathBuf,
    target: std::path::PathBuf,
    relative: &std::path::Path,
    rendered_root: &std::path::Path,
    subtree: &str, // "home" or "config"
    module_name: &str,
    templates: &mut Vec<TemplateJob>,
    symlinks: &mut Vec<SymlinkJob>,
) {
    let is_tera = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e == "tera")
        .unwrap_or(false);

    if is_tera {
        // Strip the .tera extension from the relative path for the rendered destination
        let rendered_relative = relative.with_extension("");
        // Rendered file lives under .rendered/<module>/<subtree>/<relative_without_tera>
        let destination = rendered_root
            .join(module_name)
            .join(subtree)
            .join(&rendered_relative);

        // Re-derive target without .tera extension
        // e.g. "test.conf.tera" → file_stem() = "test.conf"
        let target_no_tera = {
            let stem = relative.file_stem().unwrap_or_default();
            target
                .parent()
                .map(|p| p.join(stem))
                .unwrap_or_else(|| std::path::PathBuf::from(stem))
        };

        templates.push(TemplateJob {
            source: src,
            destination: destination.clone(),
        });

        symlinks.push(SymlinkJob {
            source_absolute: destination,
            target_absolute: target_no_tera,
        });
    } else {
        // Static file: symlink directly from modules/
        symlinks.push(SymlinkJob {
            source_absolute: src,
            target_absolute: target,
        });
    }
}

/// Executes the three phases of a `Plan` in order:
/// 1. Package installation
/// 2. Template rendering
/// 3. Symlink creation
pub fn execute_plan(plan: &Plan, theme: &Theme) -> Result<()> {
    println!("[✓] Installing packages...");
    package::get_package_manager()?.install(&plan.packages_to_install)?;

    println!("[✓] Rendering templates...");
    template::render_templates(&plan.templates_to_render, theme)?;

    println!("[✓] Creating symlinks...");
    symlink::process_symlinks(&plan.symlinks_to_create)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::TempDir;

    use crate::models::{Module, Theme, UserSelection};

    /// Helper: build a minimal Module pointing at a temp dir.
    fn make_module(
        name: &str,
        path: std::path::PathBuf,
        packages_by_os: HashMap<String, Vec<String>>,
    ) -> Module {
        Module {
            name: name.to_string(),
            path,
            packages_by_os,
        }
    }

    /// Helper: minimal no-op Theme for tests that don't actually render.
    fn make_theme() -> Theme {
        Theme {
            name: "test".to_string(),
            path: std::path::PathBuf::from("/dev/null"),
            variables: HashMap::new(),
        }
    }

    // -----------------------------------------------------------------------
    // static file in home/ → 1 SymlinkJob, 0 TemplateJobs
    // -----------------------------------------------------------------------
    #[test]
    fn test_generate_plan_static_file() {
        let tmp = TempDir::new().unwrap();
        let repo_root = tmp.path().to_path_buf();

        // Create modules/testmod/home/.testrc
        let module_dir = repo_root.join("modules").join("testmod");
        let home_dir = module_dir.join("home");
        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".testrc"), "# test config").unwrap();

        let module = make_module("testmod", module_dir, HashMap::new());
        let selection = UserSelection {
            selected_modules: vec![module],
            selected_theme: make_theme(),
        };

        let plan = generate_plan(&selection, &repo_root).unwrap();

        assert_eq!(
            plan.templates_to_render.len(),
            0,
            "no template jobs for static file"
        );
        assert_eq!(plan.symlinks_to_create.len(), 1, "exactly 1 symlink job");

        let job = &plan.symlinks_to_create[0];

        // source should end with .testrc
        assert!(
            job.source_absolute.ends_with(".testrc"),
            "source should be the static file, got: {}",
            job.source_absolute.display()
        );

        // target should be $HOME/.testrc
        let expected_target =
            std::path::PathBuf::from(std::env::var("HOME").unwrap()).join(".testrc");
        assert_eq!(
            job.target_absolute, expected_target,
            "target should be $HOME/.testrc"
        );
    }

    // -----------------------------------------------------------------------
    // .tera file in config/ → 1 TemplateJob + 1 SymlinkJob
    // -----------------------------------------------------------------------
    #[test]
    fn test_generate_plan_tera_file() {
        let tmp = TempDir::new().unwrap();
        let repo_root = tmp.path().to_path_buf();

        // Create modules/testmod/config/testapp/test.conf.tera
        let module_dir = repo_root.join("modules").join("testmod");
        let config_dir = module_dir.join("config").join("testapp");
        fs::create_dir_all(&config_dir).unwrap();
        fs::write(config_dir.join("test.conf.tera"), "# {{ meta.name }}").unwrap();

        let module = make_module("testmod", module_dir, HashMap::new());
        let selection = UserSelection {
            selected_modules: vec![module],
            selected_theme: make_theme(),
        };

        let plan = generate_plan(&selection, &repo_root).unwrap();

        assert_eq!(plan.templates_to_render.len(), 1, "exactly 1 template job");
        assert_eq!(plan.symlinks_to_create.len(), 1, "exactly 1 symlink job");

        let tmpl = &plan.templates_to_render[0];
        // source should be the .tera file
        assert!(
            tmpl.source.to_string_lossy().ends_with("test.conf.tera"),
            "template source should be .tera file, got: {}",
            tmpl.source.display()
        );
        // destination should be in .rendered/ and NOT have .tera extension
        assert!(
            tmpl.destination.to_string_lossy().contains(".rendered"),
            "template destination should be in .rendered/, got: {}",
            tmpl.destination.display()
        );
        assert!(
            !tmpl.destination.to_string_lossy().ends_with(".tera"),
            "template destination should NOT have .tera extension"
        );
        assert!(
            tmpl.destination.to_string_lossy().ends_with("test.conf"),
            "template destination should end with test.conf, got: {}",
            tmpl.destination.display()
        );

        // symlink source should be the rendered file (in .rendered/)
        let sym = &plan.symlinks_to_create[0];
        assert!(
            sym.source_absolute.to_string_lossy().contains(".rendered"),
            "symlink source should be in .rendered/, got: {}",
            sym.source_absolute.display()
        );
        // symlink target should be $HOME/.config/testapp/test.conf (no .tera)
        let expected_target = std::path::PathBuf::from(std::env::var("HOME").unwrap())
            .join(".config")
            .join("testapp")
            .join("test.conf");
        assert_eq!(
            sym.target_absolute, expected_target,
            "symlink target should be $HOME/.config/testapp/test.conf"
        );
    }

    // -----------------------------------------------------------------------
    // packages are deduplicated across modules
    // -----------------------------------------------------------------------
    #[test]
    fn test_generate_plan_deduplicates_packages() {
        let tmp = TempDir::new().unwrap();
        let repo_root = tmp.path().to_path_buf();

        // Create two modules that share the package "git"
        let os_id = os_identifier();

        let mod_a_dir = repo_root.join("modules").join("mod_a");
        fs::create_dir_all(&mod_a_dir).unwrap();
        let mut pkgs_a = HashMap::new();
        pkgs_a.insert(
            os_id.to_string(),
            vec!["git".to_string(), "curl".to_string()],
        );

        let mod_b_dir = repo_root.join("modules").join("mod_b");
        fs::create_dir_all(&mod_b_dir).unwrap();
        let mut pkgs_b = HashMap::new();
        pkgs_b.insert(
            os_id.to_string(),
            vec!["git".to_string(), "vim".to_string()],
        );

        let modules = vec![
            make_module("mod_a", mod_a_dir, pkgs_a),
            make_module("mod_b", mod_b_dir, pkgs_b),
        ];

        let selection = UserSelection {
            selected_modules: modules,
            selected_theme: make_theme(),
        };

        let plan = generate_plan(&selection, &repo_root).unwrap();

        // git appears in both modules — should only be in the list once
        let git_count = plan
            .packages_to_install
            .iter()
            .filter(|p| p.as_str() == "git")
            .count();
        assert_eq!(
            git_count, 1,
            "git should appear exactly once (deduplicated)"
        );

        // All three unique packages should be present
        assert!(plan.packages_to_install.contains(&"git".to_string()));
        assert!(plan.packages_to_install.contains(&"curl".to_string()));
        assert!(plan.packages_to_install.contains(&"vim".to_string()));
        assert_eq!(
            plan.packages_to_install.len(),
            3,
            "should have exactly 3 unique packages"
        );
    }
}
