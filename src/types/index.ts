/**
 * @file Type Definitions
 * @glinr/sentinel
 *
 * Core TypeScript type definitions for Sentinel process manager.
 *
 * Built by Glincker (A GLINR Product)
 * Copyright (c) 2025 Glincker. All rights reserved.
 *
 * @see https://glincker.com/sentinel
 */

/**
 * Process state types
 *
 * @glinr/sentinel-core
 */
export type ProcessState =
  | 'stopped'
  | 'starting'
  | 'running'
  | 'stopping'
  | { crashed: { exit_code: number } }
  | { failed: { reason: string } };

/**
 * Process information
 *
 * @glinr/sentinel-core
 */
export interface ProcessInfo {
  name: string;
  state: ProcessState;
  pid: number | null;
  started_at: string | null;
  cpu_usage: number;
  memory_usage: number;
  uptime?: string;
  restart_count?: number;
}

/**
 * CPU statistics
 *
 * @glinr/sentinel-core
 */
export interface CpuStats {
  overall: number;
  cores: number[];
  core_count: number;
}

/**
 * Memory statistics
 *
 * @glinr/sentinel-core
 */
export interface MemoryStats {
  total: number;
  used: number;
  available: number;
  swap_total: number;
  swap_used: number;
  usage_percent: number;
}

/**
 * Disk statistics
 *
 * @glinr/sentinel-core
 */
export interface DiskStats {
  read_bytes_per_sec: number;
  write_bytes_per_sec: number;
  total_space: number;
  available_space: number;
}

/**
 * System statistics
 *
 * @glinr/sentinel-core
 */
export interface SystemStats {
  cpu: CpuStats;
  memory: MemoryStats;
  disk: DiskStats;
  timestamp: number;
}

/**
 * Log entry
 *
 * @glinr/sentinel-core
 */
export interface LogEntry {
  timestamp: string;
  level: 'info' | 'warn' | 'error' | 'debug';
  message: string;
}

/**
 * Chart data point for real-time visualization
 *
 * @glinr/sentinel-core
 */
export interface DataPoint {
  timestamp: number;
  value: number;
}

/**
 * Process configuration
 *
 * @glinr/sentinel-core
 */
export interface ProcessConfig {
  name: string;
  command: string;
  args: string[];
  cwd: string | null;
  env: Record<string, string>;
  depends_on: string[];
  auto_restart: boolean | null;
  max_restarts: number | null;
  restart_delay_ms: number | null;
}
