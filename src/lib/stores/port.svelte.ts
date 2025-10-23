/**
 * Port Discovery Store
 * Uses Svelte 5 runes for reactive state management
 */

import { invoke } from '@tauri-apps/api/core';
import type { PortInfo, SortBy, SortOrder } from '../types/port';

class PortStore {
	// State
	ports = $state<PortInfo[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);
	lastScan = $state<Date | null>(null);
	lastScanDuration = $state<number>(0); // in milliseconds
	averageScanDuration = $state<number>(0); // rolling average

	// Filters and sorting
	searchQuery = $state('');
	sortBy = $state<SortBy>('port');
	sortOrder = $state<SortOrder>('asc');
	protocolFilter = $state<'all' | 'TCP' | 'UDP'>('all');
	stateFilter = $state<'all' | string>('all');
	categoryFilter = $state<'all' | 'Development' | 'Database' | 'System' | 'Application'>('all');

	// Cache (stale-while-revalidate)
	private cache: { data: PortInfo[]; timestamp: number } | null = null;
	private readonly CACHE_TTL = 5000; // 5 seconds
	private readonly SCAN_TIMEOUT = 15000; // 15 seconds

	/**
	 * Get port category helper
	 */
	private getPortCategory(port: number, processName: string): string {
		if (port < 1024) return 'System';
		if ([3000, 3001, 3002, 4200, 5000, 5173, 8000, 8080, 8888, 9000].includes(port)) {
			return 'Development';
		}
		if ([5432, 3306, 27017, 6379, 5984].includes(port)) return 'Database';
		if (
			processName.includes('postgres') ||
			processName.includes('mongo') ||
			processName.includes('redis')
		) {
			return 'Database';
		}
		return 'Application';
	}

	/**
	 * Filtered and sorted ports (derived state)
	 */
	filteredPorts = $derived.by(() => {
		let filtered = this.ports;

		// Search filter
		if (this.searchQuery.trim()) {
			const query = this.searchQuery.toLowerCase();
			filtered = filtered.filter(
				(p) =>
					p.port.toString().includes(query) ||
					p.processName.toLowerCase().includes(query) ||
					p.localAddress.toLowerCase().includes(query)
			);
		}

		// Protocol filter
		if (this.protocolFilter !== 'all') {
			filtered = filtered.filter((p) => p.protocol === this.protocolFilter);
		}

		// State filter
		if (this.stateFilter !== 'all') {
			filtered = filtered.filter((p) => p.state === this.stateFilter);
		}

		// Category filter
		if (this.categoryFilter !== 'all') {
			filtered = filtered.filter(
				(p) => this.getPortCategory(p.port, p.processName) === this.categoryFilter
			);
		}

		// Sort
		return this.sortPorts(filtered);
	});

	/**
	 * Statistics (derived)
	 */
	stats = $derived({
		total: this.ports.length,
		tcp: this.ports.filter((p) => p.protocol === 'TCP').length,
		udp: this.ports.filter((p) => p.protocol === 'UDP').length,
		listening: this.ports.filter((p) => p.state === 'Listen').length,
		established: this.ports.filter((p) => p.state === 'Established').length
	});

	/**
	 * Scan for active ports
	 */
	async scanPorts(force = false): Promise<void> {
		console.log('[PortStore] scanPorts called, force=', force);
		const now = Date.now();

		// Check cache
		if (!force && this.cache && now - this.cache.timestamp < this.CACHE_TTL) {
			console.log('[PortStore] Using cached data');
			// Serve stale data
			this.ports = this.cache.data;
			this.lastScan = new Date(this.cache.timestamp);

			// Revalidate in background
			this.revalidateInBackground();
			return;
		}

		// Fresh fetch
		console.log('[PortStore] Starting fresh scan');
		this.loading = true;
		this.error = null;

		// Start performance timer
		const startTime = performance.now();

		try {
			console.log('[PortStore] Invoking scan_ports command');

			// Add timeout to prevent hanging
			const scanPromise = invoke<PortInfo[]>('scan_ports');
			const timeoutPromise = new Promise<never>((_, reject) =>
				setTimeout(() => reject(new Error('Scan timed out after 15 seconds')), this.SCAN_TIMEOUT)
			);

			const result = await Promise.race([scanPromise, timeoutPromise]);

			// Calculate performance metrics
			const endTime = performance.now();
			const duration = endTime - startTime;
			this.lastScanDuration = Math.round(duration);

			// Update rolling average (last 10 scans)
			this.averageScanDuration = Math.round(
				this.averageScanDuration === 0
					? duration
					: (this.averageScanDuration * 9 + duration) / 10
			);

			console.log(`[PortStore] Scan completed in ${duration.toFixed(2)}ms - Received ${result.length} ports`);

			// Performance warning
			if (duration > 1000) {
				console.warn(`[PortStore] Scan took ${duration.toFixed(2)}ms (> 1s threshold)`);
			}

			this.ports = result;
			this.lastScan = new Date();
			this.cache = { data: result, timestamp: Date.now() };
		} catch (err) {
			this.error = err instanceof Error ? err.message : 'Failed to scan ports';
			console.error('[PortStore] Port scan error:', err);
		} finally {
			this.loading = false;
		}
	}

	/**
	 * Background revalidation (stale-while-revalidate)
	 */
	private async revalidateInBackground(): Promise<void> {
		try {
			// Add timeout to background revalidation too
			const scanPromise = invoke<PortInfo[]>('scan_ports');
			const timeoutPromise = new Promise<never>((_, reject) =>
				setTimeout(() => reject(new Error('Background scan timed out')), this.SCAN_TIMEOUT)
			);

			const result = await Promise.race([scanPromise, timeoutPromise]);
			this.ports = result;
			this.cache = { data: result, timestamp: Date.now() };
			this.lastScan = new Date();
		} catch (err) {
			console.error('Background revalidation error:', err);
		}
	}

	/**
	 * Get info about a specific port
	 */
	async getPortInfo(port: number): Promise<PortInfo | null> {
		try {
			return await invoke<PortInfo | null>('get_port_info', { port });
		} catch (err) {
			console.error('Get port info error:', err);
			return null;
		}
	}

	/**
	 * Kill process by port
	 */
	async killProcessByPort(port: number): Promise<boolean> {
		try {
			await invoke('kill_process_by_port', { port });
			// Refresh ports after killing
			await this.scanPorts(true);
			return true;
		} catch (err) {
			this.error = err instanceof Error ? err.message : 'Failed to kill process';
			console.error('Kill process error:', err);
			return false;
		}
	}

	/**
	 * Sort ports
	 */
	private sortPorts(ports: PortInfo[]): PortInfo[] {
		const sorted = [...ports].sort((a, b) => {
			let comparison = 0;

			switch (this.sortBy) {
				case 'port':
					comparison = a.port - b.port;
					break;
				case 'process':
					comparison = a.processName.localeCompare(b.processName);
					break;
				case 'protocol':
					comparison = a.protocol.localeCompare(b.protocol);
					break;
				case 'state':
					comparison = a.state.localeCompare(b.state);
					break;
			}

			return this.sortOrder === 'asc' ? comparison : -comparison;
		});

		return sorted;
	}

	/**
	 * Toggle sort order
	 */
	toggleSort(column: SortBy): void {
		if (this.sortBy === column) {
			this.sortOrder = this.sortOrder === 'asc' ? 'desc' : 'asc';
		} else {
			this.sortBy = column;
			this.sortOrder = 'asc';
		}
	}

	/**
	 * Reset filters
	 */
	resetFilters(): void {
		this.searchQuery = '';
		this.protocolFilter = 'all';
		this.stateFilter = 'all';
		this.categoryFilter = 'all';
	}

	/**
	 * Clear cache
	 */
	clearCache(): void {
		this.cache = null;
	}
}

// Export singleton instance
export const portStore = new PortStore();
