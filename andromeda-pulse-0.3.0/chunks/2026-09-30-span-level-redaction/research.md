# Codebase Research — 2026-09-30-span-level-redaction

## Scope
- **Depth:** deep · **Reads:** 22 · **Globs/Greps:** 24 (+1 read-only sweep agent over 33 test files + two whole-tree greps)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (153 lines, 57.4 KB; Session Additions 2026-05-03 → 2026-09-29, 21 entries); applied: 2026-08-23 (`agent-run.sh run` is not an OTLP send; producer built outside the timed section and run by path; ONE exported data dir per leg; glob the date-suffixed log family), 2026-08-30 (MSYS converts exported path env), 2026-05-19 (Windows teardown by specific PID via PowerShell), 2026-06-29 (glob `agent-latest.jsonl*`). Also `.claude/rules/testing.md` (auto-loaded) and `.claude/rules/observability.md` (auto-loaded).
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a's CI read is `in progress`, not red) and no CI-reading entry outside the operator leg.

## Files inspected
- `crates/security/src/scrubber.rs` (1-215) — `ScrubbedValue { Allowed(String), Redacted { category: &'static str } }` at :17-21 with `is_redacted()` / `category()`; `scrub_attribute` :41-48 is first-match-wins over `patterns()` and returns a BOOLEAN verdict per arm (`ArmPredicate = fn(&Regex,&str) -> bool`, :51) — no span is computed anywhere today. The catalog order (:119-205): jwt · bearer · api_key · secret_kv · provider_key · email · credit_card · ssn. `credit_card` matches `\b[0-9]+(?:[ \-][0-9]+)*\b` candidate runs, then `has_luhn_valid_window` over consecutive WHOLE groups (:60-90).
- `crates/security/Cargo.toml` — deps `serde`, `thiserror`, `tracing`, `regex`; dev `proptest`, `rstest`. No change needed for a span API.
- `crates/buffer/src/appender.rs` (110-140, 350-460, 505-540) — `scrub_otlp_field` :361-369 is the ring-buffer write-boundary site (whole value → `[REDACTED:{category}]`, `*redactions += 1` once per redacted VALUE); `extract_service_name` :118-138 funnels `service.name` through it (the three-consumer choke point); `encode_labels` :428-459 scrubs the JOINED `key=value` ∪ the bare value and rebuilds `{key}=[REDACTED:{category}]`; the span-event loop computes `compute_exception_fingerprint` from the RAW `exception.type` + `exception.stacktrace` BEFORE the scrub (:506-516).
- `crates/buffer/src/drain.rs` (362-385, 622-645) — two whole-value sites over the joined template text (`snapshot_state`, `write_template_to_table`), `[REDACTED:{category}]`; the template is re-split on whitespace after scrubbing.
- `crates/interpretation/src/markdown.rs` (325-342 + callers :76-320) — `scrub_string` (`[redacted: {category}]`) wraps every Report text field field-by-field (title, workspace, symptom, timeline, hypotheses, steps, evidence refs, `fp:` refs).
- `pulse-app/src/digest_runtime.rs` (40-70) — `pii_scrub_closure` (`[redacted:{category}]`) is the closure `build_assembler` injects; `crates/triage/src/digest/assembler.rs:480-481` applies it through `Digest::scrubbed_clone` (`crates/triage/src/contract.rs:585-625`: `payload_summary`, each `services[].service`, each `attention_cues[].summary`).
- `pulse-app/src/inference_runtime.rs` (640-690, 822, 861-884) — `scrubbed_l4_json` serializes the WHOLE `L4Output` to compact JSON and scrubs the string (`[redacted: {category}]`), feeding `resolution_summary_text` at three writers; `scrub_text` wraps `title` / `detail`.
- `pulse-app/src/investigate_router.rs` (105-120) — `scrub` at the resolver egress (`[redacted: {category}]`).
- `pulse-app/src/training_export.rs` (80-95, 120-150, 165-180) — `scrub_string` (`"[redacted:" + " {category}]"`) at the egress; `count_redactions` counts a field only when it `starts_with(REDACTED_PREFIX)`.
- `pulse-app/src/{baseline,lifecycle,storm}_persistence.rs` (scrub fns at :108 / :102 / :87) — service-key scrubs before corpus persistence (`[REDACTED:{category}]`), all over keys already produced by `extract_service_name`.
- `pulse-app/tests/unit_digest_runtime_scrub.rs` (full) — the measured pin at :35.
- `docs/v0_2_0/capability-verification-matrix.json` — P-048's only scenario is `{kind: nextest-file, ref: crates/buffer/src/appender.rs, contains: "P-048"}`.
- `crates/ingest/examples/inject_scrub_canaries.rs` (1-40) — the live-leg producer sends ONE whole-value `provider_key` canary per client-controlled column plus a control value.
- `pulse-app/tests/e2e_pii_bare_credential_stored_field.rs` (1-60, fn index) — the real-OTLP → DuckDB stored-field precedent (ephemeral loopback port, `run_consumer`, reads the column back, never echoes the stored value).
- `.andromeda/playbook.md` (:113-116) — the `verdict: escalate` pattern **Boundary widening**.
- `crates/{snapshot,curation,viz,ui-bridge,mcp-server}/Cargo.toml` — none depends on `security`.

## Graph impact (from the code-graph query; rust plane, db_state `fresh`)
- **scrub_attribute** — 30 caller rows (trace `tree-query-2026-09-30-span-level-redaction.json`, `rows: 30`); production callers, editor lines: `scrub_otlp_field @ crates/buffer/src/appender.rs:362` · `encode_labels @ appender.rs:438, :440` · `snapshot_state @ crates/buffer/src/drain.rs:375` · `write_template_to_table @ drain.rs:635` · `scrub_string @ crates/interpretation/src/markdown.rs:337` · `pii_scrub_closure @ pulse-app/src/digest_runtime.rs:55` · `scrubbed_l4_json @ pulse-app/src/inference_runtime.rs:667` · `scrub_text @ inference_runtime.rs:677` · `scrub @ pulse-app/src/investigate_router.rs:114` · `scrub_string @ pulse-app/src/training_export.rs:90` · `scrub_service_key @ pulse-app/src/baseline_persistence.rs:109` · `scrub_service_name @ pulse-app/src/lifecycle_persistence.rs:103` · `scrub_fingerprint_service @ pulse-app/src/storm_persistence.rs:88`. The rest are the scrubber's own tests. Matches the grep basis `grep -rn "scrub_attribute|ScrubbedValue" --include=*.rs crates pulse-app/src` exactly.
- **crate edges** — `security` is depended on by `buffer`, `interpretation`, `corpus` (doc mention only) and `pulse-app`; `triage` has NO `security` dep and receives the scrub as an injected closure (`Arc<dyn Fn(&str) -> String + Send + Sync>`, `assembler.rs:126`). No consumer outside those crates, so a sibling fn in `security::scrubber` threads with zero new dependency edges.
- **No `ScrubbedValue` is a struct field or crosses a serialized boundary** — `grep -rn ": ScrubbedValue|<ScrubbedValue>|Vec<ScrubbedValue" --include=*.rs crates pulse-app` returns 0; `is_redacted()` / `category()` have callers only in the scrubber's own tests (`grep -rn "\.is_redacted()|\.category()"`, 0 outside `scrubber.rs`).

## Patterns detected
- **First-match boolean verdict** (`scrubber.rs:41-48`): the arm list is scanned in order and the first `true` returns a category — a span API must compute `find_iter` spans per arm (and, for `credit_card`, the Luhn-qualified candidate runs) rather than reuse `ArmPredicate`.
- **Each consumer renders its own placeholder** — three spellings in production: `[REDACTED:{c}]` (buffer appender + drain, the three persistence adapters), `[redacted:{c}]` (digest closure), `[redacted: {c}]` (markdown, inference_runtime, investigate, training_export). A shared primitive that takes the rendering as a parameter keeps every existing spelling byte-identical.
- **Closure injection into leaf crates** (`assembler.rs:126`, `contract.rs:600`): `triage` never names the scrubber; changing `pii_scrub_closure`'s body changes the digest with no `triage` edit.
- **Real-OTLP stored-field e2e** (`e2e_pii_bare_credential_stored_field.rs`): ephemeral-port receiver + `run_consumer` + column read-back, failure messages report length/prefix only.

## Conventions to follow
- **Counter unit — one per redacted VALUE**: `scrub_otlp_field` increments once per value (`appender.rs:364-366`); `encode_labels` once per redacted pair (`:452`). obs-plan §5 states "one increment per REDACTED LABEL PAIR, not per persisted cell"; a five-cell canary reads 5.
- **pulse-app pins live in `pulse-app/tests/*.rs`** via `pub` + `#[doc(hidden)]` (testing.md 2026-05-20; the dead-lib-src ratchet is a flat zero).
- **Canary assertions never echo the stored value** (`e2e_pii_bare_credential_stored_field.rs:158` `assert_redacted` reports length and prefix only).
- **The P-048 anchor** — `crates/buffer/src/appender.rs` must keep containing the literal `P-048` (`verify:capability-matrix`).

## Mechanism re-derivations (the equalities the design needs, verified at HEAD)
- **L1 fingerprint is independent of the scrub** — `compute_exception_fingerprint(exception_type, exception_stacktrace)` runs on the RAW values at `appender.rs:513-516`, before `scrub_otlp_field` at :522-524. So `fingerprint(input)` is identical under whole-value or span masking; [Fault Identity]'s L1 half is untouched by construction.
- **`scope_id` / service identity DOES depend on the scrub output** — `extract_service_name` returns `scrub_otlp_field(service.name)` (`appender.rs:135`) to all three consumers, so the identity string of a service whose name matches an arm changes from `[REDACTED:{c}]` to its span-masked form; a name matching no arm is byte-identical. The three persistence adapters scrub keys that already passed the choke point, so their output equals their input whenever the choke-point output contains no further match (re-scrub idempotence — below).
- **A digest's other lines survive span masking for the measured case** — `secret_kv`'s `\S+` and the card candidate's `[ \-]` separators cannot cross `\n`, so for the pin's input (`DIGEST_WITH_DATE_STAMPED_PROJECT` + `"  note: card 4111 1111 1111 1111\n"`) the only span is the 16-digit run, and `scrub_attribute` on the digest alone is `Allowed` (the neighbouring pin at `unit_digest_runtime_scrub.rs:28` asserts exactly that).
- **Span masking INSIDE compact JSON can break the JSON** — `serde_json::to_string` emits no whitespace, and `secret_kv` ends in `\S+`, so a value `token=abc` inside an `L4Output` string yields a match running `abc","next":…` across quote/comma delimiters; the other seven arms' character classes contain no `"`. A span replacement over the serialized text would therefore make `resolution_summary_text` unparseable exactly where today it collapses to a marker. Masking each string LEAF of the `serde_json::Value` before serializing keeps the structure.
- **Overlap resolution by catalog order reproduces today's category** — e.g. `AuthError: token Bearer <tok> rejected`: `secret_kv` spans `token Bearer`, `bearer` spans `Bearer <tok>`; union into one placeholder labelled by the earliest arm in catalog order (`bearer`, index 1, before `secret_kv`, index 3) — the category `scrub_attribute` returns today, which the existing appender pin asserts (`contains("[REDACTED:bearer]")`).
- **Re-scrub idempotence depends on span coverage** — none of the eight arms matches a placeholder alone (`[REDACTED:email]`, `[redacted: api_key]`: `api_key` needs `[\s=:]+` right after the key word, `]` follows). But a placeholder left BEHIND a key word could re-match: with `jwt` masking only its own span, `token: eyJ…` → `token: [REDACTED:jwt]`, which `secret_kv` re-matches on the next layer. Unioning overlapping spans before rendering closes that case (`token: eyJ…` is one merged span). Idempotence is an assertion to pin, not an assumption: the layered sites (choke point → persistence adapters; digest closure → markdown) re-scrub.
- **Where a regex match is narrower than the secret, span masking would expose the tail** — `secret_kv`'s value is `\S+`, so a multi-word value after `password:` leaves the words after the first outside the match; `bearer`'s charset `[A-Za-z0-9_\-\.=]` excludes `+` and `/`, so a standard-base64 token's tail after the first `+`/`/` falls outside the match. Today both are hidden because the WHOLE value is replaced. This is the recall trade the span EXTENT decision governs (P4).

## Consumer classification (production)
| site | today | span masking |
|---|---|---|
| `appender.rs::scrub_otlp_field` (log body, exception message/stacktrace, service_name, span_events.name, metric_name, severity_text) | whole value | span-masked; counter +1 per redacted value |
| `appender.rs::encode_labels` | whole VALUE per label, key verbatim | unchanged (composed shape: a key-anchored match spans key AND value) |
| `drain.rs` ×2 (template text) | whole template | span-masked (placeholder contains no whitespace → stays one token) |
| `markdown.rs::scrub_string` (Report fields) | whole field | span-masked |
| `digest_runtime.rs::pii_scrub_closure` (digest payload, service rows, cue summaries) | whole field | span-masked — the headline |
| `inference_runtime.rs::scrubbed_l4_json` | whole JSON → marker (degraded) | per-string-leaf masking, JSON stays parseable |
| `inference_runtime.rs::scrub_text` (incident title/detail) | whole field | span-masked |
| `investigate_router.rs::scrub` | whole field | span-masked |
| `training_export.rs::scrub_string` + `count_redactions` | whole field; `starts_with` count | span-masked; the count must detect a mid-value placeholder |
| `{baseline,lifecycle,storm}_persistence.rs` service-key scrubs | whole key | span-masked (same function as the choke point) |

## Test sweep (read-only agent, 33 listed files + two whole-tree greps; coordinates re-verified at HEAD)
- `grep -rlE "scrub_attribute|pii_scrub_closure|scrub_string|scrub_text|scrubbed_clone|is_redacted\(\)|\.category\(\)" crates pulse-app/tests --include=*.rs` + `grep -rlE "\[REDACTED:|\[redacted:|\[redacted: |redactions_applied" crates pulse-app/tests xtask --include=*.rs`: **3 BREAK · 0 CHECK · the rest SAFE or not scrubber-output assertions**.
  - `pulse-app/tests/unit_digest_runtime_scrub.rs:35` `pii_scrub_closure_still_collapses_a_digest_carrying_a_card` — `assert_eq!(out, "[redacted:credit_card]")` → inverts (the scope's measured pin).
  - `pulse-app/tests/unit_incident_producer.rs:663` `pii_canary_in_l4_text_is_scrubbed_before_persist` — `persisted.title.starts_with("[redacted:")` over `"error referencing <email> in title"` → becomes a "canary absent + placeholder present + surrounding text kept" assertion.
  - `pulse-app/tests/unit_training_export.rs:218` `count_redactions_counts_redacted_fields` — breaks THROUGH production code: `pulse-app/src/training_export.rs:173` counts only `starts_with(REDACTED_PREFIX)`, so a mid-value placeholder (`"user [redacted: email]"`) counts 0. The fix is in `count_redactions`, not the test.
- SAFE highlights: the 8 `scrubber.rs` tests assert the `ScrubbedValue` enum (unchanged if `scrub_attribute` stays); the appender's embedded-canary tests assert `contains` + absence; the labels pins are untouched; `crates/triage/**` and `crates/mcp-server/**` tests never call the scrubber (their `"[redacted] …"` strings are fixtures / the sidecar's own path sanitizer); webview has no redaction rendering logic (`pulse-app/ui/src` mentions are two `detail: "[redacted]"` fixtures).
- **P-048 matrix anchor is not the digest pin** — the v0.2.0 matrix binds P-048 to `appender.rs` containing `"P-048"`; inverting or renaming the digest pin moves no anchor (tests extract's §9 contract binding assumed otherwise).

## New files to create
- `pulse-app/tests/e2e_pii_span_masking_stored_field.rs` — real OTLP gRPC → DuckDB: canaries EMBEDDED in larger text (log body, exception message, exception stacktrace, service.name), per class (keyed `password=…` AND bare `sk_live_…` AND a card), asserting the secret 0× + the surrounding text kept + one counter increment per value.
- `pulse-app/tests/unit_span_masking_consumers.rs` — the pulse-app consumers under span masking: the digest closure (multi-secret, multi-line), `scrubbed_l4_json` stays parseable with a `token=` value inside a string field, investigate scrub, training-export `count_redactions` on a mid-value placeholder, re-scrub idempotence across the persistence adapters.

## Files to modify
- `crates/security/src/scrubber.rs` — the span-level masking primitive beside `scrub_attribute` (which stays the detection verdict), plus its co-located recall / false-positive / overlap / idempotence cases.
- `crates/buffer/src/appender.rs` — `scrub_otlp_field` uses span masking; `encode_labels` unchanged; the `P-048` literal stays.
- `crates/buffer/src/drain.rs` — both template sites use span masking.
- `crates/interpretation/src/markdown.rs` — `scrub_string` uses span masking.
- `pulse-app/src/digest_runtime.rs` — `pii_scrub_closure` uses span masking.
- `pulse-app/src/inference_runtime.rs` — `scrubbed_l4_json` masks string leaves before serializing; `scrub_text` uses span masking.
- `pulse-app/src/investigate_router.rs` — `scrub` uses span masking.
- `pulse-app/src/training_export.rs` — `scrub_string` uses span masking; `count_redactions` detects a placeholder anywhere in the field.
- `pulse-app/src/baseline_persistence.rs` — service-key scrub uses span masking.
- `pulse-app/src/lifecycle_persistence.rs` — service-key scrub uses span masking.
- `pulse-app/src/storm_persistence.rs` — service-key scrub uses span masking.
- `pulse-app/tests/unit_digest_runtime_scrub.rs` — the collapse pin inverts.
- `pulse-app/tests/unit_incident_producer.rs` — the L4-title canary pin re-points to span semantics.
- `pulse-app/tests/unit_training_export.rs` — gains the mid-value count case.
- `crates/ingest/examples/inject_scrub_canaries.rs` — an embedded-canary mode for the live leg (only if P4 keeps the live leg).

## Open questions
- Span EXTENT — exact regex match, match extended per arm class (key-anchored arms to end of line, bare-shape arms to the enclosing whitespace token, card to the whole candidate run), or whole line → blocks: plan-decision (it is the never-weaken trade, and the chunk's whole semantics; asked with the Boundary-widening pattern).
- Identity cells (`service.name` → `scope_id`, `metric_name` in the primary key, `span_events.name`, `severity_text`, the persisted service keys) — span-masked with everything else, or kept whole-value → blocks: plan-decision (changes the text of a redacted identity; arch §Established Decisions [Fault Identity] + §Conventions → Primary key convention amendments at wrap if span-masked).
- The operator-slot live leg (warm boot + embedded canaries, ~6 min) — kept or dropped → blocks: implementation-scope (it decides whether `inject_scrub_canaries.rs` is modified).
