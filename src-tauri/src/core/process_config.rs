//! Process configuration management.
//!
//! This module manages process configurations for the managed process system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::error::{Result as SentinelResult, SentinelError};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FrameworkType {
    NextJs,
    Vite,
    FastAPI,
    SpringBoot,
    Django,
    Express,
    Flask,
    Unknown,
}

/// Process configuration for managed processes
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessConfig {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: String,
    pub env_vars: HashMap<String, String>,
    pub framework_type: Option<FrameworkType>,
    pub port: Option<u16>,
    pub auto_start: bool,
    pub health_check_url: Option<String>,
    #[serde(default = "default_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "default_datetime")]
    pub updated_at: DateTime<Utc>,
}

fn default_datetime() -> DateTime<Utc> {
    Utc::now()
}

/// Framework detection result
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FrameworkDetection {
    pub framework_type: FrameworkType,
    pub confidence: f32,
    pub detected_files: Vec<String>,
    pub suggested_command: String,
    pub suggested_args: Vec<String>,
    pub suggested_port: Option<u16>,
}

/// Detected project (for monorepo support)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DetectedProject {
    pub path: String,
    pub name: String,
    pub framework_type: FrameworkType,
    pub confidence: f32,
    pub suggested_command: String,
    pub suggested_args: Vec<String>,
    pub suggested_port: Option<u16>,
    pub package_manager: Option<String>,
    pub detected_files: Vec<String>,
    pub env_vars: HashMap<String, String>,
}

/// Process template for quick setup
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessTemplate {
    pub name: String,
    pub framework_type: FrameworkType,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub default_port: Option<u16>,
    pub default_env_vars: HashMap<String, String>,
    pub health_check_url: Option<String>,
    pub icon: String,
}

/// Process status information
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStatusInfo {
    pub config_id: String,
    pub running: bool,
    pub process_id: Option<String>,
    pub pid: Option<u32>,
    pub status: Option<ProcessStatus>,
    pub uptime_seconds: Option<u64>,
    pub last_health_check: Option<HealthCheckResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ProcessStatus {
    Starting,
    Running,
    Stopped,
    Crashed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResult {
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub response_time_ms: u64,
    pub error: Option<String>,
}

/// In-memory process configuration store
pub struct ProcessConfigStore {
    configs: Arc<Mutex<HashMap<String, ProcessConfig>>>,
}

impl ProcessConfigStore {
    pub fn new() -> Self {
        Self {
            configs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new process configuration
    pub async fn create(&self, mut config: ProcessConfig) -> SentinelResult<ProcessConfig> {
        // Generate new ID and timestamps
        config.id = Uuid::new_v4().to_string();
        config.created_at = Utc::now();
        config.updated_at = Utc::now();

        let mut configs = self.configs.lock().await;

        // Check for duplicate name
        if configs.values().any(|c| c.name == config.name) {
            return Err(SentinelError::InvalidInput {
                message: format!(
                    "Process configuration with name '{}' already exists",
                    config.name
                ),
            });
        }

        configs.insert(config.id.clone(), config.clone());
        Ok(config)
    }

    /// Update an existing configuration
    pub async fn update(&self, mut config: ProcessConfig) -> SentinelResult<ProcessConfig> {
        let mut configs = self.configs.lock().await;

        // Check if config exists
        if !configs.contains_key(&config.id) {
            return Err(SentinelError::ProcessNotFound {
                name: config.id.clone(),
            });
        }

        // Check for duplicate name (excluding self)
        if configs
            .values()
            .any(|c| c.name == config.name && c.id != config.id)
        {
            return Err(SentinelError::InvalidInput {
                message: format!(
                    "Process configuration with name '{}' already exists",
                    config.name
                ),
            });
        }

        // Preserve created_at, update updated_at
        if let Some(existing) = configs.get(&config.id) {
            config.created_at = existing.created_at;
        }
        config.updated_at = Utc::now();

        configs.insert(config.id.clone(), config.clone());
        Ok(config)
    }

    /// Delete a configuration
    pub async fn delete(&self, id: &str) -> SentinelResult<()> {
        let mut configs = self.configs.lock().await;
        configs
            .remove(id)
            .ok_or_else(|| SentinelError::ProcessNotFound {
                name: id.to_string(),
            })?;
        Ok(())
    }

    /// Get all configurations
    pub async fn list(&self) -> Vec<ProcessConfig> {
        let configs = self.configs.lock().await;
        configs.values().cloned().collect()
    }

    /// Get a single configuration by ID
    pub async fn get(&self, id: &str) -> SentinelResult<ProcessConfig> {
        let configs = self.configs.lock().await;
        configs
            .get(id)
            .cloned()
            .ok_or_else(|| SentinelError::ProcessNotFound {
                name: id.to_string(),
            })
    }

    /// Export all configurations as JSON
    pub async fn export(&self) -> SentinelResult<String> {
        let configs = self.list().await;
        serde_json::to_string_pretty(&configs).map_err(|e| SentinelError::InvalidInput {
            message: format!("Failed to serialize configs: {}", e),
        })
    }

    /// Import configurations from JSON
    pub async fn import(&self, json: &str) -> SentinelResult<Vec<ProcessConfig>> {
        let imported: Vec<ProcessConfig> =
            serde_json::from_str(json).map_err(|e| SentinelError::InvalidInput {
                message: format!("Failed to parse JSON: {}", e),
            })?;

        let mut result = Vec::new();
        for mut config in imported {
            // Regenerate ID to avoid conflicts
            config.id = Uuid::new_v4().to_string();
            config.created_at = Utc::now();
            config.updated_at = Utc::now();

            // Try to create, skip on name conflicts
            match self.create(config).await {
                Ok(created) => result.push(created),
                Err(_) => continue, // Skip duplicates
            }
        }

        Ok(result)
    }
}

impl Default for ProcessConfigStore {
    fn default() -> Self {
        Self::new()
    }
}
