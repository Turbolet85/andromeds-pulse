# Operator pass — CI read (on the overseer's word, 2026-09-30)

| # | Entry | Reading |
|---|---|---|
| 37 | `gate.py hygiene` | exit 0 · `hygiene: clean — read 34 (runs 31 · evidence 3) · trails 12 not read · binary 0 not read by P1` |
| 38 | `cargo xtask pre-push:linux` | exit 0 · `"verdict": "green"`, `"reason": "all-stages-ok"` · stages script-modes · npm · clippy · test · ci-gates all ok · head `ea50ca2` + worktree as tree `bead2abc` |
| — | pre-CI commit | `5fbf762 chore(2026-09-30-perf-instruments-measure-their-budgets): operator pre-CI commit` (the whole tree, 58 files) |
| 39 | guarded push | exit 0 · `ea50ca2..5fbf762  chore/migrate-pulse-to-v3 -> chore/migrate-pulse-to-v3` |
| 40 | `ci.py conclusion --sha HEAD --wait 2400` | exit 0 · `5fbf7628b19e verdict: green · checks 13/13 · wall 1640 s` · runs `ci#36765040464` completed/success, `secret-scan#36765040450` completed/success |
| 41 | `gh run view 36765040464 --log \| grep 'frame: cannot-evaluate'` | exit 0 · two lines (below) · the old fixed string `no WebGPU adapter in this run`: 0 occurrences in the whole run log |
| 42 | `gh api …/actions/cache/usage` | `active_caches_size_in_bytes` 10 605 172 169 · `active_caches_count` 8 |

## Entry 41 — the frame lines, verbatim

```
boot smoke (ubuntu-22.04)	cargo xtask ci-gates	2026-09-30T19:31:45.3325287Z ci-gates: perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log
lint / test (ubuntu-22.04)	UNKNOWN STEP	2026-09-30T19:31:37.6644674Z perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log
```

### The plan's prediction for the boot job — measured FALSE

The plan (entry 41 note) and research (§Mechanism equalities, "Adapter record reach") predicted the boot job's
`ci-gates` line would name its cause from the adapter record, e.g. `no WebGPU adapter (no_navigator_gpu)`, because that
job boots the real app under xvfb with a WebKitGTK webview. It reads `no adapter record in this log` instead.

Why, from the job's own uploaded log artifact (`logs-boot-Linux`, downloaded read-only):

- `agent-latest.jsonl.2026-09-30` holds **23 records spanning 14 ms** (`19:31:42.778 → 19:31:42.792`), all backend boot
  records (`app.boot.*`, `buffer.schema.init`, the corpus / triage / interpretation boot set). **No
  webview-originated record of any kind** — no `services.list_with_states.request`, no `viz.query.*`, no
  `metric.webgpu.*`, no `telemetry.frontend` emit.
- The step runs `agent-run.sh boot` → `status` → `cleanup` back to back, so the app is stopped before the webview's
  JavaScript issues any IPC. The job cannot witness an adapter outcome by construction; the printed cause is true for
  the log it reads.
- The predecessor's boot artifact (`ci#36741143328`) has the same shape: 40 records over 0.7 s, none from the webview.

The live adapter witness therefore rests on the dev-host frame leg (`evidence/adapter-wire.md`: two `obtained`
records). No CI job currently runs the webview long enough to record one.

Also in both boot logs (pre-existing, not this chunk's): `corpus.open.error` (ERROR) plus the three
`triage.*.persist.error` WARNs — the runner has no credential store; `ci-gates` passed with them in both runs.

## Entry 42 — cache usage against the cap

| | bytes |
|---|---|
| cap | 10 737 418 240 |
| after this round | 10 605 172 169 (8 entries) |
| P3 read, before the change | 10 605 172 169 (8 entries) |
| headroom | 132 246 071 (1.23 %) |

Keys (all on lock hash `769a9503`): `rust-lint-test` × Windows / macOS / Linux · `rust-release` × Windows / macOS ·
`rust-boot-Linux` · `rust-coverage-Linux` · `gitleaks-cache-8.24.3-linux-x64`. No owning key evicted; the cap is not
reached. The byte-identical total fits the change: a green round on an exact key hit re-saves nothing, and
`cache-on-failure` only acts on a red round, re-saving the same key. No lockfile moved, so no new key set was created.
