use crate::models::{Plan, UserSelection};

pub fn generate_plan(selection: UserSelection) -> Plan {
    Plan {
        shells: selection.shells,
        font: selection.font,
        font_size: selection.font_size,
    }
}

pub fn print_summary(plan: &Plan) {
    println!("=== Dotfiles installation plan ===");
    for module in &plan.shells {
        println!("  · Configure {:?}", module.shell);
    }
    println!("  · Font: {} {}pt", plan.font, plan.font_size);
    println!("==================================");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Module, Shell};

    #[test]
    fn test_generate_plan_with_two_shells() {
        let selection = UserSelection {
            shells: vec![Module { shell: Shell::Bash }, Module { shell: Shell::Zsh }],
            font: String::from("FiraCode Nerd Font"),
            font_size: 12,
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
        };
        let plan = generate_plan(selection);
        assert!(plan.shells.is_empty());
    }
}
