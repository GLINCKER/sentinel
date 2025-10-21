use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use comfy_table::{Cell, Table};
use sentinel::core::{ConfigManager, ProcessManager, SystemMonitor};
use sentinel::models::ProcessState;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    create_spinner, format_state, get_default_config_path, print_error, print_info, state_color,
};

/// Execute the status command
pub async fn execute(verbose: bool, format: &str) -> Result<()> {
    let config_path = get_default_config_path();

    // Load configuration
    let spinner = create_spinner("Loading status...");
    let config = ConfigManager::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;

    // Initialize managers
    let pm = Arc::new(Mutex::new(ProcessManager::new()));
    let mut sm = SystemMonitor::new();
    sm.refresh();

    let manager = pm.lock().await;
    spinner.finish_and_clear();

    match format {
        "json" => {
            // JSON output for scripting
            let mut processes = Vec::new();
            for process_config in &config.processes {
                if let Some(info) = manager.get_process(&process_config.name) {
                    processes.push(serde_json::json!({
                        "name": info.name,
                        "state": info.state,
                        "pid": info.pid,
                        "started_at": info.started_at,
                        "command": process_config.command,
                    }));
                }
            }

            let output = serde_json::json!({
                "processes": processes,
                "total": config.processes.len(),
            });

            println!("{}", serde_json::to_string_pretty(&output)?);
        }

        "table" | _ => {
            // Pretty table output
            let mut table = Table::new();

            // Set up headers
            if verbose {
                table.set_header(vec![
                    Cell::new("NAME").fg(comfy_table::Color::Cyan),
                    Cell::new("STATE").fg(comfy_table::Color::Cyan),
                    Cell::new("PID").fg(comfy_table::Color::Cyan),
                    Cell::new("CPU %").fg(comfy_table::Color::Cyan),
                    Cell::new("MEMORY").fg(comfy_table::Color::Cyan),
                    Cell::new("UPTIME").fg(comfy_table::Color::Cyan),
                    Cell::new("COMMAND").fg(comfy_table::Color::Cyan),
                ]);
            } else {
                table.set_header(vec![
                    Cell::new("NAME").fg(comfy_table::Color::Cyan),
                    Cell::new("STATE").fg(comfy_table::Color::Cyan),
                    Cell::new("PID").fg(comfy_table::Color::Cyan),
                    Cell::new("UPTIME").fg(comfy_table::Color::Cyan),
                ]);
            }

            // Add rows
            for process_config in &config.processes {
                let info = manager.get_process(&process_config.name);

                if let Some(info) = info {
                    let uptime = info
                        .started_at
                        .map(|started| format_uptime(&started))
                        .unwrap_or_else(|| "-".to_string());

                    let pid_str = info
                        .pid
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "-".to_string());

                    if verbose {
                        let (cpu, mem) = info
                            .pid
                            .and_then(|pid| sm.get_process_stats(pid))
                            .unwrap_or((0.0, 0));

                        table.add_row(vec![
                            Cell::new(&info.name),
                            Cell::new(format_state(&info.state)).fg(state_color(&info.state)),
                            Cell::new(&pid_str),
                            Cell::new(format!("{:.1}", cpu)),
                            Cell::new(format_memory(mem)),
                            Cell::new(&uptime),
                            Cell::new(&process_config.command),
                        ]);
                    } else {
                        table.add_row(vec![
                            Cell::new(&info.name),
                            Cell::new(format_state(&info.state)).fg(state_color(&info.state)),
                            Cell::new(&pid_str),
                            Cell::new(&uptime),
                        ]);
                    }
                } else {
                    // Process not found in manager (never started)
                    if verbose {
                        table.add_row(vec![
                            Cell::new(&process_config.name),
                            Cell::new(format_state(&ProcessState::Stopped))
                                .fg(state_color(&ProcessState::Stopped)),
                            Cell::new("-"),
                            Cell::new("-"),
                            Cell::new("-"),
                            Cell::new("-"),
                            Cell::new(&process_config.command),
                        ]);
                    } else {
                        table.add_row(vec![
                            Cell::new(&process_config.name),
                            Cell::new(format_state(&ProcessState::Stopped))
                                .fg(state_color(&ProcessState::Stopped)),
                            Cell::new("-"),
                            Cell::new("-"),
                        ]);
                    }
                }
            }

            println!("{table}");
            println!();

            // Summary
            let running = manager
                .list_processes()
                .iter()
                .filter(|p| matches!(p.state, ProcessState::Running))
                .count();
            let total = config.processes.len();

            print_info(&format!("{} of {} processes running", running, total));

            if verbose {
                let sys_stats = sm.get_stats();
                println!();
                print_info(&format!("System CPU: {:.1}%", sys_stats.cpu_usage));
                print_info(&format!(
                    "System Memory: {} / {} ({:.1}%)",
                    format_memory(sys_stats.memory_used),
                    format_memory(sys_stats.memory_total),
                    (sys_stats.memory_used as f64 / sys_stats.memory_total as f64) * 100.0
                ));
            }
        }
    }

    Ok(())
}

/// Format uptime from start time
fn format_uptime(started_at: &DateTime<Local>) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(*started_at);

    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Format memory in human-readable format
fn format_memory(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
