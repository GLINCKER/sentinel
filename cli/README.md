# Sentinel CLI

Command-line interface for Sentinel - Your Development Guardian

Built by **Glincker** (A GLINR Product)

## Installation

### From Source

```bash
cd cli
cargo build --release
sudo cp target/release/sentinel /usr/local/bin/
```

### Using Cargo

```bash
cargo install --path cli
```

## Quick Start

### 1. Initialize a Configuration

Create a new configuration file with an interactive template selector:

```bash
sentinel init
```

Or specify a template directly:

```bash
# Simple single-process setup
sentinel init --template simple

# Full-stack application (database + backend + frontend)
sentinel init --template full-stack

# Microservices architecture
sentinel init --template microservices
```

### 2. Start Your Processes

```bash
sentinel start
```

### 3. Check Status

```bash
sentinel status
```

## Commands

### `sentinel start [CONFIG_FILE]`

Start Sentinel with a configuration file.

**Options:**
- `-d, --daemon` - Start in daemon mode (background)

**Examples:**

```bash
# Start with default config (~/.config/sentinel/config.yaml)
sentinel start

# Start with custom config
sentinel start ./my-config.yaml

# Start in daemon mode
sentinel start --daemon
```

### `sentinel stop`

Stop all running processes.

**Options:**
- `-f, --force` - Force stop without graceful shutdown

**Examples:**

```bash
# Gracefully stop all processes
sentinel stop

# Force stop all processes
sentinel stop --force
```

### `sentinel restart`

Restart all processes.

**Options:**
- `-f, --force` - Force restart without graceful shutdown

**Examples:**

```bash
sentinel restart
sentinel restart --force
```

### `sentinel status`

Show status of all processes.

**Options:**
- `-v, --verbose` - Show detailed information (CPU, memory, full command)
- `-f, --format <FORMAT>` - Output format: `table` (default) or `json`

**Examples:**

```bash
# Basic status
sentinel status

# Detailed status with resource usage
sentinel status --verbose

# JSON output for scripting
sentinel status --format json
```

### `sentinel logs <PROCESS_NAME>`

Show logs for a specific process.

**Options:**
- `-f, --follow` - Follow log output (tail mode)
- `-n, --lines <N>` - Number of lines to show (default: 50)

**Examples:**

```bash
# Show last 50 lines
sentinel logs my-app

# Show last 100 lines
sentinel logs my-app --lines 100

# Follow logs (not yet implemented)
sentinel logs my-app --follow
```

### `sentinel add <NAME> <COMMAND>`

Add a new process to the configuration.

**Options:**
- `-d, --directory <PATH>` - Working directory
- `-r, --auto-restart` - Auto-restart on failure

**Examples:**

```bash
# Add a simple process
sentinel add my-app "node server.js"

# Add with working directory
sentinel add backend "npm run dev" --directory ./backend

# Add with auto-restart
sentinel add worker "python worker.py" --auto-restart
```

### `sentinel remove <NAME>`

Remove a process from the configuration.

**Options:**
- `-y, --yes` - Skip confirmation prompt

**Examples:**

```bash
# Remove with confirmation
sentinel remove my-app

# Remove without confirmation
sentinel remove my-app --yes
```

### `sentinel list`

List all configured processes.

**Options:**
- `-f, --format <FORMAT>` - Output format: `table` (default) or `json`

**Examples:**

```bash
# Table format
sentinel list

# JSON format
sentinel list --format json
```

### `sentinel init [OUTPUT_FILE]`

Initialize a new configuration file.

**Options:**
- `-t, --template <TEMPLATE>` - Use template: `simple`, `full-stack`, or `microservices`
- `-f, --force` - Overwrite existing file

**Examples:**

```bash
# Interactive template selection
sentinel init

# Create with specific template
sentinel init --template full-stack

# Custom output path
sentinel init ./configs/dev.yaml --template simple

# Overwrite existing file
sentinel init --force
```

## Configuration File

Sentinel uses YAML or JSON configuration files. The default location is `~/.config/sentinel/config.yaml`.

### Example Configuration

```yaml
# Global environment variables (applied to all processes)
global_env:
  NODE_ENV: development
  LOG_LEVEL: debug

# Process definitions
processes:
  - name: database
    command: docker
    args:
      - run
      - --rm
      - -p
      - "5432:5432"
      - postgres:15
    auto_restart: true
    max_restarts: 5
    restart_delay_ms: 2000

  - name: backend
    command: npm
    args:
      - run
      - dev
    cwd: ./backend
    env:
      PORT: "8101"
    depends_on:
      - database
    auto_restart: true
    health_check:
      command: curl
      args:
        - -f
        - http://localhost:8101/health
      interval_ms: 10000
      timeout_ms: 5000
      retries: 3

  - name: frontend
    command: npm
    args:
      - run
      - dev
    cwd: ./frontend
    env:
      PORT: "8100"
    depends_on:
      - backend
```

### Configuration Schema

- **global_env** (optional): Map of environment variables applied to all processes
- **processes**: Array of process definitions

#### Process Definition

- **name** (required): Unique process name
- **command** (required): Command to execute
- **args** (optional): Array of command arguments
- **cwd** (optional): Working directory
- **env** (optional): Process-specific environment variables
- **depends_on** (optional): Array of process names that must start first
- **auto_restart** (optional): Auto-restart on failure (default: false)
- **max_restarts** (optional): Maximum restart attempts (default: 3)
- **restart_delay_ms** (optional): Delay between restarts in milliseconds (default: 1000)
- **health_check** (optional): Health check configuration

#### Health Check Configuration

- **command** (required): Command to execute for health check
- **args** (optional): Array of command arguments
- **interval_ms** (required): Check interval in milliseconds
- **timeout_ms** (required): Check timeout in milliseconds
- **retries** (required): Number of retries before marking unhealthy

## Templates

### Simple Template

Basic single-process configuration. Good for:
- Standalone applications
- Simple scripts
- Single services

### Full-Stack Template

Three-tier application setup with:
- PostgreSQL database
- Backend API server
- Frontend web server

Includes:
- Dependency ordering (database → backend → frontend)
- Environment variables
- Health checks
- Auto-restart configuration

### Microservices Template

Multi-service architecture with:
- Redis cache
- PostgreSQL database
- Authentication service
- API gateway
- User service

Features:
- Complex dependency graph
- Global environment variables
- Service isolation
- Auto-restart for all services

## Environment Variables

### Runtime Configuration

- **RUST_LOG** - Set logging level (trace, debug, info, warn, error)

```bash
RUST_LOG=debug sentinel start
```

### Config File Location

By default, Sentinel looks for configuration at `~/.config/sentinel/config.yaml`. You can override this by passing a path to `sentinel start`:

```bash
sentinel start /path/to/custom-config.yaml
```

## Exit Codes

- **0** - Success
- **1** - General error (configuration error, process failure, etc.)

## Troubleshooting

### Config file not found

Make sure you've created a configuration file:

```bash
sentinel init
```

Or specify a config path:

```bash
sentinel start ./my-config.yaml
```

### Process won't start

1. Check the configuration is valid:
   ```bash
   sentinel list
   ```

2. Check the process status:
   ```bash
   sentinel status --verbose
   ```

3. Check the logs:
   ```bash
   sentinel logs <process-name>
   ```

### Dependency errors

If you see dependency cycle errors, review your `depends_on` configuration. Sentinel validates for circular dependencies.

## Features

### Current (Alpha)

- ✅ Process lifecycle management (start, stop, restart)
- ✅ Configuration via YAML/JSON
- ✅ Dependency ordering
- ✅ Environment variable management
- ✅ System resource monitoring
- ✅ Beautiful CLI output (colors, tables, spinners)
- ✅ Multiple configuration templates
- ✅ Process status reporting
- ✅ Basic log viewing

### Coming Soon

- ⏳ Daemon mode (background operation)
- ⏳ Log streaming (follow mode)
- ⏳ Health check execution
- ⏳ Auto-restart implementation
- ⏳ Advanced filtering and search
- ⏳ Process groups
- ⏳ Resource limits (CPU, memory)

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.

## License

MIT License - Copyright (c) 2025 Glincker (A GLINR Product)

See [LICENSE](../LICENSE) for details.
