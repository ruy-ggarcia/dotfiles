#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub shell: Shell,
}

#[derive(Debug, PartialEq)]
pub struct UserSelection {
    pub shells: Vec<Module>,
    pub font: String,
    pub font_size: u8,
}

#[derive(Debug)]
pub struct Plan {
    pub shells: Vec<Module>,
    pub font: String,
    pub font_size: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_selection_stores_shells() {
        let shells = vec![Module { shell: Shell::Zsh }];
        let sel = UserSelection {
            shells: shells.clone(),
            font: String::from("FiraCode Nerd Font"),
            font_size: 12,
        };
        assert_eq!(sel.shells, shells);
    }

    #[test]
    fn test_user_selection_stores_font() {
        let sel = UserSelection {
            shells: vec![],
            font: String::from("Hack Nerd Font"),
            font_size: 12,
        };
        assert_eq!(sel.font, "Hack Nerd Font");
    }

    #[test]
    fn test_user_selection_stores_font_size() {
        let sel = UserSelection {
            shells: vec![],
            font: String::from("Hack Nerd Font"),
            font_size: 14,
        };
        assert_eq!(sel.font_size, 14u8);
    }

    #[test]
    fn test_user_selection_debug_format() {
        let sel = UserSelection {
            shells: vec![],
            font: String::from("Hack Nerd Font"),
            font_size: 12,
        };
        let _ = format!("{:?}", sel);
    }

    #[test]
    fn test_plan_stores_shells() {
        let shells = vec![Module { shell: Shell::Bash }];
        let plan = Plan {
            shells: shells.clone(),
            font: String::from("FiraCode Nerd Font"),
            font_size: 12,
        };
        assert_eq!(plan.shells, shells);
    }

    #[test]
    fn test_plan_stores_font() {
        let plan = Plan {
            shells: vec![],
            font: String::from("Hack Nerd Font"),
            font_size: 12,
        };
        assert_eq!(plan.font, "Hack Nerd Font");
    }

    #[test]
    fn test_plan_stores_font_size() {
        let plan = Plan {
            shells: vec![],
            font: String::from("Hack Nerd Font"),
            font_size: 14,
        };
        assert_eq!(plan.font_size, 14u8);
    }

    #[test]
    fn test_plan_debug_format() {
        let plan = Plan {
            shells: vec![],
            font: String::from("Hack Nerd Font"),
            font_size: 12,
        };
        let _ = format!("{:?}", plan);
    }

    #[test]
    fn test_user_selection_equality() {
        let a = UserSelection {
            shells: vec![Module { shell: Shell::Bash }],
            font: String::from("FiraCode Nerd Font"),
            font_size: 11,
        };
        let b = UserSelection {
            shells: vec![Module { shell: Shell::Bash }],
            font: String::from("FiraCode Nerd Font"),
            font_size: 11,
        };
        assert_eq!(a, b);
    }
}
