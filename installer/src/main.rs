use anyhow::Result;

mod engine;
mod models;
mod package;
mod scanner;
mod symlink;
mod template;

fn main() -> Result<()> {
    println!("Dotfiles Installer v{}", env!("CARGO_PKG_VERSION"));

    // Determine repo root (current working directory)
    let repo_root = std::env::current_dir()?;

    // Phase 1: Discovery
    let modules = scanner::scan_modules(&repo_root.join("modules"))?;
    let themes = scanner::scan_themes(&repo_root.join("themes").join("palettes"))?;

    if modules.is_empty() {
        anyhow::bail!("No modules found in modules/");
    }
    if themes.is_empty() {
        anyhow::bail!("No themes found in themes/palettes/");
    }

    // Phase 2: Hardcoded selection (TUI will replace this in M3)
    let selection = models::UserSelection {
        selected_modules: modules,
        selected_theme: themes.into_iter().next().unwrap(),
    };

    // Phase 3: Execution
    let plan = engine::generate_plan(&selection, &repo_root)?;
    engine::execute_plan(&plan, &selection.selected_theme)?;

    println!("\n=========================================");
    println!("  SUCCESS: Environment provisioned.");
    println!("=========================================");

    Ok(())
}
