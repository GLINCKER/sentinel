use anyhow::{Context, Result};
use colored::Colorize;
use sentinel::core::{ConfigManager, ProcessManager};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{create_spinner, get_default_config_path, print_error, print_info, print_warning};

/// Execute the logs command
pub async fn execute(process_name: &str, follow: bool, lines: usize) -> Result<()> {
    let config_path = get_default_config_path();

    // Load configuration
    let spinner = create_spinner("Loading configuration...");
    let config = ConfigManager::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    spinner.finish_and_clear();

    // Check if process exists in config
    let process_config = config
        .processes
        .iter()
        .find(|p| p.name == process_name)
        .ok_or_else(|| anyhow::anyhow!("Process '{}' not found in configuration", process_name))?;

    // Initialize process manager
    let pm = Arc::new(Mutex::new(ProcessManager::new()));
    let manager = pm.lock().await;

    // Check if process is running
    let info = manager.get_process(process_name);

    if info.is_none() {
        print_warning(&format!("Process '{}' is not running", process_name));
        return Ok(());
    }

    let info = info.unwrap();

    // Get logs from process manager
    let logs = manager.get_logs(process_name, lines)?;

    if logs.is_empty() {
        print_info(&format!("No logs available for '{}'", process_name));
        return Ok(());
    }

    // Print logs with color coding
    println!(
        "Logs for {} (last {} lines):",
        process_name.cyan().bold(),
        lines
    );
    println!("{}", "─".repeat(80).bright_black());

    for log_entry in &logs {
        // Color code based on log level keywords
        let line = &log_entry.message;
        if line.to_lowercase().contains("error") || line.to_lowercase().contains("fatal") {
            println!("{}", line.red());
        } else if line.to_lowercase().contains("warn") {
            println!("{}", line.yellow());
        } else if line.to_lowercase().contains("info") {
            println!("{}", line.cyan());
        } else if line.to_lowercase().contains("debug") {
            println!("{}", line.bright_black());
        } else {
            println!("{}", line);
        }
    }

    if follow {
        println!();
        print_info("Following log output (Ctrl+C to stop)...");
        println!("{}", "─".repeat(80).bright_black());

        // TODO: Implement log streaming
        // This requires the ProcessManager to support streaming logs
        // For now, just print a message
        print_warning("Log streaming is not yet implemented");
        print_info("Use 'sentinel logs <name>' without --follow to see recent logs");
    }

    Ok(())
}
