use anyhow::{Context, Result};
use console::style;
use sentinel::core::ConfigManager;
use sentinel::models::{Config, HealthCheck, ProcessConfig};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::{create_spinner, print_error, print_info, print_success, print_warning};

/// Execute the init command
pub async fn execute(output_file: &Path, template: Option<&str>, force: bool) -> Result<()> {
    // Check if file exists
    if output_file.exists() && !force {
        print_error(&format!("File '{}' already exists", output_file.display()));
        print_info("Use --force to overwrite");
        std::process::exit(1);
    }

    // Get template
    let config = match template {
        Some("simple") => create_simple_template(),
        Some("full-stack") => create_fullstack_template(),
        Some("microservices") => create_microservices_template(),
        Some(other) => {
            print_error(&format!("Unknown template: '{}'", other));
            print_info("Available templates: simple, full-stack, microservices");
            std::process::exit(1);
        }
        None => {
            // Interactive template selection
            select_template_interactive()?
        }
    };

    // Ensure parent directory exists
    if let Some(parent) = output_file.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    // Save configuration
    let spinner = create_spinner("Creating configuration file...");
    ConfigManager::save_to_file(&config, output_file)
        .with_context(|| format!("Failed to save config to {}", output_file.display()))?;
    spinner.finish_and_clear();

    print_success(&format!(
        "Created configuration file: {}",
        output_file.display()
    ));
    println!();
    print_info("Edit the configuration file to match your needs");
    print_info(&format!(
        "Then run: {}",
        style(format!("sentinel start {}", output_file.display()))
            .cyan()
            .bold()
    ));

    Ok(())
}

/// Interactive template selection
fn select_template_interactive() -> Result<Config> {
    println!("{}", style("Select a template:").cyan().bold());
    println!("  1) Simple      - Basic process configuration");
    println!("  2) Full-stack  - Frontend + Backend setup");
    println!("  3) Microservices - Multiple services with dependencies");
    println!();
    print!("Enter choice [1-3]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => Ok(create_simple_template()),
        "2" => Ok(create_fullstack_template()),
        "3" => Ok(create_microservices_template()),
        _ => {
            print_warning("Invalid choice, using simple template");
            Ok(create_simple_template())
        }
    }
}

/// Create a simple template
fn create_simple_template() -> Config {
    Config {
        processes: vec![ProcessConfig {
            name: "my-app".to_string(),
            command: "node".to_string(),
            args: vec!["server.js".to_string()],
            cwd: Some(PathBuf::from(".")),
            env: HashMap::new(),
            depends_on: Vec::new(),
            auto_restart: Some(true),
            max_restarts: Some(3),
            restart_delay_ms: Some(1000),
            health_check: None,
        }],
        global_env: HashMap::new(),
    }
}

/// Create a full-stack template
/// Uses ports 8100-8199 to avoid conflicts with common dev tools
/// (3000: React/Vite, 3001: Express, 4200: Angular, 5173: Vite, 8000/8080: Python/Java)
fn create_fullstack_template() -> Config {
    let mut backend_env = HashMap::new();
    backend_env.insert("PORT".to_string(), "8101".to_string());
    backend_env.insert("NODE_ENV".to_string(), "development".to_string());

    let mut frontend_env = HashMap::new();
    frontend_env.insert("PORT".to_string(), "8100".to_string());

    Config {
        processes: vec![
            ProcessConfig {
                name: "database".to_string(),
                command: "docker".to_string(),
                args: vec![
                    "run".to_string(),
                    "--rm".to_string(),
                    "-p".to_string(),
                    "5432:5432".to_string(),
                    "postgres:15".to_string(),
                ],
                cwd: None,
                env: HashMap::new(),
                depends_on: Vec::new(),
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(2000),
                health_check: Some(HealthCheck {
                    command: "pg_isready".to_string(),
                    args: vec!["-h".to_string(), "localhost".to_string()],
                    interval_ms: 5000,
                    timeout_ms: 3000,
                    retries: 3,
                }),
            },
            ProcessConfig {
                name: "backend".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "dev".to_string()],
                cwd: Some(PathBuf::from("./backend")),
                env: backend_env,
                depends_on: vec!["database".to_string()],
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(1000),
                health_check: Some(HealthCheck {
                    command: "curl".to_string(),
                    args: vec!["-f".to_string(), "http://localhost:8101/health".to_string()],
                    interval_ms: 10000,
                    timeout_ms: 5000,
                    retries: 3,
                }),
            },
            ProcessConfig {
                name: "frontend".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "dev".to_string()],
                cwd: Some(PathBuf::from("./frontend")),
                env: frontend_env,
                depends_on: vec!["backend".to_string()],
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(1000),
                health_check: None,
            },
        ],
        global_env: HashMap::new(),
    }
}

/// Create a microservices template
fn create_microservices_template() -> Config {
    Config {
        processes: vec![
            ProcessConfig {
                name: "redis".to_string(),
                command: "docker".to_string(),
                args: vec![
                    "run".to_string(),
                    "--rm".to_string(),
                    "-p".to_string(),
                    "6379:6379".to_string(),
                    "redis:7-alpine".to_string(),
                ],
                cwd: None,
                env: HashMap::new(),
                depends_on: Vec::new(),
                auto_restart: Some(true),
                max_restarts: Some(5),
                restart_delay_ms: Some(2000),
                health_check: None,
            },
            ProcessConfig {
                name: "postgres".to_string(),
                command: "docker".to_string(),
                args: vec![
                    "run".to_string(),
                    "--rm".to_string(),
                    "-p".to_string(),
                    "5432:5432".to_string(),
                    "postgres:15".to_string(),
                ],
                cwd: None,
                env: HashMap::new(),
                depends_on: Vec::new(),
                auto_restart: Some(true),
                max_restarts: Some(5),
                restart_delay_ms: Some(2000),
                health_check: None,
            },
            ProcessConfig {
                name: "auth-service".to_string(),
                command: "npm".to_string(),
                args: vec!["start".to_string()],
                cwd: Some(PathBuf::from("./services/auth")),
                env: HashMap::new(),
                depends_on: vec!["postgres".to_string(), "redis".to_string()],
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(1000),
                health_check: None,
            },
            ProcessConfig {
                name: "api-gateway".to_string(),
                command: "npm".to_string(),
                args: vec!["start".to_string()],
                cwd: Some(PathBuf::from("./services/gateway")),
                env: HashMap::new(),
                depends_on: vec!["auth-service".to_string()],
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(1000),
                health_check: None,
            },
            ProcessConfig {
                name: "user-service".to_string(),
                command: "npm".to_string(),
                args: vec!["start".to_string()],
                cwd: Some(PathBuf::from("./services/users")),
                env: HashMap::new(),
                depends_on: vec!["postgres".to_string(), "redis".to_string()],
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(1000),
                health_check: None,
            },
        ],
        global_env: {
            let mut env = HashMap::new();
            env.insert("NODE_ENV".to_string(), "development".to_string());
            env.insert("LOG_LEVEL".to_string(), "debug".to_string());
            env
        },
    }
}
