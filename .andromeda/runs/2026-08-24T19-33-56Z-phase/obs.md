# obs extract

## Relevance
Partial — the probe/driver mechanics and capability-JSON arms are outside obs, but every landed stage's `obs` verdict half, any new emission the drag/resize/Half-B guard needs, and its allowlist registration are squarely obs-owned.

## Constraints
- Any NEW obs target this chunk introduces (a drag or resize verdict record; a Half-B re-navigate guard record) requires its OWN EXACT allowlist leaf enumerating EVERY field its emit site emits — obs-plan §8 "Default-deny posture" states `for_target`'s prefix fallback otherwise resolves a dotted target to an unrelated field set and redacts the fields, and §8 records that a partly-enumerated leaf leaves the target PARTLY redacted. Whether any such record exists at HEAD is research's question (per obs-plan §8).
- The every-time-toast repetition proof (CARRY 5) rests on obs-plan §8's exact leaf `tray.signpost.shown` → `window_label` ONLY, bounded by `sanitize_window_label` (4-label set + `unknown`), with **no bare `tray` key** permitted; §8 requires the harness match the record BY that field rather than by record presence. Whether two consecutive closes each produce a resolvable record is research's question (per obs-plan §8, `tray.signpost.shown` leaf).
- Window-event streams are hot paths: obs-plan §11 "Logs" bars `info`-level emission in hot paths and §11 "Telemetry Strategy" bars over-instrumenting them. Any obs half attached to `Moved`/`Resized` must ride the settled/debounced boundary (the scope notes a 150ms `ASPECT_DEBOUNCE`) or be level-gated, never per-event (per obs-plan §11 Logs / Telemetry Strategy).
- Emitted fields must stay bounded and PII-clean: obs-plan §8's tray leaf bans window content, title and coordinates; §8 Data classification + §11 "PII Scrubbing" ban full paths (Vector 3) and raw parameters (Vector 5). Half B's "per-webview URLs" are values that would need classification and a leaf before any in-process emission (per obs-plan §8 Data classification rules).
- Every line the harness reads must satisfy obs-plan §6 "Required fields" (timestamp / level / target / message / fields) in the §3 "Log format JSON schema", with service identity as default subscriber fields — §3 states this schema is verbatim from the test harness contract and that obs aligns TO tests, not vice versa (per obs-plan §3 Log format JSON schema, §6 Required fields).
- If path (b) ships a guard, obs-plan §6 "Log levels mapping" (`warn` row) requires degraded/fallback conditions be a once-per-boot (or once-per-window) WARN carrying bounded static `reason` / `consequence`-shaped fields, registered at BOTH §6 and §8 — §8's sibling entries are stated at both sites (per obs-plan §6 warn row, §8 whitelist).
- A headful booted run puts obs-plan §10's frame-budget check in its ACTIVE state and its `app.panic.fatal` zero-unlogged-panics invariant in force; §10 also requires any xtask check script over `agent-latest.jsonl` be NEUTRAL-tolerant and run-window scoped (per obs-plan §10 Performance budgets / Always-required SLO invariant / Load-profile constraints).

## Patterns to follow
- The predecessor's own shape, recorded in obs-plan §8: give a previously-fieldless emission ONE bounded field, register an EXACT leaf, and let the harness assert the field — this is the template for a drag or resize verdict record if one is needed.
- Allowlist guards live under `pulse-app/tests/`, never in `observability.rs`'s own `mod tests` — obs-plan §8 (`triage.baseline.bootstrap_window.override` entry) states `[lib] test = false` compiles but never runs a src-level guard, and §8's `interpretation.incident.created` entry records a live instance of exactly that dead guard.
- Mutation-checked leaf guards: obs-plan §8 records the close-signpost and delegated-timing guards as leaf-neutralized-RED → restored-GREEN. This is the same discriminate-the-pin discipline the chunk's RED mutation arms already require, applied to the obs side.
- Bounded-label sanitizer at the emit site (`sanitize_window_label` per obs-plan §8) is the established way to keep a window-identifying field bounded — reuse it rather than emitting a raw label, title, or URL.
- Once-per-boot WARN with bounded static `reason` + `consequence` (obs-plan §6 warn row: `app.boot.buffer.degraded`, `corpus.keychain.fallback`) is the registered shape for "a degraded boot is otherwise indistinguishable from a healthy one" — the exact condition a blank-victim webview presents in Half B.

## Anti-patterns to avoid
- Per-event `info` emission on `Moved` / `Resized` during a live drag or resize (obs-plan §11 Logs: never log in hot path at `info`; §11 Telemetry Strategy: never over-instrument hot paths).
- A new target with no leaf, a partly-enumerated leaf, or a bare prefix key — obs-plan §8 makes this silently redact the very field a stage asserts, producing exactly the vacuous obs half the chunk bans.
- Emitting window coordinates, window content/title, or a full path/URL as a field (obs-plan §8 tray leaf ban + §11 PII Scrubbing / Logs Vector 3).

## Contract bindings
- **obs ↔ tests harness:** obs-plan §3 "Log format JSON schema" is verbatim from the test harness contract, so the `StageHalves{obs, dom}` obs reader and the driver's `SIGNPOST_TARGET` consume obs's format; §8's leaf resolution is what makes the obs half a FIELD assertion rather than a presence check.
- **obs ↔ test-plan §6 boundary text:** Half B's durable wording is a test-plan surface, but which path lands decides whether an obs amendment is owed (a new guard target ⇒ §6 + §8 registration; a pure out-of-process measurement ⇒ none).
- **obs ↔ security:** obs-plan §8 Data classification carries security Vectors 3 (basename-only paths) and 5 (identifier + count, never parameters); these govern any URL/path field Half B would emit.
- **obs ↔ CI:** the chunk does not CI-wire the leg, but obs-plan §10's NEUTRAL-tolerant + run-window-scoped script rule still governs any check script run over the log the headful boot produces.

## Acceptance criteria contributions
- Every obs target this chunk newly emits resolves to an EXACT allowlist leaf enumerating ALL fields its emit site emits, with no bare prefix key, and its guard lives under `pulse-app/tests/` and is mutation-checked (per obs-plan §8 Default-deny posture / whitelist per module).
- The obs half of each landed stage matches a bounded FIELD on a resolvable record — for the repetition proof, `window_label` on `tray.signpost.shown` on each of the two closes — not the target alone (per obs-plan §8 `tray.signpost.shown` leaf; §6 Required fields).
- No emitted field carries window coordinates, window content/title, a full path, or a full URL; drag/resize verdict data, if emitted at all, is bounded numeric or a bounded static label (per obs-plan §8 Data classification rules / §11 PII Scrubbing).
- Any Half-B guard emission fires at WARN once per affected window/boot with bounded static fields and is registered at BOTH §6's warn row and §8's whitelist (per obs-plan §6 Log levels mapping / §8 Default-deny posture).

## Relevant amendment history
- **2026-08-23-headful-leg-extension** — registered the exact `tray.signpost.shown` → `window_label` leaf (no bare `tray` key; guard `unit_observability_allowlist_close_signpost.rs`, mutation-checked RED 3/3 → GREEN). Directly upstream of CARRY 5: it exists so the harness can assert a FIELD rather than record presence, which is the mechanism the repetition proof rides.
- **2026-08-16-baseline-family-reachability** — established the dual-site rule (a once-per-boot WARN is stated in BOTH §6 and §8; a §8-only apply leaves §6 stale) and the "neither a bare `triage` nor a bare `triage.baseline` key" precedent. Governs a Half-B guard WARN.
- **2026-08-15-tier-1-incident-path-investigation** — field-completeness precedent ("all three fields, not two" — a short leaf leaves the target partly redacted) plus the census discipline that keeps measured-but-unfixed gaps visible.
- **2026-08-16-fault-identity-semantics-decided** — recorded a live dead guard sitting in `observability.rs`'s `mod tests` under `[lib] test = false`; the reason any new guard this chunk writes belongs under `pulse-app/tests/`.
- **2026-08-14-workspace-key-alignment / 2026-08-14-fingerprint-feed-capture-repair** — both record that an `app`-prefixed dotted target without its own leaf falls back to an unrelated field set and gets redacted; the same failure mode any new `window.*`-style Half-B target would hit.
- **2026-06-10 (chunk #99 tag gate)** — folded the frame-p99 two-state posture (NEUTRAL headless / ACTIVE on a booted app), NEUTRAL-tolerant check scripts, and `write_run_window_log` run-window scoping into §10. Relevant because this chunk's probe boots a real headful window with a live webview.
