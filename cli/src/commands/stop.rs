use anyhow::{Context, Result};
use sentinel::core::{ConfigManager, ProcessManager};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{create_spinner, get_default_config_path, print_error, print_info, print_success};

/// Execute the stop command
pub async fn execute(force: bool) -> Result<()> {
    let config_path = get_default_config_path();

    // Load configuration
    let spinner = create_spinner("Loading configuration...");
    let config = ConfigManager::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    spinner.finish_and_clear();

    if force {
        print_info("Force stop enabled (SIGKILL)");
    }

    // Initialize process manager
    let pm = Arc::new(Mutex::new(ProcessManager::new()));

    print_info(&format!(
        "Stopping {} process(es)...",
        config.processes.len()
    ));

    let mut success_count = 0;
    let mut error_count = 0;

    for process_config in &config.processes {
        let spinner = create_spinner(&format!("Stopping {}...", process_config.name));

        let mut manager = pm.lock().await;
        match manager.stop(&process_config.name).await {
            Ok(_) => {
                spinner.finish_and_clear();
                print_success(&format!("Stopped {}", process_config.name));
                success_count += 1;
            }
            Err(e) => {
                spinner.finish_and_clear();
                // Don't fail if process wasn't running
                if e.to_string().contains("not found") {
                    print_info(&format!("{} was not running", process_config.name));
                } else {
                    print_error(&format!("Failed to stop {}: {}", process_config.name, e));
                    error_count += 1;
                }
            }
        }
    }

    println!();
    if error_count == 0 {
        print_success("All processes stopped successfully!");
    } else {
        print_error(&format!(
            "Stopped {} process(es), {} failed",
            success_count, error_count
        ));
        std::process::exit(1);
    }

    Ok(())
}
