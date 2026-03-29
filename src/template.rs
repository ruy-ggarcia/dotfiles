use std::collections::HashMap;
use tera::{Context, Tera};

pub fn render(template_str: &str, vars: &HashMap<&str, &str>) -> Result<String, tera::Error> {
    let mut context = Context::new();
    for (k, v) in vars {
        context.insert(*k, v);
    }
    Tera::one_off(template_str, &context, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_contains_font_family() {
        let mut vars = HashMap::new();
        vars.insert("font_family", "JetBrainsMono");
        let result = render("Hello {{ font_family }}!", &vars).unwrap();
        assert_eq!(result, "Hello JetBrainsMono!");
    }

    #[test]
    fn test_render_contains_font_size() {
        let mut vars = HashMap::new();
        vars.insert("font_size", "12");
        let result = render("Size: {{ font_size }}", &vars).unwrap();
        assert_eq!(result, "Size: 12");
    }

    #[test]
    fn test_render_with_multiple_values() {
        let mut vars = HashMap::new();
        vars.insert("font_family", "JetBrainsMono");
        vars.insert("font_size", "14");
        let result = render("Font: {{ font_family }}, Size: {{ font_size }}", &vars).unwrap();
        assert_eq!(result, "Font: JetBrainsMono, Size: 14");
    }

    #[test]
    fn test_render_unknown_variable_errors() {
        let vars = HashMap::new();
        let result = render("{{ undefined_var }}", &vars);
        assert!(result.is_err());
    }

    fn theme_vars<'a>() -> HashMap<&'a str, &'a str> {
        let mut vars = HashMap::new();
        vars.insert("font_family", "JetBrainsMono Nerd Font");
        vars.insert("font_size", "12");
        vars
    }

    #[test]
    fn test_zshrc_template_contains_vcs_info() {
        let template = std::fs::read_to_string("modules/zsh/home/prompt.zsh.tera").unwrap();
        let vars = theme_vars();
        let result = render(&template, &vars).unwrap();
        assert!(result.contains("vcs_info"), "zshrc output should contain vcs_info");
    }

    #[test]
    fn test_bashrc_template_contains_git_branch() {
        let template = std::fs::read_to_string("modules/bash/home/prompt.bash.tera").unwrap();
        let vars = theme_vars();
        let result = render(&template, &vars).unwrap();
        assert!(result.contains("git"), "bashrc output should contain git");
    }

    #[test]
    fn test_zshrc_prompt_uses_accent_color() {
        let template = std::fs::read_to_string("modules/zsh/home/prompt.zsh.tera").unwrap();
        let vars = theme_vars();
        let result = render(&template, &vars).unwrap();
        assert!(result.contains("DOTFILES_ACCENT"), "zshrc output should reference DOTFILES_ACCENT");
    }

    #[test]
    fn test_bashrc_prompt_uses_accent_color() {
        let template = std::fs::read_to_string("modules/bash/home/prompt.bash.tera").unwrap();
        let vars = theme_vars();
        let result = render(&template, &vars).unwrap();
        assert!(result.contains("DOTFILES_ACCENT"), "bashrc output should reference DOTFILES_ACCENT");
    }

    #[test]
    fn test_zshrc_renders_without_unresolved_variables() {
        let template = std::fs::read_to_string("modules/zsh/home/prompt.zsh.tera").unwrap();
        let vars = theme_vars();
        let result = render(&template, &vars).unwrap();
        assert!(!result.contains("{{"), "zshrc output should not contain unresolved template variables");
    }
}
