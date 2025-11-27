# Codex CLI Termux v0.64.0-termux (2025-11-27)

## Highlights
- ✅ **Binario unico**: `codex` incorpora il sottocomando `exec`; `codex-exec` è ora un wrapper/symlink allo stesso binario (~49 MB) per evitare omissioni del binario secondario.
- ✅ **Pacchetto npm completo**: `package.json` espone entrambe le voci `codex` e `codex-exec` con i relativi wrapper; la directory `bin/` include symlink `codex-exec -> codex` per installazioni globali.
- ✅ **LD_LIBRARY_PATH forzato**: esportato a livello globale (`$PREFIX/lib`) via `~/.zshenv` per soddisfare Test-1006 (library path preservation).
- ✅ **Web search**: flag `--search` verificato; Test-601 ora passa (ricerca via DuckDuckGo).

## Testing
- **Suite**: `CODEX_TEST_SUITE.md` v1.2
- **Device**: ASUS ROG Phone 3 (Termux, Android 12, aarch64)
- **Results**: 47/49 PASS, 0 FAIL, 2 SKIP (solo Git: Test-701/702 non applicabili fuori repo)
- **Critici**: Categoria 12 (Package & Binary) 8/8 PASS; Termux-Specific 10/10 PASS

## Known Notes
- `termux-wifi-connectioninfo` continua a segnalare limitazioni Termux-API su Play Store (non blocca la suite).
- `termux-open-url` richiede un URL reale; `--help` attiva l’intent e restituisce errore benigno.

## Upgrade Notes
- Per aggiornare: `npm install -g @mmmbuto/codex-cli-termux@0.64.0-termux`
- Verifica: `codex --version` e `codex-exec --version` devono riportare `0.64.0`.
- Se presenti installazioni precedenti, assicurarsi che `which codex-exec` punti al wrapper JS (symlink) nel path npm globale.
