# Codex CLI Termux v0.64.1-termux (2025-12-03)

## Highlights
- ✅ **Single binary**: `codex` includes the `exec` subcommand; `codex-exec` stays a wrapper/symlink to the same ~47 MB binary, preventing missing-secondary-binary issues.
- ✅ **Complete npm package**: `package.json` exposes both `codex` and `codex-exec`; `bin/` ships JS wrappers plus `codex-exec -> codex` symlink for global installs.
- ✅ **LD_LIBRARY_PATH enforced**: set globally to `$PREFIX/lib` via `~/.zshenv` to satisfy library-path preservation tests.
- ✅ **Web search verified**: `--search` flag tested (DuckDuckGo scrape) and passes Test-601.

## Testing
- **Suite**: `CODEX_TEST_SUITE.md` v1.2
- **Device**: ASUS ROG Phone 3 (Termux, Android 12, aarch64)
- **Results**: 47/49 PASS, 0 FAIL, 2 SKIP (only Git tests 701/702 skipped outside a repo)
- **Critical**: Category 12 (Package & Binary) 8/8 PASS; Termux-Specific 10/10 PASS

## Known Notes
- `termux-wifi-connectioninfo` reports Termux-API limitations on Play Store builds (non-blocking).
- `termux-open-url` requires a real URL; invoking without arguments returns usage (exit 1) as expected.

## Upgrade Notes
- Upgrade: `npm install -g @mmmbuto/codex-cli-termux@0.64.1-termux`
- Verify: `codex --version` and `codex-exec --version` should both show `0.64.1`.
- If upgrading from older installs, ensure `which codex-exec` resolves to the npm global wrapper/symlink.
