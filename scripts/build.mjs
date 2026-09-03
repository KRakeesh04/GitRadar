import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { setTimeout as sleep } from 'node:timers/promises';

// The TanStack Start SPA build leaves the event loop alive after finishing
// (the SSR/preview server never closes cleanly), so `vite build` hangs. It does,
// however, write every artifact — including the prerendered SPA shell
// `dist/client/index.html` — before hanging. So we run `vite build` as a child
// process and terminate it once that final artifact exists and has settled.
const root = resolve(process.cwd());
const shellMarker = resolve(root, 'dist/client/index.html');

const child = spawn('bunx', ['vite', 'build'], {
  cwd: root,
  stdio: ['ignore', 'inherit', 'inherit'],
  env: { ...process.env },
});

const STABLE_MS = 1500;
const TIMEOUT_MS = 5 * 60 * 1000;
const startedAt = Date.now();
let lastSeen = 0;
let done = false;

while (!done) {
  if (existsSync(shellMarker)) {
    if (lastSeen === 0) lastSeen = Date.now();
    if (Date.now() - lastSeen >= STABLE_MS) {
      done = true;
      break;
    }
  } else {
    lastSeen = 0;
  }
  if (Date.now() - startedAt > TIMEOUT_MS) {
    console.error('[build] Timed out waiting for prerendered shell.');
    child.kill('SIGTERM');
    process.exit(1);
  }
  await sleep(200);
}

child.kill('SIGTERM');
process.exit(0);
