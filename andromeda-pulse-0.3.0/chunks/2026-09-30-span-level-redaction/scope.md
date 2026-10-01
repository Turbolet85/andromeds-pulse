# Scope — 2026-09-30-span-level-redaction

**Working entry (verbatim title + hint):** Span-level redaction — a secret inside a larger value redacts only its
matched span, so one true positive no longer blanks the whole digest the model reads.

**Chunk base:** `7bb56ea` (W182) · version andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification.

## Folded freight (the entry's one CONTEXT block, 524 chars)
- Founder 2026-09-30 verbatim «да маскировать фрагмент» (overseer relay
  `pc-overseer/relays/pulse-wrap-scrubber-2026-09-30.md` §2 — a companion-repo coordinate, cited not copied).
- "today `ScrubbedValue::Redacted` carries only a category (P-048), so every consumer replaces the WHOLE
  scrubbed value" — VERIFIED at HEAD: the enum is `Allowed(String)` · `Redacted { category: &'static str }`
  (`crates/security/src/scrubber.rs:17-21`), and 12 of the 13 production call sites replace the whole value; the
  13th (`encode_labels`) replaces the whole VALUE of each label and keeps the key (research.md §Consumer
  classification).
- "measured at 2026-09-29-scrubber-path-false-positive (`pii_scrub_closure_still_collapses_a_digest_carrying_a_card`:
  one card line in a digest payload → the digest reads `[redacted:credit_card]`)" — VERIFIED at
  `pulse-app/tests/unit_digest_runtime_scrub.rs:35` (`assert_eq!(out, "[redacted:credit_card]")`).
- "a security-plan §Security Anti-Patterns → Logging amendment is expected".

## Outcome
A value that carries a secret somewhere inside a larger text keeps its non-secret content: only the matched
span is replaced by a category placeholder, so an incident digest that contains one card number (or one token,
one email) still reaches the L4 model — and the report / snapshot / MCP readers — with its other lines intact.
No secret that is redacted today becomes readable after this chunk (never-weaken: recall of every one of the
8 P-047 categories is preserved).

## What it builds
- The scrubber primitive (`crates/security/src/scrubber.rs`) gains a span-level result: the redacted form of
  the value with each matched span replaced by a placeholder naming its category, alongside the category
  information consumers already read. [open — a P4 design fork. Research: no struct carries a `ScrubbedValue`
  and `is_redacted()` / `category()` have 0 non-test callers, so a sibling function beside the unchanged
  `scrub_attribute` verdict threads with zero new dependency edges]
- Every consumer that today replaces the WHOLE value is migrated to write the span-masked text instead.
  [premise-corrected: the "files referencing `scrub_attribute`" list mixed call sites with doc comments —
  `crates/buffer/src/schema.rs`, `crates/corpus/src/contract.rs` and all five `crates/triage` files only MENTION it
  (`triage` has no `security` dependency and receives the scrub as an injected closure). The production call
  sites (code-graph + grep, research.md §Graph impact) are: `buffer` appender `scrub_otlp_field` + drain ×2;
  `interpretation` markdown `scrub_string`; `pulse-app` digest_runtime `pii_scrub_closure`, inference_runtime
  `scrubbed_l4_json` + `scrub_text`, investigate_router `scrub`, training_export `scrub_string`, and the
  baseline / lifecycle / storm persistence service-key scrubs. `encode_labels` keeps its whole-value-per-label
  contract (Boundaries)]
- The digest the model reads is the headline surface: the measured failure is
  `pii_scrub_closure_still_collapses_a_digest_carrying_a_card` (one card line in a digest payload → the digest
  reads `[redacted:credit_card]`); that pin inverts — the digest keeps its other content and masks only the
  card span.
- The persisted L4 interpretation (`resolution_summary_text`) masks each string field of the parsed output
  rather than the serialized JSON, so it stays parseable. [inferred at take-up; VERIFIED as required by
  research: `secret_kv`'s `\S+` runs across compact-JSON delimiters, so a span replacement over the serialized
  text would break the JSON]
- The training export's `count_redactions` (a production aggregate) counts a placeholder anywhere in a field,
  not only at its start. [added by research: `pulse-app/src/training_export.rs:173` counts `starts_with` only,
  so every mid-value placeholder would count 0]

## Boundaries
- Detection is unchanged: the 8 arms, their regexes, the Luhn whole-group gate on `credit_card`, the
  false-positive corpus — this chunk changes what is REPLACED, not what is MATCHED. (VERIFIED feasible:
  `scrub_attribute` stays as the verdict; the 8 `scrubber.rs` tests assert the enum and stay green.)
- The composed-form scrub sites (`metrics_points.labels` joined `key=value` ∪ bare value) keep their
  key-verbatim / value-redacted contract; span masking must not re-expose the value half of a keyed match
  (a key-anchored arm's match spans the key AND the value). (VERIFIED: `appender.rs:428-459`; labels stay
  whole-value per label.)
- Placeholder shape: [premise-corrected: consumers emit THREE spellings, not two — `[REDACTED:{category}]`
  (buffer appender + drain, the three persistence adapters), `[redacted:{category}]` (the digest closure),
  `[redacted: {category}]` (markdown, inference_runtime, investigate, training_export)]. Whether span masking
  unifies them is a P4 question, not an assumed change.
- No change to the ring buffer's encryption posture, the corpus schema, or any TauRPC procedure is expected.
  (VERIFIED: no `ScrubbedValue` crosses a serialized boundary; no table, column or procedure is involved.)
- The L1 exception fingerprint is untouched by construction — it is computed from the RAW `exception.type` +
  `exception.stacktrace` before the scrub (`appender.rs:513-524`).
- A change to what crosses the scrubber boundary (a new variant shape consumers read) is a P4 escalation if
  the playbook names it Boundary widening — asked as one, for the founder's live word (operator directive at
  take-up). The playbook names it (`.andromeda/playbook.md:114`, `verdict: escalate`).
- **RATIFIED at P4 — founder, 2026-10-01, live, verbatim: «Ок давай по типу правила»** (relayed by the overseer,
  in answer to the span-extent fork's three options). The boundary widening is ratified with the CLASS-AWARE
  extent: keyed arms (`bearer`, `api_key`, `secret_kv`) mask from the key word to the end of the line;
  bare-shape arms (`jwt`, `provider_key`, `email`, `ssn`) mask the whole whitespace-delimited token;
  `credit_card` masks the whole digit-group run; overlapping spans merge under the highest-priority category
  (catalog order). The rejected alternatives were exact-regex-match (fails never-weaken: a multi-word secret
  after a key, and a base64 bearer tail past `+`/`/`, would become readable) and whole-line (does not mask the
  fragment within a line).

## Contracts / specs touched
- P-048 — [premise-corrected: P-048 is not in the v0.3.0 matrix; it lives in `docs/v0_2_0/capability-verification-matrix.json`
  with the single scenario "`crates/buffer/src/appender.rs` contains `P-048`". No matrix edit is owed while that
  literal stays; the security-plan text stating "whole value (P-048)" remains an amendment target.]
- security-plan §Security Anti-Patterns → Logging — an amendment is expected (stated by the entry).
- `.claude/rules/security.md` credit_card bullet ("A redaction still replaces the WHOLE value (P-048), so one
  true positive anywhere in a digest blanks the whole digest") becomes false after this chunk — a wrap
  re-derive target. (VERIFIED: the sentence stands in the rule file at HEAD.)
- Identity text — if identity cells are span-masked (P4 fork), arch §Established Decisions [Fault Identity]
  (`scope_id` = the scrubbed service name) and §Conventions → Primary key convention (the "redact to one
  placeholder" collision prose) become wrap amendment targets. [inferred]

## Operator directives at take-up (2026-09-30)
- Founder rulings: nothing deferred; CI speed is a priority.
- Operator slots: any run that launches pulse-app or opens a window is the operator's slot (conductor-builder
  runs NVDA legs) — STOP and ask with its length.
- Disk 66 GB free at take-up; `cargo clean --profile dev` first if under 60 GB.

## CI read at Setup 5a
- `7bb56ea` (the last wrap's flip, = HEAD): verdict not yet available — `ci#36773076318` in progress (12 of 13
  checks open, oldest `coverage` at 192 s); `secret-scan#36773076453` completed/success. Not folded, not read
  as green. Re-read at P3: still `in progress` (3 checks open, oldest `coverage` at 875 s) — no red to disposition.
