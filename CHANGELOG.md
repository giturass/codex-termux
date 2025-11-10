# Changelog - Codex Termux

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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

| Patch | Version Added | File | Purpose |
|-------|---------------|------|---------|
| #1 | 0.50.2 | login/src/server.rs | Browser login fix |
| #2 | 0.50.2 | Cargo.toml | RAM optimizations |
| #3 | 0.50.2 | Cargo.toml | Version alignment |
| #4 | 0.53.1 | tui/src/updates.rs | Auto-update URL redirect |
| #5 | 0.55.1 | tui/src/updates.rs | Version parser fix |
| #6 | 0.55.2 | tui/src/updates.rs + Cargo.toml | NPM package name fix |

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
