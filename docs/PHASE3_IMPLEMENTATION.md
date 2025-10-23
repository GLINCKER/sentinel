# Phase 3 Implementation Guide

**Product:** Sentinel - A GLINR Product by Glincker
**Phase:** 3 - Advanced Monitoring & Network
**Status:** 95% Complete
**Last Updated:** October 23, 2025

---

## Overview

Phase 3 focuses on advanced system monitoring capabilities, network traffic analysis, and port management. This phase significantly enhances Sentinel's ability to provide deep insights into network activity and service detection.

---

## Architecture

### Module Structure

```
src-tauri/src/features/
├── port_discovery/          # Phase 3A - Port Management
│   ├── mod.rs              # Tauri commands & state
│   ├── scanner.rs          # Cross-platform port scanning
│   └── types.rs            # Port data structures
├── shell_manager/           # Phase 3B - Shell Management
│   ├── mod.rs              # Shell lifecycle management
│   └── types.rs            # Shell configuration
├── service_detection/       # Phase 3C - Service Detection
│   ├── mod.rs              # Detection engine & cache
│   ├── detectors/          # Service-specific detectors
│   │   ├── http.rs         # HTTP/HTTPS detection
│   │   ├── database.rs     # PostgreSQL, MySQL, MongoDB, Redis
│   │   └── framework.rs    # Next.js, React, Flask, etc.
│   └── types.rs            # Service metadata
└── network_monitor/         # Phase 3D - Network Monitoring
    ├── mod.rs              # Tauri commands & state
    ├── collector.rs        # sysinfo network data collection
    ├── buffer.rs           # Circular buffer (5min history)
    └── types.rs            # Network statistics types
```

### Frontend Structure

```
src/
├── routes/
│   ├── ports/+page.svelte          # Port Map view
│   └── network/+page.svelte        # Network Monitor view
├── lib/components/
│   ├── PortMap/                    # Port discovery components
│   │   ├── PortMap.svelte
│   │   ├── FilterSection.svelte
│   │   ├── ServiceBadge.svelte
│   │   ├── InfoBadge.svelte
│   │   ├── PortMapFooter.svelte
│   │   ├── PortCountInfoModal.svelte
│   │   └── BadgeInfoModal.svelte
│   └── NetworkMonitor/             # Network monitoring components
│       └── NetworkGraph.svelte     # uPlot bandwidth chart
├── stores/
│   └── port.svelte.ts              # Port discovery state
├── api/
│   └── service-detection.ts        # Service detection API
└── types/
    ├── port.ts                     # Port types
    ├── service.ts                  # Service types
    └── network.ts                  # Network types
```

---

## Phase 3A: Port Discovery & Management ✅

### Implementation Details

#### Backend ([port_discovery/mod.rs](../src-tauri/src/features/port_discovery/mod.rs))

**Commands:**
- `scan_ports()` - Scans all active TCP/UDP ports
- `kill_process_by_port(port: u16)` - Terminates process using specified port

**Scanner ([scanner.rs](../src-tauri/src/features/port_discovery/scanner.rs)):**
```rust
pub struct PortScanner {
    system: System,
}

impl PortScanner {
    pub fn scan_all_ports(&mut self) -> Vec<PortInfo> {
        // Cross-platform implementation using lsof/netstat
        // Filters: TCP, UDP, IPv4, IPv6
        // Categories: System (<1024), Development, Database, Application
    }
}
```

#### Frontend ([routes/ports/+page.svelte](../src/routes/ports/+page.svelte))

**Features:**
- Virtual scrolling (handles 1000+ ports)
- Real-time search & filtering
- Port grouping (by port/PID)
- Quick filters (Development, Database, System, Listen, Established)
- Expandable connection groups
- Kill process confirmation modal
- Pagination (20/50/100 items per page)

**Performance:**
- Scan time: < 100ms (59 ports average)
- UI render: < 50ms with virtual scrolling
- Memory: ~2MB for 1000 ports

---

## Phase 3B: Shell Integration ✅

### Implementation Details

Integrated with existing Tauri PTY plugin for terminal/shell management.

**Features:**
- Multiple shell instances
- Shell process lifecycle management
- PTY integration

---

## Phase 3C: Service Detection ✅

### Implementation Details

#### Backend ([service_detection/mod.rs](../src-tauri/src/features/service_detection/mod.rs))

**Detection Engine:**
```rust
pub struct ServiceDetector {
    http_detector: HttpDetector,
    db_detector: DatabaseDetector,
    framework_detector: FrameworkDetector,
    cache: HashMap<u16, (ServiceInfo, Instant)>,
    cache_ttl: Duration, // 5 minutes
}
```

**Supported Services:**
- **Databases:** PostgreSQL (5432), MySQL (3306), MongoDB (27017), Redis (6379)
- **Web:** HTTP/HTTPS (80, 443, 8000-9000)
- **Frameworks:** Next.js (3000-3002), React, Flask (5000), Django

**Confidence Scoring:**
- Port match: 0.3-0.4
- HTTP headers: 0.5-0.7
- Process name: 0.2-0.3
- Combined confidence: Up to 0.9

#### Frontend ([lib/components/PortMap/ServiceBadge.svelte](../src/lib/components/PortMap/ServiceBadge.svelte))

**UI Components:**
- Service badges with icons
- Confidence indicators
- Color-coded by service type

---

## Phase 3D: Network Monitoring ✅

### Implementation Details

#### Backend ([network_monitor/](../src-tauri/src/features/network_monitor/))

**Data Collection:**
```rust
pub struct TrafficCollector {
    networks: Networks,          // sysinfo network interfaces
    buffer: CircularBuffer,       // 300 samples (5 minutes @ 1s)
    last_snapshot: Option<NetworkSnapshot>,
}

pub struct NetworkSnapshot {
    timestamp: DateTime<Utc>,
    total_bytes_sent: u64,        // Cumulative since boot
    total_bytes_received: u64,
    total_packets_sent: u64,
    total_packets_received: u64,
}
```

**Commands:**
- `get_network_stats()` - Current network snapshot
- `get_network_history(duration_seconds: u64)` - Historical data
- `clear_network_history()` - Reset buffer
- `get_network_interfaces()` - Per-interface statistics

**Per-Interface Data:**
```rust
pub struct NetworkInterfaceStats {
    name: String,                 // en0, lo0, eth0
    bytes_sent: u64,
    bytes_received: u64,
    packets_sent: u64,
    packets_received: u64,
    errors_sent: u64,
    errors_received: u64,
    mac_address: Option<String>,
}
```

#### Frontend ([routes/network/+page.svelte](../src/routes/network/+page.svelte))

**Features:**
- Real-time bandwidth chart (uPlot)
- Upload/Download rate calculation (KB/s, MB/s)
- Cumulative statistics (since boot)
- Time range selection (1m, 5m, 15m, 30m)
- Info modal explaining data source
- Dark mode optimized colors

**Chart Configuration:**
```javascript
// uPlot v1.6.32 (47.9 KB)
series: [
  { label: 'Upload', stroke: '#3b82f6', fill: 'rgba(59, 130, 246, 0.15)' },
  { label: 'Download', stroke: '#10b981', fill: 'rgba(16, 185, 129, 0.15)' }
]
```

**Performance:**
- Chart render: 60 FPS
- Data polling: 1s interval
- Memory: ~1MB for 5min history
- Y-axis: Dynamic scaling (B/s → KB/s → MB/s)

---

## UI/UX Improvements Completed ✅

### Design System Consistency

**PageHeader Component:**
- Unified header design across all pages (Dashboard, Port Map, Network Monitor)
- Compact layout with reduced padding and sizing
- Draggable window region with `data-tauri-drag-region`
- Black/white monochrome icon design (adaptive for light/dark modes)
- Consistent spacing and typography

**Button Styling:**
- Standardized button components across all pages
- Refresh button with spin animation
- Dropdown controls with proper contrast in both themes
- Hover states and transitions

**Navigation:**
- Absolute-positioned badges prevent layout shifts
- Hidden "0 Running" status badge when no processes
- Port count badges on sidebar navigation

### Per-Interface Breakdown Table ✅

**Implemented Features:**
- Real-time interface statistics table
- Status indicators (Active/Inactive)
- Sort by active/inactive interfaces
- Click to view detailed interface modal
- MAC address display
- Packet and error counts
- Interface type detection (Ethernet, WiFi, Loopback, VPN)

**Backend:** Implemented via `get_network_interfaces()` command

### Active Connection Tracking

Track all active network connections:
```rust
pub struct NetworkConnection {
    local_addr: String,
    remote_addr: String,
    state: String,
    pid: u32,
    process_name: String,
    protocol: String,
}
```

### Top Bandwidth Consumers

Rank processes by network usage:
```rust
pub struct ProcessBandwidthUsage {
    pid: u32,
    process_name: String,
    bytes_sent: u64,
    bytes_received: u64,
    connections: u32,
}
```

### Network Alerts

Configurable thresholds:
```rust
pub struct NetworkAlert {
    trigger: AlertTrigger,     // HighBandwidth, SuspiciousPort
    threshold: u64,
    action: AlertAction,        // Notify, Log, Kill
}
```

### Export Statistics

Export formats:
- CSV (historical data)
- JSON (raw data)
- PDF (summary report)

---

## Testing

### Current Coverage

**Backend:**
- Port discovery: 8 unit tests
- Service detection: 5 unit tests
- Network monitoring: 9 unit tests
- **Total:** 22 tests

**Frontend:**
- Port Map: Manual testing
- Network Monitor: Manual testing
- **TODO:** Add Vitest component tests

### Test Scenarios

1. **Port Scanning**
   - Empty results
   - 1000+ ports
   - Duplicate ports
   - System ports (<1024)

2. **Service Detection**
   - Known services (PostgreSQL, MongoDB)
   - Unknown services
   - Cache hit/miss
   - Confidence scoring

3. **Network Monitoring**
   - Zero traffic
   - High bandwidth (>1 GB/s)
   - Buffer overflow (>300 samples)
   - Interface changes (hotplug)

---

## Performance Metrics

### Current Benchmarks

| Operation | Target | Actual | Status |
|-----------|--------|--------|--------|
| Port scan | < 100ms | 57ms | ✅ |
| Service detect | < 50ms | 12ms | ✅ |
| Network poll | < 10ms | 4ms | ✅ |
| Chart render | 60 FPS | 60 FPS | ✅ |
| Memory (idle) | < 50MB | 38MB | ✅ |

---

## Known Issues & Solutions

1. **macOS File Attributes** ✅ Resolved
   - Issue: Extended attributes on new files cause Vite import errors
   - Solution: Document workflow in development guidelines
   - Status: Documented

2. **sysinfo Cumulative Data** ✅ Resolved
   - Issue: Returns total bytes since boot, not per-interval
   - Solution: Calculate rate of change in frontend
   - Status: Implemented

3. **MacAddr Formatting** ✅ Resolved
   - Issue: sysinfo MacAddr doesn't implement Display
   - Solution: Manual formatting (`{:02x}:{:02x}:...`)
   - Status: Fixed

4. **Window Dragging with Overlay TitleBar** ✅ Resolved
   - Issue: `data-tauri-drag-region` requires explicit permission in Tauri 2.0
   - Solution: Added `core:window:allow-start-dragging` permission to tauri.conf.json
   - Status: Working (drag region active on PageHeader)

---

## Dependencies

### Backend (Rust)
- `sysinfo = "0.37.2"` - Network interface stats
- `tauri = "2.0"` - Desktop framework
- `chrono = "0.4"` - Timestamps

### Frontend
- `uplot = "1.6.32"` - Lightweight charting (47.9 KB)
- `lucide-svelte = "0.469.0"` - Icons
- `svelte = "5.0"` - UI framework

---

## Next Steps

### Immediate (This Sprint)
1. ✅ Complete network monitoring backend
2. ✅ Fix graph Y-axis labels
3. ✅ Update ROADMAP.md
4. ✅ Add per-interface table UI
5. ✅ Add interface details modal
6. ✅ Consistent UI design across all pages
7. ✅ Window dragging functionality
8. [ ] Add active connections tracking (Phase 3E)

### Short-Term (Next Sprint)
1. [ ] Top bandwidth consumers
2. [ ] Network alerts system
3. [ ] Export statistics
4. [ ] Write component tests

### Long-Term (Phase 3E)
1. [ ] Docker container monitoring
2. [ ] Container start/stop
3. [ ] Docker Compose support

---

## Resources

- [uPlot Documentation](https://github.com/leeoniya/uPlot)
- [sysinfo Crate](https://docs.rs/sysinfo/latest/sysinfo/)
- [Tauri Commands](https://v2.tauri.app/develop/calling-rust/)
- [Phase 3 GitHub Milestone](https://github.com/glincker/sentinel/milestone/3)

---

## Contributors

- GLINR Development Team
- Community feedback from GitHub Discussions

---

Built with ❤️ by **Glincker** (A GLINR Product)

https://glincker.com/sentinel
