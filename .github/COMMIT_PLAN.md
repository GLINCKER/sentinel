# Commit Strategy & Repository Organization Plan

**Status:** Pre-commit analysis
**Date:** October 2025

---

## Issue Identified

Duplicate documentation directories:
- `doc/` - Contains older documentation from prompts 1-7
- `docs/` - Contains newer documentation from prompt 8

---

## Recommended Structure

### Keep in Repository (Should be committed):

```
sentinel/
├── .github/
│   ├── workflows/          # CI/CD workflows
│   ├── ISSUE_TEMPLATE/     # Issue templates
│   ├── PULL_REQUEST_TEMPLATE.md
│   ├── CODE_OF_CONDUCT.md
│   └── FUNDING.yml
├── docs/                   # ✅ KEEP (main docs directory)
│   ├── README.md           # Documentation index
│   ├── QUICKSTART.md       # User guide
│   ├── FAQ.md              # Frequently asked questions
│   ├── ARCHITECTURE.md     # System architecture
│   ├── TESTING.md          # Testing guide
│   ├── ROADMAP.md          # Product roadmap
│   ├── CONTRIBUTING.md     # Moved from root
│   └── claude.md           # Code standards (internal)
├── examples/
│   ├── README.md
│   ├── mern/sentinel.yaml
│   ├── nextjs/sentinel.yaml
│   ├── python-fastapi/sentinel.yaml
│   └── spring-react/sentinel.yaml
├── scripts/
│   ├── build.sh
│   └── release.sh
├── Formula/
│   └── sentinel.rb         # Homebrew formula
├── cli/                    # CLI source
├── src/                    # Frontend source
├── src-tauri/              # Backend source
├── tests/                  # E2E tests
├── README.md               # ✅ Main readme
├── LICENSE
├── CONTRIBUTING.md         # Or move to docs/
├── .gitignore
├── package.json
└── ...config files
```

### Exclude from Repository (Add to .gitignore):

```
# Prompt completion docs (internal tracking only)
doc/PROMPT_*_COMPLETE.md
doc/IMPLEMENTATION_COMPLETE.md
doc/SETUP_COMPLETE.md
doc/GUI_IMPLEMENTATION.md
doc/REPO_ORGANIZATION.md
docs/PROMPT_8_COMPLETE.md

# Or move to local `.internal/` directory

# Build artifacts
dist/
release/
target/
node_modules/

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db
```

---

## Action Plan

### Step 1: Consolidate Documentation
```bash
# Move essential docs from doc/ to docs/
mv doc/ARCHITECTURE.md docs/
mv doc/RESEARCH.md docs/
mv doc/claude.md docs/

# Keep prompt completion docs in separate internal directory (don't commit)
mkdir .internal
mv doc/PROMPT_*.md .internal/
mv doc/IMPLEMENTATION_COMPLETE.md .internal/
mv doc/SETUP_COMPLETE.md .internal/
mv doc/GUI_IMPLEMENTATION.md .internal/
mv doc/REPO_ORGANIZATION.md .internal/
mv docs/PROMPT_8_COMPLETE.md .internal/

# Remove empty doc/ directory
rmdir doc/
```

### Step 2: Update .gitignore
Add exclusions for:
- Build artifacts (dist/, release/, target/)
- Internal docs (.internal/)
- IDE files
- OS files
- node_modules (should already be there)

### Step 3: Move CONTRIBUTING.md (optional)
```bash
# Option A: Keep in root (common for GitHub)
# Option B: Move to docs/ for cleaner root
mv CONTRIBUTING.md docs/
# Update README.md link
```

### Step 4: Update README.md
- Add shields.io badges (build, coverage, version, license)
- Follow OSS structure (no emojis)
- Professional tone
- Clear installation instructions
- Links to docs/

### Step 5: Split GitHub Actions Workflows

Create separate workflows following OSS best practices:

**`.github/workflows/ci.yml`** - Main CI
- Triggers: push to main, PR to main
- Jobs: lint, format, test, build
- Runs on: ubuntu-latest, macos-latest, windows-latest

**`.github/workflows/pr-checks.yml`** - PR validation
- Triggers: pull_request
- Jobs: conventional commits check, PR size check, label check

**`.github/workflows/release.yml`** - Release automation
- Triggers: push tags (v*)
- Jobs: build binaries, create GitHub release, upload assets
- Semantic versioning

**`.github/workflows/coverage.yml`** - Code coverage
- Triggers: push to main
- Jobs: run tests with coverage, upload to codecov

---

## Files to Commit (First Commit)

### Core Application Code
- [x] src/** (Frontend Svelte code)
- [x] src-tauri/** (Rust backend)
- [x] cli/** (CLI implementation)
- [x] tests/** (E2E tests)

### Configuration
- [x] package.json
- [x] Cargo.toml files
- [x] vite.config.js
- [x] tailwind.config.js
- [x] .eslintrc.cjs
- [x] .prettierrc.json

### Documentation (Cleaned)
- [x] README.md (improved)
- [x] LICENSE
- [x] CONTRIBUTING.md
- [x] docs/README.md
- [x] docs/QUICKSTART.md
- [x] docs/FAQ.md
- [x] docs/ARCHITECTURE.md
- [x] docs/TESTING.md
- [x] docs/ROADMAP.md
- [x] docs/claude.md

### Examples
- [x] examples/**

### Build Scripts
- [x] scripts/build.sh
- [x] scripts/release.sh
- [x] Formula/sentinel.rb

### GitHub Templates
- [x] .github/workflows/** (updated)
- [x] .github/ISSUE_TEMPLATE/**
- [x] .github/PULL_REQUEST_TEMPLATE.md
- [x] .github/CODE_OF_CONDUCT.md
- [x] .github/FUNDING.yml

### Ignore
- [x] .gitignore (updated)

---

## Files NOT to Commit

- doc/PROMPT_*_COMPLETE.md (internal tracking)
- docs/PROMPT_8_COMPLETE.md (internal tracking)
- doc/IMPLEMENTATION_COMPLETE.md (internal)
- doc/SETUP_COMPLETE.md (internal)
- doc/GUI_IMPLEMENTATION.md (internal)
- doc/REPO_ORGANIZATION.md (internal)
- node_modules/
- dist/
- release/
- target/
- .DS_Store

---

## Next: Execute Reorganization
