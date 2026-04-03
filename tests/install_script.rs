use std::{
    fs,
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};

use tempfile::tempdir;

#[test]
fn install_script_installs_stubbed_binary_and_forwards_help() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let install_script = repo_root.join("install.sh");
    let temp = tempdir().expect("temp dir");
    let env = TestEnv::new(temp.path(), stub_binary_for_help()).expect("test env");
    let launch_script = write_script(
        temp.path().join("launch-help.sh"),
        &format!("exec bash \"{}\" --help\n", install_script.display()),
    )
    .expect("launch script");

    let output = run_with_terminal(&launch_script, &env);

    assert_success(&output, "installer should succeed for --help");
    assert!(
        env.install_dir.join("dotfiles").is_file(),
        "expected installed binary at {}",
        env.install_dir.join("dotfiles").display()
    );

    let combined = combined_output(&output);
    assert_contains(&combined, "stub-dotfiles");
    assert_contains(&combined, "--help");
}

#[test]
fn piped_installer_reacquires_tty_before_launching_child() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let install_script = repo_root.join("install.sh");
    let temp = tempdir().expect("temp dir");
    let tty_marker = temp.path().join("tty-ok.marker");
    let env =
        TestEnv::new(temp.path(), stub_binary_that_requires_tty(&tty_marker)).expect("test env");
    let launch_script = write_script(
        temp.path().join("launch-piped.sh"),
        &format!("cat \"{}\" | bash -s --\n", install_script.display()),
    )
    .expect("launch script");

    let output = run_with_terminal(&launch_script, &env);

    assert_success(&output, "piped installer should recover a TTY");
    assert!(
        tty_marker.is_file(),
        "expected child to observe a usable tty\n{}",
        combined_output(&output)
    );
}

#[test]
fn piped_installer_without_tty_fails_clearly_before_child_crashes() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let install_script = repo_root.join("install.sh");
    let temp = tempdir().expect("temp dir");
    let env = TestEnv::new(temp.path(), stub_binary_that_crashes_without_tty()).expect("test env");
    let launch_script = write_script(
        temp.path().join("launch-no-tty.sh"),
        &format!("cat \"{}\" | bash -s --\n", install_script.display()),
    )
    .expect("launch script");

    let output = run_without_controlling_terminal(&launch_script, &env);

    assert!(
        !output.status.success(),
        "installer should fail without a tty\n{}",
        combined_output(&output)
    );

    let combined = combined_output(&output);
    assert_contains(
        &combined,
        "Unable to launch dotfiles interactively because no TTY was found.",
    );
    assert_not_contains(&combined, "input-reader crash");
}

#[test]
fn piped_installer_with_real_binary_reacquires_tty_for_inquire() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let install_script = repo_root.join("install.sh");
    let temp = tempdir().expect("temp dir");
    let binary_path = PathBuf::from(
        std::env::var("CARGO_BIN_EXE_dotfiles").expect("compiled dotfiles binary path"),
    );
    let env = TestEnv::from_binary_path(temp.path(), &binary_path).expect("test env");
    let launch_script = write_script(
        temp.path().join("launch-real-binary.sh"),
        &format!("cat \"{}\" | bash -s --\n", install_script.display()),
    )
    .expect("launch script");

    let output = run_with_terminal_input_when_ready(
        &launch_script,
        &env,
        "Select shells to configure:",
        b"\x03",
    );
    let combined = combined_output(&output);

    assert!(
        output.status.success(),
        "real binary installer should start interactively and handle ctrl-c\n{}",
        combined
    );
    assert_contains(&combined, "Select shells to configure:");
    assert_contains(&combined, "Setup canceled by user.");
    assert_not_contains(&combined, "Failed to initialize input reader");
}

fn run_without_controlling_terminal(command_script: &Path, env: &TestEnv) -> Output {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut command = Command::new("bash");
    command
        .arg(command_script)
        .envs(env.env_vars())
        .current_dir(&repo_root);

    #[cfg(unix)]
    {
        use std::{io, os::unix::process::CommandExt};

        unsafe extern "C" {
            fn setsid() -> std::os::raw::c_int;
        }

        unsafe {
            command.pre_exec(|| {
                if setsid() == -1 {
                    return Err(io::Error::last_os_error());
                }

                Ok(())
            });
        }
    }

    command
        .output()
        .expect("run install script without controlling terminal")
}

struct TestEnv {
    home_dir: PathBuf,
    install_dir: PathBuf,
    stub_bin_dir: PathBuf,
}

impl TestEnv {
    fn new(base: &Path, stub_binary: String) -> std::io::Result<Self> {
        let stub_tarball = build_stub_release(base, &stub_binary)?;
        Self::from_release_tarball(base, &stub_tarball)
    }

    fn from_binary_path(base: &Path, binary_path: &Path) -> std::io::Result<Self> {
        let stub_tarball = build_release_from_binary_path(base, binary_path)?;
        Self::from_release_tarball(base, &stub_tarball)
    }

    fn from_release_tarball(base: &Path, tarball: &Path) -> std::io::Result<Self> {
        let home_dir = base.join("home");
        let install_dir = base.join("bin");
        let stub_bin_dir = base.join("stub-bin");

        fs::create_dir_all(&home_dir)?;
        fs::create_dir_all(&stub_bin_dir)?;

        write_script(
            stub_bin_dir.join("curl"),
            &format!("cat \"{}\"\n", tarball.display()),
        )?;

        write_script(
            stub_bin_dir.join("wget"),
            &format!("cat \"{}\"\n", tarball.display()),
        )?;

        Ok(Self {
            home_dir,
            install_dir,
            stub_bin_dir,
        })
    }

    fn env_vars(&self) -> [(String, String); 3] {
        [
            ("HOME".into(), self.home_dir.display().to_string()),
            (
                "DOTFILES_INSTALL_DIR".into(),
                self.install_dir.display().to_string(),
            ),
            (
                "PATH".into(),
                format!(
                    "{}:/usr/bin:/bin:/usr/sbin:/sbin",
                    self.stub_bin_dir.display()
                ),
            ),
        ]
    }
}

fn build_stub_release(base: &Path, stub_binary: &str) -> std::io::Result<PathBuf> {
    let release_dir = base.join("release");
    fs::create_dir_all(&release_dir)?;

    write_script(release_dir.join("dotfiles"), stub_binary)?;

    let tarball = base.join("dotfiles-test.tar.gz");
    let status = Command::new("tar")
        .args(["czf", tarball.to_str().expect("tarball path"), "dotfiles"])
        .current_dir(&release_dir)
        .status()
        .expect("create stub tarball");

    assert!(status.success(), "failed to create stub tarball");

    Ok(tarball)
}

fn build_release_from_binary_path(base: &Path, binary_path: &Path) -> std::io::Result<PathBuf> {
    let release_dir = base.join("release-real-binary");
    fs::create_dir_all(&release_dir)?;

    let copied_binary = release_dir.join("dotfiles");
    fs::copy(binary_path, &copied_binary)?;
    fs::set_permissions(&copied_binary, fs::Permissions::from_mode(0o755))?;

    let tarball = base.join("dotfiles-real-binary.tar.gz");
    let status = Command::new("tar")
        .args(["czf", tarball.to_str().expect("tarball path"), "dotfiles"])
        .current_dir(&release_dir)
        .status()
        .expect("create real binary tarball");

    assert!(status.success(), "failed to create real binary tarball");

    Ok(tarball)
}

fn run_with_terminal(command_script: &Path, env: &TestEnv) -> Output {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let bsd_status = Command::new("script")
        .args(["-q", "/dev/null", "true"])
        .status()
        .expect("check script syntax");

    if bsd_status.success() {
        return Command::new("script")
            .args(["-q", "/dev/null"])
            .arg(command_script)
            .envs(env.env_vars())
            .current_dir(&repo_root)
            .output()
            .expect("run command under tty");
    }

    Command::new("script")
        .args(["-q", "-e", "-c"])
        .arg(command_script)
        .arg("/dev/null")
        .envs(env.env_vars())
        .current_dir(&repo_root)
        .output()
        .expect("run command under tty")
}

fn run_with_terminal_input_when_ready(
    command_script: &Path,
    env: &TestEnv,
    readiness_text: &str,
    input: &[u8],
) -> Output {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let bsd_status = Command::new("script")
        .args(["-q", "/dev/null", "true"])
        .status()
        .expect("check script syntax");

    let mut command = if bsd_status.success() {
        let mut command = Command::new("script");
        command.args(["-q", "/dev/null"]).arg(command_script);
        command
    } else {
        let mut command = Command::new("script");
        command
            .args(["-q", "-e", "-c"])
            .arg(command_script)
            .arg("/dev/null");
        command
    };

    command
        .envs(env.env_vars())
        .current_dir(&repo_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn().expect("spawn command under tty");
    let stdout = child.stdout.take().expect("script stdout");
    let stderr = child.stderr.take().expect("script stderr");
    let mut stdin = child.stdin.take().expect("script stdin");

    let readiness = readiness_text.as_bytes().to_vec();
    let (ready_tx, ready_rx) = mpsc::channel();

    let stdout_handle = spawn_output_reader(stdout, ready_tx.clone(), readiness.clone());
    let stderr_handle = spawn_output_reader(stderr, ready_tx, readiness);

    wait_for_readiness(&ready_rx, readiness_text, Duration::from_secs(10));
    stdin.write_all(input).expect("write tty input");
    drop(stdin);

    let status = child.wait().expect("wait for tty command");
    let stdout = stdout_handle.join().expect("join stdout reader");
    let stderr = stderr_handle.join().expect("join stderr reader");

    Output {
        status,
        stdout,
        stderr,
    }
}

fn spawn_output_reader(
    mut reader: impl Read + Send + 'static,
    ready_tx: mpsc::Sender<()>,
    readiness: Vec<u8>,
) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut chunk = [0_u8; 1024];
        let mut output = Vec::new();
        let mut ready_sent = false;

        loop {
            match reader.read(&mut chunk) {
                Ok(0) => break,
                Ok(n) => {
                    output.extend_from_slice(&chunk[..n]);

                    if !ready_sent
                        && output
                            .windows(readiness.len())
                            .any(|window| window == readiness)
                    {
                        let _ = ready_tx.send(());
                        ready_sent = true;
                    }
                }
                Err(err) => panic!("read command output: {err}"),
            }
        }

        output
    })
}

fn wait_for_readiness(ready_rx: &mpsc::Receiver<()>, readiness_text: &str, timeout: Duration) {
    if ready_rx.recv_timeout(timeout).is_ok() {
        return;
    }

    panic!(
        "timed out waiting for installer readiness output {:?}",
        readiness_text
    );
}

fn write_script(path: PathBuf, body: &str) -> std::io::Result<PathBuf> {
    fs::write(
        &path,
        format!("#!/usr/bin/env bash\nset -euo pipefail\n{body}"),
    )?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions)?;
    }
    Ok(path)
}

fn stub_binary_for_help() -> String {
    "printf 'stub-dotfiles %s\\n' \"$*\"\n".into()
}

fn stub_binary_that_requires_tty(marker_path: &Path) -> String {
    format!(
        "if [ -t 0 ] && stty -g >/dev/null 2>&1; then\n  : > \"{}\"\n  exit 0\nfi\n\nprintf 'tty-missing\\n' >&2\nexit 43\n",
        marker_path.display()
    )
}

fn stub_binary_that_crashes_without_tty() -> String {
    "printf 'input-reader crash\\n' >&2\nexit 99\n".into()
}

fn combined_output(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn assert_success(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{}\nstatus: {:?}\n{}",
        context,
        output.status.code(),
        combined_output(output)
    );
}

fn assert_contains(output: &str, expected: &str) {
    assert!(
        output.contains(expected),
        "expected output to contain {:?}\nactual output:\n{}",
        expected,
        output
    );
}

fn assert_not_contains(output: &str, unexpected: &str) {
    assert!(
        !output.contains(unexpected),
        "expected output to not contain {:?}\nactual output:\n{}",
        unexpected,
        output
    );
}
