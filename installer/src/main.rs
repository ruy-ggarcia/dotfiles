use anyhow::Result;

mod models;

fn main() -> Result<()> {
    println!("Dotfiles Installer v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
