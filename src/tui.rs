use inquire::{MultiSelect, Select};

use crate::models::{Shell, TerminalEmulator, Theme};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn select_shells(shells: Vec<Shell>) -> Result<Vec<Shell>> {
    let names: Vec<String> = shells.iter().map(|shell| format!("{:?}", shell)).collect();
    let selected_names = MultiSelect::new("Select shells to configure:", names.clone()).prompt()?;
    let result = shells
        .into_iter()
        .filter(|shell| selected_names.contains(&format!("{:?}", shell)))
        .collect();
    Ok(result)
}

pub fn select_terminal_emulators(
    terminal_emulators: Vec<TerminalEmulator>,
) -> Result<Vec<TerminalEmulator>> {
    if terminal_emulators.is_empty() {
        return Ok(vec![]);
    }
    let names: Vec<String> = terminal_emulators
        .iter()
        .map(|terminal_emulator| format!("{:?}", terminal_emulator))
        .collect();
    let selected_names =
        MultiSelect::new("Select terminal emulators to configure:", names).prompt()?;
    let result = terminal_emulators
        .into_iter()
        .filter(|terminal_emulator| selected_names.contains(&format!("{:?}", terminal_emulator)))
        .collect();
    Ok(result)
}

pub fn select_font(fonts: Vec<String>) -> Result<String> {
    let selected_font = Select::new("Select a Nerd Font:", fonts).prompt()?;
    Ok(selected_font)
}

pub fn select_font_size() -> Result<u8> {
    let sizes: Vec<u8> = (8..=24).collect();
    let labels: Vec<String> = sizes.iter().map(|s| s.to_string()).collect();
    let selected_font_size = Select::new("Font size:", labels)
        .with_starting_cursor(4) // default: 12 (index 4 in 8..=24)
        .with_vim_mode(true)
        .prompt()?;
    Ok(selected_font_size.parse()?)
}

pub fn select_theme(themes: Vec<Theme>) -> Result<Theme> {
    let names: Vec<String> = themes.iter().map(|p| p.name.clone()).collect();
    let selected_name = Select::new("Select a theme:", names)
        .with_vim_mode(true)
        .prompt()?;
    themes
        .into_iter()
        .find(|p| p.name == selected_name)
        .ok_or_else(|| "Theme not found".into())
}
