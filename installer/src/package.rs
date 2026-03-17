#![allow(dead_code)]

use anyhow::Result;

pub trait PackageManager {
    /// Check if the package manager binary exists on the system
    fn is_available(&self) -> bool;

    /// Update local package index (e.g., apt-get update, brew update)
    fn update_index(&self) -> Result<()>;

    /// Install packages idempotently (skip already installed)
    fn install(&self, packages: &[String]) -> Result<()>;
}

pub struct Brew;
pub struct Apt;

// ---------------------------------------------------------------------------
// Brew — macOS Homebrew
// ---------------------------------------------------------------------------

impl PackageManager for Brew {
    fn is_available(&self) -> bool {
        std::process::Command::new("which")
            .arg("brew")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn update_index(&self) -> Result<()> {
        let output = std::process::Command::new("brew")
            .arg("update")
            .output()
            .map_err(|e| anyhow::anyhow!("failed to run brew update: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("brew update failed: {}", stderr);
        }

        Ok(())
    }

    fn install(&self, packages: &[String]) -> Result<()> {
        if packages.is_empty() {
            return Ok(());
        }

        let output = std::process::Command::new("brew")
            .arg("install")
            .args(packages)
            .output()
            .map_err(|e| anyhow::anyhow!("failed to run brew install: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("brew install failed: {}", stderr);
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Apt — Debian/Ubuntu apt-get
// ---------------------------------------------------------------------------

impl PackageManager for Apt {
    fn is_available(&self) -> bool {
        std::process::Command::new("which")
            .arg("apt-get")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn update_index(&self) -> Result<()> {
        let output = std::process::Command::new("sudo")
            .args(["apt-get", "update"])
            .output()
            .map_err(|e| anyhow::anyhow!("failed to run apt-get update: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("apt-get update failed: {}", stderr);
        }

        Ok(())
    }

    fn install(&self, packages: &[String]) -> Result<()> {
        if packages.is_empty() {
            return Ok(());
        }

        let output = std::process::Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .args(packages)
            .output()
            .map_err(|e| anyhow::anyhow!("failed to run apt-get install: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("apt-get install failed: {}", stderr);
        }

        Ok(())
    }
}

pub fn get_package_manager() -> Result<Box<dyn PackageManager>> {
    match std::env::consts::OS {
        "macos" => Ok(Box::new(Brew)),
        "linux" => Ok(Box::new(Apt)),
        os => anyhow::bail!("Unsupported OS: {}", os),
    }
}

// ---------------------------------------------------------------------------
// Tests (T-017) — written BEFORE implementation (RED phase)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // T-017-1: get_package_manager() succeeds on macOS or Linux
    #[test]
    fn test_get_package_manager_returns_some() {
        let result = get_package_manager();
        assert!(
            result.is_ok(),
            "get_package_manager() should succeed on macOS or Linux, got: {:?}",
            result.err()
        );
    }

    // T-017-2: is_available() returns true (brew expected on dev Mac)
    #[test]
    fn test_brew_or_apt_is_available() {
        let pm = get_package_manager().expect("should get a package manager");
        assert!(
            pm.is_available(),
            "package manager should be available on this machine"
        );
    }

    // T-017-3: factory doesn't error on the current platform
    #[test]
    fn test_get_package_manager_os_detection() {
        // On macOS we expect Brew, on Linux we expect Apt.
        // We just verify it doesn't return an error — we can't mock consts::OS.
        let result = get_package_manager();
        assert!(result.is_ok(), "factory should not error on supported OS");
    }
}
