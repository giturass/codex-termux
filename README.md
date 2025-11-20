# 🤖 Codex CLI - Termux Edition

> **Pre-compiled OpenAI Codex for Android Termux (ARM64)**

[![npm](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![downloads](https://img.shields.io/npm/dt/@mmmbuto/codex-cli-termux?style=flat-square)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![ko-fi](https://img.shields.io/badge/☕_Support-Ko--fi-FF5E5B?style=flat-square&logo=ko-fi)](https://ko-fi.com/dionanos)

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
# Output: codex-tui 0.61.0

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

### Execpolicy Quickstart

Codex can enforce your own rules-based execution policy before it runs shell commands.

1. Create a policy directory: `mkdir -p ~/.codex/policy`.
2. Create one or more `.codexpolicy` files in that folder. Codex automatically loads every `.codexpolicy` file in there on startup.
3. Write `prefix_rule` entries to describe the commands you want to allow, prompt, or block:

```starlark
prefix_rule(
    pattern = ["git", ["push", "fetch"]],
    decision = "prompt",  # allow | prompt | forbidden
    match = [["git", "push", "origin", "main"]],  # examples that must match
    not_match = [["git", "status"]],              # examples that must not match
)
```

- `pattern` is a list of shell tokens, evaluated from left to right; wrap tokens in a nested list to express alternatives (e.g., match both `push` and `fetch`).
- `decision` sets the severity; Codex picks the strictest decision when multiple rules match (forbidden > prompt > allow).
- `match` and `not_match` act as (optional) unit tests. Codex validates them when it loads your policy, so you get feedback if an example has unexpected behavior.

In this example rule, if Codex wants to run commands with the prefix `git push` or `git fetch`, it will first ask for user approval.

Use [`execpolicy2` CLI](./codex-rs/execpolicy2/README.md) to preview decisions for policy files:

```shell
cargo run -p codex-execpolicy2 -- check --policy ~/.codex/policy/default.codexpolicy git push origin main
```

Pass multiple `--policy` flags to test how several files combine. See the [`codex-rs/execpolicy2` README](./codex-rs/execpolicy2/README.md) for a more detailed walkthrough of the available syntax.

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
- 📱 **Device testing** - Verification on real ARM64 hardware (ARM64 flagship device, other devices)
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

**Version**: Based on OpenAI Codex 0.61.0 (includes GPT-5.1 MAX support)
**Platform**: Android Termux ARM64
**Maintained**: Community-driven, not affiliated with OpenAI

---

## 📜 Changelog

### v0.61.0-termux (2025-11-20)

**Update**: Synced with upstream OpenAI Codex rust-v0.61.0 (13 commits from v0.60.1)

**Upstream Features**:
- 🚀 **Single Pass Truncation**: Improved performance for context management
- 🔐 **execpolicy2 Integration**: Enhanced security with new execution policy system
- 🐚 **Shell Fallback Improvements**: Better shell detection with automatic fallbacks (bash → zsh)
- 🎨 **Model Migration UX**: Stop showing migration screen after first time
- 🪟 **World-Writable Warnings**: Reduced false positives on Android

**Termux-Specific**:
- ✅ **All 8 patches preserved and verified**
- ✅ **Shell fallback compatible**: Android `$SHELL` detection enhanced with upstream fallbacks
- ✅ **Build optimized for 8GB RAM**: Compiled successfully on ROG Phone 3 (9m 06s)
- ✅ **Binary size**: 42MB (+13% vs 0.60.1 due to execpolicy2)
- ✅ **Test Suite**: 40/42 tests PASSED (95.2%), 10/10 Termux-specific tests

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

Full upstream changelog: https://github.com/openai/codex/compare/rust-v0.60.1...rust-v0.61.0

---

### v0.60.1-termux (2025-11-20)

**Major Update**: Synced with upstream OpenAI Codex rust-v0.60.1 (250+ commits)

**Upstream Features**:
- 🤖 **GPT-5.1 MAX Support**: New MAX model with enhanced capabilities and performance
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
