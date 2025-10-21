//! Configuration management.
//!
//! This module handles loading, validation, and saving of configuration files.

use crate::error::{Result, SentinelError};
use crate::models::{Config, ProcessConfig};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Manages configuration loading, validation, and persistence.
pub struct ConfigManager;

impl ConfigManager {
    /// Loads configuration from a YAML file.
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    /// * `Ok(Config)` - Successfully loaded and validated configuration
    /// * `Err(SentinelError)` - Failed to load or validate configuration
    ///
    /// # Examples
    /// ```no_run
    /// use sentinel::core::ConfigManager;
    /// use std::path::Path;
    ///
    /// let config = ConfigManager::load_from_file(Path::new("sentinel.yaml"))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load_from_file(path: &Path) -> Result<Config> {
        // Check if file exists
        if !path.exists() {
            return Err(SentinelError::ConfigNotFound {
                path: path.to_path_buf(),
            });
        }

        // Read file contents
        let contents = fs::read_to_string(path).map_err(|source| SentinelError::FileIoError {
            path: path.to_path_buf(),
            source,
        })?;

        // Parse based on extension
        let config = if path.extension().and_then(|s| s.to_str()) == Some("json") {
            Self::parse_json(&contents, path)?
        } else {
            Self::parse_yaml(&contents, path)?
        };

        // Validate configuration
        Self::validate(&config)?;

        Ok(config)
    }

    /// Saves configuration to a YAML file.
    ///
    /// # Arguments
    /// * `config` - Configuration to save
    /// * `path` - Path where the file should be saved
    ///
    /// # Examples
    /// ```no_run
    /// use sentinel::core::ConfigManager;
    /// use sentinel::models::Config;
    /// use std::path::Path;
    ///
    /// # let config = Config { processes: vec![], settings: Default::default() };
    /// ConfigManager::save_to_file(&config, Path::new("sentinel.yaml"))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn save_to_file(config: &Config, path: &Path) -> Result<()> {
        // Validate before saving
        Self::validate(config)?;

        let contents = if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::to_string_pretty(config).map_err(|e| SentinelError::Other(e.to_string()))?
        } else {
            serde_yaml::to_string(config).map_err(|e| SentinelError::Other(e.to_string()))?
        };

        fs::write(path, contents).map_err(|source| SentinelError::FileIoError {
            path: path.to_path_buf(),
            source,
        })?;

        Ok(())
    }

    /// Generates a default configuration.
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::ConfigManager;
    ///
    /// let config = ConfigManager::default_config();
    /// assert_eq!(config.processes.len(), 1);
    /// ```
    pub fn default_config() -> Config {
        Config {
            processes: vec![ProcessConfig {
                name: "example".to_string(),
                command: "echo 'Hello from Sentinel'".to_string(),
                cwd: None,
                env: HashMap::new(),
                auto_restart: true,
                restart_limit: 5,
                restart_delay: 1000,
                depends_on: vec![],
            }],
            settings: Default::default(),
        }
    }

    /// Validates a configuration.
    ///
    /// Checks for:
    /// - Duplicate process names
    /// - Unknown dependencies
    /// - Dependency cycles
    /// - Invalid settings
    ///
    /// # Errors
    /// Returns an error if validation fails.
    fn validate(config: &Config) -> Result<()> {
        // Check for duplicate process names
        let mut names = HashSet::new();
        for process in &config.processes {
            if !names.insert(&process.name) {
                return Err(SentinelError::InvalidConfig {
                    reason: format!("Duplicate process name: '{}'", process.name),
                });
            }
        }

        // Validate each process
        for process in &config.processes {
            Self::validate_process(process, &names)?;
        }

        // Check for dependency cycles
        Self::check_dependency_cycles(config)?;

        Ok(())
    }

    /// Validates a single process configuration.
    fn validate_process(process: &ProcessConfig, all_names: &HashSet<&String>) -> Result<()> {
        // Check name is not empty
        if process.name.trim().is_empty() {
            return Err(SentinelError::InvalidConfig {
                reason: "Process name cannot be empty".to_string(),
            });
        }

        // Check command is not empty
        if process.command.trim().is_empty() {
            return Err(SentinelError::InvalidConfig {
                reason: format!("Process '{}' has empty command", process.name),
            });
        }

        // Check dependencies exist
        for dep in &process.depends_on {
            if !all_names.contains(dep) {
                return Err(SentinelError::UnknownDependency {
                    process: process.name.clone(),
                    dependency: dep.clone(),
                });
            }
        }

        Ok(())
    }

    /// Checks for circular dependencies using depth-first search.
    fn check_dependency_cycles(config: &Config) -> Result<()> {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

        // Build dependency graph
        for process in &config.processes {
            graph.insert(
                &process.name,
                process.depends_on.iter().map(|s| s.as_str()).collect(),
            );
        }

        // DFS to detect cycles
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for process in &config.processes {
            if !visited.contains(process.name.as_str()) {
                if let Some(cycle) =
                    Self::dfs_cycle(&graph, &process.name, &mut visited, &mut rec_stack)
                {
                    return Err(SentinelError::DependencyCycle { deps: cycle });
                }
            }
        }

        Ok(())
    }

    /// Depth-first search to detect dependency cycles.
    fn dfs_cycle<'a>(
        graph: &HashMap<&'a str, Vec<&'a str>>,
        node: &'a str,
        visited: &mut HashSet<&'a str>,
        rec_stack: &mut HashSet<&'a str>,
    ) -> Option<Vec<String>> {
        visited.insert(node);
        rec_stack.insert(node);

        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if let Some(cycle) = Self::dfs_cycle(graph, neighbor, visited, rec_stack) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(neighbor) {
                    // Cycle detected
                    return Some(vec![neighbor.to_string(), node.to_string()]);
                }
            }
        }

        rec_stack.remove(node);
        None
    }

    /// Parses YAML configuration.
    fn parse_yaml(contents: &str, path: &Path) -> Result<Config> {
        serde_yaml::from_str(contents).map_err(|source| SentinelError::ConfigParseFailed {
            path: path.to_path_buf(),
            source,
        })
    }

    /// Parses JSON configuration.
    fn parse_json(contents: &str, _path: &Path) -> Result<Config> {
        serde_json::from_str(contents).map_err(|e| SentinelError::InvalidConfig {
            reason: format!("JSON parse error: {}", e),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_config() {
        let yaml = r#"
processes:
  - name: test
    command: echo test
settings:
  logLevel: info
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();

        let config = ConfigManager::load_from_file(file.path()).unwrap();
        assert_eq!(config.processes.len(), 1);
        assert_eq!(config.processes[0].name, "test");
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = ConfigManager::load_from_file(Path::new("/nonexistent/file.yaml"));
        assert!(matches!(result, Err(SentinelError::ConfigNotFound { .. })));
    }

    #[test]
    fn test_validate_duplicate_names() {
        let config = Config {
            processes: vec![
                ProcessConfig {
                    name: "dup".to_string(),
                    command: "cmd1".to_string(),
                    cwd: None,
                    env: HashMap::new(),
                    auto_restart: true,
                    restart_limit: 5,
                    restart_delay: 1000,
                    depends_on: vec![],
                },
                ProcessConfig {
                    name: "dup".to_string(),
                    command: "cmd2".to_string(),
                    cwd: None,
                    env: HashMap::new(),
                    auto_restart: true,
                    restart_limit: 5,
                    restart_delay: 1000,
                    depends_on: vec![],
                },
            ],
            settings: Default::default(),
        };

        let result = ConfigManager::validate(&config);
        assert!(matches!(result, Err(SentinelError::InvalidConfig { .. })));
    }

    #[test]
    fn test_validate_unknown_dependency() {
        let config = Config {
            processes: vec![ProcessConfig {
                name: "test".to_string(),
                command: "cmd".to_string(),
                cwd: None,
                env: HashMap::new(),
                auto_restart: true,
                restart_limit: 5,
                restart_delay: 1000,
                depends_on: vec!["nonexistent".to_string()],
            }],
            settings: Default::default(),
        };

        let result = ConfigManager::validate(&config);
        assert!(matches!(
            result,
            Err(SentinelError::UnknownDependency { .. })
        ));
    }

    #[test]
    fn test_validate_circular_dependency() {
        let config = Config {
            processes: vec![
                ProcessConfig {
                    name: "A".to_string(),
                    command: "cmd".to_string(),
                    cwd: None,
                    env: HashMap::new(),
                    auto_restart: true,
                    restart_limit: 5,
                    restart_delay: 1000,
                    depends_on: vec!["B".to_string()],
                },
                ProcessConfig {
                    name: "B".to_string(),
                    command: "cmd".to_string(),
                    cwd: None,
                    env: HashMap::new(),
                    auto_restart: true,
                    restart_limit: 5,
                    restart_delay: 1000,
                    depends_on: vec!["A".to_string()],
                },
            ],
            settings: Default::default(),
        };

        let result = ConfigManager::validate(&config);
        assert!(matches!(result, Err(SentinelError::DependencyCycle { .. })));
    }

    #[test]
    fn test_default_config() {
        let config = ConfigManager::default_config();
        assert_eq!(config.processes.len(), 1);
        assert_eq!(config.processes[0].name, "example");
        assert!(ConfigManager::validate(&config).is_ok());
    }

    #[test]
    fn test_save_and_load_config() {
        let config = ConfigManager::default_config();
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        // Close the file so we can write to it
        drop(file);

        // Save
        ConfigManager::save_to_file(&config, &path).unwrap();

        // Load
        let loaded = ConfigManager::load_from_file(&path).unwrap();
        assert_eq!(loaded.processes.len(), config.processes.len());
        assert_eq!(loaded.processes[0].name, config.processes[0].name);
    }
}
