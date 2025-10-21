# Sentinel Example Configurations

**Product:** Sentinel - A GLINR Product by Glincker

This directory contains real-world example configurations for popular development stacks.

---

## Available Examples

### 1. [MERN Stack](mern/sentinel.yaml)
**MongoDB + Express + React + Node.js**

- **Processes:** 3
  - MongoDB (Docker)
  - Express backend (Node.js)
  - React frontend (Create React App)
- **Ports:** 27017, 8100, 8101
- **Use Case:** Traditional MERN full-stack applications

**Start:**
```bash
cd examples/mern
sentinel start
```

---

### 2. [Next.js Full-Stack](nextjs/sentinel.yaml)
**Next.js App Router + API Routes + PostgreSQL**

- **Processes:** 3
  - PostgreSQL (Docker)
  - Next.js app (frontend + API routes)
  - Redis (optional caching)
- **Ports:** 5432, 6379, 8100
- **Use Case:** Modern Next.js applications with server components

**Start:**
```bash
cd examples/nextjs
sentinel start
```

---

### 3. [Python FastAPI](python-fastapi/sentinel.yaml)
**FastAPI + React + PostgreSQL + Celery**

- **Processes:** 5
  - PostgreSQL (Docker)
  - FastAPI backend (uvicorn)
  - React frontend (Vite)
  - Redis (Docker, for Celery)
  - Celery worker (async tasks)
- **Ports:** 5432, 6379, 8100, 8101
- **Use Case:** Python REST APIs with async task processing

**Start:**
```bash
cd examples/python-fastapi
sentinel start
```

---

### 4. [Spring Boot + React](spring-react/sentinel.yaml)
**Spring Boot REST API + React SPA + PostgreSQL**

- **Processes:** 4
  - PostgreSQL (Docker)
  - Spring Boot backend (Maven)
  - React frontend (Vite/CRA)
  - Redis (optional session storage)
- **Ports:** 5432, 6379, 8100, 8101
- **Use Case:** Java enterprise applications

**Start:**
```bash
cd examples/spring-react
sentinel start
```

---

## How to Use These Examples

### Option 1: Copy to Your Project

Copy the example that matches your stack:

```bash
cp examples/mern/sentinel.yaml ./my-project/sentinel.yaml
cd my-project
```

Edit `sentinel.yaml` to match your project structure:
```yaml
processes:
  - name: backend
    cwd: ./server        # ← Update this path
    command: npm
    args: [run, dev]
```

Start processes:
```bash
sentinel start
```

---

### Option 2: Run Examples Directly

Navigate to the example directory and start:

```bash
cd examples/mern
sentinel start
```

**Note:** These examples assume you have the corresponding project structure. You may need to create placeholder apps first.

---

### Option 3: Mix and Match

Create a custom config by combining parts from different examples:

```yaml
# My custom stack: Next.js + FastAPI
processes:
  # From nextjs example
  - name: postgres
    command: docker
    args: [run, --rm, -p, "5432:5432", postgres:16-alpine]

  # From python-fastapi example
  - name: backend
    command: uvicorn
    args: [main:app, --reload, --port, "8101"]
    depends_on: [postgres]

  # From nextjs example
  - name: frontend
    command: npm
    args: [run, dev]
    env:
      PORT: "8100"
    depends_on: [backend]
```

---

## Common Configuration Patterns

### 1. Port Management

All examples use the **8100-8199** port range to avoid conflicts with common dev tools:

- **Frontend:** 8100
- **Backend:** 8101
- **Additional services:** 8102+

**Avoided ports:**
- 3000 (React, Express, Create React App)
- 3001 (Common alternative)
- 5000 (Flask, Vite)
- 8000 (Django, HTTP servers)

---

### 2. Dependency Ordering

Use `depends_on` to ensure processes start in the correct order:

```yaml
processes:
  - name: database
    command: docker
    # ...

  - name: backend
    command: npm
    depends_on:
      - database  # ← Waits for database to start

  - name: frontend
    command: npm
    depends_on:
      - backend   # ← Waits for backend to start
```

---

### 3. Environment Variables

**Global (all processes):**
```yaml
global_env:
  NODE_ENV: development
  LOG_LEVEL: debug
```

**Per-process:**
```yaml
processes:
  - name: backend
    env:
      PORT: "8101"
      DATABASE_URL: postgresql://localhost:5432/myapp
```

---

### 4. Auto-Restart

Enable auto-restart for resilience:

```yaml
processes:
  - name: backend
    command: npm
    args: [run, dev]
    auto_restart: true        # ← Restart on crash
    max_restarts: 3           # ← Max attempts
    restart_delay_ms: 1000    # ← Wait 1s between restarts
```

---

### 5. Health Checks

Monitor process health:

```yaml
processes:
  - name: backend
    command: npm
    args: [run, dev]
    health_check:
      command: curl
      args:
        - -f
        - http://localhost:8101/api/health
      interval_ms: 10000      # Check every 10s
      timeout_ms: 5000
      retries: 3
```

---

### 6. Docker Integration

Run databases and services in Docker:

```yaml
processes:
  - name: postgres
    command: docker
    args:
      - run
      - --rm                  # Remove container on exit
      - --name
      - sentinel-postgres     # Container name
      - -p
      - "5432:5432"           # Port mapping
      - -e
      - POSTGRES_USER=postgres
      - -e
      - POSTGRES_PASSWORD=postgres
      - postgres:16-alpine
```

---

## Customization Tips

### Change Working Directory

```yaml
processes:
  - name: backend
    cwd: ./my-custom-path  # Run command from this directory
```

### Add Custom Scripts

```yaml
processes:
  - name: build-watch
    command: npm
    args:
      - run
      - watch:build
```

### Run Shell Commands

```yaml
processes:
  - name: setup
    command: sh
    args:
      - -c
      - "npm install && npm run migrate"
```

### Multiple Instances

```yaml
processes:
  - name: worker-1
    command: npm
    args: [run, worker]
    env:
      WORKER_ID: "1"

  - name: worker-2
    command: npm
    args: [run, worker]
    env:
      WORKER_ID: "2"
```

---

## Troubleshooting

### "Port already in use"

**Solution:** Change the port in `env` section:
```yaml
env:
  PORT: "8102"  # Use a different port
```

### "Command not found"

**Solution:** Check the command exists:
```bash
which npm     # Should output a path
which uvicorn # Should output a path
```

### "Process crashed immediately"

**Solutions:**
1. Check logs: `sentinel logs <process-name>`
2. Run command manually to see errors
3. Verify `cwd` path exists
4. Check dependencies are installed

### "Database connection failed"

**Solutions:**
1. Ensure database process started first (check `depends_on`)
2. Wait a few seconds for database to initialize
3. Check connection string in `env`
4. Verify database container is running: `docker ps`

---

## Need Help?

- **Quickstart:** [docs/QUICKSTART.md](../docs/QUICKSTART.md)
- **FAQ:** [docs/FAQ.md](../docs/FAQ.md)
- **Discord:** https://discord.gg/sentinel
- **GitHub Discussions:** https://github.com/glincker/sentinel/discussions

---

## Contributing

Have an example configuration you'd like to share?

1. Create a new directory: `examples/your-stack/`
2. Add `sentinel.yaml` with detailed comments
3. Submit a PR: [CONTRIBUTING.md](../CONTRIBUTING.md)

**Popular requests:**
- Laravel + Vue
- Django + React
- Ruby on Rails
- Elixir Phoenix
- Go + React
- Rust + WASM

---

Built with ❤️ by **Glincker** (A GLINR Product)

https://glincker.com/sentinel
