# P-075 round result — 2026-10-02-incident-events-readable-through-mcp

Written by /implement (plan Step 11) on the overseer's word (2026-10-04: "Conductor COMMITTED its round on S2 at e6e1eef
… Run entry 22 now, write round-result.md, then stop and report"). Every verdict below is READ from Conductor's committed
files at the commit named here — cited, never copied (CLAUDE.md 2026-08-21); the numbers stay in Conductor's ledger.

## The round
- **Pulse binary under test:** S2 = `cdb6c1ed572761ae384597a7ed437222e3a1d1fc` (`round-binary.md`). Conductor's ledger
  §The binary under test records the two sha256s re-measured immediately before the first launch, each EQUAL to S2's
  (`pulse-app` `23f6ef2b…6ada`, `andromeda-pulse-mcp` `e64f3688…e02e`), and the Pulse checkout HEAD reading S2, no rebuild.
- **Conductor commit:** `e6e1eefa6d0946b79e66695792abe1e3db18c09c` —
  `chore(2026-10-03-p-075-re-round-on-incident-events): operator pre-CI commit, for the run this chunk's verdict reads`,
  on Conductor's `build/conductor-0.3.0`.
- **Conductor CI:** `CI#37162108538` on `e6e1eef` — `completed success` (read with `gh run view`, 2026-10-04).
- **Evidence path:** `conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/evidence/` at `e6e1eef` — the
  ledger `round-ledger.md`, the leg output `p075-leg.txt`, and the per-leg Pulse log slices.

## Plan entry 22 — Conductor evidence names S2
`grep -rlF 'cdb6c1ed572761ae384597a7ed437222e3a1d1fc' ~/dev/projects/conductor/conductor-0.3.0/chunks --include=*.md`
(the entry's form, Conductor's location remapped to this host on the operator's word): exit 0, three hits in the round
chunk — `scope.md`, `plan.md`, `evidence/round-ledger.md`. Necessary, not sufficient: the verdicts are read below.

## The seven verdicts (read from `round-ledger.md` §The seven verdicts at `e6e1eef`)
| # | Assertion (`round-request.md`) | Verdict | Conductor test id (`crates/conductor-run/tests/`, present at `e6e1eef`) |
|---|---|---|---|
| 1 | Read-back content fidelity | `[PASS]` | `lifecycle_harvest::p075_reround_assertion_1_read_back_content_fidelity` |
| 2 | Runtime-state fidelity | `[PASS]` | `lifecycle_harvest::p075_reround_assertion_2_runtime_state_fidelity` |
| 3 | P-025 hue update ≤ 2000 ms | `[PASS]` | `delegated_timing_harvest::tests::p075_reround_assertion_3_p025_hue_update` |
| 4 | P-027 discovery ≤ 5000 ms | `[PASS]` | `delegated_timing_harvest::tests::p075_reround_assertion_4_p027_discovery` |
| 5 | P-037 report render ≤ 2000 ms | `[PASS]` | `delegated_timing_harvest::tests::p075_reround_assertion_5_p037_report_render` |
| 6 | P-045 counter refresh ≤ 1000 ms | `[PASS]` | `delegated_timing_harvest::tests::p075_reround_assertion_6_p045_counter_refresh` |
| 7 | Incident events read-back | `[PASS]` | `lifecycle_harvest::p075_reround_assertion_7_incident_events_read_back` |

**7/7 PASS.** The ledger records none UNGRADED and no leg re-fired. Assertion 7's row reads, before the resolve, the
`created` event alone with no `resolved`; after it, `created` then `resolved`, the `resolved` stamp inside the resolve
call's wall-clock window; zero events outside the four-kind vocabulary; neither read truncated — the first-event-`created`
and no-`unknown` bullets that S could not pass. Each test fn was located at `e6e1eef` with `git grep -l "fn {id}"`.

## A Pulse finding — the S2 Linux build does not stay up at its default launch posture on this host
Read from `round-ledger.md` §A Pulse finding to relay and §Deviations from the plan's entry text at `e6e1eef`, and
relayed by the overseer (2026-10-04) as a PULSE finding for these records and the wrap:
- **What happened:** the first launch, with the round plan's three env handles only, booted to `ui-bridge.ready`, loaded
  deterministic L4, then died about 1.5 s in; its last stderr line was a Gdk `Error 71 (Protocol error) dispatching to
  Wayland display`. No leg had fired; the OTLP ports never opened a listener.
- **Host:** NVIDIA GA102 (RTX 3090) · Hyprland · WebKitGTK 2.52.6 · native Wayland.
- **What the round ran under instead:** a relaunch on a fresh data dir with `WEBKIT_DISABLE_DMABUF_RENDERER=1`, still on
  native Wayland — a launch-posture deviation the overseer ruled (founder-delegated) and Conductor recorded beside the
  verdicts. It stayed up for the whole round. The crashed launch's data dir and stderr capture were left on the host,
  outside both repositories.
- **What this does and does not establish:** measured on ONE host (that GPU, compositor and WebKitGTK version) at ONE
  launch; whether every NVIDIA + Wayland + WebKitGTK host is affected, and whether the DMA-BUF renderer is the cause or
  only the lever that avoids it, is unmeasured. No confining mechanism is identified. The verdicts above are measured
  under the deviated posture; S2's default posture on this host is NOT verified to stay up.
- **Disposition:** not this chunk's to fix (no pulse-app launch path is in its scope). Surfaced for the wrap, which names
  an owner — a product question (ship a default, detect and fall back, or document) for the founder.

## What this file does not do
It writes no matrix entry: P-075 stays `verified` with its acceptance and `ref` as concretized at
2026-10-01-conductor-return (the matrix tool has no edge to reopen it from this chunk); the wrap writes the `notes` line
that names S2, `e6e1eef`, `CI#37162108538` and this file (plan §Expected amendments).
