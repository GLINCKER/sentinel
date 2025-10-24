/**
 * Docker integration types
 */

export interface PortMapping {
	containerPort: number;
	hostPort?: number;
	protocol: string;
	hostIp?: string;
}

export interface ContainerInfo {
	id: string;
	fullId: string;
	name: string;
	image: string;
	status: string;
	state: string;
	ports: PortMapping[];
	cpuPercent?: number;
	memoryUsage?: number;
	memoryLimit?: number;
	networkRxBytes?: number;
	networkTxBytes?: number;
	created: string;
	labels: Array<[string, string]>;
}

export interface ContainerStats {
	containerId: string;
	cpuPercent: number;
	memoryUsage: number;
	memoryLimit: number;
	memoryPercent: number;
	networkRxBytes: number;
	networkTxBytes: number;
	blockIoRead: number;
	blockIoWrite: number;
	pids: number;
	timestamp: string;
}

export interface DockerInfo {
	available: boolean;
	version?: string;
	containersCount?: number;
	containersRunning?: number;
	containersPaused?: number;
	containersStopped?: number;
	imagesCount?: number;
	serverVersion?: string;
	operatingSystem?: string;
	architecture?: string;
}

export interface ImageInfo {
	id: string;
	fullId: string;
	repoTags: string[];
	repoDigests: string[];
	size: number;
	created: string;
	labels: Array<[string, string]>;
}

export interface ContainerOperationResult {
	success: boolean;
	containerId: string;
	operation: string;
	error?: string;
}
