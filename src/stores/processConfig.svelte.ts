import { invoke } from '@tauri-apps/api/core';

export interface ProcessConfig {
  id: string;
  name: string;
  command: string;
  args: string[];
  workingDir: string;
  envVars: Record<string, string>;
  frameworkType?: string;
  port?: number;
  autoStart: boolean;
  healthCheckUrl?: string;
  createdAt: string;
  updatedAt: string;
}

export interface ProcessStatusInfo {
  configId: string;
  running: boolean;
  processId?: string;
  pid?: number;
  status?: 'Starting' | 'Running' | 'Stopped' | 'Crashed';
  uptimeSeconds?: number;
  lastHealthCheck?: HealthCheckResult;
}

export interface HealthCheckResult {
  timestamp: string;
  success: boolean;
  responseTimeMs: number;
  error?: string;
}

export interface FrameworkDetection {
  frameworkType: string;
  confidence: number;
  detectedFiles: string[];
  suggestedCommand: string;
  suggestedArgs: string[];
  suggestedPort?: number;
}

export interface ProcessTemplate {
  name: string;
  frameworkType: string;
  description: string;
  command: string;
  args: string[];
  defaultPort?: number;
  defaultEnvVars: Record<string, string>;
  healthCheckUrl?: string;
  icon: string;
}

export interface DetectedProject {
  path: string;
  name: string;
  frameworkType: string;
  confidence: number;
  suggestedCommand: string;
  suggestedArgs: string[];
  suggestedPort?: number;
  packageManager?: string;
  detectedFiles: string[];
  envVars: Record<string, string>;
}

class ProcessConfigStore {
  configs = $state<ProcessConfig[]>([]);
  statuses = $state<Map<string, ProcessStatusInfo>>(new Map());
  loading = $state(false);
  error = $state<string | null>(null);

  async loadConfigs() {
    try {
      this.loading = true;
      this.error = null;
      this.configs = await invoke<ProcessConfig[]>('list_process_configs');

      // Load statuses for all configs
      for (const config of this.configs) {
        const status = await invoke<ProcessStatusInfo>('get_process_status_by_config', {
          configId: config.id
        });
        this.statuses.set(config.id, status);
      }
    } catch (err) {
      this.error = String(err);
    } finally {
      this.loading = false;
    }
  }

  async createConfig(config: Omit<ProcessConfig, 'id' | 'createdAt' | 'updatedAt'>) {
    try {
      const created = await invoke<ProcessConfig>('create_process_config', { config });
      this.configs = [...this.configs, created];
      this.statuses.set(created.id, {
        configId: created.id,
        running: false,
        status: 'Stopped'
      });
      return created;
    } catch (err) {
      this.error = String(err);
      throw err;
    }
  }

  async updateConfig(config: ProcessConfig) {
    try {
      const updated = await invoke<ProcessConfig>('update_process_config', { config });
      this.configs = this.configs.map((c) => (c.id === updated.id ? updated : c));
      return updated;
    } catch (err) {
      this.error = String(err);
      throw err;
    }
  }

  async deleteConfig(id: string) {
    try {
      await invoke('delete_process_config', { id });
      this.configs = this.configs.filter((c) => c.id !== id);
      this.statuses.delete(id);
    } catch (err) {
      this.error = String(err);
      throw err;
    }
  }

  async startProcess(configId: string) {
    try {
      const status = await invoke<ProcessStatusInfo>('start_process_from_config', { configId });
      this.statuses.set(configId, status);
      return status;
    } catch (err) {
      this.error = String(err);
      throw err;
    }
  }

  async stopProcess(configId: string) {
    try {
      await invoke('stop_process_by_config_id', { configId });
      await this.refreshStatus(configId);
    } catch (err) {
      this.error = String(err);
      throw err;
    }
  }

  async restartProcess(configId: string) {
    try {
      const status = await invoke<ProcessStatusInfo>('restart_managed_process', { configId });
      this.statuses.set(configId, status);
      return status;
    } catch (err) {
      this.error = String(err);
      throw err;
    }
  }

  async detectFramework(workingDir: string): Promise<FrameworkDetection> {
    return await invoke('detect_framework_type', { workingDir });
  }

  async scanDirectory(dirPath: string): Promise<DetectedProject[]> {
    return await invoke('scan_directory_for_projects', { dirPath });
  }

  async getTemplates(): Promise<ProcessTemplate[]> {
    return await invoke('get_framework_templates_list');
  }

  async exportConfigs(): Promise<string> {
    return await invoke<string>('export_process_configs');
  }

  async importConfigs(json: string): Promise<ProcessConfig[]> {
    const imported = await invoke<ProcessConfig[]>('import_process_configs', { json });
    await this.loadConfigs();
    return imported;
  }

  getStatus(configId: string): ProcessStatusInfo | undefined {
    return this.statuses.get(configId);
  }

  private async refreshStatus(configId: string) {
    try {
      const status = await invoke<ProcessStatusInfo>('get_process_status_by_config', {
        configId
      });
      this.statuses.set(configId, status);
    } catch (err) {
      console.error('Failed to refresh status:', err);
    }
  }
}

export const processConfigStore = new ProcessConfigStore();
