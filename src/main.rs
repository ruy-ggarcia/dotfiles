mod font;
mod models;
mod scanner;

use std::path::Path;

use models::Shell;
use scanner::scan_shells;

fn main() {
    println!("dotfiles v{}", env!("CARGO_PKG_VERSION"));

    let entries = [
        (Shell::Bash, Path::new("/bin/bash")),
        (Shell::Bash, Path::new("/usr/bin/bash")),
        (Shell::Zsh, Path::new("/bin/zsh")),
        (Shell::Zsh, Path::new("/usr/bin/zsh")),
    ];

    let modules = scan_shells(&entries);

    for module in &modules {
        println!("detected shell: {:?}", module.shell);
    }

    let home = std::env::var("HOME").unwrap_or_default();
    let home_fonts = std::path::PathBuf::from(&home).join("Library/Fonts");
    let fonts = font::scan_fonts(&[
        home_fonts.as_path(),
        Path::new("/Library/Fonts"),
    ]);
    println!("Detected fonts: {:?}", fonts);
}
