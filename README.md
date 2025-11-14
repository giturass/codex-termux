# 🤖 Codex CLI - Termux Edition

> **Pre-compiled OpenAI Codex for Android Termux (ARM64)**

[![npm](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![downloads](https://img.shields.io/npm/dt/@mmmbuto/codex-cli-termux?style=flat-square)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![ko-fi](https://img.shields.io/badge/☕_Support-Ko--fi-FF5E5B?style=flat-square&logo=ko-fi)](https://ko-fi.com/dionanos)

> [!TIP]
> **Enjoying Codex on mobile?** ☕
> This project requires ~20 hours/month for ARM64 compilation, upstream tracking, and compatibility patches.
> [Support development →](https://ko-fi.com/dionanos)

---

## What This Is

Official OpenAI Codex CLI compiled for Android Termux. Since Termux is not officially supported by upstream, we apply minimal patches only for critical compatibility issues.

### What We Do:
✅ **Use official OpenAI Codex source** (https://github.com/openai/codex)
✅ **Compile for ARM64** (Android Termux native)
✅ **Apply minimal patches** only for Termux-specific issues not addressed upstream
✅ **Package as npm** for easy installation
✅ **Maintain full Apache 2.0 compliance** with OpenAI attribution

### What We DON'T Do:
❌ **NO new features**
❌ **NO behavior modifications** (works exactly like upstream)
❌ **NO replacement** of official Codex

### 🔧 Compatibility Patches

We only apply patches for issues that:
- **Prevent Codex from working on Termux**
- **Are not addressed by upstream** (Termux is not officially supported)
- **Are minimal and well-documented**

**Current patches**: See [patches/](./patches/) directory for full documentation.

**Found an issue?** Well-documented bug reports with reproduction steps are welcome! Open an [issue](https://github.com/DioNanos/codex-termux/issues).

---

## 📋 Prerequisites

```bash
# Update Termux packages
pkg update && pkg upgrade -y

# Install Node.js
pkg install nodejs-lts -y

# Verify
node --version  # v14+
npm --version   # v6+
```

**Requirements:**
- Android 7+ (Termux)
- ARM64 architecture
- Node.js ≥ 14.0.0
- ~50MB storage

---

## 📦 Installation

> [!WARNING]
> **Deprecated versions:** Versions prior to v0.57.0-termux are no longer maintained. Please upgrade to the latest release.

### Via npm (Recommended)

```bash
npm install -g @mmmbuto/codex-cli-termux
```

### Verify Installation

```bash
codex --version
# Output: codex-cli 0.58.0

codex login
# Opens browser for authentication
```

**Links:**
- npm: https://www.npmjs.com/package/@mmmbuto/codex-cli-termux
- Releases: https://github.com/DioNanos/codex-termux/releases
- Upstream: https://github.com/openai/codex

---

## 🚀 Usage

Same as official Codex CLI:

```bash
# Login to OpenAI
codex login

# Start chat
codex

# Help
codex --help
```

For full documentation, see [OpenAI Codex docs](https://github.com/openai/codex).

---

## 🔨 Building from Source

See [BUILDING.md](./BUILDING.md) for compilation instructions.

---

## 🔧 Project Maintenance

**Codex-Termux** is a community-maintained port enabling AI-powered coding on Android Termux.

**Maintenance activities:**
- 🔨 **ARM64 compilation** - Building native binaries for each upstream release (~18min per build)
- 🔄 **Upstream synchronization** - Tracking OpenAI Codex updates and merging changes
- 🐛 **Compatibility patches** - Maintaining Android-specific fixes for Termux environment
- 📱 **Device testing** - Verification on real ARM64 hardware (Pixel 9 Pro, other devices)
- 📚 **Documentation & support** - Maintaining docs, responding to GitHub issues

**Time investment:** Approximately 20 hours per month for project upkeep.

**Thank you** to all users who have reported issues, provided feedback, and helped improve this project. Your contributions make Codex accessible on mobile platforms.

---

## 📝 License

This project maintains full compliance with the Apache 2.0 license from OpenAI Codex.

**Original work**: Copyright OpenAI (https://github.com/openai/codex)
**Termux port**: Minimal patches for Android compatibility

See [LICENSE](./LICENSE) file for details.

---

## 🙏 Credits

- **OpenAI** for the amazing Codex CLI
- **Termux** community for Android terminal environment
- All contributors to upstream Codex project

---

**Version**: Based on OpenAI Codex 0.58.0 (includes GPT-5.1 support)
**Platform**: Android Termux ARM64
**Maintained**: Community-driven, not affiliated with OpenAI

---

## 📜 Changelog

### v0.58.0-termux (2025-11-13)

Synced with upstream OpenAI Codex rust-v0.58.0 (62 commits)

**Major features:**
- 🤖 **GPT-5.1 Support**: New model family (gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1)
- 🧠 **Adaptive Reasoning**: Configurable effort levels (Low, Medium, High)
- ⌨️ **Enhanced TUI**: Job control, improved navigation, better model picker
- 🔧 **Shell Detection**: Centralized command generation for unified exec
- 📊 **App-server v2**: Thread/Turn APIs improvements

**Termux-specific:**
- ✅ All Android patches preserved and verified working
- ✅ Browser login fix (termux-open-url)
- ✅ RAM optimizations (lto=false, codegen-units=16)
- ✅ Auto-update for @mmmbuto/codex-cli-termux

Full upstream changelog: https://github.com/openai/codex/compare/rust-v0.57.0...rust-v0.58.0

---

### v0.57.0-termux (2025-11-10)

Synced with upstream OpenAI Codex rust-v0.57.0 (25 commits)

**Upstream improvements:**
- ⌨️ **TUI Navigation**: CTRL-n / CTRL-p for navigating slash commands, files, history
- 🔧 **Unified Exec**: Improved safe commands handling, process group timeout fixes
- 🪟 **WSL Support**: Path normalization for Windows Subsystem for Linux
- 🚀 **App-server v2**: New Thread/Turn APIs, account endpoints
- 🧹 **Refactoring**: Terminal cleanup (deprecated flush logic removed)

**Termux-specific:**
- ✅ Android auto-update disabled (manual update instructions shown)
- ✅ `termux-open-url` for browser login (ndk-context crash fix maintained)
- ✅ RAM optimizations for 16GB devices (lto=false, codegen-units=16)

Full upstream changelog: https://github.com/openai/codex/compare/rust-v0.56.0...rust-v0.57.0
