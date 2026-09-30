# Codebase Research — 2026-09-29-scrubber-path-false-positive

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 12
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — body read in full (lines 1–108) + all 21 Session Additions read line-bounded (the file is 55.6 KB; entries 109–150 read at a 900-char-per-line clip, the leg-relevant ones — `inject_demo` built outside the timed section and run BY PATH, ONE exported data dir per leg, the `agent-latest.jsonl*` glob, the deterministic-gate-rides-the-spawn-env trap — applied); plus the operator's standing memory directive (warm boot smoke at implement P3 for any user-visible surface, `inject_demo` seeding, PID-specific PowerShell cleanup).
- **Platform issues consulted:** none — no runner-only bullet (the Setup CI read was `in progress`, no red folded) and no CI-reading entry outside the operator leg.

## Files inspected
- `crates/security/src/scrubber.rs` (full, 250 lines) — the 8-arm catalog, first-match-wins, `OnceLock`. The card arm is `\b(?:\d[ \-]?){13,19}\b` at `:118-121`, commented "Doesn't check Luhn; intent is recall-over-precision" (`:116-117`) — THIS comment, not security-plan.md, is where the arm's shape is documented. The tests (`:139-250`) are one recall `#[rstest]` (`scrubber_redacts_each_p047_category`, the card positive `4532-1234-5678-9010` at `:154`), one false-positive corpus (`scrubber_allows_non_pii_values`, `:165-182`), one identity-column corpus (`:184-218`), two enum tests and one proptest. The header doc carries the literal `P-047`, which the v0.2.0 matrix anchors on (below).
- `crates/interpretation/src/markdown.rs` (`:240-262`, `:330-341`) — `assemble_report` computes `let workspace = scrub_string(&incident.workspace); let project_context = format!("workspace={workspace}");`, where `scrub_string` maps `Redacted` → `format!("[redacted: {category}]")` (WITH a space). The WHOLE workspace string is replaced. This is the Report "## Project Context" site, projected identically for `incidents.get_report` and the MCP `retrieve_report` (P-038 single-source).
- `crates/interpretation/src/prompt.rs` (`:205-232`) — the L4 prompt's `# Project Context` block wraps the `project_context` argument verbatim between `<PROJECT>` markers. The `# Current Digest` block wraps `digest_payload`.
- `pulse-app/src/inference_runtime.rs` (`:104-113`, `:385-410`, `:426`) — `build_project_context` = `"workspace=" + digest.workspace`, NOT scrubbed. The prompt receives `&digest.payload_summary` as the digest block. `interpretation.prompt.assemble` logs `token_count = prompt.len()` (bytes).
- `crates/triage/src/contract.rs` (`:585-615`) — `Digest::scrubbed_clone` scrubs `payload_summary` WHOLE, each cue summary, each service name. `workspace` is explicitly NOT scrubbed ("first-party canonicalized path").
- `crates/triage/src/digest/assembler.rs` (`:617-660`, `:478-481`) — `render_payload` writes `PROJECT: {name} (vcs={vcs})` INSIDE `payload_summary`, with `name` = `project.project_name`. The assembler then applies `digest.scrubbed_clone(|s| scrub(s))` before the queue/broadcast.
- `pulse-app/src/digest_runtime.rs` (`:50-58`, `:87-122`) — `pii_scrub_closure` maps `Redacted` → `format!("[redacted:{category}]")` (NO space). `workspace_to_digest_context` sets `project_name` from the detected context. `resolve_workspace_for_incidents` makes the key = the detected root, or the data dir when detection failed.
- `crates/workspace-detector/src/detect.rs` (`:21-58`) + `pulse-app/src/main.rs:748` — detection runs on `current_dir()`. `project_name` = the canonical root's basename, so a pulse-app launched with cwd `…/rm-20260923-093840` carries that basename into the digest `PROJECT:` line AND into the workspace key.
- `crates/triage/src/digest/retrieval.rs` (`:136-148`) — `format_corpus_match_line` = `[{fingerprint}] {title} — {age}m ago, {outcome}`, scrubbed whole per line (`assembler.rs:318`). It carries no workspace and no nanosecond timestamp.
- Conductor `c97f697` (read-only, `git show`): `conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence/rm-capture-b2.txt:453-459` shows `## Project Context` → `workspace=[redacted: credit_card]` and `## Previously Seen` → `incident #5 @ 1790699962319180900 — Credit Card Issue ([redacted: credit_card])`. The model titled the incident "Credit Card Issue" and its investigation steps chase a "data leak" (`:440-447`). That chunk's `report.md:31` reads "renders … as `[redacted: credit_card]` in the report's Project Context (and so in the prompt's PROJECT line)".

## Graph impact (from the code-graph query)
- **`scrub_attribute`** — 12 production caller fns (rust plane, `calls` on `callee_name = 'scrub_attribute'`, trace `.andromeda/runs/2026-09-29T22-09-22Z-phase/tree-query-2026-09-29-scrubber-path-false-positive.json`, 29 rows incl. module-level `use` sites and tests). Editor lines (0-indexed +1):
  - `scrub_otlp_field` @ `crates/buffer/src/appender.rs:362`
  - `encode_labels` @ `crates/buffer/src/appender.rs:438,440`
  - `DrainMiner::snapshot_state` @ `crates/buffer/src/drain.rs:375`
  - `write_template_to_table` @ `crates/buffer/src/drain.rs:635`
  - `scrub_string` @ `crates/interpretation/src/markdown.rs:337`
  - `scrub_service_key` @ `pulse-app/src/baseline_persistence.rs:109`
  - `pii_scrub_closure` @ `pulse-app/src/digest_runtime.rs:55`
  - `scrubbed_l4_json` @ `pulse-app/src/inference_runtime.rs:667`
  - `scrub_text` @ `pulse-app/src/inference_runtime.rs:677`
  - `scrub` @ `pulse-app/src/investigate_router.rs:114`
  - `scrub_service_name` @ `pulse-app/src/lifecycle_persistence.rs:103`
  - `scrub_fingerprint_service` @ `pulse-app/src/storm_persistence.rs:88`
  - `scrub_string` @ `pulse-app/src/training_export.rs:90`

  Test callers: the 4 in `scrubber.rs` and `canned_evidence_refs_survive_the_scrubber_unredacted` @ `pulse-app/tests/unit_deterministic_inference.rs:103`.

  The signature `fn scrub_attribute(&str) -> ScrubbedValue` is UNCHANGED by any design below, so no caller is edited. Every caller inherits a narrower card arm: fewer false `credit_card` redactions and no other behavioural change. No caller is signature-threaded.

## Patterns detected
- **The whole-value replacement amplifies one false positive** (`contract.rs:604` + `digest_runtime.rs:55-58`): `ScrubbedValue::Redacted` carries only a category (P-048 by design), so every consumer replaces the ENTIRE scrubbed string with the placeholder. For the digest, one digit run anywhere in `payload_summary` (the `PROJECT:` line included) replaces the whole Appendix C body the model reads with `[redacted:credit_card]`.
- **Two placeholder spellings coexist**: `[redacted: {c}]` (markdown / training_export, with a space) and `[redacted:{c}]` (pii_scrub_closure, no space). Conductor's capture carries the spaced form, so it is the Report path.
- **The false-positive corpus pattern** (`scrubber.rs:165-182`): `#[rstest]` `#[case]` rows asserting `!is_redacted()`. The identity-column corpus (`:184-218`) additionally asserts the value is byte-identical.
- **A precision arm as anchor + structural floor, never entropy** (`scrubber.rs:88-107`, the `provider_key` precedent).

## Conventions to follow
- **Co-located `#[rstest]` corpora in `scrubber.rs`**; new pins are `#[case]` rows in the recall and false-positive tests (test-plan §4 security row).
- **pulse-app tests live in `pulse-app/tests/*.rs`** (`[lib] test = false`; the dead-`#[test]` ratchet). `pulse-app/tests/unit_digest_runtime_scrub.rs` already drives `pii_scrub_closure` (`:8-20`), and that binary links and runs, so a digest-level pin belongs there.
- **Keep the literal `P-047` in `scrubber.rs`**: `docs/v0_2_0/capability-verification-matrix.json:62` anchors a scenario on `crates/security/src/scrubber.rs` `contains "P-047"` (re-derived: `grep -n -o 'scrubber.\{0,120\}' docs/v0_2_0/capability-verification-matrix.json`).
- **Scrub diagnostics are category/count-only**: no emit carries the input (obs §8/§11, security §Logging).

## Mechanism equalities (re-derived at HEAD)
Probe: `scratchpad/probe_card.py`, Python `re`. Semantics match Rust `regex` for these ASCII inputs: `\b` between `-` and a digit is a word boundary in both.
- **E1 — the measured false positive matches the card arm.** `scrub_attribute("rm-20260923-093840")` matches the card arm on `20260923-093840`: 14 digits with one `-`, inside `{13,19}`. The same holds for `/tmp/rm-20260923-093840`, a Windows `…\Temp\rm-20260923-093840`, and `workspace=/tmp/rm-…`. **VERIFIED.** Its digits fail Luhn (sum 61).
- **E2 — the ssn arm does NOT match it** (`\b\d{3}[ \-]?\d{2}[ \-]?\d{4}\b` finds no match on any of the four forms). **VERIFIED.** Once the card arm stops matching, no other arm claims the value (jwt/bearer/api_key/secret_kv/provider_key/email need tokens absent here), so the result is `Allowed`.
- **E3 — the named recall positive `4532-1234-5678-9010` FAILS Luhn** (sum 66; the security extract's computation reproduced). A Luhn-gated arm therefore un-redacts the catalog's own pin. Luhn-valid equivalents exist: `4532015112830366` / `4532 0151 1283 0366` (sum 50), `4111111111111111` (30), and Amex `378282246310005` (60).
- **E4 — the greedy match swallows adjacent digit groups** (probe: `'4111 1111 1111 1111 22 x …'` → match `'4111 1111 1111 1111 22 '`; `'ref 12 4111111111111111'` → match `'12 4111111111111111'`, 18 digits). A Luhn check applied to the regex's match alone would MISS a valid card that sits next to another digit group, a recall loss the current arm does not have. A Luhn design must evaluate candidate WINDOWS inside a match (sub-runs of 13–19 digits, at least at separator-group boundaries), not the greedy span.
- **E5 — 19-digit nanosecond timestamps also match the current arm** (`1790699962319180900`, the incident timestamp in the b2 capture, sum 91, not Luhn-valid). Luhn rejects ~90% of arbitrary digit runs, so it narrows this class. A separator-grouping rule alone does NOT, because contiguous runs are a valid card form.
- **E6 — what reached the model** (code-read chain, above): with cwd = `…/rm-20260923-093840`, (a) the prompt's `<PROJECT>` block carries `workspace=…/rm-20260923-093840` VERBATIM (unscrubbed); (b) the digest block carries `payload_summary` scrubbed whole, which HEAD collapses to `[redacted:credit_card]` because its `PROJECT: rm-20260923-093840 (vcs=…)` line matches E1; (c) the Report's Project Context reads `workspace=[redacted: credit_card]` (the capture's form). Fixing the card arm so E1 no longer matches restores (b) and (c) with no consumer edit.

## New files to create
- none

## Files to modify
- `crates/security/src/scrubber.rs` — the `credit_card` arm's precision (candidate-window check per the P4 decision) + recall/false-positive `#[case]` pins incl. the measured value and its path/key forms; the arm comment rewritten to the new shape
- `pulse-app/tests/unit_digest_runtime_scrub.rs` — a digest-level pin: `pii_scrub_closure` over a payload carrying `PROJECT: rm-20260923-093840 (vcs=git)` returns it unchanged, while a payload carrying a Luhn-valid card still collapses
- `crates/interpretation/src/markdown.rs` — a test-module pin: `assemble_report` over an incident whose workspace is `/tmp/rm-20260923-093840` yields `project_context == "workspace=/tmp/rm-20260923-093840"`

## Sweeps
- `credit_card` over `--include=*.{rs,ts,tsx,md,json,toml}`: code hits only in `crates/security/src/scrubber.rs` (`:95`, `:120`, `:121`, `:154`). The remaining hits are planning docs and rule text, no change. Nothing outside the scrubber asserts a `credit_card` redaction.
- Card-shaped literals `4111|4532|5555[ -]?5555|3782|6011[ -]?[0-9]{4}` over `crates pulse-app/src pulse-app/tests xtask scripts`: 1 hit (`scrubber.rs:154`), which rides this list. No other test depends on a card canary.
- `P-047` anchor: `docs/v0_2_0/capability-verification-matrix.json:62` (`contains "P-047"` on scrubber.rs) is a no-change DATA pin to preserve (keep the header literal).
- `format_corpus_match_line` / `render_payload` / `build_project_context` / `assemble_report` need no change: consumers inherit (E6).

## Open questions
- Card-check form: (i) Luhn over candidate windows (E3: the `4532-1234-5678-9010` pin becomes a Luhn-valid test number — the only real cards are Luhn-valid per ISO/IEC 7812; E4: windowing required), (ii) no Luhn, but a separator-grouping grammar (contiguous or 4-digit groups / Amex 4-6-5 / Diners 4-6-4) that rejects `8-6`-grouped runs (keeps the pin; does not narrow E5), or (iii) both. → blocks: plan-decision (a contestable fork for P4)
- Live leg shape. No log field carries scrubbed text, by design, so a boot leg cannot read the Report or digest body. The only wire observable is `interpretation.prompt.assemble token_count` (prompt bytes), which is an indirect signal. The operator's standing directive still asks for a warm boot smoke on a user-visible-surface chunk (the Report's Project Context text changes). → blocks: plan-decision
- NOTE (no blocked step): the whole-value replacement (Patterns §1) means any TRUE positive anywhere in a digest still wipes the whole digest for the model. That follows from P-048's category-only `Redacted` by design and is out of this chunk's scope. It is a candidate for a separate route entry if the operator wants span-level redaction.
