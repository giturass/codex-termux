# 🔧 Termux Compatibility Patches

This document describes the Termux‑specific patches applied to the official OpenAI Codex CLI so that it works well on Android Termux (ARM64).
Validated for: **v0.58.4-termux** (upstream `rust-v0.58.0`).

---

## Patch List

### 1. Browser Login Fix

**File**: `codex-rs/login/src/server.rs`
**Lines Modified**: 10
**Date Applied**: 2025-11-05 (introduced in 0.55.x, unchanged in 0.58.x)
**Upstream Issue**: Browser login crashes on Termux with `ndk-context` error

#### Problem
Upstream uses `webbrowser` crate which calls `ndk-context` on Android.
This requires an Android Activity context, which is not available in Termux (CLI environment).

**Error:**
```
thread 'main' panicked at ndk-context-0.1.1/src/lib.rs:72:30:
android context was not initialized
```

#### Solution
On Android (`target_os = "android"`), use `termux-open-url` command directly
instead of `webbrowser::open()`. Other platforms continue using `webbrowser` crate.

**Code:**
```rust
if opts.open_browser {
    // On Termux/Android, use termux-open-url directly to avoid ndk-context crash
    #[cfg(target_os = "android")]
    {
        let _ = std::process::Command::new("termux-open-url").arg(&auth_url).status();
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = webbrowser::open(&auth_url);
    }
}
```

**Testing:**
```bash
codex login
# Browser opens without crash ✅
```

**Impact:**
- ✅ Enables OAuth login flow on Termux
- ✅ No changes to other platforms
- ✅ Minimal invasive patch (10 lines)

---

### 2. Compilation Optimizations

**File**: `codex-rs/Cargo.toml`
**Section**: `[profile.release]`
**Date Applied**: 2025-11-05 (introduced in 0.55.x, unchanged in 0.58.x)
**Purpose**: Enable compilation on RAM-constrained devices

#### Problem
Default upstream compilation settings use aggressive optimizations:
- `lto = "fat"` - Consumes 18-22GB RAM during linking
- `codegen-units = 1` - Single-threaded, large temporary files

These settings cause Out Of Memory (OOM) errors on devices with < 32GB RAM.

#### Solution
Optimize for Termux devices (8-16GB RAM):

**Changes:**
```toml
[profile.release]
lto = false                    # Was: "fat" - saves ~4GB RAM
codegen-units = 16             # Was: 1 - enables parallel compilation
opt-level = 3                  # Keep optimization level high
```

**Impact:**
- Binary size: +5% larger (~44MB instead of 42MB)
- Compilation time: +5-10 minutes
- RAM usage: 12-14GB instead of 18-22GB
- **Result: Successful compilation on 16GB devices** ✅

---

### 3. Version Alignment

**File**: `codex-rs/Cargo.toml`
**Field**: `[workspace.package] version`
**Purpose**: Display correct version in `codex --version`

#### Change
```toml
version = "0.58.4"  # Aligned with upstream rust-v0.58.0
```

**Reason:**
- Matches upstream release version
- npm package uses `<upstream>-termux` format (e.g. `0.58.4-termux`)

---

### 4. Auto-Update URL Redirect

**File**: `codex-rs/tui/src/updates.rs`
**Lines Modified**: 14
**Date Applied**: 2025-11-05 (updated in 0.58.4 for new tag format)
**Upstream Issue**: Auto-update checks OpenAI repo, not Termux fork

#### Problem
Upstream checks OpenAI releases for updates:
```rust
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/openai/codex/releases/latest";
```

Tag format mismatch:
- Upstream uses tags like: `rust-v0.58.0`
- Termux fork uses tags like: `v0.58.4-termux`

#### Solution
1. **Change URL** to point to Termux fork
2. **Update tag parser** to handle both `rust-v*` (upstream) and `v*-termux` (fork) tags

**Changes:**
```rust
// Line 56: Update URL
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/DioNanos/codex-termux/releases/latest";

// Lines 81-90: Update tag parser for upstream + Termux formats
fn extract_version_from_latest_tag(latest_tag_name: &str) -> anyhow::Result<String> {
    // Support both "rust-v" (upstream) and "v" (Termux fork)
    let version = latest_tag_name
        .strip_prefix("rust-v")
        .or_else(|| latest_tag_name.strip_prefix("v"))
        .ok_or_else(|| anyhow::anyhow!("Failed to parse latest tag name '{latest_tag_name}'"))?;

    // Remove -termux suffix if present (e.g., "0.58.0-termux" -> "0.58.0")
    let clean_version = version.split('-').next().unwrap_or(version);
    Ok(clean_version.to_string())
}
```

**Impact:**
- ✅ Auto-update now checks Termux fork releases
- ✅ Supports both `vX.Y.Z-termux` and fallback formats
- ✅ No breaking changes to existing functionality

---

### 5. Version Parser Fix for `-termux` Suffix

**File**: `codex-rs/tui/src/updates.rs`
**Lines Modified**: 3
**Date Applied**: 2025-11-05 (still used in 0.58.4)
**Upstream Issue**: Version parser fails on `-termux` suffix, blocking update detection

#### Problem
The version parser splits version string by `.` and tries to parse each part as u64:
```rust
// Old code (fails on "0.58.4-termux")
let pat = iter.next()?.parse::<u64>().ok()?;  // "0-termux" → FAIL ❌
```

When comparing versions:
- Current: `0.57.0`
- Latest from API: `0.58.4-termux`
- Parser tries: `"0-termux".parse::<u64>()` → **Returns None**
- Result: `is_newer()` returns `None` → No update notification shown ❌

**Real-world impact**: Users on 0.57.x never see notification for 0.58.4-termux without this fix.

#### Solution
Split patch version on `-` to extract numeric part before parsing:

**Changes:**
```rust
// New code (handles "0.58.4-termux" correctly)
let pat_str = iter.next()?;                    // "4-termux"
let pat = pat_str.split('-').next()?.parse::<u64>().ok()?; // "4" → OK ✅
```

**Testing:**
```rust
parse_version("0.58.4")        → Some((0, 58, 4)) ✅
parse_version("0.58.4-termux") → Some((0, 58, 4)) ✅
parse_version("0.58.5-termux") → Some((0, 58, 5)) ✅
```

**Impact:**
- ✅ Auto-update detection now works across `-termux` versions
- ✅ Users on 0.57.x → 0.58.x see update notification
- ✅ Incremental updates (e.g. 0.58.4 → 0.58.5) also work
- ✅ Backward compatible with non-suffixed versions

---

### 6. NPM Package Name Fix in Auto-Update

**File**: `codex-rs/tui/src/updates.rs` + `codex-rs/Cargo.toml`
**Lines Modified**: 4
**Date Applied**: 2025-11-05 (introduced in 0.55.x, unchanged in 0.58.x)
**Upstream Issue**: Auto-update command uses wrong npm package name

#### Problem
When user accepts auto-update prompt, Codex tries to run:
```bash
npm install -g @openai/codex@latest
```

**Two issues:**
1. **Wrong package**: Uses `@openai/codex` instead of `@mmmbuto/codex-cli-termux`
2. **Update loop**: Binary version stayed at `<old>` while npm published `<new>-termux`
   - User installs `<new>-termux`
   - Binary shows: `codex-cli <old>`
   - API returns: `<new>-termux`
   - Parser: `(0, 55, 1) > (0, 55, 0)` → **Shows update again!** ∞

**Error seen:**
```
Updating Codex via `npm install -g @openai/codex@latest`...
env: 'node': Permission denied
Error: failed with status exit status: 126
```

#### Solution
1. **Fix npm package name** in `UpdateAction::command_args()`
2. **Increment binary version** in `Cargo.toml` to match npm release

**Changes:**
```rust
// updates.rs line 182-183
UpdateAction::NpmGlobalLatest => ("npm", &["install", "-g", "@mmmbuto/codex-cli-termux@latest"]),
UpdateAction::BunGlobalLatest => ("bun", &["install", "-g", "@mmmbuto/codex-cli-termux@latest"]),
```

**Impact:**
- ✅ Auto-update now installs correct Termux package
- ✅ No more update loop (binary version matches npm version)
- ✅ Permission errors resolved
- ✅ Clean upgrade path for all users

---

### 7. Manual Update Instructions on Android/Termux

**Status**: Historical (0.55.x only) – no longer applied in 0.58.x

**Original file**: `codex-rs/cli/src/main.rs` (`run_update_action`)
**Original Date Applied**: 2025-11-05
**Original Upstream Issue**: Auto-update fails on Android with "Permission denied"

#### Problem
In early Termux builds (0.55.x), when user accepted auto-update on Termux, Codex tried to execute:
```bash
npm install -g @mmmbuto/codex-cli-termux@latest
```

**Error seen:**
```
Updating Codex via `npm install -g @mmmbuto/codex-cli-termux@latest`...
env: 'node': Permission denied
Error: failed with status exit status: 126
```

**Root cause:**
- Codex binary is spawned by Node.js wrapper (`bin/codex.js`)
- When npm tries to update, it must overwrite `/usr/lib/node_modules/.../bin/codex`
- Binary is IN USE (running process)
- Android/Termux: Cannot overwrite files in use → Permission denied
- Linux/Mac: Can overwrite (old inode remains) → Works fine

**Why loop exists:**
Even if npm succeeded:
1. npm 0.55.2-termux published with binary 0.55.1 inside
2. User accepts update → npm reinstalls same broken package
3. Binary still 0.55.1 after "update"
4. Codex checks: 0.55.2 > 0.55.1 → Shows update again
5. INFINITE LOOP

This was originally mitigated by adding an Android‑only branch to `run_update_action()` that printed a manual update command instead of executing it.

In the current 0.58.x codebase, this patch has been removed and the Termux behavior is handled at a higher level (by deciding whether to present an auto‑update action at all). This section is kept for historical context only.

---

### 8. Fix Bash Execution on Android/Termux

**Files Modified**:
- `codex-rs/core/src/safety.rs` (3 lines)
- `codex-rs/process-hardening/src/lib.rs` (29 lines)
- `codex-rs/core/src/shell.rs` (56 lines)

**Date Applied**: 2025-11-06 (introduced in 0.55.4, unchanged in 0.58.x)
**Upstream Issue**: Bash commands fail with "Permission denied" in Agent mode on Termux

#### Problem

When using Codex in TUI/Agent mode (interactive chat), all bash command executions fail:
```
/data/data/com.termux/files/usr/bin/bash: /data/data/com.termux/files/usr/bin/pkg: /data/data/com.termux/files/usr/bin/bash: bad interpreter: Permission denied
```

**Root causes (3 distinct issues):**

1. **Shell detection fails on Termux**
   - `getpwuid()` returns `/data/data/.../login` instead of actual shell
   - Codex tries to execute with "login" binary instead of bash/zsh

2. **LD_* environment variables removed**
   - Process-hardening removes `LD_LIBRARY_PATH` and other `LD_*` vars
   - Termux requires these to find libraries in `/data/data/com.termux/files/usr/lib`
   - Without them: `bash` cannot find shared libraries → Permission denied

3. **Sandbox not supported on Android**
   - Codex tries to use landlock/seccomp sandbox
   - Android kernel doesn't support these Linux features
   - Sandbox initialization can cause process failures

**Tested with v0.53.0**: Same error confirmed → Upstream bug since at least 0.53.0

#### Solution

Three independent fixes, all using `#[cfg(target_os = "android")]`:

**1. Disable sandbox on Android** (`core/src/safety.rs:102-104`)
```rust
pub fn get_platform_sandbox() -> Option<SandboxType> {
    if cfg!(target_os = "macos") {
        Some(SandboxType::MacosSeatbelt)
    } else if cfg!(target_os = "android") {
        // Android/Termux does not support landlock/seccomp sandbox
        None
    } else if cfg!(target_os = "linux") {
        Some(SandboxType::LinuxSeccomp)
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            if WINDOWS_SANDBOX_ENABLED.load(Ordering::Relaxed) {
                return Some(SandboxType::WindowsRestrictedToken);
            }
        }
        None
    } else {
        None
    }
}
```

**2. Preserve LD_* variables on Android** (`process-hardening/src/lib.rs:45-62`)
```rust
// Official Codex releases are MUSL-linked, which means that variables such
// as LD_PRELOAD are ignored anyway, but just to be sure, clear them here.
// EXCEPTION: On Android/Termux, LD_* variables are required to find shared libraries
// in non-standard paths (/data/data/com.termux/files/usr/lib), so we must preserve them.
#[cfg(not(target_os = "android"))]
{
    let ld_keys: Vec<String> = std::env::vars()
        .filter_map(|(key, _)| {
            if key.starts_with("LD_") {
                Some(key)
            } else {
                None
            }
        })
        .collect();

    for key in ld_keys {
        unsafe {
            std::env::remove_var(key);
        }
    }
}
```

**3. Use $SHELL instead of getpwuid() on Android** (`core/src/shell.rs:50-69`)
```rust
#[cfg(unix)]
fn detect_default_user_shell() -> Shell {
    // On Android/Termux, getpwuid() returns "/data/data/.../login" instead of the actual shell.
    // Use $SHELL environment variable instead, which correctly points to bash/zsh.
    #[cfg(target_os = "android")]
    {
        if let Ok(shell_path) = std::env::var("SHELL") {
            let home_path = std::env::var("HOME").unwrap_or_else(|_| "/data/data/com.termux/files/home".to_string());

            if shell_path.ends_with("/zsh") {
                return Shell::Zsh(ZshShell {
                    shell_path,
                    zshrc_path: format!("{home_path}/.zshrc"),
                });
            }

            if shell_path.ends_with("/bash") {
                return Shell::Bash(BashShell {
                    shell_path,
                    bashrc_path: format!("{home_path}/.bashrc"),
                });
            }
        }
        return Shell::Unknown;
    }

    // On Linux/BSD/other Unix, use getpwuid() which works correctly
    #[cfg(not(target_os = "android"))]
    {
        use libc::getpwuid;
        use libc::getuid;
        use std::ffi::CStr;

        unsafe {
            let uid = getuid();
            let pw = getpwuid(uid);

            if !pw.is_null() {
                let shell_path = CStr::from_ptr((*pw).pw_shell)
                    .to_string_lossy()
                    .into_owned();
                let home_path = CStr::from_ptr((*pw).pw_dir).to_string_lossy().into_owned();

                if shell_path.ends_with("/zsh") {
                    return Shell::Zsh(ZshShell {
                        shell_path,
                        zshrc_path: format!("{home_path}/.zshrc"),
                    });
                }

                if shell_path.ends_with("/bash") {
                    return Shell::Bash(BashShell {
                        shell_path,
                        bashrc_path: format!("{home_path}/.bashrc"),
                    });
                }
            }
        }
        Shell::Unknown
    }
}
```

#### Testing

**Before Patch #8:**
```bash
$ codex
> Ask Codex: "run pkg help"
Error: /data/data/com.termux/files/usr/bin/bash: Permission denied ❌
```

**After Patch #8 (Expected):**
```bash
$ codex --version
codex-cli 0.55.4 ✅

$ codex
> Ask Codex: "run pkg help"
Usage: pkg command [arguments] ✅
```

#### Impact
- ✅ **Enables Agent mode bash execution** - Critical functionality restored
- ✅ **Shell detection fixed** - Correctly identifies bash/zsh on Termux
- ✅ **Library loading fixed** - LD_* preserved for dynamic linking
- ✅ **Sandbox disabled on Android** - Prevents sandbox-related crashes
- ✅ **No changes to other platforms** - Linux/Mac/Windows unchanged
- ✅ **Minimal invasive** - 88 lines modified across 3 files

**Note:** This patch fixes a critical bug that has existed since at least v0.53.0, making Agent mode completely unusable on Termux.

---

## 📊 Patch Categories

### Core Functionality (Required)
- **Patch #1**: Browser login fix (termux-open-url)
- **Patch #2**: RAM optimizations (compilation settings)
- **Patch #3**: Version alignment with upstream

### Auto-Update System (Required)
- **Patch #4**: Auto-update URL redirect (GitHub API)
- **Patch #5**: Version parser (-termux suffix handling)
- **Patch #6**: NPM package name fix
- **Patch #7**: Manual update instructions on Android *(historical, 0.55.x only)*

### Bash Execution (Critical)
- **Patch #8**: Fix bash execution in Agent mode (shell detection, LD_*, sandbox)

For the current **0.58.4-termux** release, active patches are **#1–#6 and #8** and are all critical for correct behavior on Termux. Patch **#7** was critical for the legacy 0.55.x line and is kept here only for historical reference.

---

## Versioning Strategy

| Component | Version | Example |
|-----------|---------|---------|
| **Binary** | Upstream version | `codex-cli 0.58.4` |
| **npm package** | `<upstream>-termux` | `0.58.4-termux` |

**Why:**
- Binary version matches upstream for compatibility
- npm suffix indicates Termux-specific release
- Clear traceability to upstream version

---

## Testing Checklist

Before each release:
- [ ] `codex --version` shows correct upstream version (0.58.4)
- [ ] `codex login` opens browser without crash
- [ ] OAuth flow completes successfully
- [ ] Binary size < 50MB
- [ ] Compilation completes on 16GB device
- [ ] Auto-update checks correct URL

---

## Contributing

Found a bug specific to Termux? Please open an issue with:
1. Steps to reproduce
2. Expected vs actual behavior
3. Device specs (RAM, Android version)
4. Error logs

We only accept patches for Termux-specific issues, not general feature requests.

---

**Last Updated**: 2025-11-19
**Patches Applied**: 8 (carried from 0.55.x and revalidated in 0.58.4-termux)
**Based on**: OpenAI Codex rust-v0.58.0
**Platform**: Android Termux ARM64
