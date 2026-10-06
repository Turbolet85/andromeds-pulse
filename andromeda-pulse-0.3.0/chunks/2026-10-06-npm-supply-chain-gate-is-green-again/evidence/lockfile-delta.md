# Lockfile delta — `pulse-app/ui/package-lock.json`

From `git diff b1fcba57c1c8cfebf562bcb32002dac6351702e7 -- pulse-app/ui/package-lock.json`, read 2026-10-06
~22:33Z after `PUPPETEER_SKIP_DOWNLOAD=1 npm ci --prefix pulse-app/ui` and, in `pulse-app/ui`,
`PUPPETEER_SKIP_DOWNLOAD=1 npm update seroval seroval-plugins source-map-js` (npm 11.19.1, Node v26.8.2).

| entry | old | new | lines changed |
|---|---|---|---|
| `node_modules/seroval` | 1.5.4 | 1.6.8 | `version` · `resolved` · `integrity` |
| `node_modules/seroval-plugins` | 1.5.4 | 1.6.8 | `version` · `resolved` · `integrity` |
| `node_modules/source-map-js` | 1.2.1 | 1.2.2 | `version` · `resolved` · `integrity` |

- 9 lines out, 9 lines in; at `-U0` three hunks (`@@ -10824,3`, `@@ -10833,3`, `@@ -11072,3`). The delta guard (entry 4)
  printed `18` at exit 0.
- No entry added or removed: the added-entry guard (entry 5) printed `0` at exit 1. The lockfile still holds 884
  entries.
- The changed lines are byte-identical to research's recorded delta
  (`.andromeda/runs/2026-10-06T22-06-57Z-phase/research-lock-delta.diff`): `cmp` over the `+`/`-` lines of both
  read exit 0. The registry published nothing newer in range between research and this run.
- New `resolved` / `integrity` values:
  - `seroval-1.6.8.tgz` — `sha512-HlSgSAkTk4EqHcje1ptJjfZi1YDv5KbhVJ/d3P7T/nAXua2VmDu+AKDX5VTdFfZf48nDkWB2TKYt0DrCSa+3wg==`
  - `seroval-plugins-1.6.8.tgz` — `sha512-N7mWAMydj89EnYTHtRpS3LBrAz3J9O6oVrqpuXuvlAHfAUG8WEKqtbAd7SBiJNY5NfCAFhSdyAh7wI4Shjqy4g==`
  - `source-map-js-1.2.2.tgz` — `sha512-KGj/8Y43x35aZVDtt+J4mK1hoLGHULMYfSkODJNQjNDC3oW1PqPoxMwo0pLUsWM/UEGzON/NxeHywEfNXNP3Vw==`
- Requirement and resolution, stated separately: the dependents' ranges are unchanged (`@tanstack/router-core`
  1.169.2 → `seroval ^1.5.4`, `seroval-plugins ^1.5.4`; `@tailwindcss/node` 4.2.4 and `postcss` 8.5.26 →
  `source-map-js ^1.2.1`); the versions above are what those ranges resolved to on 2026-10-06.
- `package.json` and `npm-policy.json` are unchanged (the scope guard, entry 6: exit 0, no output).
