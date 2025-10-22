# Sentinel Release Process

**Last Updated:** 2025-10-21
**Status:** Production-Ready
**Built by:** Glincker (A GLINR Product)

---

## Table of Contents

- [Overview](#overview)
- [Release Strategy](#release-strategy)
- [Pre-Push Validation (Local)](#pre-push-validation-local)
- [GitHub Actions Workflows](#github-actions-workflows)
- [Versioning & Changelog](#versioning--changelog)
- [Artifact Distribution](#artifact-distribution)
- [Release Checklist](#release-checklist)

---

## Overview

**Philosophy:** Catch issues locally before wasting GitHub Actions minutes.

### Key Principles

1. ✅ **Local-First Quality:** All checks run locally via git hooks before push
2. ✅ **Manual Release Approval:** Releases are triggered manually, not automatic
3. ✅ **Multi-Platform Builds:** Automated builds for macOS, Linux, Windows
4. ✅ **Controlled Versioning:** Manual version bumps with changelog updates
5. ✅ **GitHub Releases:** Artifacts published as GitHub Releases (not package registries yet)

---

## Release Strategy

### Industry Standard: Semantic Versioning + GitHub Releases

We follow **standard OSS practices**:

- **Semantic Versioning (SemVer):** `MAJOR.MINOR.PATCH[-PRERELEASE]`
  - `MAJOR`: Breaking changes (v1.0.0 → v2.0.0)
  - `MINOR`: New features, backwards compatible (v0.1.0 → v0.2.0)
  - `PATCH`: Bug fixes (v0.1.0 → v0.1.1)
  - `PRERELEASE`: alpha, beta, rc (v0.1.0-alpha, v0.1.0-beta.1)

- **GitHub Releases:** Artifact distribution via GitHub Releases page
  - Download links: `https://github.com/glincker/sentinel/releases/latest`
  - Automatic asset hosting (DMG, AppImage, .deb, .msi)
  - Changelog embedded in release notes
  - No cleanup needed (GitHub hosts indefinitely)

- **Release Frequency:**
  - **Stable releases:** Manual approval only (avoid version bloat)
  - **Pre-releases (alpha/beta):** As needed for testing
  - **Patch releases:** Bug fixes only (not every commit)

### Why This Approach?

✅ **Standard OSS practice** (used by VS Code, Rust, Docker, etc.)
✅ **No infrastructure needed** (GitHub hosts everything)
✅ **Version control** (manual approval prevents spam)
✅ **User-friendly** (one download page for all platforms)
✅ **Free** (no hosting costs)

---

## Pre-Push Validation (Local)

### Lefthook Pre-Push Hook

**File:** [lefthook.yml](../lefthook.yml)

Runs **before every `git push`** to catch issues locally:

```yaml
pre-push:
  parallel: false
  commands:
    # Step 1: Run all tests
    backend-tests:
      run: cd src-tauri && cargo test --all --quiet

    frontend-tests:
      run: pnpm run test --run --silent

    # Step 2: Linting
    backend-lint:
      run: cd src-tauri && cargo clippy --all-targets -- -D warnings

    frontend-lint:
      run: pnpm run lint && pnpm run format:check

    # Step 3: Production build verification (macOS only for local)
    verify-build:
      run: pnpm run build
```

### What It Catches

- ✅ Failing unit tests (backend + frontend)
- ✅ Linting errors (ESLint, Clippy, Prettier)
- ✅ Build failures (frontend bundle)
- ✅ Type errors (TypeScript)

### Install Hooks

```bash
# One-time setup
pnpm run hooks:install

# Now every `git push` will run checks
git push origin main  # Hooks run automatically
```

### Skip Hooks (Emergency Only)

```bash
# If you need to bypass (not recommended)
git push --no-verify
```

---

## GitHub Actions Workflows

### 1. Test Workflow (`.github/workflows/test.yml`)

**Trigger:** Every push, every PR
**Purpose:** Comprehensive testing across platforms

**Jobs:**
- `backend-tests` (Ubuntu): Cargo tests, clippy, formatting
- `frontend-tests` (Ubuntu): Vitest, ESLint, Prettier, coverage
- `integration-tests` (Linux, macOS, Windows): Cross-platform validation
- `code-quality` (Ubuntu): TypeScript check, build verification

**Why:** Validates code quality before merge.

---

### 2. Release Workflow (`.github/workflows/release.yml`)

**Trigger:** **Manual Only** (via GitHub UI)
**Purpose:** Build and publish multi-platform releases

#### How to Trigger

1. Go to: `https://github.com/glincker/sentinel/actions/workflows/release.yml`
2. Click **"Run workflow"**
3. Fill in:
   - **Version:** `0.1.0` (without `v` prefix)
   - **Pre-release:** Check if alpha/beta/rc
4. Click **"Run workflow"** button

#### What It Does

```
┌─────────────────────────────────────────┐
│ 1. Create GitHub Release (Ubuntu)       │
│    - Creates git tag (v0.1.0)           │
│    - Creates GitHub Release             │
│    - Generates release notes            │
└─────────────────────────────────────────┘
              ↓
┌─────────────────────────────────────────┐
│ 2. Build macOS (x86_64 + ARM64)         │
│    - Builds .dmg installer              │
│    - Uploads to GitHub Release          │
└─────────────────────────────────────────┘
              ↓
┌─────────────────────────────────────────┐
│ 3. Build Linux (x86_64)                 │
│    - Builds .deb package                │
│    - Builds .AppImage                   │
│    - Uploads to GitHub Release          │
└─────────────────────────────────────────┘
              ↓
┌─────────────────────────────────────────┐
│ 4. Build Windows (x64)                  │
│    - Builds .msi installer              │
│    - Builds .exe (NSIS)                 │
│    - Uploads to GitHub Release          │
└─────────────────────────────────────────┘
```

#### Build Artifacts

| Platform | Files Generated |
|----------|----------------|
| **macOS** | `Sentinel-0.1.0-macOS-x86_64.dmg`<br/>`Sentinel-0.1.0-macOS-aarch64.dmg` |
| **Linux** | `sentinel_0.1.0_amd64.deb`<br/>`Sentinel-0.1.0-x86_64.AppImage` |
| **Windows** | `Sentinel-0.1.0-x64.msi`<br/>`Sentinel-0.1.0-x64-setup.exe` |

---

## Versioning & Changelog

### Manual Versioning (Current Approach)

**Files to Update:**

1. [`package.json`](../package.json) - Line 3: `"version": "0.1.0"`
2. [`src-tauri/Cargo.toml`](../src-tauri/Cargo.toml) - Line 3: `version = "0.1.0"`
3. [`src-tauri/tauri.conf.json`](../src-tauri/tauri.conf.json) - Line 3: `"version": "0.1.0"`
4. [`CHANGELOG.md`](../CHANGELOG.md) - Add new version section

**Process:**

```bash
# 1. Create feature branch
git checkout -b release/v0.2.0

# 2. Update version in all files
# Edit package.json, Cargo.toml, tauri.conf.json manually

# 3. Update CHANGELOG.md
# Add new version section with changes

# 4. Commit changes
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"

# 5. Push and create PR
git push origin release/v0.2.0
# Create PR, get approval, merge to main

# 6. Trigger release workflow (GitHub UI)
# Actions → Release → Run workflow → Enter "0.2.0"
```

### Automated Versioning (Future Enhancement)

**Tool:** [release-please](https://github.com/googleapis/release-please)

**Benefits:**
- Auto-generates changelog from conventional commits
- Auto-bumps version based on commit types
- Creates release PR automatically
- Used by Google, Angular, VS Code

**When to implement:** After v1.0.0 (stable release)

---

## Artifact Distribution

### Current: GitHub Releases

**Download URL:** `https://github.com/glincker/sentinel/releases/latest`

**How Users Download:**
1. Visit: `https://github.com/glincker/sentinel/releases`
2. Find latest release (e.g., `v0.1.0`)
3. Download platform-specific file
4. Install:
   - **macOS:** Open `.dmg`, drag to Applications
   - **Linux (deb):** `sudo dpkg -i sentinel_0.1.0_amd64.deb`
   - **Linux (AppImage):** `chmod +x Sentinel-0.1.0-x86_64.AppImage && ./Sentinel-0.1.0-x86_64.AppImage`
   - **Windows:** Run `.msi` or `.exe` installer

**Advantages:**
- ✅ Free hosting (GitHub)
- ✅ Automatic checksums (SHA256)
- ✅ Version history (all releases preserved)
- ✅ API access (`/repos/glincker/sentinel/releases/latest`)
- ✅ No maintenance needed

**Disadvantages:**
- ❌ Not discoverable (users must know GitHub URL)
- ❌ No auto-updates (users must manually check)

### Future: Package Managers (Post v1.0.0)

Once stable (v1.0.0+), submit to official package managers:

| Platform | Package Manager | Effort |
|----------|----------------|--------|
| **macOS** | [Homebrew](https://brew.sh) | Medium (create Formula) |
| **Linux** | [Snap Store](https://snapcraft.io) | Low (snapcraft.yaml) |
| **Linux** | [Flathub](https://flathub.org) | Medium (Flatpak manifest) |
| **Windows** | [winget](https://github.com/microsoft/winget-pkgs) | Low (submit manifest) |
| **Cross-platform** | [Cargo](https://crates.io) | Low (already published) |

**Benefits:**
- ✅ Discoverable (`brew install sentinel`, `snap install sentinel`)
- ✅ Auto-updates (package managers handle it)
- ✅ Trusted sources (official app stores)

**Cons:**
- ❌ Review process (1-7 days per platform)
- ❌ Maintenance (update formulas/manifests per release)

**Recommendation:** Submit after v1.0.0 when API is stable.

---

## Release Checklist

### Pre-Release (Local)

- [ ] All tests passing (`pnpm run test && cd src-tauri && cargo test`)
- [ ] No linting errors (`pnpm run lint && pnpm run format:check && cd src-tauri && cargo clippy`)
- [ ] Build succeeds (`pnpm run build && pnpm run tauri build`)
- [ ] Version bumped in `package.json`, `Cargo.toml`, `tauri.conf.json`
- [ ] `CHANGELOG.md` updated with changes
- [ ] Pre-push hooks passing (`git push` triggers checks)

### Release (GitHub)

- [ ] Merge release branch to `main`
- [ ] Trigger release workflow:
  - Go to: `https://github.com/glincker/sentinel/actions/workflows/release.yml`
  - Click "Run workflow"
  - Enter version (e.g., `0.1.0`)
  - Select pre-release if needed
  - Click "Run workflow"
- [ ] Wait for all builds to complete (~10-15 minutes)
- [ ] Verify artifacts uploaded to release page
- [ ] Test download links work
- [ ] Test installers on each platform (macOS, Linux, Windows)

### Post-Release

- [ ] Announce on Discord/Twitter
- [ ] Update documentation if needed
- [ ] Close milestone (if using GitHub milestones)
- [ ] Plan next version features

---

## FAQ

### Q: How do I test release builds locally?

```bash
# Test build (doesn't create installer)
pnpm run tauri build

# Output: src-tauri/target/release/sentinel

# Test full release script (creates installer)
./scripts/release.sh 0.1.0-test

# Output: release/Sentinel-0.1.0-test-macOS.dmg (or Linux/Windows equivalents)
```

### Q: How do I avoid version bloat?

**Strategy:** Release only when significant changes occur.

- **Patch (0.1.X):** Critical bugs only (not every fix)
- **Minor (0.X.0):** New features, every 2-4 weeks
- **Major (X.0.0):** Breaking changes, rare (6+ months)

**Rule of thumb:** If users wouldn't notice, don't release.

### Q: What if GitHub Actions fails?

1. Check logs: `https://github.com/glincker/sentinel/actions`
2. Fix issue locally
3. Re-trigger workflow (delete failed release first if needed)
4. If stuck, create issue

### Q: How do I delete a failed release?

```bash
# Delete release via GitHub CLI
gh release delete v0.1.0

# Delete git tag
git tag -d v0.1.0
git push origin :refs/tags/v0.1.0
```

### Q: Can I automate changelog generation?

Yes! Use [Conventional Commits](https://www.conventionalcommits.org/) + [release-please](https://github.com/googleapis/release-please).

**Setup** (future):
```yaml
# .github/workflows/release-please.yml
on:
  push:
    branches: [main]

jobs:
  release-please:
    runs-on: ubuntu-latest
    steps:
      - uses: google-github-actions/release-please-action@v3
        with:
          release-type: node
          package-name: sentinel
```

**Not implemented yet** because we're still in rapid development (pre-v1.0.0).

---

## Summary

✅ **Local checks** prevent wasted CI time (pre-push hooks)
✅ **Manual releases** prevent version spam (workflow_dispatch)
✅ **Multi-platform builds** automated (macOS, Linux, Windows)
✅ **GitHub Releases** for artifact distribution (standard OSS practice)
✅ **Future-ready** for package managers (post-v1.0.0)

**Next Steps:**
1. Finish CHANGELOG.md for v0.1.0
2. Test release workflow locally
3. Publish v0.1.0 when ready!

---

**Built with precision by [Glincker](https://glincker.com) (A GLINR Product)**
