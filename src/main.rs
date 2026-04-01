mod engine;
mod font;
mod models;
mod scanner;
mod symlink;
mod template;
mod tui;

use std::path::Path;

use inquire::InquireError;
use models::{Emulator, Shell, UserSelection};
use scanner::{scan_emulators, scan_shells, scan_themes};

fn main() {
    if let Err(e) = run() {
        match e.downcast_ref::<InquireError>() {
            Some(InquireError::OperationCanceled) | Some(InquireError::OperationInterrupted) => {
                print!("\r\x1b[2K");
                println!("Setup canceled by user.");
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

    let emulator_entries = [
        (Emulator::Kitty, Path::new("/usr/local/bin/kitty")),
        (Emulator::Kitty, Path::new("/usr/bin/kitty")),
        (
            Emulator::Kitty,
            Path::new("/Applications/kitty.app/Contents/MacOS/kitty"),
        ),
        (Emulator::Alacritty, Path::new("/usr/local/bin/alacritty")),
        (Emulator::Alacritty, Path::new("/usr/bin/alacritty")),
        (
            Emulator::Alacritty,
            Path::new("/Applications/Alacritty.app/Contents/MacOS/alacritty"),
        ),
    ];
    let detected_emulators = scan_emulators(&emulator_entries);

    let home = std::env::var("HOME").unwrap_or_default();
    let font_dirs = font::font_dirs(&home);
    let font_dir_refs: Vec<&std::path::Path> = font_dirs.iter().map(|p| p.as_path()).collect();
    let detected_fonts = font::scan_fonts(&font_dir_refs);

    let themes = scan_themes(std::path::Path::new("themes"));

    let selection = UserSelection {
        shells: tui::select_shells(detected_shells)?,
        emulators: tui::select_emulators(detected_emulators)?,
        font: tui::select_font(detected_fonts)?,
        font_size: tui::select_font_size()?,
        theme: tui::select_theme(themes)?,
    };

    let plan = engine::generate_plan(selection);
    engine::print_summary(&plan);

    let output_dir = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default())
        .join(".config/dotfiles/rendered");
    std::fs::create_dir_all(&output_dir).ok();
    engine::execute_plan(&plan, &output_dir);

    Ok(())
}
