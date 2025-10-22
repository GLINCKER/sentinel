import { writable, derived } from 'svelte/store';
import type { SystemStats } from '../types';

export interface MetricsDataPoint {
  timestamp: number;
  cpuUsage: number;
  memoryUsage: number;
  memoryUsagePercent: number;
  diskReadRate: number;
  diskWriteRate: number;
}

const MAX_DATA_POINTS = 60; // Keep last 60 data points (2 minutes at 2s intervals)

function createMetricsHistory() {
  const { subscribe, update } = writable<MetricsDataPoint[]>([]);

  return {
    subscribe,
    addDataPoint: (stats: SystemStats) => {
      const dataPoint: MetricsDataPoint = {
        timestamp: stats.timestamp,
        cpuUsage: stats.cpu.overall,
        memoryUsage: stats.memory.used,
        memoryUsagePercent: stats.memory.usage_percent,
        diskReadRate: stats.disk.read_bytes_per_sec,
        diskWriteRate: stats.disk.write_bytes_per_sec,
      };

      update(history => {
        const newHistory = [...history, dataPoint];
        // Keep only last MAX_DATA_POINTS
        if (newHistory.length > MAX_DATA_POINTS) {
          return newHistory.slice(newHistory.length - MAX_DATA_POINTS);
        }
        return newHistory;
      });
    },
    clear: () => update(() => []),
  };
}

export const metricsHistory = createMetricsHistory();

// Derived stores for specific metrics
export const cpuHistory = derived(
  metricsHistory,
  $history => $history.map(d => ({ timestamp: d.timestamp, value: d.cpuUsage }))
);

export const memoryHistory = derived(
  metricsHistory,
  $history => $history.map(d => ({ timestamp: d.timestamp, value: d.memoryUsagePercent }))
);

export const diskHistory = derived(
  metricsHistory,
  $history => $history.map(d => ({
    timestamp: d.timestamp,
    read: d.diskReadRate,
    write: d.diskWriteRate
  }))
);

export const diskReadHistory = derived(
  metricsHistory,
  $history => $history.map(d => ({ timestamp: d.timestamp, value: d.diskReadRate / (1024 * 1024) })) // Convert to MB/s
);

export const diskWriteHistory = derived(
  metricsHistory,
  $history => $history.map(d => ({ timestamp: d.timestamp, value: d.diskWriteRate / (1024 * 1024) })) // Convert to MB/s
);
