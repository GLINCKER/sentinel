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
}

/**
 * System statistics
 *
 * @glinr/sentinel-core
 */
export interface SystemStats {
  cpu_usage: number;
  memory_used: number;
  memory_total: number;
  disk_used: number;
  disk_total: number;
  uptime: number;
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
