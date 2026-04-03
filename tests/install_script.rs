use std::{path::PathBuf, process::Command};

use tempfile::tempdir;

#[test]
fn install_script_installs_binary_and_launches_help() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let install_script = repo_root.join("install.sh");

    let temp = tempdir().expect("temp dir");
    let home_dir = temp.path().join("home");
    let install_dir = temp.path().join("bin");
    std::fs::create_dir_all(&home_dir).expect("create home dir");

    let output = Command::new("bash")
        .arg(&install_script)
        .arg("--help")
        .env("HOME", &home_dir)
        .env("DOTFILES_INSTALL_DIR", &install_dir)
        .current_dir(&repo_root)
        .output()
        .expect("run install script");

    assert!(
        output.status.success(),
        "installer failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let installed_binary = install_dir.join("dotfiles");
    assert!(
        installed_binary.is_file(),
        "expected installed binary at {}",
        installed_binary.display()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "dotfiles");
    assert_contains(&stdout, "--help");
}

fn assert_contains(output: &str, expected: &str) {
    assert!(
        output.contains(expected),
        "expected output to contain {:?}\nactual output:\n{}",
        expected,
        output
    );
}
