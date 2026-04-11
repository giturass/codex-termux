# CLI Runtime Report

**Goal**: practical no-crash validation only

## Status

- `PASS` binary version check
- `PASS` top-level help
- `PASS` non-interactive help routing
- `PASS` wrapper-based subcommand routing
- `PASS` login status
- `PASS` MCP listing
- `PASS` features listing
- `PASS` real non-interactive execution
- `PASS` no runtime crash in the tested command surface

## Practical Checks

- `codex --version`
- `codex-exec --version`
- `codex --help`
- `codex exec --help`
- `codex review --help`
- `codex login --help`
- `codex logout --help`
- `codex resume --help`
- `codex fork --help`
- `codex mcp --help`
- `codex sandbox --help`
- `codex app-server --help`
- `codex completion --help`
- `codex login status`
- `codex mcp list`
- `codex features list`
- `codex exec --ephemeral "Reply with exactly: OK"`
- `node .../codex.js --help`
- `node .../codex.js fork --help`
- `node .../codex.js debug --help`
- `node .../codex.js review --help`
- `node .../codex.js exec --help`
- `node .../codex.js login --help`
- `node .../codex.js logout --help`
- `node .../codex.js resume --help`

## Result

The installed CLI responds correctly on the main user-facing entry points and completed a real non-interactive execution path without crashing.

## Notes

- No recompilation was performed.
- This report is intentionally runtime-only.
- The checks target the installed CLI, not source-tree build validation.

## Screenshot Block

```text
CLI RUNTIME REPORT

STATUS: PASS

What was checked:
- version checks
- help routing
- login status
- MCP listing
- features listing
- real non-interactive execution
- wrapper subcommand routing

Outcome:
- Installed CLI responds correctly
- No crash on the tested command surface
- Runtime-only validation, no compile step
```
