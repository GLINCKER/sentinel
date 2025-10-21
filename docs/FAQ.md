# Sentinel FAQ

**Product:** Sentinel - A GLINR Product by Glincker
**Last Updated:** October 2025

---

## General Questions

### What is Sentinel?

Sentinel is an open-source desktop application that combines process management with real-time system monitoring. Think of it as a more powerful, developer-focused alternative to PM2, Foreman, or systemd for local development.

**Key Features:**
- Start/stop/restart development servers with one click
- Monitor CPU, RAM, and disk usage in real-time
- Configure processes via YAML/JSON
- Beautiful desktop GUI + powerful CLI
- Cross-platform (macOS, Linux, Windows)

### Who is Sentinel for?

Sentinel is designed for:
- **Full-stack developers** managing multiple services (frontend, backend, database)
- **DevOps engineers** testing infrastructure locally
- **Students** learning microservices architecture
- **Teams** standardizing local development environments
- **Anyone** tired of juggling terminal tabs!

### Is Sentinel free?

Yes! Sentinel is 100% free and open-source (MIT License).

**Free Forever:**
- Unlimited processes
- All core features
- Desktop app + CLI
- Local-only (no account required)

**Pro Features (Future):**
- Cloud config sync
- Team collaboration
- Advanced analytics
- Priority support

**Pricing:** Free tier will always exist. Pro starts at $9/month (when launched).

### How is Sentinel different from PM2?

| Feature | Sentinel | PM2 |
|---------|----------|-----|
| GUI | ✅ Beautiful desktop app | ❌ CLI only |
| Language | Rust (fast, safe) | JavaScript (Node.js required) |
| Cross-platform | ✅ Mac, Linux, Windows | ✅ Mac, Linux, Windows |
| System monitoring | ✅ Built-in | ⚠️ Via pm2-monit |
| Docker support | 🚧 Phase 3 | ❌ Limited |
| Config format | YAML/JSON | JSON/JS |
| Startup time | < 2s | ~3-5s |
| Memory usage | ~35MB idle | ~80MB idle |
| License | MIT (FOSS) | MIT (FOSS) |

**TL;DR:** Sentinel is faster, has a GUI, and is built for modern development workflows.

---

## Installation & Setup

### How do I install Sentinel?

**macOS (Homebrew):**
```bash
brew install glincker/tap/sentinel
sentinel --version
```

**Linux (AppImage):**
```bash
wget https://github.com/glincker/sentinel/releases/latest/download/sentinel.AppImage
chmod +x sentinel.AppImage
./sentinel.AppImage
```

**Windows (Installer):**
Download `.exe` from [Releases](https://github.com/glincker/sentinel/releases)

**From Source:**
```bash
git clone https://github.com/glincker/sentinel.git
cd sentinel
cargo build --release
```

### Where does Sentinel store configuration?

**Default Config Location:**
- **macOS/Linux:** `~/.config/sentinel/config.yaml`
- **Windows:** `%APPDATA%\sentinel\config.yaml`

**Custom Location:**
```bash
sentinel start ./my-config.yaml
```

### How do I create my first config?

```bash
# Interactive template selector
sentinel init

# Or specify a template
sentinel init --template simple      # Single process
sentinel init --template full-stack  # Frontend + Backend + DB
sentinel init --template microservices
```

Then edit the generated `sentinel.yaml` and run:
```bash
sentinel start
```

---

## Usage Questions

### How do I start a process?

**Via GUI:**
1. Open Sentinel app
2. Click "Start All" or click on a process card
3. View logs in Process Detail

**Via CLI:**
```bash
# Start all processes
sentinel start

# Add a new process
sentinel add my-app "npm run dev" --directory ./my-app
```

### Can I use Sentinel with Docker?

**Current (v0.1.0-alpha):** You can add Docker commands as processes:

```yaml
processes:
  - name: postgres
    command: docker
    args:
      - run
      - --rm
      - -p
      - "5432:5432"
      - postgres:15
```

**Future (Phase 3):** Native Docker integration with:
- Container monitoring
- Docker Compose support
- Image management

### How do I view logs?

**CLI:**
```bash
# Show last 50 lines
sentinel logs my-app

# Show last 100 lines
sentinel logs my-app --lines 100

# Follow logs (coming soon)
sentinel logs my-app --follow
```

**GUI:**
Click on a process card → View logs in Process Detail view

### Can I set environment variables?

**Yes! Two ways:**

**1. Per-Process:**
```yaml
processes:
  - name: backend
    command: npm
    args: [run, dev]
    env:
      PORT: "3001"
      NODE_ENV: development
```

**2. Global (all processes):**
```yaml
global_env:
  NODE_ENV: development
  LOG_LEVEL: debug

processes:
  - name: backend
    command: npm
    # ...
```

### How do I handle process dependencies?

Use `depends_on` to ensure processes start in order:

```yaml
processes:
  - name: database
    command: docker
    # ...

  - name: backend
    command: npm
    depends_on:
      - database  # Starts after database
```

Sentinel detects circular dependencies and rejects invalid configs.

---

## Troubleshooting

### Sentinel won't start - "Failed to load config"

**Solution 1:** Check config file exists:
```bash
ls ~/.config/sentinel/config.yaml
```

**Solution 2:** Validate YAML syntax:
```bash
# Create a new valid config
sentinel init --force
```

**Solution 3:** Check file permissions:
```bash
chmod 644 ~/.config/sentinel/config.yaml
```

### Process shows "Crashed" state

**Common causes:**
1. **Command not found** - Check `command` path is correct
2. **Port already in use** - Another process is using the port
3. **Missing dependencies** - Install required packages
4. **Wrong working directory** - Check `cwd` is correct

**Debug:**
```bash
# View logs
sentinel logs my-app

# Run command manually
cd /path/to/app
npm run dev  # Or whatever your command is
```

### "Permission denied" error

**Linux/Mac:**
```bash
# Make sure Sentinel binary is executable
chmod +x sentinel

# If managing system services, use sudo
sudo sentinel start
```

**Windows:**
Run Command Prompt as Administrator

### High CPU/memory usage

**Normal behavior:**
- **Startup:** ~20% CPU for 1-2 seconds
- **Idle:** ~2% CPU, ~35MB RAM
- **10 processes:** ~5% CPU, ~50MB RAM

**If usage is high:**
1. Check process count: `sentinel status`
2. Stop unused processes: `sentinel stop`
3. Update to latest version
4. Report a bug: https://github.com/glincker/sentinel/issues

### GUI won't open

**macOS:**
```bash
# If "damaged app" error, allow in Security & Privacy
xattr -d com.apple.quarantine /Applications/Sentinel.app
```

**Linux:**
```bash
# Install required dependencies
sudo apt-get install libwebkit2gtk-4.1-dev
```

**Windows:**
Install WebView2 Runtime: https://go.microsoft.com/fwlink/p/?LinkId=2124703

---

## Configuration

### What config formats are supported?

- **YAML** (recommended) - `.yaml` or `.yml`
- **JSON** - `.json`

**Example YAML:**
```yaml
processes:
  - name: frontend
    command: npm
    args: [run, dev]
    cwd: ./frontend
```

**Same in JSON:**
```json
{
  "processes": [
    {
      "name": "frontend",
      "command": "npm",
      "args": ["run", "dev"],
      "cwd": "./frontend"
    }
  ]
}
```

### Can I use variables in config?

**Not yet.** Workaround: Use environment variables in your command:

```yaml
processes:
  - name: api
    command: sh
    args:
      - -c
      - "PORT=${API_PORT:-3000} npm start"
```

**Future:** Native variable substitution planned for Phase 2.

### How many processes can Sentinel handle?

**Tested:**
- ✅ **100 processes** - Stable, < 1GB RAM
- ✅ **10 processes** - Typical use case

**Limits:**
- **Hard limit:** 1,000 processes (safety check)
- **Recommended:** < 50 processes per config

**Stress test:**
```bash
cargo test test_100_processes_stress -- --ignored
```

---

## Performance

### Why is startup slow?

**Expected startup:** < 2 seconds

**If slower:**
1. **Large config** - 100+ processes take longer
2. **Slow disk** - Config loading from HDD vs SSD
3. **Antivirus** - Windows Defender may scan binary

**Optimize:**
- Use SSD
- Reduce process count
- Add Sentinel to antivirus exclusions

### Does Sentinel affect system performance?

**No significant impact:**
- **CPU:** ~2% idle (polling every 2 seconds)
- **Memory:** ~35MB (desktop app) + ~10MB per process overhead
- **Disk:** Config file only (~1KB per process)

**Comparison:**
- Docker Desktop: ~200MB idle
- VS Code: ~150MB idle
- Chrome: ~300MB idle
- **Sentinel: ~35MB idle** ✅

---

## Security

### Is Sentinel secure?

**Yes.** We take security seriously:

**Security Features:**
- ✅ **Input validation** - All user input sanitized
- ✅ **No shell injection** - Direct process spawning (no `sh -c`)
- ✅ **Path validation** - Prevent directory traversal
- ✅ **Memory safety** - Written in Rust (no buffer overflows)
- ✅ **Dependency scanning** - `cargo audit` in CI/CD
- ✅ **Security tests** - 15 dedicated security tests

**Audit:**
- Last security audit: October 2025
- Test coverage: 94.7% (includes security tests)
- No known vulnerabilities

**Report a vulnerability:** security@glincker.com (GPG key on our website)

### Does Sentinel send telemetry?

**No.** Sentinel is 100% local and offline.

**We do NOT collect:**
- ❌ Usage statistics
- ❌ Crash reports
- ❌ Analytics
- ❌ Personally identifiable information

**Future (opt-in only):**
When Pro features launch, cloud sync will be **opt-in** and:
- End-to-end encrypted
- Audited by third-party
- GDPR compliant
- Deletable on request

### Can I run Sentinel in production?

**Not recommended (yet).**

Sentinel is designed for **local development**, not production servers.

**Use instead:**
- **Linux:** systemd, Docker, Kubernetes
- **Cloud:** AWS ECS, GCP Cloud Run, Azure Container Instances

**Future:** Production mode planned for Phase 5 with:
- Daemon mode (background service)
- Logging to syslog
- Auto-restart on system boot
- Resource limits (ulimit)

---

## Contributing

### How can I contribute?

**Ways to help:**
1. ⭐ **Star the repo** - Helps with visibility
2. 🐛 **Report bugs** - Use issue templates
3. 💡 **Suggest features** - GitHub Discussions
4. 📝 **Improve docs** - Fix typos, add examples
5. 🔧 **Submit PRs** - See [CONTRIBUTING.md](../CONTRIBUTING.md)
6. 💬 **Help others** - Answer questions in Discussions

**Good first issues:** https://github.com/glincker/sentinel/labels/good-first-issue

### I found a bug. What should I do?

1. **Check if already reported:** [Issues](https://github.com/glincker/sentinel/issues)
2. **Create a new issue:** [Bug Report](https://github.com/glincker/sentinel/issues/new?template=bug_report.md)
3. **Include:**
   - Sentinel version (`sentinel --version`)
   - OS and version
   - Steps to reproduce
   - Expected vs actual behavior
   - Config file (if relevant)

**Critical bugs:** Email security@glincker.com

### Where can I get help?

**Community:**
- Discord: https://discord.gg/sentinel (fastest)
- GitHub Discussions: https://github.com/glincker/sentinel/discussions
- Twitter: @GlinckerHQ

**Official:**
- Documentation: https://docs.glincker.com/sentinel
- Email: sentinel@glincker.com
- Pro support: pro@glincker.com (paid plans only)

---

## Comparison with Alternatives

### Sentinel vs PM2

| Feature | Sentinel | PM2 |
|---------|----------|-----|
| GUI | ✅ Desktop app | ❌ |
| CLI | ✅ | ✅ |
| Language | Rust | Node.js |
| Startup | < 2s | ~5s |
| Memory | ~35MB | ~80MB |
| System monitor | ✅ Built-in | ⚠️ Plugin |
| Config | YAML/JSON | JSON/JS |
| Best for | Local dev | Production |

### Sentinel vs Foreman

| Feature | Sentinel | Foreman |
|---------|----------|---------|
| GUI | ✅ | ❌ |
| Language | Rust | Ruby |
| Config | YAML/JSON | Procfile |
| System monitor | ✅ | ❌ |
| Auto-restart | ✅ | ❌ |
| Cross-platform | ✅ | ⚠️ (Ruby req.) |

### Sentinel vs Docker Compose

| Feature | Sentinel | Docker Compose |
|---------|----------|----------------|
| GUI | ✅ | ❌ |
| Containers | 🚧 Phase 3 | ✅ |
| Native processes | ✅ | ❌ |
| System monitor | ✅ | ❌ |
| Startup | < 2s | ~10s |
| Best for | Mixed workflows | Containers only |

**Use both!** Sentinel + Docker Compose = 🚀

---

## Miscellaneous

### What does the name "Sentinel" mean?

**sentinel** (noun): A soldier or guard whose job is to stand and keep watch.

Sentinel watches over your development processes, keeping them running smoothly and alerting you to issues.

**Mascot:** 🛡️ A guardian shield (coming soon!)

### Who builds Sentinel?

Sentinel is built by [**Glincker**](https://glincker.com), a division of [**GLINR**](https://glinr.com).

**Team:**
- Core maintainers: 1 (hiring!)
- Contributors: [See all](https://github.com/glincker/sentinel/graphs/contributors)
- Sponsors: [See all](https://github.com/sponsors/glincker)

**Careers:** We're hiring! https://glincker.com/careers

### Is there a roadmap?

Yes! See [ROADMAP.md](ROADMAP.md) for:
- Upcoming features
- Release timeline
- Long-term vision

**Next up (Phase 2):**
- Real-time log viewer
- CPU/memory graphs
- System tray icon
- Auto-restart implementation

### How can I stay updated?

**Social:**
- Twitter: @GlinckerHQ
- Discord: https://discord.gg/sentinel
- Blog: https://glincker.com/blog

**GitHub:**
- Watch releases: https://github.com/glincker/sentinel
- Subscribe to Discussions

**Email:**
Newsletter: https://glincker.com/newsletter

---

## Still Have Questions?

**Ask the community:**
- Discord: https://discord.gg/sentinel
- GitHub Discussions: https://github.com/glincker/sentinel/discussions

**Contact us:**
- General: sentinel@glincker.com
- Security: security@glincker.com
- Press: press@glincker.com

---

Built with ❤️ by **Glincker** (A GLINR Product)

https://glincker.com/sentinel
