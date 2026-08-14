# arch extract

## Relevance
relevant — This chunk implements core triage queue reliability (hard-signal → digest → L4 → incident coalescing + elastic buffering), foundational to the product's incident detection pipeline.

## Constraints
1. Code lives in `triage` crate (locked workspace member per architecture §Occupied Resources Cargo workspace crate names).
2. Async runtime is `tokio` — elastic queue uses `tokio::sync::mpsc` / `tokio::sync::broadcast` primitives per architecture §Established Decisions [In-Process Channel Architecture].
3. Error enums use `thiserror` 2.x in modules; boundary conversions to `AppError` per architecture §Established Decisions [Error Handling Pattern].
4. No new workspace crates — all code within `triage` + `pulse-app` binary wiring per architecture §Cross-cutting Patterns [Module dependency direction].
5. L4 inference latency is ~4s per digest per architecture §Established Decisions [LLM Inference Runtime]; elastic queue + coalescing COMPENSATE for latency, do not reduce it.
6. Heartbeat-drain tick wired at `pulse-app` runtime boundary, not in triage module, per architecture §Cross-cutting Patterns [Module dependency direction — DAG with `pulse-app` as only root].
7. Observability is aggregate-only via `pipeline.l3.*` counters/gauges; no per-service identifiers per triage cardinality discipline and architecture §Conventions [Database entity naming — plural snake_case, aggregate-only].

## Patterns to follow
1. **Modular monolith via `tokio::sync::broadcast`** — digest drain/heartbeat signaling matches buffer / interpretation precedent per architecture §Stack and Technologies [In-process channels].
2. **Queue-to-inference pipeline** — digests flow through coalescing → elastic queue → cadence heartbeat → L4 inference, consistent with architecture §Design Philosophy [Token-efficient curation as a first-class output].
3. **TauRPC contract conformance** — any new IPC for queue introspection uses `<router>.<verb>` dotted namespace per architecture §Conventions [Endpoint naming].
4. **Runtime-agnostic heartbeat task** — implementation lives at `pulse-app/src/digest_runtime.rs` boundary, not in triage crate, per architecture §Cross-cutting Patterns [Development Style — agent-driven harness integration].

## Anti-patterns to avoid
1. Do NOT introduce `triage` → `pulse-app` dependency or circular edge (violates architecture §Cross-cutting Patterns [Module dependency direction DAG]).
2. Do NOT export queue internals via `pub` across crate boundaries — only `Digest`, `DigestKind`, `DigestLwwMode` contract is `pub` per architecture §Conventions [Module visibility discipline].
3. Do NOT spawn new workspace crate for queue work; §Occupied Resources [Cargo workspace crate names] is locked.

## Contract bindings
- **triage ↔ interpretation**: digest drain feeds L4 inference; chunk respects ~4s latency per architecture §Established Decisions [LLM Inference Runtime].
- **triage ↔ buffer**: hard-signals originate in pattern matching, enqueue into elastic queue per architecture §Design Philosophy [Single-process modular monolith].
- **triage ↔ pulse-app**: heartbeat tick wired at binary boundary via existing `pulse://stream/cadence-events` or new independent tick; pulse-app may expose new diagnostics IPC per architecture §Occupied Resources [Tauri IPC routes].
- **triage ↔ tests harness**: acceptance test injects ≥100 synthetic hard-signals via OTLP and verifies one coalesced incident per architecture §Cross-cutting Patterns [Test-time telemetry injection].

## Acceptance criteria contributions
- (arch) Code lives in `triage` crate per workspace boundary rules (arch §Inherited Defaults [Module Boundaries]).
- (arch) No new workspace crate introduced; all code in `triage` + `pulse-app` binary boundary (arch §Occupied Resources Cargo workspace crate names).
- (arch) New aggregate-only observability (`pipeline.l3.coalesce_count`, `pipeline.l3.elastic_queue_depth`, `pipeline.l3.heartbeat_drain_tick`) conforms to triage cardinality discipline (arch §Conventions + obs-plan).
- (arch) Heartbeat-drain mechanism wired at `pulse-app` runtime boundary, not in triage module (arch §Cross-cutting Patterns [Module dependency direction]).

## Relevant amendment history
- **2026-05-23** — Acknowledge `pulse://stream/digests` (chunk #81 L3 digest assembler topic) — the coalescing + elastic-queue work feeds into this existing broadcast channel; no new broadcast needed.
- **2026-05-23** — Acknowledge `pulse://stream/cadence-events` (chunk #80 cadence coordinator L6-visibility topic) — the chunk will interact with or reuse this tick for heartbeat-drain signaling per scope open question "Heartbeat tick owner: new tick in `digest_runtime.rs` vs reuse of the cadence coordinator tick."
- **2026-06-28** — Acknowledge `ANDROMEDA_PULSE_L4_DETERMINISTIC` env var (P-073 deterministic L4 mode) — the chunk's acceptance test likely runs under deterministic L4 to make "one coalesced incident" reproducible per scope §Boundaries.
