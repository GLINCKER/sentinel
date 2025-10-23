/**
 * Network Monitoring Types
 * Matches Rust backend types from features/network_monitor
 */

export interface NetworkSnapshot {
	timestamp: string;
	totalBytesSent: number;
	totalBytesReceived: number;
	totalPacketsSent: number;
	totalPacketsReceived: number;
	processes: ProcessNetworkStats[];
	protocolStats: ProtocolStats;
}

export interface ProcessNetworkStats {
	pid: number;
	processName: string;
	bytesSent: number;
	bytesReceived: number;
	packetsSent: number;
	packetsReceived: number;
	connections: number;
}

export interface ProtocolStats {
	tcpConnections: number;
	udpConnections: number;
	httpRequests: number;
	httpsRequests: number;
}

export interface NetworkInterfaceStats {
	name: string;
	bytesSent: number;
	bytesReceived: number;
	packetsSent: number;
	packetsReceived: number;
	errorsSent: number;
	errorsReceived: number;
	macAddress: string | null;
	interfaceType: string;
	isUp: boolean;
}
