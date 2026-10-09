sweep: base: blame — no earlier sweep; each citing line's own last commit · re-pointed 3 · changed 6 · stretched 5 · first sweep: "yes, as read. I re-read all six rows and the two unlisted ps1 numbers at 569604b and they stand as you say: write the three rows that hold, withhold the three refused, re-point by hand agent-run.sh:29 and :30, xtask/src/main.rs:306, agent-run.ps1:24 and :25" — the operator, 2026-10-09 · withheld 3

# Citation dispositions — the first sweep, 2026-10-09T15-11-06Z wrap of 2026-10-09-ci-on-linux-alone

Each row as `cites.py apply --first` printed it, then its disposition. The block's read is `first-sweep-read.md`.

## The block (written)

- `moved .andromeda/architecture.md:244 scripts/agent-run.sh:99 → 230 «if [ -z "${ANDROMEDA_PULSE_DATA_DIR_KEEP:-}" ] && …»` → holds
- `moved .andromeda/architecture.md:244 scripts/agent-run.ps1:71 → 100 «if (-not $env:ANDROMEDA_PULSE_DATA_DIR_KEEP -and …»` → holds
- `moved .andromeda/obs-plan.md:568 crates/buffer/src/retention.rs:72-86 → 69-83 «let sweep_conn: Arc<Mutex<Connection>> …»` → holds

## Masters

- `moved .andromeda/architecture.md:242 scripts/agent-run.sh:22 → 24 — listed, not written: withheld «export PYTHONIOENCO…»` → holds — re-pointed by hand to 29 (`PIDFILE="${ANDROMEDA_PULSE_PIDFILE:-…}"` read at `scripts/agent-run.sh:29`)
- `hand .andromeda/architecture.md:242 scripts/agent-run.ps1:24 — base 15 · map 15 «# The Console pair is the Windows-onl…»` → holds (the `$PidFile` read from `$env:ANDROMEDA_PULSE_PIDFILE` stands at `scripts/agent-run.ps1:24`; re-pointed by hand before the write, on the operator's word)
- `moved .andromeda/architecture.md:243 scripts/agent-run.sh:23 → 25 — listed, not written: withheld «»` → holds — re-pointed by hand to 30 (`LOGFILE="${ANDROMEDA_PULSE_LOGFILE:-…}"` read at `scripts/agent-run.sh:30`)
- `hand .andromeda/architecture.md:243 scripts/agent-run.ps1:25 — base 16 · map 16 «# child-process output through the OE…»` → holds (the `$LogFile` read from `$env:ANDROMEDA_PULSE_LOGFILE` stands at `scripts/agent-run.ps1:25`; re-pointed by hand before the write, on the operator's word)
- `changed .andromeda/test-plan.md:221@c941 crates/buffer/src/schema.rs:320-323 → ? — both ends rewritten «// check via d…»` → claim false — raised at Validate. The sentence says the source comment there "still states the disproved cause and is owned by its own route entry". Read: the comment it cited (above `spans_primary_key_is_composite_trace_id_span_id`) was rewritten at `f9c472c` (chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep) and now states the measured behaviour, at `crates/buffer/src/schema.rs:264-269`; lines 320-323 are code of another test. A sibling comment at `crates/buffer/src/schema.rs:342-343`, above the `metrics_points` key test, still reads "the duplicate-INSERT probe hangs on this build". No route entry, residual or requirement names it (`grep -n -i 'schema\.rs|duplicate-INSERT|disproved cause'` over the master route, residuals, both versions' working routes and the 0.4.0 requirements: 0 hits for it). The amendment that settles it rewrites the citation in this wrap.
- `moved .andromeda/test-plan.md:493 xtask/src/main.rs:174 → 207 — listed, not written: withheld «#[command(»` → holds — re-pointed by hand to 306 (`Cmd::TestA11y { extra } => run_npm_script("test:a11y", extra).await` read at `xtask/src/main.rs:306`)
- `moved .andromeda/obs-plan.md:453@c680 observability.rs:1934 → 1971 — listed, not written: suffix` → not a citation of the tree — no change. The sentence records where a since-removed bare `interpretation` key stood when it was measured on 2026-08-26; the key is gone, so the number is a dated coordinate of a past file state and no line of today's file shows it.

## Leaves

A leaf is re-derived, never patched; a citing line in a preserve-verbatim home goes to P3.

- `moved .claude/rules/frontend.md:99 xtask/src/main.rs:550 → 766 — listed, not written: leaf` → curation home — P3 (`## Session Additions` opens at `:94`)
- `moved .claude/rules/observability.md:68 observability.rs:1934 → 1971 — listed, not written: leaf` → leaf — re-derived (the rule's body restates obs-plan §8's dated coordinate above; re-derived from the master, it keeps that number)
- `stretched .claude/rules/observability.md:134 pulse-app/src/observability.rs:593-613 → 1070-1107 — changed inside · listed, not written: leaf` → curation home — P3 (`## Session Additions` opens at `:123`)
- `out-of-range .claude/rules/observability.md:148@c2263 pulse-app/src/heartbeat.rs:1078 — the file held 967 lines at the base` → curation home — P3
- `out-of-range .claude/rules/observability.md:148@c2272 pulse-app/src/heartbeat.rs:1934 — the file held 967 lines at the base` → curation home — P3
- `moved .claude/rules/security.md:159 xtask/src/main.rs:813 → 1040 — listed, not written: leaf` → curation home — P3 (`## Session Additions` opens at `:94`)
- `moved .claude/rules/testing.md:159 crates/plugins/src/engine.rs:38-45 → 42-49 — listed, not written: leaf` → curation home — P3 (`## Session Additions` opens at `:110`)
- `moved .claude/docs/obs-summary.md:151 observability.rs:1934 → 1971 — listed, not written: leaf` → leaf — re-derived (the same dated coordinate, restated from obs-plan §8)
- `stretched .claude/docs/session-learnings.md:825 pulse-app/src/main.rs:266-360 → 327-487 — changed inside · listed, not written: leaf` → curation home — P3
- `moved .claude/docs/session-learnings.md:1272 Cargo.toml:42 → 47 — listed, not written: leaf` → curation home — P3
- `changed .claude/docs/session-learnings.md:1349 CLAUDE.md:52 → ? — rewritten «| Roadmap (9 epochs / 56 chunks) | `.andr…»` → curation home — P3
- `moved .claude/docs/session-learnings.md:1516 Cargo.toml:12 → 18 — listed, not written: leaf` → curation home — P3
- `changed .claude/docs/session-learnings.md:1516 crates/workspace-detector/Cargo.toml:12 → ? — deleted «strict-path.work…»` → curation home — P3
- `stretched .claude/docs/session-learnings.md:1544 pulse-app/Cargo.toml:53-56 → 71-80 — changed inside · listed, not written: leaf` → curation home — P3
- `stretched .claude/docs/session-learnings.md:1789 pulse-app/src/main.rs:280-282 → 1118-1128 — changed inside · listed, not written: leaf` → curation home — P3
- `changed .claude/docs/session-learnings.md:1809 crates/ui-bridge/src/lib.rs:3-19 → ? — the start deleted «pub mod snaps…»` → curation home — P3
- `stretched .claude/docs/session-learnings.md:1809 crates/ui-bridge/src/telemetry.rs:93-134 → 252-400 — changed inside · listed, not written: leaf` → curation home — P3
- `changed .claude/docs/session-learnings.md:1809 pulse-app/src/main.rs:19-20 → ? — the start deleted «use ui_bridge::sna…»` → curation home — P3
- `changed .claude/docs/session-learnings.md:1821 crates/ui-bridge/src/snapshot_ipc.rs:11 → ? — the file is gone` → curation home — P3

## Not owed

`unresolved 17: their rows are map's — another repository's file is never touched` — no disposition is owed on them.
