# The live readings (plan step 14)

Both live entries were fired by the gate tool inside /andromeda-implement's one whole-block run
(`gate v1.13 · 3718c868`, run dir `.andromeda/runs/2026-10-10T00-36-34Z-implement/`, finished before
2026-10-10T00:48:23Z). The tree they ran on: HEAD `8f655cae` plus this chunk's uncommitted edits (tree id
`1afa176a…`, the verdict's own `tree` member).

The tool's lines for the chunk's own entries, as printed:

```
 12 probe       green · exit 1 · 0.0s · 2 B → 12.log · cat xtask/src/pre_push.rs xtask/src/main.rs | grep -i -c w… (60 chars)
 13 probe       green · exit 0 · 0.03s · 9 B → 13.log · d="$(mise where node@24)" && "$d/bin/node" --version
 14 integration green · exit 2 · 0.38s · 182 B → 14.log · env PATH="$HOME/.cargo/bin:/usr/bin:/bin" cargo xtask pre-… (68 chars)
 15 probe       recorded · exit 0 · 0.0s · 30 B → 15.log · cat /proc/loadavg
 16 integration green · exit 0 · 90.25s · 1500025 B → 16.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
 17 probe       recorded · exit 0 · 0.0s · 31 B → 17.log · cat /proc/loadavg
 18 probe       green · exit 0 · 0.02s · 119 B → 18.log · python -X utf8 -c "import json; raw = open('target/pre-pus… (233 chars)
 19 probe       green · exit 0 · 0.01s · 0 B → 19.log · git diff --quiet 8f655caef468e90844fef54dcc33cef7bd4e426c … (95 chars)
 20 probe       green · exit 0 · 0.01s · 0 B → 20.log · git diff --name-only 8f655caef468e90844fef54dcc33cef7bd4e4… (224 chars)
entries 23 · green 18 · red 0 · recorded 2 · timeout 0 · not-run 3
```

## The cannot-evaluate reading (entry 14) — no Node on the verb's PATH

Exit 2. The whole of the entry's log, which is the verdict and nothing else (no stage ran, so no stage output):

```json
{
  "head": null,
  "missing": [
    "node:24 (found none)",
    "tool:npm"
  ],
  "reason": "provisioning-missing",
  "stages": [],
  "tree": null,
  "verdict": "cannot-evaluate"
}
```

## The green reading (entry 16) — Node 24 first on PATH

Exit 0. The verdict the verb printed, which is equal, member for member, to `target/pre-push/report.json` (compared
after the run by parsing both):

```json
{
  "head": "8f655caef468e90844fef54dcc33cef7bd4e426c",
  "missing": [],
  "reason": "all-stages-ok",
  "stages": [
    {"ms": 2, "name": "script-modes", "ok": true},
    {"ms": 413, "name": "source-lint", "ok": true},
    {"ms": 67780, "name": "npm", "ok": true},
    {"ms": 1616, "name": "clippy", "ok": true},
    {"ms": 19162, "name": "test", "ok": true},
    {"ms": 368, "name": "ci-gates", "ok": true}
  ],
  "tree": "1afa176ad83aaec5061114809b4102ce1a120e8a",
  "verdict": "green"
}
```

(The stage objects are folded onto one line each here; the verb prints them pretty.)

The `ci-gates` stage's own lines, from the entry's log:

```
ci-gates: zero-spans PASS (3 log records across 1 file(s))
ci-gates: zero-panic PASS
ci-gates: heartbeat-gap PASS
ci-gates: perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log
ci-gates: perf-budget: memory NEUTRAL — no metric.buffer.memory_bytes record
ci-gates: perf-budget: snapshot NEUTRAL — no metric.snapshot.token_count_ms record
ci-gates: perf-budget NEUTRAL
```

The report-twin read (entry 18) printed:
`head missing reason stages tree verdict ['script-modes', 'source-lint', 'npm', 'clippy', 'test', 'ci-gates'] True True`

## The timings, with the load they were taken under

These are one reading on a shared host and are not a budget.

| | 1 min | 5 min | 15 min |
|---|---|---|---|
| load average before the green run (entry 15) | 6.76 | 6.82 | 8.57 |
| load average after it (entry 17) | 10.56 | 7.59 | 8.65 |

The host has 32 logical CPUs (`nproc`). Another project's builder was running on it throughout (a `cargo-mutants`
run with its own `cargo-nextest` children, read from the process list after the run).

| stage | ms | what the log shows it did |
|---|---|---|
| `script-modes` | 2 | one `git ls-files -s` |
| `source-lint` | 413 | `"verdict": "clean"` |
| `npm` | 67 780 | `npm ci` (`added 800 packages, and audited 801 packages in 1m`), then the build (`built in 2.35s`) |
| `clippy` | 1 616 | `Checking pulse-app`, then `Finished` in 1.47 s; nothing else checked |
| `test` | 19 162 | `Compiling pulse-app`, `Finished` in 4.10 s; 2836 test `ok` events, 0 `failed` |
| `ci-gates` | 368 | the seven lines above |

Whole entry: 90.25 s wall (the tool's reading, the `cargo xtask` start included).

Both cargo stages ran on a warm shared `target/`: the standard gate set of this same run had built and tested the
workspace minutes earlier (entry 2 clippy 2.13 s, entry 9 the workspace suite 17.36 s, 2836 passed). A cold
`target/` was not measured.

## What the plan left to be measured here

- **Does a stage run on the shared `target/` rebuild what a developer build had current?** In this run one crate:
  `pulse-app` was checked once by the `clippy` stage and compiled once by the `test` stage; no other `Compiling` or
  `Checking` line stands in the log. The cause was not isolated. The `npm` stage had rewritten `pulse-app/ui/dist`
  just before, which the crate embeds; whether that alone explains it, or the stage's different `PATH` and `HOME`
  do, is unmeasured. Whether the developer's next build then recompiles `pulse-app` again is unmeasured too.
- **How long each stage takes natively:** the table above, warm, under the stated load.

## What the run left on the host

- `target/pre-push/run/`: 1.3 GB, almost all of it the per-run browser cache (`puppeteer/chrome`,
  `puppeteer/chrome-headless-shell`). The next run of the verb removes and recreates it.
- `pulse-app/ui/node_modules` reinstalled under Node 24.21.0 / npm 11.19.0, and `pulse-app/ui/dist` rebuilt (both
  gitignored).
- Untouched, read after the run: the ten dated `puppeteer-*` and ten dated `data-*` directories and the logs the
  hand-runs left under `target/pre-push/` are all still listed; the shared `~/.cache/puppeteer` directory's
  modification time is still 2026-10-03.
- The tracked tree: `git status --short` lists `xtask/src/main.rs` and `xtask/src/pre_push.rs` as the only modified
  files outside the pipeline's own ledgers; the bindings close (entry 19) and the scope guard (entry 20) read
  clean after the green run.

## Stated, not measured

- Inside the `test` stage the credential-store legs are expected to clean-skip (the stage environment carries no
  session bus). This log cannot show it: the stage prints nextest's libtest-json stream, which holds no captured
  output of a passing test, and its count of `no OS credential store` lines is 0 either way.
