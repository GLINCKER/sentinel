use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use comfy_table::{Cell, Color, Table};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use sentinel::models::{Config, ProcessConfig, ProcessState};
use std::path::PathBuf;
use std::time::Duration;

mod commands;

/// Sentinel - Your Development Guardian
///
/// Process Manager & System Monitor built by Glincker (A GLINR Product)
#[derive(Parser)]
#[command(name = "sentinel")]
#[command(author = "Glincker (A GLINR Product)")]
#[command(version = "0.1.0-alpha")]
#[command(about = "Your Development Guardian - Process Manager & System Monitor", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start Sentinel with a configuration file
    Start {
        /// Path to the configuration file (YAML or JSON)
        #[arg(value_name = "CONFIG_FILE")]
        config_file: Option<PathBuf>,

        /// Start in daemon mode (background)
        #[arg(short, long)]
        daemon: bool,
    },

    /// Stop all running processes
    Stop {
        /// Force stop without graceful shutdown
        #[arg(short, long)]
        force: bool,
    },

    /// Restart all processes
    Restart {
        /// Force restart without graceful shutdown
        #[arg(short, long)]
        force: bool,
    },

    /// Show status of all processes
    Status {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,

        /// Output format (table, json)
        #[arg(short = 'f', long, default_value = "table")]
        format: String,
    },

    /// Show logs for a process
    Logs {
        /// Name of the process
        #[arg(value_name = "PROCESS_NAME")]
        process_name: String,

        /// Follow log output
        #[arg(short, long)]
        follow: bool,

        /// Number of lines to show
        #[arg(short = 'n', long, default_value = "50")]
        lines: usize,
    },

    /// Add a new process to the configuration
    Add {
        /// Name of the process
        #[arg(value_name = "NAME")]
        name: String,

        /// Command to run
        #[arg(value_name = "COMMAND")]
        command: String,

        /// Working directory
        #[arg(short = 'd', long)]
        directory: Option<PathBuf>,

        /// Auto-restart on failure
        #[arg(short = 'r', long)]
        auto_restart: bool,
    },

    /// Remove a process from the configuration
    Remove {
        /// Name of the process
        #[arg(value_name = "NAME")]
        name: String,

        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// List all configured processes
    List {
        /// Output format (table, json)
        #[arg(short = 'f', long, default_value = "table")]
        format: String,
    },

    /// Initialize a new configuration file
    Init {
        /// Output file path
        #[arg(value_name = "OUTPUT_FILE", default_value = "sentinel.yaml")]
        output_file: PathBuf,

        /// Use example template (simple, full-stack, microservices)
        #[arg(short = 't', long)]
        template: Option<String>,

        /// Overwrite existing file
        #[arg(short = 'f', long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start {
            config_file,
            daemon,
        } => commands::start::execute(config_file, daemon).await?,

        Commands::Stop { force } => commands::stop::execute(force).await?,

        Commands::Restart { force } => commands::restart::execute(force).await?,

        Commands::Status { verbose, format } => commands::status::execute(verbose, &format).await?,

        Commands::Logs {
            process_name,
            follow,
            lines,
        } => commands::logs::execute(&process_name, follow, lines).await?,

        Commands::Add {
            name,
            command,
            directory,
            auto_restart,
        } => commands::add::execute(&name, &command, directory, auto_restart).await?,

        Commands::Remove { name, yes } => commands::remove::execute(&name, yes).await?,

        Commands::List { format } => commands::list::execute(&format).await?,

        Commands::Init {
            output_file,
            template,
            force,
        } => commands::init::execute(&output_file, template.as_deref(), force).await?,
    }

    Ok(())
}

/// Create a spinner with consistent styling
pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Print a success message
pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Print an error message
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg.red());
}

/// Print a warning message
pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg.yellow());
}

/// Print an info message
pub fn print_info(msg: &str) {
    println!("{} {}", "ℹ".cyan().bold(), msg);
}

/// Get color for process state
pub fn state_color(state: &ProcessState) -> Color {
    match state {
        ProcessState::Running => Color::Green,
        ProcessState::Stopped => Color::Grey,
        ProcessState::Starting => Color::Cyan,
        ProcessState::Stopping => Color::Yellow,
        ProcessState::Crashed { .. } => Color::Red,
        ProcessState::Failed { .. } => Color::Red,
    }
}

/// Format process state with color
pub fn format_state(state: &ProcessState) -> String {
    match state {
        ProcessState::Running => "Running".green().to_string(),
        ProcessState::Stopped => "Stopped".bright_black().to_string(),
        ProcessState::Starting => "Starting".cyan().to_string(),
        ProcessState::Stopping => "Stopping".yellow().to_string(),
        ProcessState::Crashed { exit_code } => format!("Crashed ({})", exit_code).red().to_string(),
        ProcessState::Failed { reason } => format!("Failed: {}", reason).red().to_string(),
    }
}

/// Get default config path
pub fn get_default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("sentinel")
        .join("config.yaml")
}
