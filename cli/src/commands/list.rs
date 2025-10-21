use anyhow::{Context, Result};
use comfy_table::{Cell, Table};
use sentinel::core::ConfigManager;

use crate::{create_spinner, get_default_config_path, print_info};

/// Execute the list command
pub async fn execute(format: &str) -> Result<()> {
    let config_path = get_default_config_path();

    // Load configuration
    let spinner = create_spinner("Loading configuration...");
    let config = ConfigManager::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    spinner.finish_and_clear();

    if config.processes.is_empty() {
        print_info("No processes configured");
        print_info("Use 'sentinel add' to add a process");
        return Ok(());
    }

    match format {
        "json" => {
            // JSON output for scripting
            let processes: Vec<_> = config
                .processes
                .iter()
                .map(|p| {
                    serde_json::json!({
                        "name": p.name,
                        "command": p.command,
                        "args": p.args,
                        "cwd": p.cwd,
                        "auto_restart": p.auto_restart,
                        "depends_on": p.depends_on,
                    })
                })
                .collect();

            let output = serde_json::json!({
                "processes": processes,
                "total": config.processes.len(),
            });

            println!("{}", serde_json::to_string_pretty(&output)?);
        }

        "table" | _ => {
            // Pretty table output
            let mut table = Table::new();

            table.set_header(vec![
                Cell::new("NAME").fg(comfy_table::Color::Cyan),
                Cell::new("COMMAND").fg(comfy_table::Color::Cyan),
                Cell::new("WORKING DIR").fg(comfy_table::Color::Cyan),
                Cell::new("AUTO-RESTART").fg(comfy_table::Color::Cyan),
                Cell::new("DEPENDS ON").fg(comfy_table::Color::Cyan),
            ]);

            for process in &config.processes {
                let full_command = if process.args.is_empty() {
                    process.command.clone()
                } else {
                    format!("{} {}", process.command, process.args.join(" "))
                };

                let cwd = process
                    .cwd
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "-".to_string());

                let auto_restart = process
                    .auto_restart
                    .map(|v| if v { "Yes" } else { "No" })
                    .unwrap_or("-");

                let depends_on = if process.depends_on.is_empty() {
                    "-".to_string()
                } else {
                    process.depends_on.join(", ")
                };

                table.add_row(vec![
                    Cell::new(&process.name),
                    Cell::new(&full_command),
                    Cell::new(&cwd),
                    Cell::new(auto_restart),
                    Cell::new(&depends_on),
                ]);
            }

            println!("{table}");
            println!();
            print_info(&format!(
                "{} process(es) configured",
                config.processes.len()
            ));
        }
    }

    Ok(())
}
