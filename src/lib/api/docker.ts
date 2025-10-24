import { invoke } from '@tauri-apps/api/core';
import type {
	ContainerInfo,
	ContainerOperationResult,
	ContainerStats,
	DockerInfo,
	ImageInfo
} from '$lib/types/docker';

/**
 * Get Docker system information
 */
export async function getDockerInfo(): Promise<DockerInfo> {
	return await invoke('get_docker_info');
}

/**
 * List Docker containers
 * @param all If true, list all containers (including stopped). If false, only running containers.
 */
export async function listDockerContainers(all: boolean = false): Promise<ContainerInfo[]> {
	return await invoke('list_docker_containers', { all });
}

/**
 * List Docker images
 */
export async function listDockerImages(): Promise<ImageInfo[]> {
	return await invoke('list_docker_images');
}

/**
 * Get container statistics
 * @param containerId Container ID
 */
export async function getDockerContainerStats(
	containerId: string
): Promise<ContainerStats | null> {
	return await invoke('get_docker_container_stats', { containerId });
}

/**
 * Start a Docker container
 * @param containerId Container ID
 */
export async function startDockerContainer(
	containerId: string
): Promise<ContainerOperationResult> {
	return await invoke('start_docker_container', { containerId });
}

/**
 * Stop a Docker container
 * @param containerId Container ID
 * @param timeout Timeout in seconds before force-killing the container
 */
export async function stopDockerContainer(
	containerId: string,
	timeout?: number
): Promise<ContainerOperationResult> {
	return await invoke('stop_docker_container', { containerId, timeout });
}

/**
 * Restart a Docker container
 * @param containerId Container ID
 * @param timeout Timeout in seconds
 */
export async function restartDockerContainer(
	containerId: string,
	timeout?: number
): Promise<ContainerOperationResult> {
	return await invoke('restart_docker_container', { containerId, timeout });
}

/**
 * Pause a Docker container
 * @param containerId Container ID
 */
export async function pauseDockerContainer(
	containerId: string
): Promise<ContainerOperationResult> {
	return await invoke('pause_docker_container', { containerId });
}

/**
 * Unpause a Docker container
 * @param containerId Container ID
 */
export async function unpauseDockerContainer(
	containerId: string
): Promise<ContainerOperationResult> {
	return await invoke('unpause_docker_container', { containerId });
}
