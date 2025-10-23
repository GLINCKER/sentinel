/**
 * Connection monitoring types
 */

export interface Connection {
	protocol: string;
	localAddress: string;
	localPort: number;
	remoteAddress: string;
	remotePort: number;
	state: string;
	pid?: number;
	processName?: string;
	timestamp: string;
}

export interface ProcessBandwidth {
	pid: number;
	processName: string;
	bytesSentPerSec: number;
	bytesReceivedPerSec: number;
	totalBytesSent: number;
	totalBytesReceived: number;
	connectionCount: number;
	timestamp: string;
}

export interface ConnectionSummary {
	totalConnections: number;
	tcpConnections: number;
	udpConnections: number;
	listeningSockets: number;
	establishedConnections: number;
	timestamp: string;
}
