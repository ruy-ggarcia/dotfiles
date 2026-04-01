use std::path::Path;

use crate::models::{Plan, Shell, TerminalEmulator, UserSelection};

pub fn generate_plan(selection: UserSelection) -> Plan {
    Plan {
        shells: selection.shells,
        terminal_emulators: selection.terminal_emulators,
        font: selection.font,
        theme: selection.theme,
    }
}

pub fn print_summary(plan: &Plan) {
    println!("=== Dotfiles installation plan ===");
    for shell in &plan.shells {
        println!("  · Configure {:?}", shell);
    }
    for terminal_emulator in &plan.terminal_emulators {
        println!("  · Configure {:?}", terminal_emulator);
    }
    println!("  · Font: {} {}pt", plan.font.family, plan.font.size);
    println!("  · Theme: {}", plan.theme.name);
    println!("==================================");
}

pub fn execute_plan(plan: &Plan, output_dir: &Path) {
    use crate::{symlink, template};
    use std::collections::HashMap;

    for shell in &plan.shells {
        let (template_path, prompt_name, rc_name) = match shell {
            Shell::Zsh => ("modules/zsh/home/prompt.zsh.tera", "prompt.zsh", ".zshrc"),
            Shell::Bash => (
                "modules/bash/home/prompt.bash.tera",
                "prompt.bash",
                ".bashrc",
            ),
        };

        let template_str = match std::fs::read_to_string(template_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot read {template_path}: {e}");
                continue;
            }
        };

        let mut vars = HashMap::new();
        vars.insert("font_family", plan.font.family.as_str());
        let size_str = plan.font.size.to_string();
        vars.insert("font_size", size_str.as_str());

        for (key, value) in &plan.theme.colors {
            vars.insert(key.as_str(), value.as_str());
        }

        let rendered = match template::render(&template_str, &vars) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Render error for {template_path}: {e}");
                continue;
            }
        };

        let prompt_file = output_dir.join(prompt_name);
        if let Err(e) = std::fs::write(&prompt_file, &rendered) {
            eprintln!("Write error {}: {e}", prompt_file.display());
            continue;
        }

        let home = std::env::var("HOME").unwrap_or_default();
        let rc_path = std::path::Path::new(&home).join(rc_name);
        if let Err(e) = symlink::inject_source_line(&rc_path, &prompt_file) {
            eprintln!("RC inject error: {e}");
        }
    }

    for terminal_emulator in &plan.terminal_emulators {
        let (template_path, config_name, config_subdir) = match terminal_emulator {
            TerminalEmulator::Kitty => (
                "modules/kitty/kitty.conf.tera",
                "kitty.conf",
                ".config/kitty",
            ),
            TerminalEmulator::Alacritty => (
                "modules/alacritty/alacritty.toml.tera",
                "alacritty.toml",
                ".config/alacritty",
            ),
        };

        let template_str = match std::fs::read_to_string(template_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot read {template_path}: {e}");
                continue;
            }
        };

        let mut vars = HashMap::new();
        vars.insert("font_family", plan.font.family.as_str());
        let size_str = plan.font.size.to_string();
        vars.insert("font_size", size_str.as_str());
        for (key, value) in &plan.theme.colors {
            vars.insert(key.as_str(), value.as_str());
        }

        let rendered = match template::render(&template_str, &vars) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Render error for {template_path}: {e}");
                continue;
            }
        };

        let out_file = output_dir.join(config_name);
        if let Err(e) = std::fs::write(&out_file, &rendered) {
            eprintln!("Write error {}: {e}", out_file.display());
            continue;
        }

        let home = std::env::var("HOME").unwrap_or_default();
        let config_dir = std::path::Path::new(&home).join(config_subdir);
        if let Err(e) = std::fs::create_dir_all(&config_dir) {
            eprintln!("Cannot create {}: {e}", config_dir.display());
            continue;
        }
        let dest = config_dir.join(config_name);
        if let Err(e) = symlink::create_symlink(&out_file, &dest) {
            eprintln!("Symlink error for {config_name}: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Font, Shell, Theme};
    use std::collections::HashMap;

    fn make_theme(name: &str) -> Theme {
        let mut colors = HashMap::new();
        colors.insert("name".to_string(), name.to_string());
        colors.insert("base".to_string(), "#24273a".to_string());
        colors.insert("text".to_string(), "#cad3f5".to_string());
        colors.insert("accent".to_string(), "#8aadf4".to_string());
        colors.insert("surface".to_string(), "#363a4f".to_string());
        colors.insert("overlay".to_string(), "#6e738d".to_string());
        colors.insert("cursor".to_string(), "#f4dbd6".to_string());
        colors.insert("selection_bg".to_string(), "#5b6078".to_string());
        colors.insert("selection_fg".to_string(), "#cad3f5".to_string());
        colors.insert("color0".to_string(), "#494d64".to_string());
        colors.insert("color1".to_string(), "#ed8796".to_string());
        colors.insert("color2".to_string(), "#a6da95".to_string());
        colors.insert("color3".to_string(), "#eed49f".to_string());
        colors.insert("color4".to_string(), "#8aadf4".to_string());
        colors.insert("color5".to_string(), "#f5bde6".to_string());
        colors.insert("color6".to_string(), "#8bd5ca".to_string());
        colors.insert("color7".to_string(), "#b8c0e0".to_string());
        colors.insert("color8".to_string(), "#5b6078".to_string());
        colors.insert("color9".to_string(), "#ed8796".to_string());
        colors.insert("color10".to_string(), "#a6da95".to_string());
        colors.insert("color11".to_string(), "#eed49f".to_string());
        colors.insert("color12".to_string(), "#8aadf4".to_string());
        colors.insert("color13".to_string(), "#f5bde6".to_string());
        colors.insert("color14".to_string(), "#8bd5ca".to_string());
        colors.insert("color15".to_string(), "#a5adcb".to_string());
        Theme {
            name: name.to_string(),
            colors,
        }
    }

    #[test]
    fn test_generate_plan_includes_theme() {
        let theme = make_theme("Catppuccin Macchiato");
        let selection = UserSelection {
            shells: vec![Shell::Zsh],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("FiraCode Nerd Font"),
                size: 12,
            },
            theme,
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.theme.name, "Catppuccin Macchiato");
    }

    #[test]
    fn test_generate_plan_with_two_shells() {
        let selection = UserSelection {
            shells: vec![Shell::Bash, Shell::Zsh],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("FiraCode Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.shells, vec![Shell::Bash, Shell::Zsh]);
    }

    #[test]
    fn test_generate_plan_with_single_shell() {
        let selection = UserSelection {
            shells: vec![Shell::Zsh],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("FiraCode Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.shells, vec![Shell::Zsh]);
    }

    #[test]
    fn test_generate_plan_preserves_font() {
        let selection = UserSelection {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("Hack Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.font.family, "Hack Nerd Font");
    }

    #[test]
    fn test_generate_plan_preserves_font_size() {
        let selection = UserSelection {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("Hack Nerd Font"),
                size: 16,
            },
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.font.size, 16u8);
    }

    #[test]
    fn test_generate_plan_with_empty_shells() {
        let selection = UserSelection {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("FiraCode Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert!(plan.shells.is_empty());
    }

    #[test]
    fn test_kitty_template_renders_with_theme_vars() {
        let template = std::fs::read_to_string("modules/kitty/kitty.conf.tera").unwrap();
        let theme = make_theme("Test");
        let mut vars = HashMap::new();
        vars.insert("font_family", "JetBrainsMono Nerd Font");
        vars.insert("font_size", "12");
        for (key, value) in &theme.colors {
            vars.insert(key.as_str(), value.as_str());
        }

        let rendered = crate::template::render(&template, &vars).unwrap();
        assert!(rendered.contains("font_family JetBrainsMono Nerd Font"));
        assert!(rendered.contains("background #24273a"));
    }

    #[test]
    fn test_alacritty_template_renders_with_theme_vars() {
        let template = std::fs::read_to_string("modules/alacritty/alacritty.toml.tera").unwrap();
        let theme = make_theme("Test");
        let mut vars = HashMap::new();
        vars.insert("font_family", "JetBrainsMono Nerd Font");
        vars.insert("font_size", "12");
        for (key, value) in &theme.colors {
            vars.insert(key.as_str(), value.as_str());
        }

        let rendered = crate::template::render(&template, &vars).unwrap();
        assert!(rendered.contains("family = \"JetBrainsMono Nerd Font\""));
        assert!(rendered.contains("background = \"#24273a\""));
    }
}
