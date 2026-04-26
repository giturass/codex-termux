# CODEX TEST REPORT

- Date: 2026-04-26 23:00 CEST
- Device: Pixel9Pro Termux
- Repo: /data/data/com.termux/files/home/Dev/05_termux/codex-termux
- Commit under test: 07f2d07bb4 (develop)
- Global package under test: codex-cli 0.125.0-termux
- Suite type: runtime-only validation of the installed Termux package
- Suite reference: test-reports/latest/CLI_RUNTIME_REPORT.md

## Runtime Checks

- `PASS` Package installed check
- `PASS` codex --version
- `PASS` codex-exec --version
- `PASS` codex --help
- `PASS` codex exec --help
- `PASS` codex review --help
- `PASS` codex login --help
- `PASS` codex logout --help
- `PASS` codex resume --help
- `PASS` codex fork --help
- `PASS` codex mcp --help
- `PASS` codex sandbox --help
- `PASS` codex app-server --help
- `PASS` codex login status
- `PASS` codex mcp list
- `PASS` codex features list
- `PASS` codex completion bash
- `PASS` codex debug prompt-input --help
- `PASS` codex app-server generate-json-schema --help
- `PASS` codex app-server generate-json-schema --out
- `PASS` node wrapper fork --help
- `PASS` node wrapper debug --help
- `PASS` node wrapper review --help
- `PASS` node wrapper exec --help
- `PASS` node wrapper login --help
- `PASS` node wrapper logout --help
- `PASS` node wrapper resume --help
- `PASS` codex-exec workspace list files
- `PASS` codex-exec create/read hello.txt
- `PASS` codex-exec network smoke
- `PASS` command path info
- `PASS` runtime wrapper files
- `PASS` codex.bin/codex-exec.bin runpath
- `PASS` codex.bin/codex-exec.bin needed libs
- `PASS` Termux runtime
- `PASS` node version
- `PASS` npm version
- `PASS` termux-open-url presence
- `PASS` verify-patches
- `PASS` temporary artifact cleanup

## Result

PASS

