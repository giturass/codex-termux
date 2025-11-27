#!/usr/bin/env node

import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const binaryPath = join(__dirname, 'codex');
const args = ['exec', ...process.argv.slice(2)];

const child = spawn(binaryPath, args, {
  stdio: 'inherit',
  env: { ...process.env, CODEX_MANAGED_BY_NPM: '1' }
});

child.on('exit', (code) => {
  process.exit(code);
});
