mod engine;
mod font;
mod models;
mod scanner;
mod symlink;
mod template;
mod tui;

use std::path::Path;

use inquire::InquireError;
use models::{Shell, UserSelection};
use scanner::{scan_shells, scan_themes};

fn main() {
    if let Err(e) = run() {
        match e.downcast_ref::<InquireError>() {
            Some(InquireError::OperationCanceled) | Some(InquireError::OperationInterrupted) => {
                std::process::exit(0)
            }
            _ => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
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

    let themes = scan_themes(std::path::Path::new("themes"));

    let selection = UserSelection {
        shells: tui::select_shells(detected_shells)?,
        font: tui::select_font(detected_fonts)?,
        font_size: tui::select_font_size()?,
        theme: tui::select_theme(themes)?,
    };

    let plan = engine::generate_plan(selection);
    engine::print_summary(&plan);

    let output_dir = std::path::PathBuf::from(
        std::env::var("HOME").unwrap_or_default()
    ).join(".config/dotfiles/rendered");
    std::fs::create_dir_all(&output_dir).ok();
    engine::execute_plan(&plan, &output_dir);

    Ok(())
}
