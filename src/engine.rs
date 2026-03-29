use std::path::Path;

use crate::models::{Plan, Shell, UserSelection};

pub fn generate_plan(selection: UserSelection) -> Plan {
    Plan {
        shells: selection.shells,
        font: selection.font,
        font_size: selection.font_size,
        theme: selection.theme,
    }
}

pub fn print_summary(plan: &Plan) {
    println!("=== Dotfiles installation plan ===");
    for module in &plan.shells {
        println!("  · Configure {:?}", module.shell);
    }
    println!("  · Font: {} {}pt", plan.font, plan.font_size);
    println!("  · Theme: {}", plan.theme.name);
    println!("==================================");
}

pub fn execute_plan(plan: &Plan, output_dir: &Path) {
    use std::collections::HashMap;
    use crate::{symlink, template};

    for module in &plan.shells {
        let (template_path, dest_name) = match module.shell {
            Shell::Zsh  => ("modules/zsh/home/.zshrc.tera",  ".zshrc"),
            Shell::Bash => ("modules/bash/home/.bashrc.tera", ".bashrc"),
        };

        let template_str = match std::fs::read_to_string(template_path) {
            Ok(s) => s,
            Err(e) => { eprintln!("Cannot read {template_path}: {e}"); continue; }
        };

        let mut vars = HashMap::new();
        vars.insert("font_family", plan.font.as_str());
        let size_str = plan.font_size.to_string();
        vars.insert("font_size", size_str.as_str());

        for (key, value) in &plan.theme.colors {
            vars.insert(key.as_str(), value.as_str());
        }

        let rendered = match template::render(&template_str, &vars) {
            Ok(s) => s,
            Err(e) => { eprintln!("Render error for {template_path}: {e}"); continue; }
        };

        let out_file = output_dir.join(dest_name);
        if let Err(e) = std::fs::write(&out_file, &rendered) {
            eprintln!("Write error {}: {e}", out_file.display()); continue;
        }

        let home = std::env::var("HOME").unwrap_or_default();
        let dest = std::path::Path::new(&home).join(dest_name);
        if let Err(e) = symlink::create_symlink(&out_file, &dest) {
            eprintln!("Symlink error: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::models::{Module, Theme, Shell};

    fn make_theme(name: &str) -> Theme {
        let mut colors = HashMap::new();
        colors.insert("name".to_string(), name.to_string());
        colors.insert("base".to_string(), "#24273a".to_string());
        colors.insert("text".to_string(), "#cad3f5".to_string());
        colors.insert("accent".to_string(), "#8aadf4".to_string());
        colors.insert("surface".to_string(), "#363a4f".to_string());
        colors.insert("overlay".to_string(), "#6e738d".to_string());
        Theme {
            name: name.to_string(),
            colors,
        }
    }

    #[test]
    fn test_generate_plan_includes_theme() {
        let theme = make_theme("Catppuccin Macchiato");
        let selection = UserSelection {
            shells: vec![Module { shell: Shell::Zsh }],
            font: String::from("FiraCode Nerd Font"),
            font_size: 12,
            theme,
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.theme.name, "Catppuccin Macchiato");
    }

    #[test]
    fn test_generate_plan_with_two_shells() {
        let selection = UserSelection {
            shells: vec![Module { shell: Shell::Bash }, Module { shell: Shell::Zsh }],
            font: String::from("FiraCode Nerd Font"),
            font_size: 12,
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.shells, vec![Module { shell: Shell::Bash }, Module { shell: Shell::Zsh }]);
    }

    #[test]
    fn test_generate_plan_with_single_shell() {
        let selection = UserSelection {
            shells: vec![Module { shell: Shell::Zsh }],
            font: String::from("FiraCode Nerd Font"),
            font_size: 12,
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.shells, vec![Module { shell: Shell::Zsh }]);
    }

    #[test]
    fn test_generate_plan_preserves_font() {
        let selection = UserSelection {
            shells: vec![],
            font: String::from("Hack Nerd Font"),
            font_size: 12,
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.font, "Hack Nerd Font");
    }

    #[test]
    fn test_generate_plan_preserves_font_size() {
        let selection = UserSelection {
            shells: vec![],
            font: String::from("Hack Nerd Font"),
            font_size: 16,
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert_eq!(plan.font_size, 16u8);
    }

    #[test]
    fn test_generate_plan_with_empty_shells() {
        let selection = UserSelection {
            shells: vec![],
            font: String::from("FiraCode Nerd Font"),
            font_size: 12,
            theme: make_theme("Test"),
        };
        let plan = generate_plan(selection);
        assert!(plan.shells.is_empty());
    }
}
