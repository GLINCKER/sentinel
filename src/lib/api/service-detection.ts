/**
 * Service Detection API
 * Frontend bindings for service_detection Tauri commands
 */

import { invoke } from '@tauri-apps/api/core';
import type { ServiceInfo } from '$lib/types/service';

/**
 * Detect service from port information
 */
export async function detectService(
  port: number,
  pid: number,
  processName: string,
  command?: string
): Promise<ServiceInfo | null> {
  try {
    const result = await invoke<ServiceInfo | null>('detect_service', {
      port,
      pid,
      processName,
      command: command || null,
    });
    return result;
  } catch (error) {
    console.error('Failed to detect service:', error);
    return null;
  }
}

/**
 * Clear service detection cache
 */
export async function clearServiceCache(): Promise<void> {
  try {
    await invoke('clear_service_cache');
  } catch (error) {
    console.error('Failed to clear service cache:', error);
  }
}

/**
 * Get service detection cache size
 */
export async function getServiceCacheSize(): Promise<number> {
  try {
    return await invoke<number>('get_service_cache_size');
  } catch (error) {
    console.error('Failed to get cache size:', error);
    return 0;
  }
}
