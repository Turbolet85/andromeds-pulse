# Scope — Delegated timing observables

**Marker:** `2026-08-21-delegated-timing-observables`
**Working entry:** first markerless entry of `andromeda-pulse-0.3.0/working-route.md`, minted at the
2026-08-21 operator adaptation wrap (`edb949f`).

## What this chunk builds

Three timing observables that do not exist today, so that the three delegated timing bounds become
measurable **at the wire** with real values:

| Cap | Title | Delegated bound | Observable today |
|---|---|---|---|
| P-025 | Halo Hue Encoding | pipeline latency ≤ 2s | none |
| P-027 | Service Constellation Auto-Discovery | span → constellation dot ≤ 5s | none |
| P-045 | Counter Derivation from Corpus | counter refresh ≤ 1s | none |

All three carry an identical `notes` clause in `docs/v0_2_0/capability-verification-matrix.json`:
the bound is *"delegated to Conductor dynamic verification per the 2026-06-12 audit §Remediation
defaults item 8 — intentionally NOT faked as a sleep-based unit test"*, while each cap's static /
functional half is already automated. **This chunk supplies the missing measurement surface; it does
not re-verify the static halves and does not itself assert the bounds.**

## Why now

Conductor's Epoch 4 blocks only on this. Its "Delegated timing budgets" entry cannot proceed while
three of the four delegated caps have no `metric.*` surface on the Pulse side. `P-037` is the fourth
and is already observable, which is why it is the shape precedent rather than a target here.

## Boundaries

- **In:** the three observables (emit sites + their allowlist leaves + whatever webview-side mark each
  needs), their unit coverage, and the obs-plan/test-plan reconciliation the new targets require.
- **Out:** asserting the ≤2s / ≤5s / ≤1s budgets. That assertion is Conductor-side and belongs to
  P-075, which **stays pooled** (`chunk:null`, operator-decided 2026-08-21): its acceptance is
  `dynamic-external` and additionally needs a Conductor assertion round, so building the observables
  cannot claim it without producing a hollow `verified`.
- **Out:** the other three v0.3.0 unclaimed caps (P-076 / P-077) and every entry behind this one.
- **Out:** changing what the three surfaces *do* — this is instrumentation, not behaviour.

## Surfaces and contracts touched

- **Two shape precedents, both verified in place 2026-08-21:**
  - *Backend-measured:* `tracing::info!(target: "metric.report.render_ms", value = duration_ms, …)` at
    `pulse-app/src/incidents_router.rs:561` — a dedicated `metric.*` target emitted from a Rust
    resolver around server-side work.
  - *Webview-originated:* `telemetry.frontend.record_frame_ms(FrameDurationInput)` at
    `crates/ui-bridge/src/telemetry.rs:99` — validates its input, level-gates on INFO, then emits
    `metric.webgpu.frame_duration_ms`. This is the **only** frontend→backend telemetry procedure that
    exists.
- **Obs allowlist** (`pulse-app/src/observability.rs`): each new `metric.*` target needs its own leaf.
  The registry is default-deny and `for_target` resolves exact → strip `.tick` → first `.`-segment →
  `::`-prefix, so a bare-prefix key is not a substitute for an exact leaf (`.claude/rules/observability.md`,
  2026-05-07 + 2026-05-03 entries).
- **Cardinality discipline:** aggregate-only fields, no per-service identifiers in self-observation
  events (`.claude/rules/observability.md` 2026-05-17).

## Open premises — closed at P3 research (2026-08-21)

1. **Measurement side — VERIFIED: webview-originated.** obs-plan §1/§4 states it as a domain
   *rule*, not a preference: any webview-originated measurement must reach the log **only** through
   a TauRPC `telemetry.frontend.*` command. The infrastructure already exists end to end
   (`pulse-app/ui/src/canvas/frame-metrics.ts` → `crates/ui-bridge/src/telemetry.rs:99` →
   `metric.webgpu.frame_duration_ms`) and already emits from the reduced-motion branch
   (`CanvasContainer.test.tsx:154`), which is what the design + a11y reduce-motion constraint
   demands. A backend-only timestamp cannot measure these bounds at all — all three end instants
   are paints inside the webview. **The fork is settled by the artifacts; it does not go to the
   operator.**
2. **Binding cost — `[premise-corrected: no per-procedure capability JSON exists in this project;
   `default.json` grants only `core:default` + 7 `core:window:*` + `updater:default`, and
   `telemetry.frontend` is already in `EXPECTED_PROCEDURES` and already merged in both the
   production router (main.rs:990) and the `emit_taurpc_bindings` test (main.rs:206)]`.** The route
   entry's "a full TauRPC quadruple binding" (quoting the P-075 record) overstates it. Real cost:
   trait+resolver, one `EXPECTED_PROCEDURES` line, a bindings regen, the a11y `mock-tauri.ts` entry,
   and the `bindings.test.ts` procedure-list pin.
3. **Three procedures vs one — OPEN, and it is the chunk's real design choice.** `FrameDurationInput`
   is a working precedent for one procedure carrying bounded enums; security is explicitly neutral
   provided the discriminant is a closed enum; obs requires three distinct
   `metric.{module}.{measure}` targets under either shape. Goes to the operator at P4.
4. **Start instants — VERIFIED available for all three.** `ServiceListItem.last_seen_unix_nano` is
   already on the wire for P-027; `IncidentRecord.opened_at_unix_nano` / `updated_at_unix_nano` back
   the severity driving P-025's hue; P-045 needs no backend reference — its bound is the
   webview-local poll→commit interval (`use-findings.ts:81`).

**Constraint discovered at research, load-bearing for implementation:** a bare `"metric"` allowlist
key IS registered, carrying exactly `["value", "unit", "module"]`. Since `for_target` falls back to
the first `.`-segment, a new `metric.*` target with no exact leaf resolves to that 3-field set and
**every field except `value` is silently redacted** — the measured mechanism behind the still-open
`metric.pipeline.l1a.*` backlog. Any observable carrying a surface discriminant MUST have its own
exact leaf.

## Folded annotations

- **PREREQ (re-pinned from P-076 at the 2026-08-21 adaptation wrap, origin
  `2026-08-15-corpus-key-persistence`):** re-check `cargo audit`. It is under a ratified standing
  deferral with an every-3rd-wrap interval; **the last wrap was session 30 and recorded the interval
  skip, so the probe FIRES at session 31** — i.e. at this chunk's wrap. Basis to re-verify: the
  upstream RustSec DB cannot load (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`, real exit
  1). Named overlap that runs green every chunk: `cargo deny check advisories`, expected at exactly
  the 7 owned upgradeable IDs. Full rationale: the `2026-08-15-corpus-key-persistence` report.
- No `CARRY` and no `BLOCKED-ON` on this entry.

## Provenance

Minted from an overseer relay (2026-08-21) citing Conductor chunk reports at HEAD `2c36369`. Per that
relay's own instruction the evidence is **cited, not copied**. Every Pulse-side coordinate it supplied
was re-verified first-hand at HEAD `efabe8e` before this scope was written; one relayed claim in the
same batch (intake #7's `buffer.tick` half) reproduced FALSE, which is why the coordinates above carry
their verification date.
