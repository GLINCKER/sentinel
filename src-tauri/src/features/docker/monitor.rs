//! Docker container monitoring implementation

use super::types::{
    ContainerInfo, ContainerOperationResult, ContainerStats, DockerInfo, ImageInfo, PortMapping,
};
use bollard::container::{ListContainersOptions, Stats, StatsOptions};
use bollard::image::ListImagesOptions;
use bollard::models::{ContainerSummary, ImageSummary};
use bollard::system::Version;
use bollard::Docker;
use chrono::{DateTime, Utc};

/// Monitors Docker containers and provides control operations
pub struct DockerMonitor {
    docker: Option<Docker>,
    available: bool,
}

impl Default for DockerMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerMonitor {
    /// Create a new Docker monitor
    /// Tries multiple connection strategies for better macOS compatibility
    pub fn new() -> Self {
        // Try multiple connection methods in order:
        // 1. Local defaults (works on Linux and some Docker Desktop installations)
        // 2. Docker Desktop on macOS (~/.docker/run/docker.sock)
        // 3. Unix socket at /var/run/docker.sock (fallback)

        let docker = Docker::connect_with_local_defaults()
            .or_else(|_| {
                // Try macOS Docker Desktop socket path
                #[cfg(target_os = "macos")]
                {
                    use bollard::API_DEFAULT_VERSION;
                    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users".to_string());
                    let socket_path = format!("{}/.docker/run/docker.sock", home);
                    tracing::debug!("Trying Docker socket at: {}", socket_path);
                    Docker::connect_with_unix(&socket_path, 120, API_DEFAULT_VERSION)
                }
                #[cfg(not(target_os = "macos"))]
                {
                    Err(bollard::errors::Error::DockerConnectionError)
                }
            })
            .or_else(|_| {
                // Final fallback: try unix defaults
                tracing::debug!("Trying Docker unix defaults");
                Docker::connect_with_unix_defaults()
            });

        let available = docker.is_ok();

        if !available {
            tracing::warn!("Docker connection failed, feature will be unavailable");
        } else {
            tracing::info!("Docker connected successfully");
        }

        Self {
            docker: docker.ok(),
            available,
        }
    }

    /// Check if Docker is available
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Reconnect to Docker daemon (useful after Docker starts/stops)
    pub fn reconnect(&mut self) {
        tracing::info!("Reconnecting to Docker daemon...");
        let new_monitor = Self::new();
        self.docker = new_monitor.docker;
        self.available = new_monitor.available;
    }

    /// Get Docker system information
    pub async fn get_info(&self) -> crate::error::Result<DockerInfo> {
        if !self.available || self.docker.is_none() {
            return Ok(DockerInfo {
                available: false,
                version: None,
                containers_count: None,
                containers_running: None,
                containers_paused: None,
                containers_stopped: None,
                images_count: None,
                server_version: None,
                operating_system: None,
                architecture: None,
            });
        }

        let docker = self.docker.as_ref().unwrap();

        // Get version info
        let version: Option<Version> = docker.version().await.ok();

        // Get system info
        let info = docker.info().await.ok();

        Ok(DockerInfo {
            available: true,
            version: version.as_ref().and_then(|v| v.version.clone()),
            containers_count: info.as_ref().and_then(|i| i.containers.map(|c| c as usize)),
            containers_running: info
                .as_ref()
                .and_then(|i| i.containers_running.map(|c| c as usize)),
            containers_paused: info
                .as_ref()
                .and_then(|i| i.containers_paused.map(|c| c as usize)),
            containers_stopped: info
                .as_ref()
                .and_then(|i| i.containers_stopped.map(|c| c as usize)),
            images_count: info.as_ref().and_then(|i| i.images.map(|img| img as usize)),
            server_version: version.as_ref().and_then(|v| v.version.clone()),
            operating_system: info.as_ref().and_then(|i| i.operating_system.clone()),
            architecture: version.as_ref().and_then(|v| v.arch.clone()),
        })
    }

    /// List all containers
    pub async fn list_containers(&self, all: bool) -> crate::error::Result<Vec<ContainerInfo>> {
        if !self.available || self.docker.is_none() {
            return Ok(Vec::new());
        }

        let docker = self.docker.as_ref().unwrap();

        let options = Some(ListContainersOptions::<String> {
            all,
            ..Default::default()
        });

        let containers = docker.list_containers(options).await?;

        let mut result = Vec::new();
        for container in containers {
            result.push(self.convert_container_summary(container));
        }

        Ok(result)
    }

    /// List all Docker images
    pub async fn list_images(&self) -> crate::error::Result<Vec<ImageInfo>> {
        if !self.available || self.docker.is_none() {
            return Ok(Vec::new());
        }

        let docker = self.docker.as_ref().unwrap();

        let options = Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        });

        let images = docker.list_images(options).await?;

        let mut result = Vec::new();
        for image in images {
            result.push(self.convert_image_summary(image));
        }

        Ok(result)
    }

    /// Get detailed stats for a specific container
    pub async fn get_container_stats(
        &self,
        container_id: &str,
    ) -> crate::error::Result<Option<ContainerStats>> {
        if !self.available || self.docker.is_none() {
            return Ok(None);
        }

        let docker = self.docker.as_ref().unwrap();

        let options = StatsOptions {
            stream: false,
            one_shot: true,
        };

        // Get one-shot stats
        let mut stats_stream = docker.stats(container_id, Some(options));

        // Get first (and only) stats result
        use futures_util::stream::StreamExt;
        if let Some(result) = stats_stream.next().await {
            match result {
                Ok(stats) => Ok(Some(self.convert_stats(container_id, stats))),
                Err(e) => {
                    tracing::warn!("Failed to get stats for container {}: {}", container_id, e);
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Start a container
    pub async fn start_container(
        &self,
        container_id: &str,
    ) -> crate::error::Result<ContainerOperationResult> {
        if !self.available || self.docker.is_none() {
            return Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "start".to_string(),
                error: Some("Docker is not available".to_string()),
            });
        }

        let docker = self.docker.as_ref().unwrap();

        match docker.start_container::<String>(container_id, None).await {
            Ok(_) => Ok(ContainerOperationResult {
                success: true,
                container_id: container_id.to_string(),
                operation: "start".to_string(),
                error: None,
            }),
            Err(e) => Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "start".to_string(),
                error: Some(e.to_string()),
            }),
        }
    }

    /// Stop a container
    pub async fn stop_container(
        &self,
        container_id: &str,
        timeout: Option<i64>,
    ) -> crate::error::Result<ContainerOperationResult> {
        if !self.available || self.docker.is_none() {
            return Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "stop".to_string(),
                error: Some("Docker is not available".to_string()),
            });
        }

        let docker = self.docker.as_ref().unwrap();

        match docker
            .stop_container(
                container_id,
                timeout.map(|t| bollard::container::StopContainerOptions { t }),
            )
            .await
        {
            Ok(_) => Ok(ContainerOperationResult {
                success: true,
                container_id: container_id.to_string(),
                operation: "stop".to_string(),
                error: None,
            }),
            Err(e) => Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "stop".to_string(),
                error: Some(e.to_string()),
            }),
        }
    }

    /// Restart a container
    pub async fn restart_container(
        &self,
        container_id: &str,
        timeout: Option<i64>,
    ) -> crate::error::Result<ContainerOperationResult> {
        if !self.available || self.docker.is_none() {
            return Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "restart".to_string(),
                error: Some("Docker is not available".to_string()),
            });
        }

        let docker = self.docker.as_ref().unwrap();

        match docker
            .restart_container(
                container_id,
                timeout.map(|t| bollard::container::RestartContainerOptions { t: t as isize }),
            )
            .await
        {
            Ok(_) => Ok(ContainerOperationResult {
                success: true,
                container_id: container_id.to_string(),
                operation: "restart".to_string(),
                error: None,
            }),
            Err(e) => Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "restart".to_string(),
                error: Some(e.to_string()),
            }),
        }
    }

    /// Pause a container
    pub async fn pause_container(
        &self,
        container_id: &str,
    ) -> crate::error::Result<ContainerOperationResult> {
        if !self.available || self.docker.is_none() {
            return Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "pause".to_string(),
                error: Some("Docker is not available".to_string()),
            });
        }

        let docker = self.docker.as_ref().unwrap();

        match docker.pause_container(container_id).await {
            Ok(_) => Ok(ContainerOperationResult {
                success: true,
                container_id: container_id.to_string(),
                operation: "pause".to_string(),
                error: None,
            }),
            Err(e) => Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "pause".to_string(),
                error: Some(e.to_string()),
            }),
        }
    }

    /// Unpause a container
    pub async fn unpause_container(
        &self,
        container_id: &str,
    ) -> crate::error::Result<ContainerOperationResult> {
        if !self.available || self.docker.is_none() {
            return Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "unpause".to_string(),
                error: Some("Docker is not available".to_string()),
            });
        }

        let docker = self.docker.as_ref().unwrap();

        match docker.unpause_container(container_id).await {
            Ok(_) => Ok(ContainerOperationResult {
                success: true,
                container_id: container_id.to_string(),
                operation: "unpause".to_string(),
                error: None,
            }),
            Err(e) => Ok(ContainerOperationResult {
                success: false,
                container_id: container_id.to_string(),
                operation: "unpause".to_string(),
                error: Some(e.to_string()),
            }),
        }
    }

    /// Convert bollard ContainerSummary to our ContainerInfo
    fn convert_container_summary(&self, summary: ContainerSummary) -> ContainerInfo {
        let id = summary.id.clone().unwrap_or_default();
        let short_id = if id.len() > 12 {
            id[..12].to_string()
        } else {
            id.clone()
        };

        let name = summary
            .names
            .as_ref()
            .and_then(|names| names.first())
            .map(|n| n.trim_start_matches('/').to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let image = summary
            .image
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        let status = summary
            .status
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        let state = summary
            .state
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        // Parse ports
        let ports = summary
            .ports
            .as_ref()
            .map(|port_vec| {
                port_vec
                    .iter()
                    .map(|p| PortMapping {
                        container_port: p.private_port,
                        host_port: p.public_port,
                        protocol: p
                            .typ
                            .as_ref()
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| "tcp".to_string()),
                        host_ip: p.ip.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Parse created timestamp
        let created = summary
            .created
            .and_then(|ts| DateTime::from_timestamp(ts, 0))
            .unwrap_or_else(Utc::now);

        // Parse labels
        let labels = summary
            .labels
            .as_ref()
            .map(|l| l.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();

        ContainerInfo {
            id: short_id,
            full_id: id,
            name,
            image,
            status,
            state,
            ports,
            cpu_percent: None,
            memory_usage: None,
            memory_limit: None,
            network_rx_bytes: None,
            network_tx_bytes: None,
            created,
            labels,
        }
    }

    /// Convert bollard Stats to our ContainerStats
    fn convert_stats(&self, container_id: &str, stats: Stats) -> ContainerStats {
        // Calculate CPU percentage
        let cpu_percent = {
            let cpu_stats = &stats.cpu_stats;
            let precpu_stats = &stats.precpu_stats;

            let cpu_delta =
                cpu_stats.cpu_usage.total_usage as f64 - precpu_stats.cpu_usage.total_usage as f64;
            let system_delta = cpu_stats.system_cpu_usage.unwrap_or(0) as f64
                - precpu_stats.system_cpu_usage.unwrap_or(0) as f64;
            let num_cpus = cpu_stats.online_cpus.unwrap_or_else(|| {
                cpu_stats
                    .cpu_usage
                    .percpu_usage
                    .as_ref()
                    .map(|v| v.len() as u64)
                    .unwrap_or(1)
            }) as f64;

            if system_delta > 0.0 && cpu_delta > 0.0 {
                (cpu_delta / system_delta) * num_cpus * 100.0
            } else {
                0.0
            }
        };

        // Memory stats
        let memory_usage = stats.memory_stats.usage.unwrap_or(0);
        let memory_limit = stats.memory_stats.limit.unwrap_or(0);
        let memory_percent = if memory_limit > 0 {
            (memory_usage as f64 / memory_limit as f64) * 100.0
        } else {
            0.0
        };

        // Network stats
        let (network_rx_bytes, network_tx_bytes) = stats
            .networks
            .as_ref()
            .map(|networks| {
                networks.values().fold((0u64, 0u64), |(rx, tx), net| {
                    (rx + net.rx_bytes, tx + net.tx_bytes)
                })
            })
            .unwrap_or((0, 0));

        // Block I/O stats
        let (block_io_read, block_io_write) = {
            let blkio = &stats.blkio_stats;
            let read = blkio
                .io_service_bytes_recursive
                .as_ref()
                .map(|v| {
                    v.iter()
                        .filter(|s| s.op == "read" || s.op == "Read")
                        .map(|s| s.value)
                        .sum()
                })
                .unwrap_or(0);

            let write = blkio
                .io_service_bytes_recursive
                .as_ref()
                .map(|v| {
                    v.iter()
                        .filter(|s| s.op == "write" || s.op == "Write")
                        .map(|s| s.value)
                        .sum()
                })
                .unwrap_or(0);

            (read, write)
        };

        // PIDs
        let pids = stats.pids_stats.current.unwrap_or(0);

        ContainerStats {
            container_id: container_id.to_string(),
            cpu_percent,
            memory_usage,
            memory_limit,
            memory_percent,
            network_rx_bytes,
            network_tx_bytes,
            block_io_read,
            block_io_write,
            pids,
            timestamp: Utc::now(),
        }
    }

    /// Convert bollard ImageSummary to our ImageInfo
    fn convert_image_summary(&self, summary: ImageSummary) -> ImageInfo {
        let id = summary.id.clone();
        let short_id = if id.len() > 19 {
            // Docker image IDs are "sha256:..." format
            if id.starts_with("sha256:") {
                id[7..19].to_string()
            } else if id.len() > 12 {
                id[..12].to_string()
            } else {
                id.clone()
            }
        } else {
            id.clone()
        };

        let repo_tags = summary.repo_tags;
        let repo_digests = summary.repo_digests;
        let size = summary.size as u64;

        // Parse created timestamp
        let created = DateTime::from_timestamp(summary.created, 0).unwrap_or_else(Utc::now);

        // Parse labels - ImageSummary.labels is HashMap<String, String>, not Option
        let labels: Vec<(String, String)> = summary.labels.into_iter().collect();

        ImageInfo {
            id: short_id,
            full_id: id,
            repo_tags,
            repo_digests,
            size,
            created,
            labels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() {
        let monitor = DockerMonitor::new();
        // Don't assert availability as Docker might not be installed
        let _ = monitor.is_available();
    }

    #[tokio::test]
    async fn test_get_info() {
        let monitor = DockerMonitor::new();
        let result = monitor.get_info().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_containers() {
        let monitor = DockerMonitor::new();
        let result = monitor.list_containers(true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_container_operations_when_docker_unavailable() {
        let monitor = DockerMonitor {
            docker: None,
            available: false,
        };

        let result = monitor.start_container("test").await;
        assert!(result.is_ok());
        assert!(!result.unwrap().success);
    }
}
