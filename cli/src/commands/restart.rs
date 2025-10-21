use anyhow::{Context, Result};
use sentinel::core::{ConfigManager, ProcessManager};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{create_spinner, get_default_config_path, print_error, print_info, print_success};

/// Execute the restart command
pub async fn execute(force: bool) -> Result<()> {
    let config_path = get_default_config_path();

    // Load configuration
    let spinner = create_spinner("Loading configuration...");
    let config = ConfigManager::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    spinner.finish_and_clear();

    if force {
        print_info("Force restart enabled");
    }

    // Initialize process manager
    let pm = Arc::new(Mutex::new(ProcessManager::new()));

    print_info(&format!(
        "Restarting {} process(es)...",
        config.processes.len()
    ));

    let mut success_count = 0;
    let mut error_count = 0;

    for process_config in &config.processes {
        // Stop process
        let spinner = create_spinner(&format!("Stopping {}...", process_config.name));
        let mut manager = pm.lock().await;

        if let Err(e) = manager.stop(&process_config.name).await {
            // Ignore "not found" errors since process might not be running
            if !e.to_string().contains("not found") {
                spinner.finish_and_clear();
                print_error(&format!("Failed to stop {}: {}", process_config.name, e));
                error_count += 1;
                continue;
            }
        }
        spinner.finish_and_clear();

        // Start process
        let spinner = create_spinner(&format!("Starting {}...", process_config.name));
        match manager.start(process_config.clone()).await {
            Ok(info) => {
                spinner.finish_and_clear();
                print_success(&format!(
                    "Restarted {} (PID: {})",
                    process_config.name,
                    info.pid.unwrap_or(0)
                ));
                success_count += 1;
            }
            Err(e) => {
                spinner.finish_and_clear();
                print_error(&format!("Failed to start {}: {}", process_config.name, e));
                error_count += 1;
            }
        }
    }

    println!();
    if error_count == 0 {
        print_success(&format!(
            "All {} process(es) restarted successfully!",
            success_count
        ));
    } else {
        print_error(&format!(
            "Restarted {} process(es), {} failed",
            success_count, error_count
        ));
        std::process::exit(1);
    }

    Ok(())
}
