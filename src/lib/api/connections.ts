import { invoke } from '@tauri-apps/api/core';
import type { Connection, ConnectionSummary, ProcessBandwidth } from '$lib/types/connections';

/**
 * Get all active network connections
 */
export async function getActiveConnections(): Promise<Connection[]> {
	return await invoke('get_active_connections');
}

/**
 * Get connection summary statistics
 */
export async function getConnectionSummary(): Promise<ConnectionSummary> {
	return await invoke('get_connection_summary');
}

/**
 * Get top bandwidth consuming processes
 * @param limit Maximum number of processes to return (default: 10)
 */
export async function getBandwidthConsumers(limit?: number): Promise<ProcessBandwidth[]> {
	return await invoke('get_bandwidth_consumers', { limit });
}
