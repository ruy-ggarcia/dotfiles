use anyhow::Result;

mod models;
mod package;
mod scanner;
mod symlink;
mod template;

fn main() -> Result<()> {
    println!("Dotfiles Installer v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
