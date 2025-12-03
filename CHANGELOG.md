# Changelog - Codex Termux

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [0.64.1-termux] - 2025-12-03

### Highlights
- **Single binary**: `codex` bundles `exec`; `codex-exec` is a wrapper/symlink to the same ~47 MB binary.
- **Package completeness**: `package.json` exposes both commands; `bin/` ships JS wrappers + symlink to avoid missing binaries.
- **LD_LIBRARY_PATH enforced**: `$PREFIX/lib` persisted via `~/.zshenv` (patch #8) for Termux library loading.
- **Web search verified**: `--search` flag passes Test-601.

### Testing
- Suite v1.2 on ASUS ROG Phone 3 (Android 12, aarch64): 47/49 PASS, 0 FAIL, 2 SKIP (Git optional); Category 12 Package & Binary 8/8 PASS; Termux-specific 10/10 PASS.
- Known notes: `termux-wifi-connectioninfo` limited on Play Store; `termux-open-url` requires an URL (usage exit 1 when missing).

### Changed
- Binary version: 0.64.1
- npm package: 0.64.1-termux (dist-tag `latest`)
- Based on: upstream rust-v0.64.0-alpha.9

---

## [0.62.0-termux] - 2025-11-21

### Upstream Changes (rust-v0.62.0)

**New Features:**
- **codex-shell-tool-mcp**: New MCP server for shell tools with Bash/exec wrappers (#7005)
- **execpolicycheck**: New CLI command for exec policy debugging (#7012)
- **TUI reasoning default**: Changed to "medium" level (#7040)
- **resume --last**: Allow reading prompt from last session (#6719)
- **v2 apply_patch approval flow**: Enhanced patch approval process (#6760)
- **v2 app-server error events**: Improved error reporting (#6938)
- **TUI animations toggle**: Feature switch to disable animations (#6870)
- **Shell timeout 1 hour**: Increased user shell timeout from default to 1 hour (#7025)

**Improvements:**
- FreeBSD shell behavior portability (#7039)
- Fuzzy search results increased 8 → 20 (#7013)
- Markdown styling: inline code now cyan (#7023)
- Text encoding improvements for shell output in VSCode (#6182)
- Windows Sandbox: network_access and exclude_tmpdir_env_var support (#7030)
- Cancellation token support for exec tool calls (#6972)
- Elicitation wait no longer counts against shell timeout (#6973)

**Breaking/Refactoring:**
- `execpolicy` → `execpolicy-legacy`, `execpolicy2` → `execpolicy` (#6956)
- Removed `tiktoken-rs` dependency (#7018)
- Removed `shell_command` feature flag (#7024)
- `ExecParams.timeout_ms` replaced with `ExecExpiration` enum

**Stats:** 195 files changed, +5915 insertions, -2293 deletions

### Termux Patches

All 9 Termux patches verified compatible - **no conflicts expected**:
- Patch #1: Browser login (termux-open-url) ✅
- Patch #2: RAM optimizations ✅
- Patch #3: Version alignment ✅
- Patch #4: Auto-update URL redirect ✅
- Patch #5: Version parser (-termux suffix) ✅
- Patch #6: NPM package name ✅
- Patch #8: Bash execution (sandbox, LD_*, shell detection) ✅
- Patch #9: Auto-update execution ✅

### Changed
- Binary version: 0.62.0
- npm package: 0.62.0-termux
- Based on: upstream rust-v0.62.0

---

## [0.61.1-termux] - 2025-11-20

### Fixed
- **Critical**: Auto-update execution restored - was broken in v0.60.1 and v0.61.0
- Update prompt showed but npm install never executed

### Added
- **Patch #9**: Auto-update execution in `tui/src/main.rs`
- Execution logic after `run_main()` returns with `UpdateAction`

### Changed
- Binary version: 0.61.1
- npm package: 0.61.1-termux

### Deprecated
- npm deprecate @mmmbuto/codex-cli-termux@0.61.0-termux
- npm deprecate @mmmbuto/codex-cli-termux@0.60.1-termux

### Test Results
- Test Suite: 40/42 PASSED (95.2%)
- Termux-specific: 10/10 PASSED

---

## [0.61.0-termux] - 2025-11-19

### Upstream Changes (rust-v0.61.0)
- ExecPolicy2 integration as default
- Single-pass truncation improvements
- Shell fallback reliability
- Error reporting enhancements

### Changed
- Binary version: 0.61.0
- npm package: 0.61.0-termux
- 13 upstream commits merged

### Known Issues
- ⚠️ Auto-update execution broken (fixed in 0.61.1)

---

## [0.60.1-termux] - 2025-11-13

### Upstream Changes (rust-v0.60.1)
- Bug fixes from 0.60.0
- Stability improvements

### Changed
- Binary version: 0.60.1
- npm package: 0.60.1-termux

### Known Issues
- ⚠️ Auto-update execution broken (fixed in 0.61.1)

---

## [0.58.4-termux] - 2025-11-08

### Fixed
- Auto-update detection restored
- Version checking improvements

### Changed
- Binary version: 0.58.4
- npm package: 0.58.4-termux
- Last version with working auto-update before regression

---

## [0.58.0-termux] - 2025-11-07

### Upstream Changes (rust-v0.58.0)
- Major upstream sync
- New features and improvements

### Changed
- Binary version: 0.58.0
- npm package: 0.58.0-termux

---

## [0.57.0-termux] - 2025-11-07

### Upstream Changes (rust-v0.57.0)
- Intermediate release for upstream tracking

### Changed
- Binary version: 0.57.0
- npm package: 0.57.0-termux

---

## [0.56.x-termux] - 2025-11-06

### Changed
- Multiple patch releases (0.56.0 through 0.56.3)
- Stability and compatibility improvements

---

## [0.55.4-termux] - 2025-11-06

### Fixed
- **Critical**: Bash command execution in Agent mode (TUI interactive chat) now works on Termux
- **Critical**: Resolved "Permission denied" errors when running bash commands in Agent mode
- Shell detection on Termux (was incorrectly detecting "login" instead of bash/zsh)
- LD_* environment variables preserved on Android (required for dynamic library loading)
- Sandbox disabled on Android (landlock/seccomp not supported)

### Added
- **Patch #8**: Fix bash execution with three independent Android-specific fixes
  1. Disable sandbox on Android (`core/src/safety.rs`)
  2. Preserve LD_* environment variables (`process-hardening/src/lib.rs`)
  3. Use $SHELL instead of getpwuid() for shell detection (`core/src/shell.rs`)
- Missing npm wrapper script (`npm-package/bin/codex.js`)

### Changed
- Binary version: 0.55.4
- npm package: 0.55.4-termux
- 88 lines modified across 3 files (all platform-specific with `#[cfg(target_os = "android")]`)

### Documentation
- Complete Patch #8 documentation with root cause analysis
- Updated `patches/README.md` with Bash Execution category
- Three distinct problems identified and fixed independently

### Notes
- This bug existed since at least v0.53.0 (tested and confirmed)
- Agent mode was completely unusable on Termux before this fix
- No changes to Linux/Mac/Windows behavior

---

## [0.55.3-termux] - 2025-11-06

### Fixed
- **Critical**: Auto-update on Android/Termux now shows manual instructions instead of failing with "Permission denied"
- **Critical**: Resolved infinite update loop - binary version synchronized to 0.55.3
- Android binary in use can no longer block npm update attempts

### Added
- **Patch #7**: Platform-specific update handling with `#[cfg(target_os = "android")]`
- User-friendly manual update instructions on Termux
- Organized patch documentation by categories in `patches/README.md`

### Changed
- Binary version: 0.55.3 (matches npm base version)
- npm package: 0.55.3-termux
- Auto-update on Android: Manual instructions only (platform limitation)
- Auto-update on Linux/Mac: Automatic execution (unchanged)

### Documentation
- Complete Patch #7 documentation with root cause analysis
- Updated all patches with categorization (Core + Auto-Update)
- Documented Android file-in-use limitation

---

## [0.55.2-termux] - 2025-11-05

### Fixed
- **Critical**: Auto-update command now installs correct package (`@mmmbuto/codex-cli-termux` instead of `@openai/codex`)
- **Critical**: Resolved infinite update loop by synchronizing binary version (0.55.1) with npm version
- Permission errors when running auto-update

### Changed
- Binary version incremented to 0.55.1 to match npm base version
- Updated `UpdateAction::command_args()` in `tui/src/updates.rs`

### Documentation
- Added Patch #6 in `patches/README.md`

---

## [0.55.1-termux] - 2025-11-05

### Fixed
- **Critical**: Version parser now handles `-termux` suffix correctly
- Users on 0.53.x now see update notification for 0.55.x versions
- Version comparison bug where `"0-termux".parse::<u64>()` failed

### Changed
- Modified `parse_version()` function to split on `-` before parsing patch version

### Documentation
- Added Patch #5 in `patches/README.md`

---

## [0.55.0-termux] - 2025-11-05

### Added
- Major upstream sync to OpenAI Codex 0.55.0 (46 commits ahead of 0.53.0)
- Comprehensive patch documentation in `patches/README.md`
- All 4 existing patches reapplied and verified

### Changed
- Upgraded from 0.53.0 to 0.55.0 upstream base
- Restored Termux-specific README after upstream merge
- Updated all version references

### Fixed
- Compilation compatibility with new upstream changes

---

## [0.53.2-termux] - 2025-11-01

### Added
- Node.js wrapper (`bin/codex.js`) for full auto-update functionality
- `CODEX_MANAGED_BY_NPM` environment variable support

### Changed
- npm package now uses wrapper script instead of direct binary
- Auto-update commands can now execute automatically

---

## [0.53.1-termux] - 2025-10-31

### Fixed
- Auto-update URL now points to `DioNanos/codex-termux` instead of upstream `openai/codex`
- Tag parser updated to handle `v`-prefixed tags (`vX.Y.Z-termux`)

### Documentation
- Added Patch #4 in `patches/README.md`

---

## [0.53.0-termux] - 2025-10-31

### Added
- Clean release based on upstream rust-v0.53.0
- Security cleanup of git history (force push to remove sensitive data)
- All patches re-applied from scratch

### Changed
- Upgraded from 0.50.2 to 0.53.0 upstream base
- Simplified patch set to 4 essential patches

### Fixed
- Removed sensitive data from git history
- Streamlined codebase for better maintainability

---

## [0.50.2-termux] - 2025-10-24

### Added
- Initial public release
- Browser login fix using `termux-open-url`
- RAM optimizations for 16GB devices
- SSH auto-start configuration

### Changed
- Compilation settings: `lto=false`, `codegen-units=16`
- Version alignment with upstream

---

## Patch Overview

| Patch | Version | File | Purpose |
|-------|---------|------|---------|
| #1 | 0.50.2 | login/src/server.rs | Browser login (termux-open-url) |
| #2 | 0.50.2 | Cargo.toml | RAM optimizations |
| #3 | 0.50.2 | Cargo.toml | Version alignment |
| #4 | 0.53.1 | tui/src/updates.rs | Auto-update URL redirect |
| #5 | 0.55.1 | tui/src/updates.rs | Version parser (-termux suffix) |
| #6 | 0.55.2 | tui/src/updates.rs + Cargo.toml | NPM package name |
| #7 | 0.55.3 | cli/src/main.rs | Manual update on Android (historical) |
| #8 | 0.55.4 | safety.rs, process-hardening, shell.rs | Bash execution fix |
| #9 | 0.61.1 | tui/src/main.rs | Auto-update execution |

---

## Links

- **GitHub Repository**: https://github.com/DioNanos/codex-termux
- **npm Package**: https://www.npmjs.com/package/@mmmbuto/codex-cli-termux
- **Upstream**: https://github.com/openai/codex

---

**Legend**:
- 🔴 **Critical**: Security or blocking issues
- 🟡 **Important**: Functionality improvements
- 🟢 **Minor**: Documentation or small fixes
