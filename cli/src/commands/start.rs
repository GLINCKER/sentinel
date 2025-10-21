use anyhow::{Context, Result};
use sentinel::core::{ConfigManager, ProcessManager, SystemMonitor};
use sentinel::state::AppState;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{create_spinner, get_default_config_path, print_error, print_info, print_success};

/// Execute the start command
pub async fn execute(config_file: Option<PathBuf>, daemon: bool) -> Result<()> {
    let config_path = config_file.unwrap_or_else(get_default_config_path);

    // Show what we're doing
    print_info(&format!(
        "Loading configuration from {}",
        config_path.display()
    ));

    // Load configuration with spinner
    let spinner = create_spinner("Loading configuration...");
    let config = ConfigManager::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    spinner.finish_and_clear();

    print_success(&format!(
        "Loaded configuration with {} process(es)",
        config.processes.len()
    ));

    if daemon {
        print_info("Daemon mode is not yet implemented. Starting in foreground mode.");
    }

    // Initialize application state
    let state = AppState {
        process_manager: Arc::new(Mutex::new(ProcessManager::new())),
        system_monitor: Arc::new(Mutex::new(SystemMonitor::new())),
        config: Arc::new(Mutex::new(config.clone())),
    };

    // Start all processes
    print_info(&format!(
        "Starting {} process(es)...",
        config.processes.len()
    ));

    let mut success_count = 0;
    let mut error_count = 0;

    for process_config in &config.processes {
        let spinner = create_spinner(&format!("Starting {}...", process_config.name));

        let mut pm = state.process_manager.lock().await;
        match pm.start(process_config.clone()).await {
            Ok(info) => {
                spinner.finish_and_clear();
                print_success(&format!(
                    "Started {} (PID: {})",
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
            "All {} process(es) started successfully!",
            success_count
        ));
    } else {
        print_error(&format!(
            "Started {} process(es), {} failed",
            success_count, error_count
        ));
        std::process::exit(1);
    }

    if !daemon {
        print_info("Press Ctrl+C to stop all processes");

        // Wait for Ctrl+C
        tokio::signal::ctrl_c()
            .await
            .context("Failed to listen for Ctrl+C")?;

        println!();
        print_info("Shutting down...");

        // Stop all processes
        let mut pm = state.process_manager.lock().await;
        for process_config in &config.processes {
            if let Err(e) = pm.stop(&process_config.name).await {
                print_error(&format!("Failed to stop {}: {}", process_config.name, e));
            }
        }

        print_success("All processes stopped");
    }

    Ok(())
}
