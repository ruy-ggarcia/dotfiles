use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TerminalEmulator {
    Kitty,
    Alacritty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PromptEngine {
    Custom,
    Starship,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Font {
    pub family: String,
    pub size: u8,
}

#[derive(Debug, PartialEq)]
pub struct UserSelection {
    pub shells: Vec<Shell>,
    pub terminal_emulators: Vec<TerminalEmulator>,
    pub font: Font,
    pub theme: Theme,
}

#[derive(Debug)]
pub struct Plan {
    pub shells: Vec<Shell>,
    pub terminal_emulators: Vec<TerminalEmulator>,
    pub font: Font,
    pub theme: Theme,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_theme(name: &str) -> Theme {
        let mut colors = HashMap::new();
        colors.insert("base".to_string(), "#24273a".to_string());
        colors.insert("text".to_string(), "#cad3f5".to_string());
        colors.insert("accent".to_string(), "#8aadf4".to_string());
        colors.insert("surface".to_string(), "#363a4f".to_string());
        colors.insert("overlay".to_string(), "#6e738d".to_string());
        colors.insert("name".to_string(), name.to_string());
        Theme {
            name: name.to_string(),
            colors,
        }
    }

    #[test]
    fn test_theme_stores_name() {
        let theme = make_theme("Catppuccin Macchiato");
        assert_eq!(theme.name, "Catppuccin Macchiato");
    }

    #[test]
    fn test_theme_stores_colors() {
        let mut colors: HashMap<String, String> = HashMap::new();
        colors.insert("base".to_string(), "#24273a".to_string());
        let theme = Theme {
            name: "Test".to_string(),
            colors: colors.clone(),
        };
        assert_eq!(theme.colors, colors);
    }

    #[test]
    fn test_user_selection_stores_shells() {
        let shells = vec![Shell::Zsh];
        let sel = UserSelection {
            shells: shells.clone(),
            terminal_emulators: vec![],
            font: Font {
                family: String::from("FiraCode Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        assert_eq!(sel.shells, shells);
    }

    #[test]
    fn test_user_selection_stores_font() {
        let sel = UserSelection {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("Hack Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        assert_eq!(sel.font.family, "Hack Nerd Font");
    }

    #[test]
    fn test_user_selection_stores_font_size() {
        let sel = UserSelection {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("Hack Nerd Font"),
                size: 14,
            },
            theme: make_theme("Test"),
        };
        assert_eq!(sel.font.size, 14u8);
    }

    #[test]
    fn test_user_selection_debug_format() {
        let sel = UserSelection {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("Hack Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        let _ = format!("{:?}", sel);
    }

    #[test]
    fn test_plan_stores_shells() {
        let shells = vec![Shell::Bash];
        let plan = Plan {
            shells: shells.clone(),
            terminal_emulators: vec![],
            font: Font {
                family: String::from("FiraCode Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        assert_eq!(plan.shells, shells);
    }

    #[test]
    fn test_plan_stores_font() {
        let plan = Plan {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("Hack Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        assert_eq!(plan.font.family, "Hack Nerd Font");
    }

    #[test]
    fn test_plan_stores_font_size() {
        let plan = Plan {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("Hack Nerd Font"),
                size: 14,
            },
            theme: make_theme("Test"),
        };
        assert_eq!(plan.font.size, 14u8);
    }

    #[test]
    fn test_plan_debug_format() {
        let plan = Plan {
            shells: vec![],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("Hack Nerd Font"),
                size: 12,
            },
            theme: make_theme("Test"),
        };
        let _ = format!("{:?}", plan);
    }

    #[test]
    fn test_user_selection_equality() {
        let a = UserSelection {
            shells: vec![Shell::Bash],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("FiraCode Nerd Font"),
                size: 11,
            },
            theme: make_theme("Test"),
        };
        let b = UserSelection {
            shells: vec![Shell::Bash],
            terminal_emulators: vec![],
            font: Font {
                family: String::from("FiraCode Nerd Font"),
                size: 11,
            },
            theme: make_theme("Test"),
        };
        assert_eq!(a, b);
    }
}
