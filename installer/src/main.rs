use anyhow::Result;

mod engine;
mod font;
mod models;
mod package;
mod scanner;
mod symlink;
mod template;
mod tui;

fn main() -> Result<()> {
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

    // Phase 2: TUI wizard
    tui::display_welcome();

    let selected_modules = tui::select_modules(modules)?;
    if selected_modules.is_empty() {
        println!("Setup canceled by user.");
        std::process::exit(0);
    }

    // Err from select_theme means cancelled
    let selected_theme = match tui::select_theme(themes) {
        Ok(theme) => theme,
        Err(e) => {
            // "Setup canceled by user." is the cancellation message from tui.rs
            println!("{}", e);
            std::process::exit(0);
        }
    };

    let theme_name = selected_theme.name.clone();

    // Font selection — skip gracefully if manifest is missing or empty
    let font_manifest_path = repo_root.join("fonts").join("manifest.toml");
    let selected_fonts = if font_manifest_path.exists() {
        match font::load_font_manifest(&repo_root) {
            Ok(font_entries) if !font_entries.is_empty() => {
                let chosen = tui::select_fonts(font_entries)?;
                if chosen.is_empty() {
                    // Ctrl+C during font selection — exit cleanly
                    println!("Setup canceled by user.");
                    std::process::exit(0);
                }
                chosen
            }
            _ => vec![],
        }
    } else {
        vec![]
    };

    let selection = models::UserSelection {
        selected_modules,
        selected_theme,
    };

    // Phase 3: Plan generation + confirmation
    let plan = engine::generate_plan(&selection, &repo_root)?;

    let confirmed = tui::confirm_plan(&plan, &theme_name)?;
    if !confirmed {
        println!("Setup canceled by user.");
        std::process::exit(0);
    }

    // Phase 4: Execution
    engine::execute_plan(&plan, &selection.selected_theme)?;

    // Font installation — runs after the main plan so symlinks are already in place
    font::install_fonts(&selected_fonts)?;

    tui::display_success();

    Ok(())
}
