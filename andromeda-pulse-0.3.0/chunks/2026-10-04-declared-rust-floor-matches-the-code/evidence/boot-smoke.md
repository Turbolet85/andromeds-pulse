# Boot smoke — operator entries 20 and 21, fired once by hand at /implement

Port slot 4317/4318 granted for this chunk (overseer review, founder-delegated, 2026-10-04). Both entries were
driven with their exact plan `run` text, after the block's `cargo build -p pulse-app --bin pulse-app` (entry 19,
green) and before the re-fired bindings regen (entries 22–23 re-run afterwards, see below).

## Entry 20 — non-priming precondition (2026-10-04T21:57:40Z)
- `test -x target/debug/pulse-app && test "$(ss -ltnH | grep -cE ':(4317|4318) ')" = 0` → exit 0.
- Side reads: `xvfb-run` at `/usr/bin/xvfb-run`; no listener on :4317/:4318; no `pulse-app` process.

## Entry 21 — unattended Xvfb boot smoke (2026-10-04T21:57:51Z → 21:58:26Z)
- The entry's body run verbatim, bounded by `timeout 150` (the entry's own `timeout` key) because it was driven
  by hand rather than through the gate tool. Exit 0.
- Printed lines (the atoms' subject):
  - `data_dir=…/target/floor-smoke/xvfb-20261004T215754Z`
  - `ended=alive`
  - `ports_after=0`
  - `posture=applied`
  - `errors=0`
  - `panics=0`
- Atoms: `exit 0` ✓ · `contains ended=alive` ✓ · `contains posture=` ✓ · `contains errors=0` ✓ ·
  `contains panics=0` ✓ · `contains ports_after=0` ✓.
- Artifact freshness: the data dir `target/floor-smoke/xvfb-20261004T215754Z` was created at 21:57:54Z, inside
  this run; the log `agent-latest.jsonl.2026-10-04` last written 21:58:24Z.
- **Posture value, recorded not asserted:** `applied` — `{"posture": "applied", "lever": "__NV_DISABLE_EXPLICIT_SYNC"}`,
  as the P5 expectation correction predicted (`render_posture::decide` keys on Linux + an unset lever only).
- Supporting reads from the same log (476 records; levels INFO 472 · WARN 4):
  - `app.boot.otlp.grpc.bind` 1 · `app.boot.otlp.http.bind` 1 · `app.boot.webview.init` 1 · `ingest.tick` 2 ·
    `buffer.tick` 2 · `app.exit` 1.
  - Fields rendered `<redacted>` across all records: **0** — the rewritten `FieldAllowlist::for_target` resolved
    every emitted field.
  - The 4 WARNs: `interpretation.model.allow_root {confinement: unconfined}` (no L4 root set) ·
    `ui.webgpu.adapter {outcome: no_navigator_gpu}` for `compact-widget` and `main` (no WebGPU under Xvfb) ·
    `app.exit {exit_class: signal, signal: sigterm, exit_code_known: false}` (the leg's own TERM; no KILL needed).
- The real-display (native Wayland) leg was not run this chunk (plan Constraints).

## Process census after the leg (2026-10-04T21:58Z)
- `ps -eo pid,comm,args` filtered for `pulse-app|Xvfb|xvfb-run`: none.
- Listeners on :4317/:4318: 0.

## Bindings after the smoke
Running the default-features binary re-emits the no-mcp bindings, so entries 22 (mcp-server regen) and 23
(`git diff --quiet {base} -- pulse-app/ui/src/bindings/index.ts`) were re-run through the gate tool after this
leg (`--only 22,23`); their result is in the /implement report.
