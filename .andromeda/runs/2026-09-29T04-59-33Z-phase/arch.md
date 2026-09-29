# arch extract

## Relevance
relevant: §A adds a field to a TauRPC payload that crosses the bridge and reshapes a delegated-timing observable registered in §Occupied Resources. §B moves an arch-registered env var within CI workflows that arch §Infrastructure Patterns and §Occupied Resources both describe.

## Constraints
- **Delegated-timing route is fixed.** Per architecture.md §Occupied Resources → Tauri IPC routes (`telemetry.frontend.record_constellation_hue_latency` entry), P-025 is a delegated-timing observable. Its interval ends at a webview paint the backend cannot see, so `telemetry.frontend.*` is its only sanctioned route to the log. The same entry names the measured surface: the constellation DOT hue (`severityToHueFraction`). It is NOT the Halo State Pulse canvas, which has no production render site. The corrected duration must therefore still be computed webview-side (paint instant − `tier_effective_at`) and cross the bridge through the `ui-bridge` telemetry router (`crates/ui-bridge/src/telemetry.rs`).
- **Each leaf needs its own exact allowlist entry.** Per the same §Occupied Resources entry, each delegated-timing target carries its OWN exact obs allowlist leaf. A bare `metric` key exists with only `value`/`unit`/`module`, so a leaf-less `metric.*` target has every label silently redacted. Two consequences:
  - A corrected `duration_ms` on `metric.constellation.hue_update_ms` must stay inside that leaf's admitted set.
  - A sibling leaf needs its own exact leaf and its own §Occupied Resources registration line. The superseded leaf is recorded as retired or superseded, not silently dropped.
- **Adding a field changes the payload shape, not the procedure set.** `tier_effective_at` rides `ServiceListItem` inside `ServiceListPayload`, returned by `services.list_with_states` (pulse-app crate; §Occupied Resources). The shape change falls under:
  - §Cross-cutting Patterns → Cross-bridge data shape (must be `serde::Serialize`).
  - §Conventions → Workspace API style (the frontend consumes only TauRPC-generated `.ts` bindings; no manual TS re-declaration).
  - §Cross-cutting Patterns → Webview IPC capability policy: a NEW procedure would need router registration + an §Occupied Resources entry + an `EXPECTED_PROCEDURES` pin, staged together with the regenerated bindings. Whether a pure field addition leaves the procedure set and the staged-bindings verdict unchanged is research's question. The scope marks it `[inferred]`.
- **Dependency direction constrains where the fall instant is sourced.** Per §Cross-cutting Patterns → Module dependency direction and §Established Decisions [Module Boundaries], tier derivation lives at the `pulse-app` binary boundary (`services_router.rs`) over `triage` types.
  - A new registry-held last-transition instant, or a broadened `Incident` field, belongs in `triage`, never as a reverse edge.
  - Per §Established Decisions [Corpus Write Arbitration], `Incident` is a persisted bincode BLOB. Research must name the codec before any `Incident` shape change. Reusing an existing field (`updated_unix_nano` / `resolved_unix_nano`) or holding the instant in the registry avoids a wire-shape change.
- **The tier the rise/fall rule anchors to is the max-tier reduction.** Per §Established Decisions [Fault Identity] (L2 paragraph), the per-service constellation join reduces by max tier over all matching active incidents. Several concurrent incidents per service are N-safe by design. The rise/fall rule must follow that same max-over-active reduction, so the fall case is "the last incident holding the maximum leaves", not "any incident leaves".
- **Fall sources that miss a leave path.** The rule under §Constraints 5 above means every path by which an incident leaves the active set must be seen. Per §Established Decisions [Corpus Write Arbitration] (Residual CLOSED paragraph):
  - The persist-cycle reconciler resolves externally resolved incidents via `IncidentRegistry::mark_resolved` and deliberately emits NO `IncidentLifecycleEvent`.
  - Per §Occupied Resources → Tauri IPC events, `pulse://stream/incidents` is producer-only.

  So a fall instant sourced only from the broadcast event's `transitioned_at_unix_nano` would miss reconciler-driven falls. Whether every leave path is covered by the chosen source is research's question.
- **§B moves `ANDROMEDA_PULSE_DATA_DIR` without changing what it means.**
  - It is an arch-reserved env var (§Occupied Resources → Environment variables).
  - Per the §Occupied Resources → xtask CLI surfaces `scripts/agent-run.{sh,ps1}` entry, ci.yml's Linux-only "Boot pulse-app smoke" step is GATING. Its three invocations share ONE `ANDROMEDA_PULSE_DATA_DIR`, which that entry records as workflow-level at `ci.yml:12`.
  - The relocated placement must keep a single identical value across those three invocations: job-level `env:`, or a per-step derivation that resolves identically.
  - The CI shape itself (matrix Linux/macOS/Windows, `release.yml` on tag, `update-channels.yml` on release completion) is per §Infrastructure Patterns → CI/CD approach.

## Patterns to follow
- **Label for the tier, not an enum.** Use the bounded-enum telemetry input pattern: `ConstellationHueLatencyInput` carries a bounded `HueSeverityTier` (none/curious/suggested/autonomous), and its siblings validate aggregate counts with explicit bounds (§Occupied Resources → Tauri IPC routes, delegated-timing entry). If a new input field is needed, it carries a validated bound, never a service identity.
- **Regenerate bindings and stage them together.** Follow the regen discipline under §Cross-cutting Patterns → Webview IPC capability policy: a bindings shape change is regenerated, then staged together so the git-INDEX copy of `pulse-app/ui/src/bindings/index.ts` matches. The `capability-drift` gate reads the staged copy (the `check:staged-artifacts` entry under §Occupied Resources → xtask CLI surfaces).
- **Template crate shape.** Per §Project Intent → Template patterns, a new timestamp accessor on `triage` goes through the crate's public contract module (`triage::contract`), with `pub(crate)` internals, so `pulse-app` reads it without widening the crate surface. §Conventions → Module visibility discipline also applies.
- **Per-cycle outcome recording.** §Established Decisions [Corpus Write Arbitration] precedes every registry mutator (`mark_resolved` / `mark_read` / `attach_*`) with an `updated_at_unix_nano = now` bump. Research should check whether that already-present instant identifies the leave moment before introducing new state.

## Anti-patterns to avoid
- **A service identifier in the hue observable.** Do not add a service name or service id to the hue-latency input or its leaf. Service is an OTLP resource attribute, and a leaf-less or label-bearing `metric.*` target is redacted or cardinality-bound (§Occupied Resources delegated-timing entry). Per-changed-service emission is the only admissible shape.
- **A new TauRPC procedure or capability-JSON grant.** Do not add either for this observable. Per §Cross-cutting Patterns → Webview IPC capability policy, a new procedure triggers the full registration triple, and the scope's boundary says none is needed. If research finds one IS needed, it is an §Occupied Resources amendment, not a silent addition.
- **A reverse crate edge, or pinning the observable to the Halo canvas.**
  - `triage` must not depend on `pulse-app`, and `ui-bridge` must not depend on `triage` just to compute the instant (§Cross-cutting Patterns → Module dependency direction).
  - The observable must not be re-anchored to the Halo State Pulse canvas (no production render site; §Occupied Resources delegated-timing entry).

## Contract bindings
- **arch ↔ obs:** the `metric.constellation.hue_update_ms` target and its exact allowlist leaf (`pulse-app/src/observability.rs`). A field rename, new field or sibling leaf must be mirrored in the obs allowlist and in the §Occupied Resources route entry.
- **arch ↔ tests:**
  - The `ServiceListItem` shape change flows through TauRPC bindings regeneration.
  - `capability-drift` / `check:staged-artifacts` assert the staged bindings.
  - The allowlist unit test `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs` pins the leaf's admitted set.
- **arch ↔ security:** keeping service identity off the telemetry label set is shared with security-plan §Logging.
- **arch ↔ CI/infra:** §B edits the three workflow files §Infrastructure Patterns describes. The `agent-run` gating smoke step's shared `ANDROMEDA_PULSE_DATA_DIR` is recorded in §Occupied Resources → xtask CLI surfaces (the `ci.yml:12` workflow-level placement), so that registry text is owed an amendment at wrap once the placement moves.

## Acceptance criteria contributions
- **Tier-effective instant is serialized into the bindings.** `ServiceListItem` gains the tier-effective instant as a `serde`-serialized field visible in the regenerated TauRPC bindings. The webview consumes it only through the generated `.ts` types, with no hand-written re-declaration. (per architecture.md §Cross-cutting Patterns → Cross-bridge data shape; §Conventions → Workspace API style)
- **The procedure set is unchanged.**
  - `EXPECTED_PROCEDURES` has zero delta.
  - `cargo xtask capability-drift` exits 0, including the staged-bindings assertion, with the regenerated bindings staged.
  - `pulse-app/capabilities/*.json` is byte-identical.

  (per architecture.md §Cross-cutting Patterns → Webview IPC capability policy; §Occupied Resources → xtask CLI surfaces, `check:staged-artifacts`)
- **No reverse dependency edge.** No new edge from a library crate toward `pulse-app`, or between siblings, is introduced to source the tier-effective instant. The `Cargo.toml` dependency graph is unchanged or grows only along the existing DAG direction. (per architecture.md §Cross-cutting Patterns → Module dependency direction)
- **One `ANDROMEDA_PULSE_DATA_DIR` value in the Boot smoke step.** After the §B fix, all three `agent-run` invocations in ci.yml's gating "Boot pulse-app smoke" step resolve the same `ANDROMEDA_PULSE_DATA_DIR` value. No workflow-level `env:` references the `runner` context. (per architecture.md §Occupied Resources → xtask CLI surfaces, `scripts/agent-run.{sh,ps1}` entry; §Infrastructure Patterns → CI/CD approach)
