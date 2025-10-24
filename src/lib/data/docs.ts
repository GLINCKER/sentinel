/**
 * @file Documentation Content
 * @glinr/sentinel
 *
 * Help documentation, guides, FAQ, and troubleshooting content.
 *
 * Built by Glincker (A GLINR Product)
 * Copyright (c) 2025 Glincker. All rights reserved.
 */

import type { ComponentType } from 'svelte';
import { BookOpen, Book, Wrench, HelpCircle, FlaskConical } from 'lucide-svelte';

export interface DocSection {
	id: string;
	title: string;
	icon: ComponentType;
	content: DocArticle[];
}

export interface DocArticle {
	id: string;
	title: string;
	content: string;
	tags?: string[];
}

export const docs: DocSection[] = [
	{
		id: 'getting-started',
		title: 'Getting Started',
		icon: BookOpen,
		content: [
			{
				id: 'welcome',
				title: 'Welcome to Sentinel',
				content: `
# Welcome to Sentinel

Sentinel is your development guardian - a powerful process manager and system monitor built for developers.

## Key Features

- **Process Management**: Start, stop, and monitor your development processes
- **Port Discovery**: See what's running on which ports
- **Network Monitoring**: Track bandwidth usage and network activity
- **Docker Integration**: Manage containers directly from Sentinel
- **Shell Access**: Built-in terminal for quick commands
- **System Monitoring**: Real-time CPU, memory, and disk usage

## Quick Start

1. **Dashboard**: View all running processes and system stats
2. **Port Map**: See active ports and kill processes by port
3. **Network**: Monitor network traffic and bandwidth
4. **Docker**: Manage containers and images
5. **Terminal**: Access built-in shell

Get started by exploring the dashboard!
				`
			},
			{
				id: 'first-steps',
				title: 'First Steps',
				content: `
# First Steps with Sentinel

## 1. Understanding the Dashboard

The dashboard shows you:
- **System Metrics**: CPU, RAM, and Disk usage
- **Running Processes**: All managed processes
- **Quick Actions**: Start, stop, and restart controls

## 2. Port Management

Navigate to **Port Map** to:
- View all active ports (TCP & UDP)
- See which process owns each port
- Kill processes by port number
- Filter by port range or service type

## 3. Network Monitoring

The **Network** page provides:
- Real-time bandwidth graphs
- Upload/download rates
- Per-interface statistics
- Historical data (1m, 5m, 15m, 30m views)

## 4. Docker Management

Access **Docker** to:
- List all containers and images
- Start/stop/restart containers
- View real-time container stats
- Pause/unpause containers

## 5. Built-in Terminal

Click **Terminal** for:
- Multiple shell instances
- Full terminal emulation
- Quick command access
				`
			}
		]
	},
	{
		id: 'user-guide',
		title: 'User Guide',
		icon: Book,
		content: [
			{
				id: 'port-map',
				title: 'Using Port Map',
				tags: ['ports', 'processes', 'network'],
				content: `
# Port Map Guide

## Overview
Port Map shows all active network ports and the processes using them.

## Features

### Port Categories
- **Development** (3000-4999, 5173, 8000-8999): Common dev servers
- **Database** (3306, 5432, 6379, 27017, etc.): Database ports
- **System** (22, 80, 443, etc.): System services
- **Custom**: User-defined ranges

### Service Detection
Sentinel automatically detects services:
- HTTP servers (Node.js, Python, Go)
- Databases (PostgreSQL, MySQL, MongoDB, Redis)
- Frameworks (Next.js, React, Flask, Django)

### Filtering & Search
- Search by port number or process name
- Filter by protocol (TCP/UDP)
- Filter by port category
- Filter by connection state

### Actions
- **Kill Process**: Terminate process by port
- **View Details**: See full process information
- **Copy Port**: Quick copy to clipboard

## Keyboard Shortcuts
- \`/\` - Focus search
- \`Esc\` - Clear filters
- \`↑/↓\` - Navigate table
				`
			},
			{
				id: 'network-monitor',
				title: 'Network Monitoring',
				tags: ['network', 'bandwidth', 'traffic'],
				content: `
# Network Monitoring Guide

## Real-Time Graphs
View live bandwidth usage with interactive charts:
- **Upload Rate**: Outgoing traffic in MB/s
- **Download Rate**: Incoming traffic in MB/s
- **Combined View**: Total bandwidth usage

## Time Ranges
Select different time windows:
- **1 minute**: Most recent activity
- **5 minutes**: Short-term trends
- **15 minutes**: Medium-term patterns
- **30 minutes**: Long-term overview

## Network Interfaces
See detailed stats per interface:
- **Interface Name**: eth0, wlan0, lo, etc.
- **Status**: Active/Inactive indicators
- **Bytes**: Sent and received
- **Packets**: Transmitted and received
- **Errors**: Network errors and drops

## Interface Details
Click any interface to see:
- IP addresses and MAC address
- Network speeds and MTU
- Full packet statistics
- Error rates and details
				`
			},
			{
				id: 'docker',
				title: 'Docker Management',
				tags: ['docker', 'containers', 'images'],
				content: `
# Docker Management Guide

## Container Views

### Grid View
Visual card layout showing:
- Container name and image
- Status with color indicators
- CPU and memory usage
- Quick action buttons

### List View
Compact table layout with:
- All container details at a glance
- Sortable columns
- Batch actions

## Container Actions
- **Start**: Launch stopped containers
- **Stop**: Gracefully stop running containers
- **Restart**: Quick restart
- **Pause/Unpause**: Temporarily freeze containers

## Container Stats
Real-time metrics include:
- **CPU Usage**: Percentage of total CPU
- **Memory**: Used/Available (in MB)
- **Network**: RX/TX in MB
- **Block I/O**: Read/Write in MB

## Images Tab
View all Docker images:
- Image name and tag
- Size on disk
- Creation date
- Image ID

## Requirements
- Docker Desktop (macOS/Windows)
- Docker Engine (Linux)
- Docker socket access
				`
			},
			{
				id: 'terminal',
				title: 'Built-in Terminal',
				tags: ['terminal', 'shell', 'cli'],
				content: `
# Terminal Guide

## Multiple Shells
Create and manage multiple terminal instances:
- Click **+** to add new shell
- Switch between shells via tabs
- Close shells with **×** button

## Shell Features
- **Full Terminal Emulation**: xterm.js based
- **Color Support**: ANSI colors and styling
- **Copy/Paste**: Standard shortcuts work
- **Resize**: Automatic terminal resizing
- **Scrollback**: Full command history

## Use Cases
- Quick git commands
- Package manager operations
- Process inspection (ps, top, htop)
- File operations
- System diagnostics

## Tips
- Use built-in shell for quick tasks
- For long-running processes, use Dashboard
- Terminal inherits system PATH
- Each shell is independent
				`
			}
		]
	},
	{
		id: 'troubleshooting',
		title: 'Troubleshooting',
		icon: Wrench,
		content: [
			{
				id: 'common-issues',
				title: 'Common Issues',
				tags: ['issues', 'problems', 'fixes'],
				content: `
# Common Issues & Solutions

## Port Not Showing in Port Map

**Problem**: Expected port doesn't appear in Port Map

**Solutions**:
1. Check if process is actually running (\`ps aux | grep <process>\`)
2. Verify port is bound (\`lsof -i :<port>\` or \`netstat -an | grep <port>\`)
3. Try refreshing the view (click refresh button)
4. Check if filtering is hiding the port

## Docker Containers Not Appearing

**Problem**: Docker section shows "Docker not available"

**Solutions**:
1. Ensure Docker Desktop/Engine is running
2. Check Docker socket: \`docker ps\` in terminal
3. On macOS: Verify Docker Desktop is started
4. On Linux: Check Docker daemon status
5. Restart Sentinel after starting Docker

## High CPU Usage

**Problem**: Sentinel using excessive CPU

**Solutions**:
1. Check polling intervals in settings
2. Reduce number of monitored processes
3. Disable unused monitoring features
4. Close unused terminal instances
5. Check for background process loops

## Network Stats Not Updating

**Problem**: Network graphs frozen or not showing data

**Solutions**:
1. Verify network interfaces are active
2. Check system permissions for network monitoring
3. Try switching time ranges (1m, 5m, etc.)
4. Restart Sentinel
5. Check if VPN is interfering

## Process Won't Stop

**Problem**: Can't stop a process from Sentinel

**Solutions**:
1. Try "Force Kill" option if available
2. Check process permissions (may need sudo)
3. Use terminal: \`kill -9 <PID>\`
4. Check if process is protected by system
5. Verify process actually exists

## Slow Performance

**Problem**: Sentinel UI is laggy or slow

**Solutions**:
1. Reduce dashboard refresh rate
2. Close unused views (network, docker)
3. Limit port scan range in Port Map
4. Clear network history
5. Restart application
6. Check available system RAM
				`
			},
			{
				id: 'permissions',
				title: 'Permission Issues',
				tags: ['permissions', 'access', 'sudo'],
				content: `
# Permission Issues

## macOS Specific

### Can't Access Certain Ports
**Problem**: Some ports don't show up

**Solution**:
- Sentinel needs accessibility permissions
- Go to System Settings → Privacy & Security → Accessibility
- Enable Sentinel

### Can't Kill Processes
**Problem**: "Permission denied" when killing process

**Solution**:
- Some processes require admin rights
- Use terminal with sudo: \`sudo kill <PID>\`
- Check process ownership: \`ps aux | grep <PID>\`

## Linux Specific

### Docker Socket Access
**Problem**: Can't access Docker

**Solution**:
\`\`\`bash
# Add user to docker group
sudo usermod -aG docker $USER
# Logout and login again
\`\`\`

### Network Monitoring
**Problem**: No network stats

**Solution**:
\`\`\`bash
# Grant network capabilities
sudo setcap cap_net_raw,cap_net_admin=eip /path/to/sentinel
\`\`\`

## Windows Specific

### Administrator Required
**Problem**: Features not working

**Solution**:
- Run Sentinel as Administrator
- Right-click → "Run as administrator"
- Some features require elevated privileges
				`
			}
		]
	},
	{
		id: 'faq',
		title: 'FAQ',
		icon: HelpCircle,
		content: [
			{
				id: 'general-faq',
				title: 'Frequently Asked Questions',
				tags: ['faq', 'questions', 'help'],
				content: `
# Frequently Asked Questions

## General Questions

### What is Sentinel?
Sentinel is a modern development guardian - a process manager and system monitor built specifically for developers. It helps you manage development processes, monitor system resources, track network activity, and manage Docker containers.

### Is Sentinel free?
Yes, Sentinel is open-source and completely free to use.

### What platforms are supported?
- macOS (Intel and Apple Silicon)
- Linux (x86_64, ARM64)
- Windows (coming soon)

### Does Sentinel collect data?
No. Sentinel runs entirely locally on your machine. No telemetry, no tracking, no data collection.

## Feature Questions

### Can I manage production processes?
Sentinel is designed for development environments. For production, consider dedicated process managers like systemd, PM2, or Supervisor.

### Does it support process configuration files?
Not yet. Currently Sentinel monitors existing processes. Configuration file support is planned for a future release.

### Can I monitor remote servers?
Not currently. Sentinel only monitors the local machine. Remote monitoring is planned for future versions.

### What's the difference between Port Map and Network?
- **Port Map**: Shows which processes are using which ports (static view)
- **Network**: Shows real-time bandwidth usage and traffic graphs (dynamic view)

## Technical Questions

### How does Port Map detect services?
Sentinel uses multiple detection methods:
1. Checks HTTP endpoints for server headers
2. Analyzes process command-line arguments
3. Tests database connections
4. Matches against known port-service patterns

### Why do some ports show "Unknown" service?
If a service can't be detected, it may be:
- A custom service without standard signatures
- Behind authentication/firewall
- Not responding to detection probes

### How accurate is network monitoring?
Network stats are gathered from system APIs:
- macOS: sysctl network interface stats
- Linux: /proc/net/dev
- Accuracy depends on system reporting

### Can I export metrics?
Not currently, but planned for future release.

## Performance Questions

### How much RAM does Sentinel use?
Typically 50-150MB depending on:
- Number of monitored processes
- Active features (Docker, Network graphs)
- Number of shell instances

### Does it affect system performance?
Minimal impact:
- Low CPU usage (< 1% idle, < 5% active)
- Polling intervals are optimized
- Resource monitoring is throttled

### How often does it poll for updates?
- Dashboard: 2-3 seconds
- Port Map: On-demand
- Network: 1-2 seconds
- Docker: 3-5 seconds

All intervals are configurable in settings.

## Troubleshooting Questions

### Sentinel won't start
1. Check system requirements
2. Verify no port conflicts
3. Check logs in console
4. Try reinstalling

### Features are grayed out
Some features require:
- System permissions (Settings → Privacy)
- Running services (Docker, etc.)
- Administrator rights (Windows)

### Numbers seem wrong
1. Check system time is correct
2. Verify monitoring permissions
3. Compare with system tools (Activity Monitor, Task Manager)
4. File a bug report if persistent
				`
			}
		]
	},
	{
		id: 'api-reference',
		title: 'API Reference',
		icon: FlaskConical,
		content: [
			{
				id: 'rust-docs-intro',
				title: 'Rust API Documentation',
				content: `
# Rust API Documentation

Sentinel's backend is built with Rust and Tauri. The full API documentation is available in the **API Docs** tab above.

## Main Modules

### Core Modules
- **ProcessManager**: Process lifecycle management
- **SystemMonitor**: CPU, memory, disk monitoring
- **ConfigManager**: Configuration handling
- **LogBuffer**: Process log management

### Feature Modules
- **port_discovery**: Port scanning and process mapping
- **service_detection**: HTTP and database service detection
- **network_monitor**: Network traffic collection and stats
- **docker**: Docker container and image management

### Commands Module
All Tauri commands exposed to the frontend:
- Process commands (start, stop, restart, list)
- System commands (stats, info)
- Port discovery commands
- Network monitoring commands
- Docker commands

## Viewing Full Docs

Click the **API Docs** tab above to browse the complete Rust documentation with:
- Full module documentation
- Function signatures and examples
- Type definitions and traits
- Interactive search
- Source code links

## For Developers

If you're building on Sentinel or creating plugins:

1. Check the full API docs in the API Docs tab
2. Read module-level documentation
3. Review function examples
4. Check return types and error handling

The API docs are auto-generated from Rust source code comments.
				`
			}
		]
	}
];
