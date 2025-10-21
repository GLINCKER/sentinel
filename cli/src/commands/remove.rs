use anyhow::{Context, Result};
use console::style;
use sentinel::core::ConfigManager;
use std::io::{self, Write};

use crate::{create_spinner, get_default_config_path, print_error, print_info, print_success};

/// Execute the remove command
pub async fn execute(name: &str, yes: bool) -> Result<()> {
    let config_path = get_default_config_path();

    // Load configuration
    let spinner = create_spinner("Loading configuration...");
    let mut config = ConfigManager::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    spinner.finish_and_clear();

    // Check if process exists
    let index = config
        .processes
        .iter()
        .position(|p| p.name == name)
        .ok_or_else(|| anyhow::anyhow!("Process '{}' not found in configuration", name))?;

    let process = &config.processes[index];

    // Confirmation prompt (unless --yes flag)
    if !yes {
        println!(
            "Are you sure you want to remove process '{}'?",
            style(name).cyan().bold()
        );
        println!("  Command: {}", process.command);
        if let Some(cwd) = &process.cwd {
            println!("  Working Directory: {}", cwd.display());
        }
        println!();
        print!("Confirm removal [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            print_info("Removal cancelled");
            return Ok(());
        }
    }

    // Remove process
    config.processes.remove(index);

    // Save configuration
    let spinner = create_spinner("Saving configuration...");
    ConfigManager::save_to_file(&config, &config_path)
        .with_context(|| format!("Failed to save config to {}", config_path.display()))?;
    spinner.finish_and_clear();

    print_success(&format!("Removed process '{}' from configuration", name));
    print_info(&format!("Configuration saved to {}", config_path.display()));

    Ok(())
}
