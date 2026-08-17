# Codebase Research — 2026-08-16-fault-identity-semantics-decided

## Scope
- **Depth:** deep · **Reads:** 8 · **Globs/Greps:** 10 · **Code-graph queries:** 4, all `db_state=fresh`, rows 5 / 41 / 43 / 32 (trace: `.andromeda/runs/2026-08-16T19-21-36Z-phase/tree-query-2026-08-16-fault-identity-semantics-decided.json` — the trace's `rows` field is the authority for every count cited below)

## Headline finding — the chunk's premise needs correcting

**There are TWO unrelated "fingerprint" namespaces in this codebase, and they do not meet.** The
working entry (and `scope.md` as promoted) reads as though "fingerprint normalization" and "incident
dedupe" range over one identity. They do not:

| | **Namespace A — `ExceptionFingerprint`** | **Namespace B — `L4Output.fingerprint`** |
|---|---|---|
| Defined | `crates/buffer/src/fingerprint.rs:37` | `crates/interpretation/src/schema.rs:170` |
| Type | `[u8; 16]` blake3 digest | `String`, 1–256 chars |
| Produced by | `compute_exception_fingerprint` over `(exception.type + normalized 3-frame stack)` | the **LLM**, free-form |
| Guidance | deterministic code (**this is what Layer 1 governs**) | one schema line: *"Content-stable identity used for cross-incident deduplication."* |
| Lands in | `span_events.fingerprint` BLOB + storm detector | `Incident.fingerprint` (`inference_runtime.rs:687`) |

`crates/interpretation/src/prompt.rs` contains **zero** occurrences of "fingerprint" — the prompt
never feeds the model the real digests and never tells it what to emit. So namespace B is whatever
the model invents (`schema.rs:366` fixture: `"db-saturation:service-a"`; the deterministic runner
hardcodes `"deterministic-l4-fixture"`, `deterministic_inference.rs:36`).

**Consequence — a live dead branch.** `select_corpus_matches` (`digest/retrieval.rs:86-94`) compares
`c.fingerprint` (namespace B) against `current_fingerprints`, which the assembler builds as
`hex_lower(&row.fingerprint)` over Q3 rows (namespace A, `assembler.rs:283-288`). An LLM would have
to emit a 32-char lowercase hex string exactly equal to a blake3 digest. **The `fingerprint_match`
arm is therefore effectively dead in production; only `scope_match` fires.** Its unit tests pass
because they supply symbolic strings on *both* sides (`retrieval.rs:176, 211, 218`) — self-consistent,
but not the production shape. (`select_previously_seen`, `retrieval.rs:112-115`, compares B-to-B and
IS live.)

## Files inspected
- `crates/buffer/src/fingerprint.rs` (full, 1–254) — Layer 1. `is_absolute_path_start:186` fires on any `/` + path char; `is_path_char:216-218` **includes `/` and `\`**, so `skip_absolute_path` consumes the whole remaining token. Only the leading segment survives. Module doc `:28` and fn doc `:134-137` both claim absolute-only — impl and docs conflict, as the entry states.
- `pulse-app/src/inference_runtime.rs` (620–757) — Layer 2. Dedupe predicate `:664` over `list_active(&digest.workspace)`; identity tuple derived at `:648-654` **from the cue**; `Incident.fingerprint` set at `:687` but **not in the key**; outcome emitted at `:735-757`.
- `crates/triage/src/pattern/storm.rs` (150–320) — `record_occurrence` keys detector state `DashMap<fingerprint, StormState>` (`.entry(fingerprint)`), but `synthesize_cue:309-319` **hardcodes `kind: RetryStorm, scope: Service, scope_id: Some(service)` and carries no fingerprint**.
- `crates/triage/src/contract.rs` (`AttentionCue` struct) — 9 fields, **no fingerprint field**. The digest's `DigestCueRef` (`assembler.rs:263-273`) likewise carries only kind/tier/scope/scope_id.
- `crates/triage/src/digest/retrieval.rs` (60–150) — the two match selectors above.
- `crates/triage/src/digest/assembler.rs` (210–305, 684–692) — `list_active` consumed **only as a boolean** (`has_critical_active = …any(…)`, `:221-224`) to pick the LWW mode.
- `pulse-app/src/services_router.rs` (80–124) — the per-service severity join.
- `crates/triage/src/incident/persistence.rs:41` + `state_machine.rs:44` — `DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS = 120`, with a test pinning it to spec P-022 (`persistence.rs:377`).

## Graph impact
- **`normalize_stacktrace` / `normalize_frame` / `is_absolute_path_start`** — 5 callers, **all inside `crates/buffer/src/fingerprint.rs`** (3 production at `:86,:124,:150`, 2 tests). Layer 1's normalization chain has **zero cross-file blast radius**.
- **`compute_exception_fingerprint` / `ExceptionFingerprint` / `FingerprintObserver`** — cross-file refs land in exactly 6 files (Q3, 43 rows): `buffer/appender.rs` (compute + column write + observer dispatch), `buffer/consumer.rs` (plumbing), `pulse-app/src/storm_observer.rs` (adapter → detector), `pulse-app/src/main.rs:648` (boot wiring), plus `pulse-app/tests/e2e_{storm_detection,drain_template_assignment}.rs`. **No viz, MCP, corpus, or UI consumer** — answering scope research Q3: namespace A never leaves the ingest→storm path.
- **`create_incident_from_l4_output`** — 1 production caller (`inference_runtime.rs:157`, the L4 subscriber) and **~20 test call sites across 5 files**, densest in `pulse-app/tests/unit_incident_producer.rs` — which already contains `reemission_dedup_does_not_create_duplicate()` (`:321,:329`), `distinct_services_create_distinct_incidents()` (`:361,:368`) and `reflection_dedups_one_per_workspace()` (`:555,:562`). **The current Layer-2 semantic is already pinned by tests**; changing it means updating them, not just the predicate.
- `grep` for the `span_events.fingerprint` column in `crates/viz/` + `crates/mcp-server/` returns **no SELECT of that column**; the MCP `fingerprint_refs` (`tools.rs:436`) reads `incident.evidence_refs.fingerprint_hashes` — a third, separate string list.

## Patterns detected
- **Cue-derived incident identity** (`inference_runtime.rs:648-654`): identity comes from the *triggering cue*, with a synthetic `(ReflectionTrend, Global, None)` for reflection digests. Any fingerprint-aware key must therefore reach the incident site **through the cue**.
- **Max-reduction severity join** (`services_router.rs:97-104`): `filter(scope == Service && scope_id == service).map(priority_tier).max_by_key(tier_rank)`. Already correct for N concurrent incidents per service.
- **Boolean-only active-incident read** (`assembler.rs:221-224`): the digest only asks *"is any active incident ≥ Suggested?"*.
- **Per-fingerprint detector state, per-service cue** (`storm.rs`): the detector distinguishes fingerprints internally, then discards the distinction when it synthesizes the cue.

## Conventions to follow
- **Layer-2 probes go in `pulse-app/tests/*.rs`**, never a co-located `mod tests` — `[lib] test = false` (tests extract; `unit_incident_producer.rs` is the established host file).
- **Layer-1 tests stay co-located** in `crates/buffer/src/fingerprint.rs` `mod tests` (`:238`), where the existing normalization tests live (`:369`, `:375`).
- **`AttentionCue` field additions** are a `triage::contract` change — the contract module is the crate's only `pub` surface (arch extract).
- **Aggregate-only obs on these paths** — the incident-outcome event (`inference_runtime.rs:743-756`) emits bounded labels only; no `scope_id`, no fingerprint.

## Obs readability of the decision evidence — resolved (the obs extract flagged this)
The obs extract warned that obs-plan §8 enumerates **no** leaf for the incident-outcome emit site,
so the `created`/`deduped` evidence this chunk leans on might be redacted. **It is not.** The emit
site's target is `interpretation.incident.created` (`inference_runtime.rs:86,744`), and
`AllowList::production()` carries an **exact leaf** for it at `observability.rs:2058-2063` permitting
`["created","deduped","severity","priority_tier"]` — precisely the 4 fields the site emits
(`:743-750`). The sibling counter `metric.pipeline.l4.incidents_created_total` has its own leaf at
`:2070`. **The evidence is readable in production today.**

Two consequences: (a) the gap is in **obs-plan §8's enumeration**, not in the code — a
registry-completeness drift finding for wrap (routine per obs's own 2026-08-16 precedent, since the
fields are bounded: 2 bools + 2 static labels); (b) **if Layer 2 adds any field to that emit site**
(e.g. a dedupe-key label), the leaf must be extended in the same change, with the guard under
`pulse-app/tests/` — never a src-level `mod tests`, which cannot run.

## Open questions
- **Which fingerprint should Layer 2 key on, if any?** → blocks: **plan-decision**. Namespace B is LLM-authored and is a **constant** (`"deterministic-l4-fixture"`) under `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` — the mode chosen for e2e verification *because* it is reproducible — so keying on it would be a **no-op in exactly that mode**, reproducing the vacuous-verification failure class the route already records against P-075. Namespace A is real and deterministic but is **not available at the incident site** (dropped at `synthesize_cue`), so using it requires threading a fingerprint field through `AttentionCue` → `DigestCueRef` → the digest → the producer. P4 must put this to the operator.
- **Does the dead `fingerprint_match` arm get fixed here or handed off?** → blocks: **plan-decision**. It is the same truth-in-diagnostics class the "Diagnostics un-muting + harness-truth sweep" route entry owns, but it is *this* chunk's subject matter (fault identity). Operator's call whether it lands here or is handed to that entry.

## Scope premise closure
Applied to `scope.md` before P4 (see that file's amended bullets):
1. `[inferred]` — *"the canary deduped because its storm carried the same `(kind, scope, scope_id)` tuple"* → **VERIFIED**. `synthesize_cue` (`storm.rs:309-319`) hardcodes `kind`/`scope` and sets `scope_id = service`, so **every** storm cue from one service is tuple-identical regardless of fingerprint. Tag dropped.
2. *"the ~5min auto-resolve window"* (from the working entry) → **FALSIFIED**. `DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS = 120` (2 minutes), pinned to spec P-022 by `persistence.rs:377`. Corrected in scope.
3. *"fingerprint normalization and incident dedupe agree on when two faults are one"* (the entry's own framing) → **PREMISE CORRECTED**. The two layers do not range over one identity; namespace A never reaches the incident path and namespace B never reaches the storm path. Corrected in scope and surfaced to the operator at P4.
