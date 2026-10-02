# Premise-check — the discriminator run

**Verdict: BRANCH B. The in-window regression premise is FALSIFIED.**
A detected storm DOES produce an incident on HEAD. The storm→cue→digest→L4→incident chain is
healthy end to end, and the measured behaviour matches the threshold arithmetic research predicted
at `/andromeda-phase` P3 exactly.

## 1. Conditions

| | |
|---|---|
| HEAD | `3ac3d9d` — working tree source-clean at measurement time (`git status --short -- crates/ pulse-app/src/` returned nothing) |
| Binaries | `target/debug/pulse-app.exe` + `target/debug/examples/inject_demo.exe`, **both rebuilt at HEAD** immediately before the run (`CARGO_INCREMENTAL=0 cargo build -p pulse-app --bin pulse-app --jobs 4 && cargo build -p ingest --example inject_demo --jobs 4`, exit 0) |
| Data dir | `<scratchpad>/pc-run1` — **created fresh for this run**, never previously written |
| Env gates | `ANDROMEDA_PULSE_DATA_DIR=<fresh>` · `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` (P-073) · `ANDROMEDA_PULSE_LOG_LEVEL=info` |
| Driver | `./target/debug/examples/inject_demo.exe` over the real OTLP receiver `127.0.0.1:4317` — no in-process bypass, no test-mode flag |
| Ground truth | keyless read of `<data_dir>/corpus/corpus.db` plaintext columns |
| Shutdown | `powershell -NoProfile -Command "Stop-Process -Id 34636 -Force"`; `netstat` then showed `:4317`/`:4318` closed, zero orphans |

**Ground-truth tooling note.** `sqlite3` is not on this host's PATH, so the keyless read went through
Python's stdlib `sqlite3` opened read-only (`sqlite3.connect("file:...?mode=ro", uri=True)`). Same
engine, same plaintext columns, no key, equally independent of the app's decryption path — the
property the evidence discipline requires. Probe scripts ran from files, not inline.

## 2. What was driven

`inject_demo`'s `payment-service` runs `error_pct: 100` across 2 ops × 3 spans = **6 identical
-fingerprint exceptions per 500 ms batch** (`crates/ingest/examples/inject_demo.rs:69-101`), after a
10-batch healthy warmup. The run was stopped at batch ~121: **666+ exceptions, all one fingerprint**
(`c33df842`).

Worth stating plainly: **one inject_demo batch is exactly 6 exceptions — the same count as
Conductor's `CANARY_STORM_COUNT`.** The canary is one batch's worth of storm. inject_demo differs
only in that it keeps going.

## 3. THE DISCRIMINATOR — the storm tier, not the count

`storms_detected_total` increments on **both** the Suggested and Autonomous branches
(`crates/triage/src/pattern/storm.rs:255`, `:275`), so `0→N` alone cannot distinguish them.
`severity_hint` on `triage.pattern.storm.detected` is what names the tier.

> query: `grep '"target":"triage.pattern.storm.detected"' agent-latest.jsonl.2026-08-15`

```
occurrence_count=5   severity_hint=suggested    fingerprint_hex=c33df842  window_seconds=30
occurrence_count=10  severity_hint=autonomous   fingerprint_hex=c33df842  window_seconds=30
storms_detected_total: first=0  last=2
```

**This is the P3 arithmetic, measured.** Production thresholds are `DEFAULT_SUGGESTED_THRESHOLD = 5`
/ `DEFAULT_AUTONOMOUS_THRESHOLD = 10` (`storm.rs:73-78`). The detector crossed 5 → Suggested, then
crossed 10 → Autonomous, in that order, from one fingerprint. The Conductor arm's 6 events reach the
first line and **cannot** reach the second.

## 4. The full chain, measured

> query: per-target counts over `<data_dir>/logs/agent-latest.jsonl*` (32,985 JSON lines)

| hop | target | count | reading |
|---|---|---|---|
| 1 feed | `buffer.tick` | 7 | feed alive |
| 2 detect | `triage.pattern.storm.detected` / `.emit` | 2 / 2 | both tiers fired |
| 3 cue | `triage.cue.emit` | 10 | cues emitted |
| 4-5 trigger→digest | `digest.runtime.cadence_tick` | 11 | modes `{tier1: 4, tier2: 6, tier3: 1}` · **`cue_present: {True: 10, False: 1}`** |
| 5 persist | `digest.runtime.persist` | 11 | digests persisted |
| 6 L4 | `interpretation.inference.request` | 11 | all `result: success`, `model_tier: primary` |
| 6 skip-gate | `l4.inference.skipped` | **0** | no backoff skip |
| 7 incident | `interpretation.incident.created` | 10 | **2 `created=true`, 8 `deduped=true`**, all `priority_tier: autonomous` |

The single `cue_present: False` is the tier3 baseline-cadence digest, which carries no cue by design.

## 5. GROUND TRUTH — the keyless corpus read

> query: `SELECT count(*) FROM <table>;` and `SELECT id, workspace, status FROM incidents;`
> against `file:<data_dir>/corpus/corpus.db?mode=ro`

```
incidents            2
incident_events      2
digest_archive      11
pipeline_metrics     2
baseline_state       0
service_registry     5

incidents rows:
  (1, '\\?\D:\dev\projects\andromeda-pulse', 'active')
  (2, '\\?\D:\dev\projects\andromeda-pulse', 'active')
```

**Two active incident rows exist.** 10 creation events deduped to 2 persisted incidents — the
re-emission dedup on `(kind, scope, scope_id)` working as designed
(`pulse-app/src/inference_runtime.rs:660-681`), not a loss.

**P-079 alignment confirmed live, incidentally:** the stamped `workspace` is
`\\?\D:\dev\projects\andromeda-pulse` — the canonicalized *detected root*, not the data dir. The
ranked suspect (`resolve_workspace_for_incidents` stamping something other than what the filter
uses) is disproven by observation, not only by the single-caller graph argument.

## 6. Health

0 `app.panic.fatal` · 0 `ERROR` · 3 `WARN`, all one benign kind:

```
digest.lww.drop  drop_reason=queue_cap_reached  {tier1, tier2, tier3}
```

## 7. What this settles, and what it does not

**Settles:** there is no regression in the storm→incident path at HEAD. Step 2's three-point bisect
(`b08e10a` → `2961f4e` → `3ac3d9d`) is **not warranted** and was not run — running it would have
spent the chunk chasing a premise this measurement disproves. No break was located, so no fix was
made; the plan explicitly bars manufacturing one.

**Does not settle:** why the corpus-key chunk's two smoke boots showed zero incidents. This run
shows only that a *sustained* storm produces incidents. If those boots never drove a fingerprint
past 10 they carry the same explanation as the Conductor arm; that remains a handoff question.

## 8. The finding — a cross-project constant mismatch

Conductor's canary emits exactly `CANARY_STORM_COUNT = 6` same-fingerprint events. Pulse forms an
incident only at `DEFAULT_AUTONOMOUS_THRESHOLD = 10`. `5 ≤ 6 < 10` places the canary in the
**Suggested** tier, which Tier-1 drops by design (`crates/triage/src/cadence/coordinator.rs:390`).

The canary undershoots Pulse's incident-forming threshold **by construction**. The natural fix is
Conductor-side — drive the canary past 10 — and explicitly **not** any Pulse tier-semantics change.
Recorded as a hand-back for the consolidated Conductor visit, joining the queued
`architecture.md:174` correction and the verdict-doc appends. Nothing in Conductor's tree was
written by this chunk.

---

# Post-fix verification run (`pc-run2`)

A **second** boot on a second fresh data dir, after the obs-allowlist change, with the same driver.
This is what proves the §6 repair: the pre-fix run above showed these fields `<redacted>`; this one
shows their values.

## The three targets, before and after

| target | `pc-run1` (pre-fix) | `pc-run2` (post-fix) |
|---|---|---|
| `incidents.list_active.request` | `item_count: <redacted>` ×319 | **`{"item_count": 0}` ×57, `{"item_count": 1}` ×2, `{"item_count": 2}` ×100** |
| `triage.incident.corpus_restore` | `kind: <redacted>`, `restored_incident_count: <redacted>` | **`{"kind": "incident", "restored_incident_count": 0}`** |
| `triage.incident.persist` | `incident_count` / `persist_kind` / `duration_ms` all `<redacted>` | **`{"duration_ms": 37, "incident_count": 2, "persist_kind": "incident"}`** |

`item_count` now shows the incident count *forming in the log* — 0 → 1 → 2. That progression is
precisely the diagnostic whose absence forced the out-of-band `sqlite3` read during the arm-zero
classification.

`triage.incident.persist` carries **`duration_ms`**, which the plan's field list did not name. The
emit site (`crates/triage/src/incident/persistence.rs:176-182`) emits it alongside the other two;
allowlisting only the two named fields would have left the target partly redacted — the same defect
in miniature. Caught by reading the emit site rather than transcribing the list.

The five *other* muted targets this probe surfaced (`metric.pipeline.l1a.*` at 100× each,
`triage.cue.tick`, `triage.incident.auto_resolve.tick`, `interpretation.model.load`) are **not**
in this chunk's scope and were deliberately left alone — handed off rather than silently absorbed.

## Branch B reproduced independently

Same tier progression (`occurrence_count=5 → suggested`, `occurrence_count=10 → autonomous`), same
`storms_detected_total` 0→2, same `interpretation.incident.created` split (2 created / 8 deduped,
all `autonomous`), same **2 active incident rows** at the same detected-root workspace.

## Health

26,101 JSON lines · **0 `app.panic.fatal` · 0 `ERROR`** · 3 WARN, all `digest.lww.drop
drop_reason=queue_cap_reached` · clean `Stop-Process` shutdown with `:4317`/`:4318` released and
zero orphans.
