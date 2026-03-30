pub const ZSH_TEMPLATE: &str = include_str!("../modules/zsh/home/prompt.zsh.tera");
pub const BASH_TEMPLATE: &str = include_str!("../modules/bash/home/prompt.bash.tera");
pub const KITTY_TEMPLATE: &str = include_str!("../modules/kitty/kitty.conf.tera");
pub const ALACRITTY_TEMPLATE: &str = include_str!("../modules/alacritty/alacritty.toml.tera");

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
}
