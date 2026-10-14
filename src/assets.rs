pub const ZSH_TEMPLATE: &str = include_str!("../modules/zsh/home/prompt.zsh.tera");
pub const BASH_TEMPLATE: &str = include_str!("../modules/bash/home/prompt.bash.tera");
pub const KITTY_TEMPLATE: &str = include_str!("../modules/kitty/kitty.conf.tera");
pub const ALACRITTY_TEMPLATE: &str = include_str!("../modules/alacritty/alacritty.toml.tera");
pub const STARSHIP_CONFIG: &str = include_str!("../modules/starship/home/starship.toml");

pub const CATPPUCCIN_MACCHIATO: &str = include_str!("../themes/catppuccin_macchiato.toml");
pub const KANAGAWA_DRAGON: &str = include_str!("../themes/kanagawa_dragon.toml");

pub const DEFAULT_THEMES: &[(&str, &str)] = &[
    ("catppuccin_macchiato.toml", CATPPUCCIN_MACCHIATO),
    ("kanagawa_dragon.toml", KANAGAWA_DRAGON),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn shell_vars<'a>() -> HashMap<&'a str, &'a str> {
        let mut vars = HashMap::new();
        vars.insert("font_family", "JetBrainsMono Nerd Font");
        vars.insert("font_size", "12");
        vars.insert("use_starship", "false");
        vars
    }

    #[test]
    fn test_zsh_template_is_not_empty() {
        assert!(!ZSH_TEMPLATE.is_empty());
    }

    #[test]
    fn test_bash_template_is_not_empty() {
        assert!(!BASH_TEMPLATE.is_empty());
    }

    #[test]
    fn test_kitty_template_is_not_empty() {
        assert!(!KITTY_TEMPLATE.is_empty());
    }

    #[test]
    fn test_alacritty_template_is_not_empty() {
        assert!(!ALACRITTY_TEMPLATE.is_empty());
    }

    #[test]
    fn test_starship_config_is_not_empty() {
        assert!(!STARSHIP_CONFIG.is_empty());
    }

    // ANSI color roles documented in D16; the terminal maps them to the theme.
    const STARSHIP_ROLE_COLORS: &[&str] = &["3", "4", "5", "6", "8", "9", "10", "15"];
    const STARSHIP_MODIFIERS: &[&str] = &[
        "bold",
        "italic",
        "underline",
        "dimmed",
        "inverted",
        "blink",
        "hidden",
        "strikethrough",
    ];

    fn starship_config() -> toml::Table {
        STARSHIP_CONFIG
            .parse()
            .expect("starship.toml should be valid TOML")
    }

    /// Collects every style string: `*style*` keys and `[text](style)` groups.
    fn collect_starship_styles(table: &toml::Table, out: &mut Vec<String>) {
        for (key, value) in table {
            match value {
                toml::Value::Table(nested) => collect_starship_styles(nested, out),
                toml::Value::String(s) => {
                    if key.contains("style") {
                        out.push(s.clone());
                    }
                    let mut rest = s.as_str();
                    while let Some(start) = rest.find("](") {
                        let after = &rest[start + 2..];
                        let end = after.find(')').unwrap_or(after.len());
                        out.push(after[..end].to_string());
                        rest = &after[end..];
                    }
                }
                _ => {}
            }
        }
    }

    fn prompt_color(template: &str, var: &str) -> String {
        let prefix = format!("export {var}=\"");
        let line = template
            .lines()
            .find(|line| line.starts_with(&prefix))
            .unwrap_or_else(|| panic!("{var} not found in prompt template"));
        line[prefix.len()..].split('"').next().unwrap().to_string()
    }

    #[test]
    fn test_starship_styles_only_use_documented_color_roles() {
        let mut styles = Vec::new();
        collect_starship_styles(&starship_config(), &mut styles);
        assert!(!styles.is_empty());

        for style in &styles {
            for token in style.split_whitespace() {
                if token.starts_with('$') || STARSHIP_MODIFIERS.contains(&token) {
                    continue;
                }
                assert!(
                    STARSHIP_ROLE_COLORS.contains(&token),
                    "style '{style}' uses '{token}', which is not a documented color role"
                );
            }
        }
    }

    #[test]
    fn test_starship_shares_color_roles_with_builtin_prompt() {
        let config = starship_config();
        let style = |module: &str, key: &str| config[module][key].as_str().unwrap().to_string();

        for template in [ZSH_TEMPLATE, BASH_TEMPLATE] {
            let accent = prompt_color(template, "DOTFILES_ACCENT");
            let overlay = prompt_color(template, "DOTFILES_OVERLAY");
            let success = prompt_color(template, "DOTFILES_SUCCESS");
            let error = prompt_color(template, "DOTFILES_ERROR");

            assert_eq!(style("os", "style"), accent);
            assert_eq!(style("directory", "style"), accent);
            assert_eq!(style("git_branch", "style"), overlay);
            assert_eq!(style("git_state", "style"), overlay);
            assert!(
                config["format"]
                    .as_str()
                    .unwrap()
                    .contains(&format!("]({overlay})")),
                "git wrapper in format should use the overlay role"
            );
            assert_eq!(
                style("character", "success_symbol"),
                format!("[❯]({success})")
            );
            assert_eq!(style("character", "error_symbol"), format!("[❯]({error})"));
        }
    }

    #[test]
    fn test_zsh_template_renders() {
        let result = crate::template::render(ZSH_TEMPLATE, &shell_vars());
        assert!(
            result.is_ok(),
            "zsh template failed to render: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_bash_template_renders() {
        let result = crate::template::render(BASH_TEMPLATE, &shell_vars());
        assert!(
            result.is_ok(),
            "bash template failed to render: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_zsh_template_has_no_unresolved_vars() {
        let result = crate::template::render(ZSH_TEMPLATE, &shell_vars()).unwrap();
        assert!(!result.contains("{{"));
    }

    #[test]
    fn test_bash_template_has_no_unresolved_vars() {
        let result = crate::template::render(BASH_TEMPLATE, &shell_vars()).unwrap();
        assert!(!result.contains("{{"));
    }

    fn find_shell(candidates: &[&str]) -> Option<std::path::PathBuf> {
        candidates
            .iter()
            .map(std::path::PathBuf::from)
            .find(|path| path.exists())
    }

    /// Sources the Starship-enabled prompt in `shell` with PATH limited to a
    /// directory that holds a fake `starship` only when `starship_installed`.
    fn source_starship_prompt(
        shell: &std::path::Path,
        shell_args: &[&str],
        template: &str,
        starship_installed: bool,
        probe: &str,
    ) -> String {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let mut vars = shell_vars();
        vars.insert("use_starship", "true");
        let prompt_file = dir.path().join("prompt");
        std::fs::write(
            &prompt_file,
            crate::template::render(template, &vars).unwrap(),
        )
        .unwrap();

        let bin_dir = dir.path().join("bin");
        std::fs::create_dir(&bin_dir).unwrap();
        if starship_installed {
            let starship = bin_dir.join("starship");
            std::fs::write(&starship, "#!/bin/sh\necho 'STARSHIP_INIT=1'\n").unwrap();
            std::fs::set_permissions(&starship, std::fs::Permissions::from_mode(0o755)).unwrap();
        }

        let output = std::process::Command::new(shell)
            .args(shell_args)
            .arg("-c")
            .arg(format!("source '{}'; {probe}", prompt_file.display()))
            .env("PATH", &bin_dir)
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    const BASH_PROBE: &str = r#"printf '%s|%s' "$PROMPT_COMMAND" "$STARSHIP_INIT""#;
    const ZSH_PROBE: &str = r#"printf '%s|%s' "$PROMPT" "$STARSHIP_INIT""#;

    #[test]
    fn test_bash_starship_prompt_falls_back_when_starship_missing() {
        let bash = find_shell(&["/bin/bash", "/usr/bin/bash"]).expect("bash not found");
        let output = source_starship_prompt(
            &bash,
            &["--norc", "--noprofile"],
            BASH_TEMPLATE,
            false,
            BASH_PROBE,
        );
        assert_eq!(output, "build_prompt|");
    }

    #[test]
    fn test_bash_starship_prompt_inits_starship_when_installed() {
        let bash = find_shell(&["/bin/bash", "/usr/bin/bash"]).expect("bash not found");
        let output = source_starship_prompt(
            &bash,
            &["--norc", "--noprofile"],
            BASH_TEMPLATE,
            true,
            BASH_PROBE,
        );
        assert_eq!(output, "|1");
    }

    #[test]
    fn test_zsh_starship_prompt_falls_back_when_starship_missing() {
        let Some(zsh) = find_shell(&["/bin/zsh", "/usr/bin/zsh"]) else {
            eprintln!("zsh not installed; skipping");
            return;
        };
        let output = source_starship_prompt(&zsh, &["-f"], ZSH_TEMPLATE, false, ZSH_PROBE);
        assert!(output.contains('❯'), "custom prompt not set: {output:?}");
        assert!(
            output.ends_with('|'),
            "starship was initialised: {output:?}"
        );
    }

    #[test]
    fn test_zsh_starship_prompt_inits_starship_when_installed() {
        let Some(zsh) = find_shell(&["/bin/zsh", "/usr/bin/zsh"]) else {
            eprintln!("zsh not installed; skipping");
            return;
        };
        let output = source_starship_prompt(&zsh, &["-f"], ZSH_TEMPLATE, true, ZSH_PROBE);
        assert!(!output.contains('❯'), "custom prompt was set: {output:?}");
        assert!(
            output.ends_with("|1"),
            "starship not initialised: {output:?}"
        );
    }
}
