# Codex-Termux v0.65.0-termux Release Notes

**Release Date**: 2025-12-05
**Base Version**: OpenAI Codex rust-v0.65.0
**Upstream Commits**: ~30 commits from v0.64.0

---

## 🎯 Highlights

### New Features
- **Skills Support**: New `/skills` command and `$` prefix for listing and selecting skills (#7506)
- **Slash Resume**: Added `/resume` command for continuing previous sessions (#7302)
- **Tool Tips**: Interactive tool tips in TUI for better UX (#7440)
- **Model Warning**: Warning system when using apply_patch with certain models (#7494)
- **Retroactive Image Placeholder**: Prevents context poisoning from images (#6774)

### Architecture Improvements
- **ModelsManager**: Introduced centralized models manager migrated across app-server, TUI, and core (#7552, #7555, #7565)
- **Codex Max Migration**: Updated codex max model handling (#7566)
- **Model Family Refactor**: Migrated model family to models manager (#7565)

### TUI Enhancements
- **Ctrl-P/N Navigation**: Map Ctrl-P/N to arrow navigation in textarea (#7530)
- **Shell Output Limiting**: Limit user shell output by screen lines to prevent overflow (#7448)
- **Long Exec Lines Wrapping**: Fix wrapping of long exec lines in transcript overlay (#7481)
- **Image Paste on Windows**: Support image paste from clipboard on native Windows (#7514)

### Fixes & Improvements
- **Features Immutability**: Features are now immutable over session/thread lifetime (#7540)
- **Thread ID Migration**: Unified conversation_id → thread_id in app-server feedback/upload (#7538)
- **Config File Path**: Made file_path for config optional in app-server (#7560)
- **Unified Exec Shell**: Use platform default shell when unified_exec shell not specified (#7486)
- **Sandbox**: Allow openpty() in seatbelt (#7507)
- **Python Cache**: Added __pycache__ to excluded directories (#7545)

### Dependency Updates
- **image**: 0.25.8 → 0.25.9
- **rmcp**: 0.9.0 → 0.10.0
- **regex-lite**: Added 0.1.7

---

## 🤖 Termux-Specific Patches

### Maintained Patches
✅ **Termux Browser Login Fix** (codex-rs/login/src/server.rs)
- Uses `termux-open-url` instead of `webbrowser::open()` on Android
- Prevents ndk-context crash in Termux environment
- Applied conditionally with `#[cfg(target_os = "android")]`

---

## 📊 Testing Status

**Build Status**: ✅ Verified on ASUS ROG Phone 3 (49/50 tests pass; 1 Git optional skip; Package & Binary 8/8)
**Platform**: ASUS ROG Phone 3 (Snapdragon 865+, 8GB RAM)
**Compiler**: Rust toolchain ARM64

### Test Plan
- [x] Compilation successful
- [x] Binary version check (`codex --version` / `codex-exec --version` → 0.65.0)
- [ ] Agent mode test
- [ ] TUI navigation test
- [ ] Login with browser (termux-open-url)
- [x] Bash commands execution
- [ ] Skills listing (`/skills`)
- [ ] Resume command (`/resume`)
- [ ] Tool tips display

---

## 🔗 Links

- **Upstream Tag**: [rust-v0.65.0](https://github.com/openai/codex/releases/tag/rust-v0.65.0)
- **Fork Repository**: [DioNanos/codex-termux](https://github.com/DioNanos/codex-termux)
- **npm Package**: [@mmmbuto/codex-cli-termux](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)

---

## 📝 Changelog Summary

### Added
- Skills support via `/skills` and `$` prefix
- Slash `/resume` command
- Tool tips in TUI
- Model warning for apply_patch
- Retroactive image placeholder
- Ctrl-P/N keyboard navigation in TUI
- Python __pycache__ exclusion

### Changed
- Centralized ModelsManager architecture
- conversation_id → thread_id migration
- Unified exec shell defaults
- Shell output limiting by screen lines
- Long exec lines wrapping

### Fixed
- Features immutability over session lifetime
- Image paste on Windows
- Config file_path optionality
- Sandbox openpty() permission

---

**Full Upstream Changelog**: https://github.com/openai/codex/compare/rust-v0.64.0...rust-v0.65.0
