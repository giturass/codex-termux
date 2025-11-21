# Release Notes - v0.62.0-termux

## Summary

Upstream sync to **OpenAI Codex rust-v0.62.0** with all 9 Termux patches verified compatible.

## What's New (Upstream)

### New Features
- **codex-shell-tool-mcp** - New MCP server for shell tools with Bash/exec wrappers
- **execpolicycheck** - New CLI command for exec policy debugging
- **TUI reasoning default** - Changed to "medium" level for better responses
- **resume --last** - Allow reading prompt from last session
- **TUI animations toggle** - Feature switch to disable animations
- **Shell timeout 1 hour** - Increased from default for long-running commands

### Improvements
- FreeBSD shell behavior portability (may benefit Termux)
- Fuzzy search results increased 8 → 20
- Markdown styling: inline code now cyan
- Text encoding improvements for shell output
- Cancellation token support for exec tool calls
- Elicitation wait no longer counts against shell timeout

### Breaking Changes
- `execpolicy` migration: `execpolicy` → `execpolicy-legacy`, `execpolicy2` → `execpolicy`
- Removed `tiktoken-rs` dependency
- Removed `shell_command` feature flag
- `ExecParams.timeout_ms` replaced with `ExecExpiration` enum

## Termux Patches Status

All 9 patches verified **no conflicts** with upstream changes:

| Patch | Status | File |
|-------|--------|------|
| #1 Browser login | ✅ Compatible | login/src/server.rs |
| #2 RAM optimizations | ✅ Compatible | Cargo.toml |
| #3 Version alignment | ✅ Compatible | Cargo.toml |
| #4 Auto-update URL | ✅ Compatible | tui/src/updates.rs |
| #5 Version parser | ✅ Compatible | tui/src/updates.rs |
| #6 NPM package name | ✅ Compatible | tui/src/updates.rs |
| #8 Bash execution | ✅ Compatible | safety.rs, process-hardening, shell.rs |
| #9 Auto-update exec | ✅ Compatible | tui/src/main.rs |

## Installation

```bash
npm install -g @mmmbuto/codex-cli-termux@0.62.0-termux
```

Or upgrade:
```bash
npm update -g @mmmbuto/codex-cli-termux
```

## Requirements

- **Platform**: Android Termux (ARM64)
- **Node.js**: v18+
- **RAM**: 8GB+ recommended
- **Storage**: ~50MB for binary

## Stats

- **Upstream commits**: 40+
- **Files changed**: 195
- **Insertions**: +5,915
- **Deletions**: -2,293

## Links

- **Upstream release**: https://github.com/openai/codex/releases/tag/rust-v0.62.0
- **Full changelog**: [CHANGELOG.md](CHANGELOG.md)
- **Patch details**: [patches/README.md](patches/README.md)

---

**Based on**: OpenAI Codex rust-v0.62.0
**Release date**: 2025-11-21
