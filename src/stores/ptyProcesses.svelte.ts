import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface PtyProcess {
	process_id: string;
	pid: number;
	status: 'running' | 'exited';
	exit_code?: number | null;
}

interface ProcessExitEvent {
	process_id: string;
	exit_code: number | null;
	timestamp: string;
}

class PtyProcessStore {
	processes = $state<PtyProcess[]>([]);

	constructor() {
		this.init();
	}

	async init() {
		// Listen for process exits to update status
		await listen<ProcessExitEvent>('process-exit', (event) => {
			const proc = this.processes.find(p => p.process_id === event.payload.process_id);
			if (proc) {
				proc.status = 'exited';
				proc.exit_code = event.payload.exit_code;
			}
		});

		// Load initial processes
		await this.refresh();
	}

	async refresh() {
		try {
			const processInfos = await invoke<any[]>('list_pty_processes');
			this.processes = processInfos.map(p => ({
				process_id: p.process_id,
				pid: p.pid,
				status: 'running' as const,
			}));
		} catch (error) {
			console.error('Failed to refresh PTY processes:', error);
		}
	}

	async spawnProcess(
		processId: string,
		command: string,
		args: string[],
		cwd?: string,
		env?: Record<string, string>
	): Promise<number> {
		const pid = await invoke<number>('spawn_pty_process', {
			processId,
			command,
			args,
			cwd: cwd || null,
			env: env || null
		});

		// Add to local state
		this.processes.push({
			process_id: processId,
			pid,
			status: 'running'
		});

		return pid;
	}

	async killProcess(processId: string) {
		await invoke('kill_pty_process', { processId });
		await this.refresh();
	}

	getProcess(processId: string): PtyProcess | undefined {
		return this.processes.find(p => p.process_id === processId);
	}

	get runningCount(): number {
		return this.processes.filter(p => p.status === 'running').length;
	}

	get exitedCount(): number {
		return this.processes.filter(p => p.status === 'exited').length;
	}
}

export const ptyProcessStore = new PtyProcessStore();
