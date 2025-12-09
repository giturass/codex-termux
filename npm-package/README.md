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
# Output: codex-cli 0.66.0

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

### Non-Interactive Mode (Automation)

The `codex` binary is a multitool that includes the `exec` subcommand for automation and scripting:

```bash
# Run non-interactively with JSON output
codex exec --json "list files in current directory"

# With custom sandbox mode
codex exec --json -s danger-full-access "run npm test"

# Skip git repo check for non-repo directories
codex exec --json --skip-git-repo-check "echo hello"

# Output to file
codex exec --json -o output.json "describe this project"
```

**Key flags:**
- `--json` - Output events as JSONL (for parsing)
- `-s, --sandbox` - Sandbox mode: `read-only`, `workspace-write`, `danger-full-access`
- `--skip-git-repo-check` - Run outside git repositories
- `-o, --output-last-message` - Save final response to file

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

Use the `codex execpolicy check` subcommand to preview decisions before you save a rule (see the [`codex-execpolicy` README](./codex-rs/execpolicy/README.md) for syntax details):

```shell
codex execpolicy check --policy ~/.codex/policy/default.codexpolicy git push origin main
```

Pass multiple `--policy` flags to test how several files combine, and use `--pretty` for formatted JSON output. See the [`codex-rs/execpolicy` README](./codex-rs/execpolicy/README.md) for a more detailed walkthrough of the available syntax.

## Note: `execpolicy` commands are still in preview. The API may have breaking changes in the future.

## 🧪 Testing & Validation

### Automated Test Suite

This project includes a comprehensive test suite specifically designed for Termux validation:

**Test Suite**: [`CODEX_TEST_SUITE.md`](./CODEX_TEST_SUITE.md)

**Coverage**:
- ✅ **82 automated tests** across 12 categories
- ✅ **10 Termux-specific tests** validating all 8 compatibility patches
- ✅ **8 Package & Binary tests** for npm installation verification
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
12. **Package & Binary (8 tests)** ⭐ - Validates npm installation and binaries

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

**Example Report** (v0.66.0):
```
CODEX CLI TEST SUITE - FINAL REPORT
====================================
Platform: Android Termux ARM64 (ROG Phone 3)
Codex Version: 0.66.0
Total Tests: 50
✅ Passed: 49
❌ Failed: 0
⚠️ Skipped: 1 (Git optional)

Termux-Specific: 10/10 passed ✅
Package & Binary: 8/8 passed ✅

VERDICT: ⚠️ PASS WITH WARNINGS
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

**Version**: Based on OpenAI Codex 0.66.0 (includes GPT-5.1 MAX support)
**Platform**: Android Termux ARM64
**Maintained**: Community-driven, not affiliated with OpenAI

---

## 📜 Changelog

### v0.66.0-termux (2025-12-09) – latest
**Dist-tag**: `latest`

- ⬆️ Upstream bump to OpenAI Codex rust-v0.66.0 (minor stability fixes; no new Termux patches needed).
- 🖥️ Single entrypoint: `codex` defaults to `codex exec ...`; `codex tui` launches UI; `codex-exec` kept as alias wrapper → both commands hit the same binary.
- 📦 Package verification: bin includes `codex`, symlinked `codex-exec`, and JS wrappers; `--json` / `--output-schema` documented; `LD_LIBRARY_PATH` preserved.
- ✅ Tests: CODEX_TEST_SUITE v1.2 on ROG Phone 3 → 45 passed / 0 failed / 4 skipped (WebSearch, Git in non-repo, manual code analysis); Package & Binary 8/8; Termux-Specific 10/10.

---

### v0.64.1-termux (2025-12-03) – stable
**Dist-tag**: `stable`

- Maintenance follow-up to 0.64.0: docs/test report refresh and npm package verification (codex-exec symlink + bin entries).
- Base upstream: rust-v0.64.0; Termux patches #1-6, #8, #9 revalidated.
- ✅ Tests: CODEX_TEST_SUITE v1.2 → 47/49 pass (Git optional skipped), Package & Binary 8/8 pass, Termux-Specific 10/10 pass.
