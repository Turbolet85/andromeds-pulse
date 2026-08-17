# Codebase Research — 2026-08-17-incident-fingerprint-producer-repaired

## Scope
- **Depth:** deep · **Reads:** 12 targeted (`sed`/`grep` sections, no blanket scans) · **Globs/Greps:** 14 · **Code-graph queries:** 2 (trace at `.andromeda/runs/2026-08-17T17-52-59Z-phase/tree-query-2026-08-17-incident-fingerprint-producer-repaired.json`)

## Headline findings

**1. The threading path exists end-to-end and the fingerprint is already in hand at the one site that matters.**
`record_occurrence(detector, fingerprint: [u8; 16], service, now_nanos)` (`crates/triage/src/pattern/storm.rs:206`)
holds the raw 16-byte fingerprint and calls `synthesize_cue(...)` (`:256`, `:276`) — which is **not** passed it
(`:293-320`). That single omission is the whole starvation. Downstream, `DigestCueRef` already carries a
structurally-threaded `scope`/`scope_id` added by chunk #92 for exactly this reason, and the producer reads
`digest.attention_cues.first()` for `(kind, scope, scope_id)` at `pulse-app/src/inference_runtime.rs:651`. The
repair is one more field down the same rail.

**2. THE TRAP — an 8-char helper sits next to a 32-char match target.** `triage` has its own private
`fingerprint_to_hex_prefix(&[u8;16]) -> String` at `crates/triage/src/pattern/storm.rs:450` that encodes only
**the first 4 bytes** (8 hex chars — it exists for bounded-cardinality *tracing*, `:445-452`). The value the
retrieval arm compares against is `hex_lower(&row.fingerprint)` (`crates/triage/src/digest/assembler.rs:287`,
helper at `:686-692`) over `Q3FingerprintRow.fingerprint: Vec<u8>` (`crates/triage/src/baseline/sql.rs:220-225`)
— the **full 16 bytes, 32 hex chars**. Reaching for the adjacent, correctly-named, already-imported helper
produces a value that can never match, silently re-starving the arm while every test that only asserts
"is lowercase hex" still passes. **The cue must carry full 32-char hex.** This is the chunk's primary
implementation risk and needs a length-pinned assertion, not a shape-only one.

## Files inspected
- `crates/triage/src/pattern/storm.rs` (1-20, 206-340, 440-460) — `record_occurrence` holds `[u8;16]`; `synthesize_cue` builds the cue without it; module doc `:13-18` states the deliberate no-`buffer`-dep posture ("receives opaque `[u8; 16]` fingerprint bytes … via the `FingerprintObserver` trait"); local 8-char hex helper at `:450`.
- `crates/triage/src/contract.rs` (281-310, 328-350, 449-467, 548-570) — `AttentionCue` (9 fields, `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]` `:284`); `Incident.fingerprint: String` documented as *"Anonymized fingerprint hash for cross-incident grouping per capability spec P-047"* `:340-342`; `DigestCueRef` `:449` with the chunk-#92 `#[serde(default)]` back-compat precedent on `scope`/`scope_id` `:458-466`; the production cue→`DigestCueRef` map inside `scrubbed_clone` `:558-565`.
- `crates/triage/src/digest/retrieval.rs` (1-175) — `select_corpus_matches` `:88` with the in-code guard *"The mismatch is the producer's; do not 'simplify' this arm away"* `:76-86`; `select_previously_seen` `:111`; `format_corpus_match_line` `:136` embeds `incident.fingerprint` verbatim into the digest line.
- `crates/triage/src/digest/assembler.rs` (262-300, 686-692) — `DigestCueRef` build `:262-272`; `current_fingerprints` = `hex_lower` of Q3 bytes `:283-288`; `hex_lower` private `:686`.
- `crates/buffer/src/fingerprint.rs` (37, 79, 95-115) — `pub type ExceptionFingerprint = [u8; 16]` `:37` (a plain type alias, so **no `buffer` dep is needed to carry it**); buffer's own 8-char `fingerprint_to_hex_prefix` `:102`.
- `pulse-app/src/inference_runtime.rs` (640-730) — the producer: identity derivation from the first cue `:645-654`, the coalesce predicate with its DECIDED-semantic comment `:661-673`, and the defect `fingerprint: parsed.fingerprint.clone()` `:700`.
- `pulse-app/src/incidents_router.rs` (440-480) — the P-036 report path calling `select_previously_seen` `:461`.
- `crates/interpretation/src/markdown.rs` (59-80) — `PreviouslySeenMatch { incident_id, opened_at_unix_nano, title, workspace }` — **carries no fingerprint**.
- `pulse-app/tests/unit_incident_producer.rs` (1-130, 355-540) — the pinned producer suite (730 lines); `digest_with_cue` helper `:101` builds a `DigestCueRef` literal `:111`; pins at `:361` (coalesce), `:445` (PII canary), `:482` (aggregate-only observability); `reflection_digest` `:538`.
- `crates/triage/Cargo.toml` (1-40) — dependency list; **no `buffer` entry**.
- `pulse-app/ui/src/bindings/index.ts` — `fingerprint` appears **only in comments** (`:129`, `:276`); the webview `IncidentRecord` view genuinely drops it.
- `crates/triage/src/baseline/sql.rs` (220-225) — `Q3FingerprintRow.fingerprint: Vec<u8>`.

## Graph impact (code-graph query, 84 distinct sites over `AttentionCue` / `DigestCueRef` / `synthesize_cue` / `record_occurrence`)
- **`crates/triage/src/pattern/storm.rs::synthesize_cue`** — 2 callers, both internal to `record_occurrence` (`:256` Autonomous arm, `:276` Suggested arm). Private fn, zero external blast radius: the signature can change freely.
- **`crates/triage/src/pattern/storm.rs::record_occurrence`** — callers at `crates/triage/src/pattern/mod.rs:42` (re-export), `storm.rs:332` (`observe_and_dispatch_storm`, which **also already holds the fingerprint and hex-encodes it at `:340`**), plus 16 in-crate test sites (`:488`–`:633`) and 3 pulse-app test sites (`pulse-app/tests/unit_storm_persistence.rs:53,161`; `pulse-app/tests/e2e_pii_scrubber_persistence_coverage.rs:144`). Signature is unchanged by this work — the fingerprint is already a parameter.
- **`AttentionCue` struct-literal sites — 9 (Rust requires every field named, so a new field touches all):** production `crates/triage/src/cue/evaluate.rs:63`, `:129`, `:186` (three baseline-derived families — **no fingerprint available at any of them**) and `crates/triage/src/pattern/storm.rs:309` (the only fingerprint-bearing producer); test-only `crates/triage/src/cadence/broadcast.rs:205`, `crates/triage/src/cadence/coordinator.rs:571`, `crates/triage/src/contract.rs:589`, `crates/triage/src/cue/broadcast.rs:94`, `crates/triage/src/pattern/suppression.rs:223` and `:432`. **⇒ the field must be `Option<String>`.**
- **`DigestCueRef` struct-literal sites — 8:** production `crates/triage/src/contract.rs:558` (`scrubbed_clone`) and `crates/triage/src/digest/assembler.rs:262`; test-only `crates/triage/src/contract.rs:643`, `pulse-app/tests/unit_incident_producer.rs:111`, `pulse-app/tests/integration_tier1_storm_one_incident.rs:90`, `pulse-app/tests/integration_deterministic_l4_mode.rs:80`, `pulse-app/tests/integration_incident_producer_persists_across_restart.rs:37`, `pulse-app/tests/integration_constellation_severity_workspace_key.rs:135`.
- **`crate_edges`** — inbound to `buffer`: `mcp-server`, `pulse-app`, `ui-bridge`. **No `triage → buffer` edge**, and none is needed: `ExceptionFingerprint` is a `[u8; 16]` alias and `triage` already receives the bytes opaquely. Creating one would be an unforced new sibling edge.

## Patterns detected
- **Structural cue-field threading with BLOB back-compat** (`crates/triage/src/contract.rs:454-466`): chunk #92 added `scope` + `scope_id` to `DigestCueRef` with `#[serde(default)]` explicitly so *"`#[serde(default)]` keeps pre-chunk-#92 archived digest BLOBs deserializable."* This chunk's field is the same shape and takes the same treatment — the precedent needed no `SCHEMA_VERSION` bump.
- **Deliberate helper duplication over a crate edge** (`crates/triage/src/pattern/storm.rs:13-18, 450`): triage re-implements the 8-char hex helper rather than depending on `buffer`. Follow the posture (no new edge), not the helper (wrong width).
- **Egress scrub at the cue→digest projection** (`crates/triage/src/contract.rs:561-564`): `summary` and `scope_id` pass through the injected `scrub` closure. A hex digest is construction-exempt (no PII surface), but the new field sits in that literal and its treatment must be a stated decision, not an omission.
- **Empty-guarded match arms** (`crates/triage/src/digest/retrieval.rs:96`, `:120`): both selectors test `!fingerprint.is_empty()` before comparing, so an absent fingerprint degrades to scope-only matching — the reflection/baseline path needs no special casing.
- **pulse-app probes as integration targets** (`pulse-app/tests/unit_incident_producer.rs`): the producer suite already lives in the sanctioned `pulse-app/tests/*.rs` location per the 2026-08-14 test-location exception — new pins extend this file rather than creating a src-level `mod tests`.

## Conventions to follow
- **`Option<String>` for optionally-available cue metadata**, mirroring `AttentionCue.scope_id: Option<String>` (`crates/triage/src/contract.rs:292`) — the same "not every cue family has one" situation.
- **`#[serde(default)]` on every field added to a BLOB-archived contract type** (`crates/triage/src/contract.rs:458`, `:465`).
- **Full-width lowercase hex via the `hex_lower` encoding** (`crates/triage/src/digest/assembler.rs:686-692`, pinned by `hex_lower_encodes_q3_fingerprint_bytes` `:1050`) — the cue-side encoder must produce byte-identical output for the same bytes.
- **Doc-comment the *why* on threaded contract fields** — chunk #92's `scope`/`scope_id` comments explain what the producer does with them; the new field carries the same burden (it is the field a future reader would otherwise "simplify").

## New files to create
- (none) — every change lands in existing files.

## Files to modify
- `crates/triage/src/contract.rs` — add the fingerprint field to `AttentionCue` (`:285`) and `DigestCueRef` (`:449`, `#[serde(default)]`); thread it in the `scrubbed_clone` projection (`:558`); update the `:589` + `:643` test fixtures.
- `crates/triage/src/pattern/storm.rs` — pass the fingerprint into `synthesize_cue` (`:256`, `:276`, `:293`) and populate the cue (`:309`); supply a full-32-char hex encoder (the local `:450` helper is 8-char and must NOT be reused for this).
- `crates/triage/src/cue/evaluate.rs` — three baseline-family literals (`:63`, `:129`, `:186`) get `None`.
- `crates/triage/src/digest/assembler.rs` — the `DigestCueRef` build (`:262-272`) carries the field through.
- `crates/triage/src/cadence/broadcast.rs:205` · `crates/triage/src/cadence/coordinator.rs:571` · `crates/triage/src/cue/broadcast.rs:94` · `crates/triage/src/pattern/suppression.rs:223,:432` — test-fixture literal updates (mechanical).
- `pulse-app/src/inference_runtime.rs` — the producer (`:700`) reads the cue's fingerprint instead of `parsed.fingerprint`; the `:661-673` DECIDED-semantic comment needs updating (its stated rationale *"the cue does not carry one"* becomes false, while the decision itself stands).
- `pulse-app/tests/unit_incident_producer.rs` — `digest_with_cue` literal (`:111`) + new pins; the `:361` coalesce pin must stay semantically intact.
- `pulse-app/tests/integration_tier1_storm_one_incident.rs:90` · `integration_deterministic_l4_mode.rs:80` · `integration_incident_producer_persists_across_restart.rs:37` · `integration_constellation_severity_workspace_key.rs:135` — `DigestCueRef` literal updates (mechanical).
- `pulse-app/src/observability.rs` — **only if** the producer's emitted field set changes (obs §8 leaf completeness); today `interpretation.incident.created` enumerates exactly 4 fields and none is a fingerprint.

**Not touched:** `pulse-app/ui/**` (the field never reaches the TS surface), `crates/interpretation/src/markdown.rs` (`PreviouslySeenMatch` carries no fingerprint), corpus DDL / `SCHEMA_VERSION`, `crates/triage/src/digest/retrieval.rs` (the arm is already correct — this chunk feeds it, it does not edit it).

## Open questions
- **Encoder placement for the 32-char hex** — reuse `assembler.rs::hex_lower` (private; would need promotion within the crate) vs a new `pattern/storm.rs`-local full-width encoder vs widening the existing 8-char helper (rejected: it has a live tracing caller at `:340` that wants 8 chars). → blocks: **plan-decision** (P4 picks one before synthesis).
- **Does the producer's `Incident.fingerprint` get scrubbed on the way in?** Today `:700` is unscrubbed while `title`/`detail` use `scrub_text`. A hex digest is construction-exempt, but the plan must state the disposition explicitly rather than inherit it. → blocks: **plan-decision**.

## Scope premise closure
Re-read `scope.md`'s five `[inferred]` bullets against the above:
1. *Layer-2 pin must still pass unchanged* → **[premise-corrected]** — it must pass **semantically** unchanged, but `pulse-app/tests/unit_incident_producer.rs` changes mechanically because its `digest_with_cue` helper builds a `DigestCueRef` literal (`:111`) and Rust requires every field named. "Do not touch the file" would be the wrong reading.
2. *No corpus DDL / `SCHEMA_VERSION` change* → **VERIFIED** — `Incident.fingerprint` stays `String` inside the bincode `payload` BLOB; the chunk-#92 `#[serde(default)]` precedent covers the `DigestCueRef` addition with no version bump.
3. *No new TauRPC procedure / MCP tool / port / env var* → **VERIFIED** — the field is absent from `bindings/index.ts` except in prose; no new surface.
4. *Repairing the producer closes the `select_previously_seen` deterministic-constant consequence* → **VERIFIED** — storm-derived incidents gain per-fault hashes so matching becomes fault-scoped; reflection/baseline incidents carry `None` → empty → both arms' `!is_empty()` guards drop them to scope-only matching.
5. *No capability becomes fully verifiable here* → **VERIFIED** — P-075 / P-076 / P-077 remain `chunk:null`; this chunk is a partial advance toward P-075/P-076.
