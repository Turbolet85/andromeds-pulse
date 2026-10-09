# Scope — 2026-08-22-pii-scrubber-recall

**Version:** andromeda-pulse-0.3.0 · **Epoch:** Epoch 4 — Polish & ship: verification
**Working entry:** "PII scrubber recall — a bare provider key in telemetry is redacted before it reaches stored fields, not only its keyed form (P-047)"

## The claim under test

A **bare provider key** — a standalone `sk_live_…` / `sk-proj-…`-shape credential appearing in an OTLP
attribute value with **no `key=` name preceding it** — survives raw into stored fields, because the scrubber
catalog is keyed-only.

**This chunk is MEASURE-FIRST.** The claim is about STORED FIELDS, so the qualifying measurement is
end-to-end: drive a bare-key value through the real ingest path and read it back out of the stored field.
A unit assertion that the regex does not fire demonstrates the MECHANISM, **not the leak** — it is
necessary evidence and insufficient proof.

**A premise correction is a first-class good outcome.** If some other stage redacts the value anyway, the
finding dies and the chunk records that instead. Do not reshape the chunk to preserve the finding.

## Verified at HEAD (f0c38f5) — hypotheses that survived re-derivation

Directive- and annotation-supplied coordinates were re-derived first-hand before entering this scope
(per the 2026-08-21 external-relay rule). All Pulse-side claims hold; two line numbers differed by one
from the working entry and are corrected here:

- `crates/security/src/scrubber.rs:77` — `api_key` pattern is
  `(?i)(api[_\-]?key|access[_\-]?token|secret[_\-]?key|auth[_\-]?token)[\s=:]+[\w\-]{12,}`.
  The key-name alternation is **required before the value**, so a bare token matches nothing.
  (Working entry said `:78`; the regex line is `:77`.)
- `crates/security/src/scrubber.rs:84` — `secret_kv` is `(?i)(password|passwd|secret|token)[\s=:]+\S+`:
  also key-name-anchored. (Directive said `:83`, the `(` opener.)
- No sibling pattern can catch a standalone provider key: `jwt` (`:63`) keys on the `eyJ` header shape,
  `bearer` (`:69`) requires the literal `bearer`, `email`/`credit_card`/`ssn` (`:90`/`:97`/`:104`) are
  shape-specific to other classes. **The catalog has no bare-credential arm.**
- `crates/security/src/scrubber.rs:120–129` — the rstest corpus exercises ONLY keyed forms; its api_key
  case is literally `"api_key=sk-proj-1234567890abcdef"` at `:125`. "No test covers the bare case" is
  **verified, not assumed**.
- `andromeda-pulse-0.3.0/verification-matrix.json` — 22 capabilities (P-061…P-082); **P-047 is absent**.

**Found during scope re-derivation, not carried by the directive:**

- P-047 IS owned by a **legacy ledger**: `docs/v0_2_0/capability-verification-matrix.json:62`
  (`verification_mode: automated-nextest`), whose refs are `crates/security/src/scrubber.rs` and
  `pulse-app/tests/e2e_pii_scrubber_persistence_coverage.rs` — **and that test file exists**.
  **[premise-corrected at P3: that file is NOT the natural home — it is in-process and corpus-shaped
  (5 tests over the four CORPUS persistence adapters asserting `[REDACTED:email]` inside bincode bytes);
  it never speaks OTLP and never touches DuckDB, so the stored-field leg needs a NEW file.]**
  **[premise VERIFIED at P3: the legacy ledger is a LIVE per-PR gate]** — `xtask/src/main.rs:637` reads it and
  asserts every scenario file ref exists and every `contains` anchor greps non-empty (60/60 today). So both
  referenced paths are **rename-locked**, and the literal `P-047` must survive every edit to them. Note
  `scrubber.rs:3` reads "the **7** P-047 categories" — an eighth arm makes that sentence stale while its
  anchor still greps, so the count must be updated in the same edit.
- Production scrub call sites (the "stored fields" boundary this claim names): `crates/buffer/src/appender.rs:304`
  (`scrub_otlp_field`, the OTLP→DuckDB persistence path), `crates/buffer/src/drain.rs:375` + `:635`
  (template mining), `crates/interpretation/src/markdown.rs:337`, plus the `scrubbed_clone` closure
  injections in `crates/triage`.

## Capability claim posture

**Expect to claim NOTHING in the 0.3.0 matrix, and that is the correct outcome.** P-047 was closed in the
v0.2.0 era under the legacy route; this is a **recall gap in an already-verified capability**, so the
coverage gate no-ops and the proof lives in the tests plus the report rather than in a matrix flip.
If P3 research concludes a 0.3.0 entry is genuinely warranted, **surface it to the operator — never mint
one silently**.

## The design tension — named here, DECIDED in the plan

Any pattern broad enough to catch a bare provider key by shape alone (a prefix like `sk_live_`/`sk-proj-`
plus an entropy or length floor) buys recall at the cost of false positives over ordinary high-entropy
identifiers — trace ids, hashes, base64 blobs. **Over-redaction in telemetry destroys the diagnostic value
the corpus exists for.** Both directions are defensible. The plan MUST state which it takes and why, rather
than letting the regex decide. Note the existing negative corpus already pins `trace_id=abc-123` as
Allowed (`scrubber.rs:142`) — a shape-based arm must not regress it.

**[premise VERIFIED at P3]** The anchored-prefix direction is the settled lean, confirmed twice: `obs-plan.md:543–544`
names the exact shape family (`sk-[a-zA-Z0-9]{40,}` · `ghp_[a-zA-Z0-9]{36}` · `AKIA[0-9A-Z]{16}` ·
`Bearer [a-zA-Z0-9]{40,}` · `password=[^\s]+`) and warns that over-broad patterns false-positive; and
`scrubber.rs:55–58` independently declares the catalog's posture as "recall over precision — false positives
are acceptable (over-redaction); false negatives leak secrets." Honest nuance: the obs §8 passage was authored
for *grep-based CI heuristics*, not the runtime catalog, so it supplies a named shape vocabulary and a strong
analogical lean rather than a direct mandate. The plan still states the decision explicitly.

## Folded annotations

- **CARRY (#10) — external scrub observability.** No counter or trace line in `buffer` + `security` makes
  P-047 gradeable from outside the process; a redaction-count field on `duckdb.append` or `buffer.tick`
  would. Folded at promotion per the edb949f adaptation-wrap disposition; **whether it lands in this chunk
  is the plan's call**. **[premise VERIFIED at P3]** it pairs with the measurement leg on identified,
  adjacent machinery: `record_feed_counts` (`crates/buffer/src/appender.rs:411` → `state.rs:21/80` →
  `contract.rs:47/69`) is the shipped tick-fold in the same file and flow as the scrub call sites, and the
  obs `buffer` allowlist leaf (`pulse-app/src/observability.rs:176`) exists but enumerates no redaction field.
- **PREREQ — `cargo audit` standing deferral.** Ratified 2026-08-16 (pin #4), every-3rd-wrap INTERVAL.
  The interval point FIRED at session 31 (probe RAN, real exit 1, `parse error: duplicate advisory ID:
  RUSTSEC-2026-0244`, basis byte-identical and upstream). **Next point is session 34; this chunk's wrap is
  session 32, so it records `probe skipped per ratified interval (next: 34)` — never a silent skip** — while
  re-verifying basis + overlap. Overlap `cargo deny check advisories` now reports **EIGHT** owned upgradeable
  IDs: the standing seven (0189/0190/0194/0195/0204/0222/0253) plus **RUSTSEC-2026-0258** (`h2` 0.4.14,
  safe upgrade `>=0.4.16`), owned by the Advisory-backlog entry and **never ignore-listed**.
- **EVIDENCE** — Conductor pii-scrub chunk leg-verdict §New measurements (cite the path, never copy content).

## Cross-project consequence — record, do not act

Conductor's `scenarios/pii-scrub.toml` emits exactly the seven categories
`["email","jwt","bearer","api_key","credit_card","ssn","secret_key_value"]` and claims P-035/P-047/P-048,
with v2-14 VERIFIED (2026-08-19). But `conductor-emit/src/pii.rs:32-34` documents that set as "the seven
P-047 PII categories Pulse's scrubber must detect and redact" — a seven-variant enum with `all() -> [PiiCategory; 7]`.
**The instrument's catalog was DERIVED FROM the scrubber's catalog, so a hole in the catalog is invisible to
the external proof by construction.** Conductor's green is honest for what it emits and structurally
incapable of finding this class. (Relayed 2026-08-21; **not re-derivable from this repo** — recorded as
relayed, not as measured.)

Consequence if the gap is real and gets closed: Conductor needs an eighth category, or the new recall ships
with **no external witness** and Conductor's P-047 claim stays narrower than Pulse's implementation.
**RECORD this in the chunk report. Do NOT touch the Conductor repo from here** — its route owns its own work
and picks this up at its next visit.

## Boundaries

**In scope:** the bare-credential recall gap in `crates/security/src/scrubber.rs`; the end-to-end
measurement through a real stored-field path; the false-positive guard corpus; the report's cross-project
record; the CARRY #10 observability decision.

**Out of scope:** the Conductor repo (any file); the other two members of the "pii trio" beyond what the
measurement incidentally proves; a 0.3.0 matrix mint (operator-surfaced only); the Advisory-backlog upgrades
(owned by their own entry); re-litigating the v0.2.0-era P-047 closure.
