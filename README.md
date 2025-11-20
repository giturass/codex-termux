# 🤖 Codex CLI - Termux Edition

> **Pre-compiled OpenAI Codex for Android Termux (ARM64)**

[![npm](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![downloads](https://img.shields.io/npm/dt/@mmmbuto/codex-cli-termux?style=flat-square)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![ko-fi](https://img.shields.io/badge/☕_Support-Ko--fi-FF5E5B?style=flat-square&logo=ko-fi)](https://ko-fi.com/dionanos)

---
> 💡 Like CLI tools? Check out my next project, **NexusCLI** — an open, developer-focused AI terminal cockpit: https://github.com/DioNanos/NexusCLI
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

Serve aiuto per debuggare gli avvisi di upgrade? Consulta
[docs/termux-upgrade-checks.md](./docs/termux-upgrade-checks.md) per cause note e
strategie di fix.

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
# Output: codex-tui 0.60.1

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

## 🧪 Testing & Validation

### Automated Test Suite

This project includes a comprehensive test suite specifically designed for Termux validation:

**Test Suite**: [`CODEX_TEST_SUITE.md`](./CODEX_TEST_SUITE.md)

**Coverage**:
- ✅ **74 automated tests** across 11 categories
- ✅ **10 Termux-specific tests** validating all 8 compatibility patches
- ✅ File operations, shell execution, environment detection
- ✅ Android permissions, library paths, package manager
- ✅ Error handling and edge cases

**How to use**:

```bash
# Start Codex
codex

# Feed the test suite
> Read and execute all tests in https://github.com/DioNanos/codex-termux/blob/main/CODEX_TEST_SUITE.md
```

Codex will automatically:
1. Execute all 74 tests sequentially
2. Report PASS/FAIL for each test
3. Generate a final summary with:
   - Total passed/failed counts
   - Category breakdowns
   - Critical failures (if any)
   - Overall verdict

**Test Categories**:
1. System Information (3 tests)
2. File Operations (8 tests)
3. Search & Discovery (3 tests)
4. Shell Execution (4 tests)
5. Text Processing (2 tests)
6. Web & Network (2 tests - optional)
7. Git Operations (2 tests - optional)
8. AI Capabilities (3 tests)
9. Error Handling (3 tests)
10. **Termux-Specific (10 tests)** ⭐ - Validates all Android patches
11. Cleanup (1 test)

**Termux-Specific Tests Include**:
- ✅ Environment paths (`$PREFIX`, `$HOME`, `$LD_LIBRARY_PATH`)
- ✅ Shell detection (bash/zsh on Android)
- ✅ Package manager (`pkg` commands)
- ✅ Storage access (`/sdcard`, `~/storage`)
- ✅ Android permissions and sandbox isolation
- ✅ Library path preservation (Patch #8 validation)
- ✅ Browser opener availability (Patch #1 validation)
- ✅ Architecture detection (aarch64/ARM64)

**Success Criteria**:
- All System, Files, Shell, and Termux tests must pass
- At least 80% overall pass rate
- No critical crashes

**Example Report**:
```
CODEX CLI TEST SUITE - FINAL REPORT
====================================
Total Tests: 74
✅ Passed: 71
❌ Failed: 1
⚠️ Skipped: 2 (WebSearch, Git - optional)

Termux-Specific: 10/10 passed ✅

VERDICT: ✅ PASS
```

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

**Version**: Based on OpenAI Codex 0.60.1 (includes GPT-5.1 support)
**Platform**: Android Termux ARM64
**Maintained**: Community-driven, not affiliated with OpenAI

---

## 📜 Changelog

### v0.60.1-termux (2025-11-20)

**Major Update**: Synced with upstream OpenAI Codex rust-v0.60.1 (250+ commits)

**Upstream Features**:
- 🤖 **GPT-5.1 Enhancements**: Continued improvements to GPT-5.1 model family
- 🔧 **App-Server Protocol**: Enhanced v2 APIs for thread management
- ⚡ **Performance Optimizations**: Improved TUI responsiveness and memory usage
- 🪟 **Windows Sandbox**: Enhanced security features (not applicable to Termux)
- 🐛 **Bug Fixes**: 250+ commits with stability improvements and fixes

**Termux-Specific**:
- ✅ **All 8 patches preserved and verified**
- ✅ **Patch #8 updated**: Shell detection refactored for upstream changes
- ✅ **Build optimized for 8GB RAM**: Compiled successfully on ROG Phone 3
- ✅ **Binary size**: 37MB (24% smaller than 0.58.4)
- ✅ **Test Suite**: 74 automated tests including 10 Termux-specific validations

**Patches Validated**:
1. ✅ Browser login (`termux-open-url`)
2. ✅ RAM optimizations (`lto=false`, `codegen-units=16`)
3. ✅ Android shell detection (`$SHELL` env var)
4. ✅ Android sandbox disabled
5. ✅ LD_* environment variables preserved
6. ✅ Auto-update URL (`DioNanos/codex-termux`)
7. ✅ Version parser (`-termux` suffix support)
8. ✅ NPM package name (`@mmmbuto/codex-cli-termux`)

**Breaking Changes**: None - fully backward compatible

**Testing**: Comprehensive test suite with 74 tests available at [`CODEX_TEST_SUITE.md`](./CODEX_TEST_SUITE.md)

Full upstream changelog: https://github.com/openai/codex/compare/rust-v0.58.0...rust-v0.60.1

---

### v0.58.4-termux (2025-11-14)

**Critical bugfix**: Auto-update detection now working

**Fixes:**
- 🐛 **Auto-update detection restored** - Fixed version parser losing `-termux` suffix support after upstream merge
- 🐛 **Tag parsing fixed** - `extract_version_from_latest_tag` now supports both `rust-v*` (upstream) and `v*-termux` (fork) formats
- 🔧 **Test coverage added** - New test for Termux tag format validation

**Technical details:**
- **Root cause**: v0.58.0 upstream merge overwrote previous `-termux` suffix fix in `parse_version()`
- **Additional issue**: New upstream code only accepted `rust-v` prefix, rejecting our `v0.58.0-termux` tags
- **Impact**: `~/.config/codex/version.json` was never created, preventing "Update available" banner
- **Solution**: Re-applied `-termux` suffix support + added `v*` prefix support in tag parser

**Affected versions**: v0.58.0 through v0.58.3 had broken auto-update detection.

**Termux patches (4 total):**
- ✅ **Patch #1**: Browser login fix (`termux-open-url`)
- ✅ **Patch #2**: RAM optimizations (`lto=false`, `codegen-units=16`)
- ✅ **Patch #3**: Auto-update URL (`@mmmbuto/codex-cli-termux`)
- ✅ **Patch #4**: Auto-update detection (this release)

---

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
