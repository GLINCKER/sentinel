import { invoke } from '@tauri-apps/api/core';

export interface ProcessInfo {
	name: string;
	state: 'Starting' | 'Running' | 'Stopped' | 'Stopping' | { Crashed: { exit_code: number } };
	pid?: number;
	command: string;
	cwd?: string;
	cpuUsage: number;
	memoryUsage: number;
	restartCount: number;
	startedAt?: string;
	stoppedAt?: string;
}

export interface LogLine {
	timestamp: string;
	stream: 'stdout' | 'stderr';
	line: string;
}

class ProcessLogStore {
	processes = $state<ProcessInfo[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	async loadProcesses() {
		try {
			this.loading = true;
			this.error = null;
			const processes = await invoke<ProcessInfo[]>('list_processes');
			this.processes = processes;
		} catch (err) {
			this.error = String(err);
			console.error('Failed to load processes:', err);
		} finally {
			this.loading = false;
		}
	}

	async getLogs(name: string): Promise<LogLine[]> {
		try {
			return await invoke<LogLine[]>('get_process_logs', { name });
		} catch (err) {
			console.error(`Failed to get logs for ${name}:`, err);
			throw err;
		}
	}

	async getRecentLogs(name: string, count: number): Promise<LogLine[]> {
		try {
			return await invoke<LogLine[]>('get_recent_process_logs', { name, count });
		} catch (err) {
			console.error(`Failed to get recent logs for ${name}:`, err);
			throw err;
		}
	}

	async searchLogs(name: string, query: string): Promise<LogLine[]> {
		try {
			return await invoke<LogLine[]>('search_process_logs', { name, query });
		} catch (err) {
			console.error(`Failed to search logs for ${name}:`, err);
			throw err;
		}
	}

	async clearLogs(name: string): Promise<void> {
		try {
			await invoke('clear_process_logs', { name });
		} catch (err) {
			console.error(`Failed to clear logs for ${name}:`, err);
			throw err;
		}
	}

	async stopProcess(name: string): Promise<void> {
		try {
			await invoke('stop_process_gracefully', { name });
			await this.loadProcesses();
		} catch (err) {
			console.error(`Failed to stop process ${name}:`, err);
			throw err;
		}
	}
}

export const processLogStore = new ProcessLogStore();
