use anyhow::Result;

mod models;
mod package;
mod scanner;
mod template;

fn main() -> Result<()> {
    println!("Dotfiles Installer v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
