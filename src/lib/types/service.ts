/**
 * Service Detection Types
 * Matches Rust backend types from features/service_detection
 */

export type ServiceCategory =
  | 'WebFramework'
  | 'Database'
  | 'MessageQueue'
  | 'Cache'
  | 'Proxy'
  | 'Development';

export type HealthStatus =
  | 'Healthy'
  | 'Degraded'
  | 'Unhealthy'
  | 'Unknown';

export interface ServiceInfo {
  name: string;
  category: ServiceCategory;
  version?: string;
  description: string;
  docs_url?: string;
  health_check_path?: string;
  health_status?: HealthStatus;
  confidence: number;
  icon: string;
  detected_at: string;
}

/**
 * Service pattern for detection
 */
export interface ServicePattern {
  name: string;
  category: ServiceCategory;
  process_patterns: string[];
  port_hints: number[];
  command_patterns: string[];
  description: string;
  docs_url?: string;
  health_check_path?: string;
  icon: string;
}
