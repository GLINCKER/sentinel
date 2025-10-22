/**
 * Port Discovery Types
 * Matches Rust backend types from features/port_discovery
 */

export interface PortInfo {
	port: number;
	protocol: Protocol;
	processName: string;
	pid: number;
	state: PortState;
	localAddress: string;
	remoteAddress: string | null;
	traffic: NetworkTraffic;
}

export type Protocol = 'TCP' | 'UDP';

export type PortState = 'Listen' | 'Established' | 'TimeWait' | 'CloseWait' | 'Unknown';

export interface NetworkTraffic {
	bytesSent: number;
	bytesReceived: number;
	packetsSent: number;
	packetsReceived: number;
	connections: number;
}

/**
 * UI-specific types
 */
export type SortBy = 'port' | 'process' | 'protocol' | 'state';
export type SortOrder = 'asc' | 'desc';

export interface PortFilters {
	searchQuery: string;
	protocol: Protocol | 'all';
	state: PortState | 'all';
}
