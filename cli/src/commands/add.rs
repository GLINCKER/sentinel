use anyhow::{Context, Result};
use sentinel::core::ConfigManager;
use sentinel::models::{Config, ProcessConfig};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::{create_spinner, get_default_config_path, print_error, print_info, print_success};

/// Execute the add command
pub async fn execute(
    name: &str,
    command: &str,
    directory: Option<PathBuf>,
    auto_restart: bool,
) -> Result<()> {
    let config_path = get_default_config_path();

    // Load existing configuration or create new
    let spinner = create_spinner("Loading configuration...");
    let mut config = if config_path.exists() {
        ConfigManager::load_from_file(&config_path)
            .with_context(|| format!("Failed to load config from {}", config_path.display()))?
    } else {
        print_info("No existing configuration found, creating new one");
        Config {
            processes: Vec::new(),
            global_env: HashMap::new(),
        }
    };
    spinner.finish_and_clear();

    // Check if process already exists
    if config.processes.iter().any(|p| p.name == name) {
        print_error(&format!(
            "Process '{}' already exists in configuration",
            name
        ));
        print_info("Use 'sentinel remove' first to replace it");
        std::process::exit(1);
    }

    // Parse command and args
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        print_error("Command cannot be empty");
        std::process::exit(1);
    }

    let cmd = parts[0].to_string();
    let args = parts[1..].iter().map(|s| s.to_string()).collect();

    // Create new process config
    let process_config = ProcessConfig {
        name: name.to_string(),
        command: cmd,
        args,
        cwd: directory,
        env: HashMap::new(),
        depends_on: Vec::new(),
        auto_restart: Some(auto_restart),
        max_restarts: Some(3),
        restart_delay_ms: Some(1000),
        health_check: None,
    };

    // Add to config
    config.processes.push(process_config);

    // Validate configuration
    let spinner = create_spinner("Validating configuration...");
    if let Err(e) = ConfigManager::validate(&config) {
        spinner.finish_and_clear();
        print_error(&format!("Configuration validation failed: {}", e));
        std::process::exit(1);
    }
    spinner.finish_and_clear();

    // Save configuration
    let spinner = create_spinner("Saving configuration...");

    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    ConfigManager::save_to_file(&config, &config_path)
        .with_context(|| format!("Failed to save config to {}", config_path.display()))?;
    spinner.finish_and_clear();

    print_success(&format!("Added process '{}' to configuration", name));
    print_info(&format!("Configuration saved to {}", config_path.display()));
    println!();
    print_info("Run 'sentinel start' to start all processes");
    print_info(&format!(
        "Or run 'sentinel start {}' to start just this process (when implemented)",
        name
    ));

    Ok(())
}
