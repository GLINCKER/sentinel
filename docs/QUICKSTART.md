# Quickstart: First 5 Minutes with Sentinel

**Product:** Sentinel - A GLINR Product by Glincker
**Time to Complete:** 5 minutes
**Level:** Beginner

---

## What You'll Build

By the end of this tutorial, you'll have:
- ✅ Sentinel installed and running
- ✅ A simple full-stack app managed by Sentinel
- ✅ Real-time system monitoring dashboard
- ✅ Understanding of core commands

---

## Prerequisites

- macOS, Linux, or Windows
- Node.js 18+ (for example app)
- Docker (optional, for database examples)

---

## Step 1: Install Sentinel (30 seconds)

### macOS (Homebrew)
```bash
brew install glincker/tap/sentinel
sentinel --version
```

### Linux (AppImage)
```bash
wget https://github.com/glincker/sentinel/releases/latest/download/sentinel.AppImage
chmod +x sentinel.AppImage
./sentinel.AppImage --version
```

### Windows (Installer)
Download `.exe` from [Releases](https://github.com/glincker/sentinel/releases) and run installer.

---

## Step 2: Create Your First Config (1 minute)

Navigate to a project directory:
```bash
cd ~/my-project
```

Initialize Sentinel with interactive template selector:
```bash
sentinel init
```

You'll see:
```
🛡️  Sentinel Configuration Generator

Select a template:
  1. Simple (single process)
  2. Full-stack (frontend + backend + database)
  3. Microservices (multiple services)
  4. Custom (blank template)

Enter your choice (1-4): 2
```

Choose **2** for full-stack. Sentinel creates `sentinel.yaml`:

```yaml
processes:
  - name: database
    command: docker
    args:
      - run
      - --rm
      - -p
      - "5432:5432"
      - postgres:16-alpine

  - name: backend
    command: npm
    args:
      - run
      - dev
    cwd: ./server
    env:
      PORT: "8101"
    depends_on:
      - database

  - name: frontend
    command: npm
    args:
      - run
      - dev
    cwd: ./client
    env:
      PORT: "8100"
    depends_on:
      - backend
```

---

## Step 3: Customize Your Config (1 minute)

Open `sentinel.yaml` and update paths/commands for your project:

```yaml
processes:
  # If you don't have Docker, remove the database process
  # - name: database
  #   ...

  - name: api
    command: npm
    args:
      - run
      - start
    cwd: ./my-api-folder  # ← Update this
    env:
      PORT: "8101"

  - name: web
    command: npm
    args:
      - run
      - dev
    cwd: ./my-frontend-folder  # ← Update this
    env:
      PORT: "8100"
    depends_on:
      - api
```

**Tip:** Start simple! Just 1-2 processes for your first run.

---

## Step 4: Start Processes (10 seconds)

### Option A: CLI (Recommended for first run)
```bash
sentinel start
```

You'll see:
```
🛡️  Starting processes...

✓ api started (PID: 12345)
✓ web started (PID: 12346)

All processes running. Use 'sentinel status' to check.
```

### Option B: Desktop GUI
```bash
sentinel gui
```

Click **"Start All"** in the dashboard.

---

## Step 5: Monitor Your Processes (1 minute)

### Check Status
```bash
sentinel status
```

Output:
```
┌──────────┬──────────┬────────┬─────────┬────────────┐
│ Name     │ Status   │ PID    │ CPU     │ Memory     │
├──────────┼──────────┼────────┼─────────┼────────────┤
│ api      │ running  │ 12345  │ 2.3%    │ 45 MB      │
│ web      │ running  │ 12346  │ 1.8%    │ 38 MB      │
└──────────┴──────────┴────────┴─────────┴────────────┘

System: CPU 18.5% | RAM 4.2 GB / 16 GB | Disk 120 GB / 500 GB
```

### View Logs
```bash
sentinel logs api
```

Output:
```
[2025-10-20 10:30:15] Server listening on port 8101
[2025-10-20 10:30:16] Database connected
[2025-10-20 10:30:17] Ready to accept requests
```

### Open GUI Dashboard
```bash
sentinel gui
```

Explore:
- **Dashboard:** Real-time CPU/RAM graphs
- **Process Cards:** Click to see details
- **Quick Actions:** Start/stop/restart buttons

---

## Step 6: Manage Processes (1 minute)

### Restart a Process
```bash
sentinel restart api
```

### Stop a Process
```bash
sentinel stop web
```

### Stop All Processes
```bash
sentinel stop
```

---

## Common Use Cases

### Case 1: React + Node.js App
```yaml
processes:
  - name: backend
    command: node
    args:
      - server.js
    env:
      PORT: "8101"

  - name: frontend
    command: npm
    args:
      - start
    env:
      PORT: "8100"
```

### Case 2: Python FastAPI + React
```yaml
processes:
  - name: api
    command: uvicorn
    args:
      - main:app
      - --reload
      - --port
      - "8101"

  - name: web
    command: npm
    args:
      - run
      - dev
```

### Case 3: Multiple Microservices
```yaml
processes:
  - name: auth-service
    command: npm
    args: [run, dev]
    cwd: ./services/auth
    env:
      PORT: "8101"

  - name: user-service
    command: npm
    args: [run, dev]
    cwd: ./services/users
    env:
      PORT: "8102"

  - name: gateway
    command: npm
    args: [run, dev]
    cwd: ./services/gateway
    env:
      PORT: "8100"
    depends_on:
      - auth-service
      - user-service
```

---

## Troubleshooting

### "Failed to load config"
**Problem:** `sentinel.yaml` has invalid syntax.

**Solution:**
```bash
# Validate YAML syntax online: https://www.yamllint.com/
# Or regenerate:
sentinel init --force
```

### "Process crashed immediately"
**Problem:** Command not found or wrong directory.

**Solutions:**
1. Check command exists:
   ```bash
   which npm  # Should output a path
   ```

2. Check working directory:
   ```yaml
   processes:
     - name: api
       cwd: ./server  # Make sure this folder exists!
   ```

3. Run command manually:
   ```bash
   cd ./server
   npm run dev  # Does this work?
   ```

### "Port already in use"
**Problem:** Another process is using the port.

**Solutions:**
1. Find and kill the process:
   ```bash
   # macOS/Linux
   lsof -ti:8101 | xargs kill

   # Windows
   netstat -ano | findstr :8101
   taskkill /PID <PID> /F
   ```

2. Or change the port in `sentinel.yaml`:
   ```yaml
   env:
     PORT: "8102"  # Use a different port
   ```

---

## Next Steps

### 1. Explore Example Configs
Browse real-world examples:
```bash
ls examples/
# mern/               - MongoDB + Express + React
# nextjs/             - Next.js full-stack
# python-fastapi/     - FastAPI + React
# spring-react/       - Spring Boot + React
```

Copy one to your project:
```bash
cp examples/mern/sentinel.yaml ./sentinel.yaml
```

### 2. Add More Processes
```bash
sentinel add my-new-service "npm run dev" --directory ./new-service
```

### 3. Enable Auto-Restart
Edit `sentinel.yaml`:
```yaml
processes:
  - name: api
    command: npm
    args: [run, dev]
    auto_restart: true        # ← Restart on crash
    max_restarts: 3           # ← Max attempts
    restart_delay_ms: 1000    # ← Wait 1s between restarts
```

### 4. Set Up Health Checks
```yaml
processes:
  - name: api
    command: npm
    args: [run, dev]
    health_check:
      command: curl
      args:
        - -f
        - http://localhost:8101/health
      interval_ms: 10000      # Check every 10s
      timeout_ms: 5000
      retries: 3
```

### 5. Read Full Documentation
- **Architecture:** [docs/ARCHITECTURE.md](ARCHITECTURE.md)
- **Development Guide:** [docs/DEVELOPMENT.md](DEVELOPMENT.md)
- **FAQ:** [docs/FAQ.md](FAQ.md)
- **Roadmap:** [docs/ROADMAP.md](ROADMAP.md)

---

## CLI Cheat Sheet

```bash
# Initialize config
sentinel init

# Start all processes
sentinel start

# Start specific process
sentinel start api

# Stop all processes
sentinel stop

# Stop specific process
sentinel stop api

# Restart process
sentinel restart api

# Check status
sentinel status

# View logs
sentinel logs api

# View last 100 lines
sentinel logs api --lines 100

# Add new process
sentinel add my-service "npm run dev" --directory ./my-service

# Remove process
sentinel remove my-service

# List all processes
sentinel list

# Open GUI
sentinel gui

# Show version
sentinel --version

# Show help
sentinel --help
```

---

## Keyboard Shortcuts (GUI)

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd + S` | Start all processes |
| `Ctrl/Cmd + Q` | Stop all processes |
| `Ctrl/Cmd + R` | Restart all processes |
| `Ctrl/Cmd + ,` | Open settings |
| `Ctrl/Cmd + 1` | Go to Dashboard |
| `Ctrl/Cmd + 2` | Go to Process Detail |
| `Ctrl/Cmd + 3` | Go to Settings |

---

## Tips & Best Practices

### 1. Use Dependency Ordering
```yaml
processes:
  - name: database
    command: docker
    # ...

  - name: backend
    command: npm
    depends_on:
      - database  # ← Waits for database to start
```

### 2. Avoid Port Conflicts
Don't use common dev ports (3000, 3001, 5000, 8000):
```yaml
env:
  PORT: "8100"  # Good! Less likely to conflict
```

### 3. Set Working Directory
```yaml
processes:
  - name: api
    command: npm
    args: [run, dev]
    cwd: ./server  # ← Run command from this folder
```

### 4. Use Environment Variables
```yaml
global_env:
  NODE_ENV: development  # Applied to all processes

processes:
  - name: api
    env:
      PORT: "8101"  # Applied to this process only
```

### 5. Keep Configs Simple
Start with 1-2 processes, add more as needed.

---

## What You've Learned

✅ How to install Sentinel
✅ How to create and customize configs
✅ How to start/stop/restart processes
✅ How to monitor system resources
✅ How to troubleshoot common issues
✅ How to use the CLI and GUI

---

## Get Help

- **Discord:** https://discord.gg/sentinel
- **GitHub Discussions:** https://github.com/glincker/sentinel/discussions
- **Documentation:** https://docs.glincker.com/sentinel
- **Email:** sentinel@glincker.com

---

## What's Next?

🚀 **Share your setup!** Tweet your `sentinel.yaml` with #SentinelDev
⭐ **Star the repo:** https://github.com/glincker/sentinel
💬 **Join the community:** https://discord.gg/sentinel
📝 **Contribute:** See [CONTRIBUTING.md](../CONTRIBUTING.md)

---

Built with ❤️ by **Glincker** (A GLINR Product)

https://glincker.com/sentinel
