# Scope — 2026-09-29-scrubber-path-false-positive

**Chunk:** Scrubber path false positive
**Version:** andromeda-pulse-0.3.0
**Working entry:** `andromeda-pulse-0.3.0/working-route.md:134` (promoted by this phase)

## Intent (the working entry, verbatim title + hint)
Scrubber path false positive — a digit run in a path or workspace key is not redacted as a card number, so
the model's PROJECT line stops reporting a credit-card leak that does not exist.

## What this chunk builds
- Fix the CAUSE in the scrubber (`crates/security/src/scrubber.rs`), not at one consumer: the `credit_card`
  arm must stop matching a digit run that is not a card number — the measured instance being a data-dir
  basename `rm-20260923-093840` (a date-time stamp) inside a workspace key.
- A real card check replaces the bare 13–19-digit shape. Its exact form is a P4 decision over the candidates
  research laid out (research.md §Mechanism equalities E3–E5, §Open questions): Luhn over candidate windows,
  a separator-grouping grammar, or both. Luhn un-redacts the Luhn-invalid pin `4532-1234-5678-9010`
  (sum 66), and a greedy-match Luhn misses a card beside another digit group (E4), so windowing is required.
- Real PII redaction is NOT weakened: real (Luhn-valid) card numbers in every form the catalog targets stay
  redacted, and every other arm's known positive stays as it is. [amended at P5 val-1, intent-incomplete: the
  recall pin `4532-1234-5678-9010` is not a card number (Luhn sum 66, research E3). Under the founder's
  "Luhn on windows" answer (2026-09-30) it moves to the documented Allowed boundary and is replaced by the
  Luhn-valid `4532-0151-1283-0366`. The founder accepted the trade that a mistyped, Luhn-invalid card no
  longer redacts]
- Tests pin both directions: the known positives (real card-shaped values that must still redact) AND the
  measured false positive (`rm-20260923-093840`, and the workspace-key form it reached the model in) as
  `Allowed`. The false-positive corpus in `scrubber.rs` is the place such pins live (security rule §Logging &
  redaction: "the false-positive corpus … is what bounds it").

## Boundaries
- In: the `credit_card` arm and its tests. [premise-corrected: no sibling arm is involved — `ssn` does not
  match `rm-20260923-093840` or its path/key forms (research E2), and once the card arm stops matching, no
  other arm claims the value]
- In: the paths by which the value reaches the model and the Report, READ and pinned end-to-end, not
  redesigned. [premise-corrected: there are two paths at HEAD, and the prompt's `<PROJECT>` block is not one
  of them — (a) the Report's `assemble_report` scrubs `incident.workspace` WHOLE → `workspace=[redacted:
  credit_card]` (the capture's spaced form, `markdown::scrub_string`); (b) the L4 digest block: the digest's
  `PROJECT: {cwd basename}` line lives inside `payload_summary`, which `Digest::scrubbed_clone` scrubs WHOLE,
  so one false match replaces the ENTIRE digest the model reads with `[redacted:credit_card]`; the prompt's
  `<PROJECT>` block itself carries `digest.workspace` UNSCRUBBED — research E6]
- Out: removing scrubbing from workspace keys or paths altogether (that would weaken redaction — the fix is
  precision in the card arm, not an exemption for a field). (security + arch + obs extracts concur)
- Out: the Conductor repo (read-only from here); Conductor's v3-09 series consumes the fix.
- Out: re-scrubbing or rewriting already-stored corpus rows (arch history: stored L4 text keeps the old
  placeholder; the report states that residual).
- Out: the whole-value replacement that makes one match wipe a whole digest. It follows from P-048's
  category-only `Redacted` and is a candidate separate route entry, not this chunk (research §Open questions
  NOTE).

## Surfaces / contracts touched
- `security::scrubber::scrub_attribute` — the P-047 8-category catalog; the category COUNT, arm ORDER and
  signature do not change (verified: security extract + graph).
- Every consumer of `scrub_attribute` inherits the change, with no edit (verified: 12 production caller fns in
  buffer / interpretation / pulse-app, research §Graph impact).
- The card arm's shape ("13-19 digits … Doesn't check Luhn") is documented in the scrubber's own source
  comment (`scrubber.rs:116-117`), which this chunk rewrites. [premise-corrected: security-plan.md's body names
  `credit_card` only as a catalog member and never states the arm's shape, so any wrap amendment records the
  new precision rule in the catalog enumeration rather than correcting a stated shape]

## Folded freight (from the working entry)
- CONTEXT (measured by Conductor, re-verify at /phase): Conductor `c97f697`'s real-model series — the data-dir
  basename `rm-20260923-093840` rendered as `[redacted: credit_card]` in the workspace key that reaches the
  model's PROJECT line, and the model then diagnosed a credit-card leak (Conductor's b2 graded NotIdentified).
  Mechanism claim "a 14-digit run with one `-` separator matches `\b(?:\d[ \-]?){13,19}\b`" (measured by
  Conductor) — VERIFIED at HEAD (research E1: the match is `20260923-093840`, which fails Luhn at sum 61). The
  capture re-read at `c97f697` (`evidence/rm-capture-b2.txt:453-459`) confirms `workspace=[redacted:
  credit_card]` and a model-authored "Credit Card Issue". Conductor's parenthetical "(and so in the prompt's
  PROJECT line)" is corrected to the digest-wipe mechanism above.
- CONTEXT (FOUNDER 2026-09-29, asked, answered): fix the cause; Conductor's v3-09 series waits on this entry;
  placed right after 2026-09-29-ci-wall-time-and-round-trips and before the dual license, on the overseer's
  relay at that chunk's wrap (`pc-overseer/relays/pulse-wrap-ci-2026-09-29.md`, part 2).
- watch: Linux boot silent death — re-watched after its recurrence on `4502d5d` (ci#36625507595: `boot: ready`,
  then `harness:status` `not-running` 0.7 s later; no panic, no new ERROR; 0 of 30 WSL trials reproduced it);
  instrument `harness:status`'s `ended` (the end record `agent-run.sh boot`'s waiting wrapper writes to
  `run/andromeda-pulse.exit`, naming the signal or exit code); retires on a recurrence or 3 green runs
  (1/3; since 2026-09-29-ci-wall-time-and-round-trips; green so far ci#36632205717).

## Operator directive (this phase's invocation)
- Evidence is Conductor c97f697 (b2 capture). Fix the cause in the scrubber; a real card check such as
  length + Luhn is research; do not weaken real PII redaction; pin with tests covering known positives and the
  measured false positive.
- Any diff-shaped gate probe names the chunk base (W182). The chunk base is `a08ae29` (HEAD at take-up).

## CI read at Setup (5a)
- `a08ae29` (the last wrap's flip = HEAD): **verdict not yet available** — ci#36637499895 in progress
  (12 checks running, oldest 206 s), secret-scan#36637500009 completed/success. Not a red; nothing folded.
  Wall-clock not yet measurable.
