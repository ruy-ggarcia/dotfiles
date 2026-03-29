mod font;
mod models;
mod scanner;
mod tui;

use std::path::Path;

use models::{Shell, UserSelection};
use scanner::scan_shells;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = [
        (Shell::Bash, Path::new("/bin/bash")),
        (Shell::Bash, Path::new("/usr/bin/bash")),
        (Shell::Zsh, Path::new("/bin/zsh")),
        (Shell::Zsh, Path::new("/usr/bin/zsh")),
    ];

    let detected_shells = scan_shells(&entries);

    let home = std::env::var("HOME").unwrap_or_default();
    let home_fonts = std::path::PathBuf::from(&home).join("Library/Fonts");
    let detected_fonts = font::scan_fonts(&[
        home_fonts.as_path(),
        Path::new("/Library/Fonts"),
    ]);

    let _selection = UserSelection {
        shells: tui::select_shells(detected_shells)?,
        font: tui::select_font(detected_fonts)?,
        font_size: tui::select_font_size()?,
    };

    Ok(())
}
