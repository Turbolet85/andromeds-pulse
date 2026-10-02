# Report — 2026-09-29-scrubber-path-false-positive

**Chunk:** Scrubber path false positive — card arm stops redacting non-card digit runs in paths/keys
**Date:** 2026-09-30T06:30Z
**Commits:** `fcc31b2` chore(2026-09-29-scrubber-path-false-positive): operator pre-CI commit · `85e0736` chore(2026-09-29-scrubber-path-false-positive): operator pre-CI commit (basis: `git log --format='%h %s' a08ae29..HEAD`; chunk base `a08ae29`)

## Changes (structured — detectors read this)
- **Files:**
  - `crates/security/src/scrubber.rs` — the card arm's predicate plus its tests.
  - `pulse-app/tests/unit_digest_runtime_scrub.rs` — 2 new tests.
  - `crates/interpretation/src/markdown.rs` — 1 new test in `#[cfg(test)]`; no production edit.
  - `pulse-app/ui/package-lock.json` — the operator-pass npm fix; see Dependencies.
  - Basis: `git diff --name-only a08ae29 -- crates pulse-app xtask scripts`, 4 files.
- **Symbols / APIs:**
  - `security::scrubber::scrub_attribute(&str) -> ScrubbedValue` has an UNCHANGED signature and an unchanged `pub` surface: same `pub` items, same `ScrubbedValue` variants, same 8 category labels in the same order.
  - What changed is the MATCH PREDICATE of the `credit_card` arm only. Its regex is now the candidate `\b[0-9]+(?:[ \-][0-9]+)*\b` (a maximal run of ASCII digit groups joined by single ` `/`-`). The arm matches iff some window of CONSECUTIVE WHOLE groups carrying 13–19 digits passes Luhn.
  - Replaced: `\b(?:\d[ \-]?){13,19}\b` with no checksum.
  - New PRIVATE items in `scrubber.rs`: `type ArmPredicate`, `regex_matches`, `card_number_matches`, `has_luhn_valid_window`, `luhn_valid`, `CARD_DIGITS_MIN`/`MAX`. The catalog tuple became `(Regex, &str, ArmPredicate)`; the other seven arms use `regex_matches` with their regexes untouched.
  - All 12 production callers of `scrub_attribute` inherit the change with no edit (basis: `.andromeda/runs/2026-09-29T22-09-22Z-phase/tree-query-2026-09-29-scrubber-path-false-positive.json`, a rust-plane `calls` query). They are in buffer (`scrub_otlp_field`, `encode_labels`, drain ×2), interpretation (`markdown::scrub_string`), and pulse-app (`pii_scrub_closure`, `scrubbed_l4_json`, `scrub_text`, investigate `scrub`, lifecycle/storm/baseline persistence, `training_export::scrub_string`).
- **Crates / modules:** none added or removed. `security` changed internally.
- **Dependencies:**
  - Rust: none. `git diff --name-only a08ae29 -- Cargo.lock ":(glob)**/Cargo.toml"` prints nothing; the Luhn check is hand-rolled.
  - npm (`pulse-app/ui/package-lock.json` only; `package.json` untouched), three in-range bumps made by `npm update brace-expansion ip-address`:
    - brace-expansion 1.1.18 → 1.1.21 (under minimatch 3)
    - brace-expansion 5.0.9 → 5.0.12 (under `@typescript-eslint/typescript-estree`)
    - ip-address 10.7.0 → 10.7.2 (under socks)
  - They close GHSA-6j4f-fj2g-mc7p, GHSA-qhr7-859c-m2p7, GHSA-q2hr-2g5m-vwhr (brace-expansion) and GHSA-h3mg-xc3c-68pw, GHSA-j6r3-76f7-8jcv (ip-address). These were published after `a08ae29`'s green supply-chain run.
  - No advisory exception was added. `npm-policy.json` is unchanged: its two extract-zip exceptions (GHSA-jmr9, GHSA-7pqw) still stand.
- **Schema / config — the scrub/redaction shape:**
  - The `credit_card` arm moved from "13–19 digits, recall-over-precision, no Luhn" to "candidate digit-group run → whole-group windows of 13–19 digits → Luhn".
  - The accepted recall trade (founder, P4 live word "Luhn on windows", 2026-09-30): a Luhn-invalid (mistyped) card number, or one fused inside a longer single digit group (e.g. `004111111111111111`), is no longer redacted.
  - Now Allowed:
    - date-time stamps such as the harness data-dir basename `rm-20260923-093840`, in path, `workspace=` key and digest `PROJECT:` line forms
    - 19-digit nanosecond timestamps such as `1790699962319180900`
    - the old catalog pin `4532-1234-5678-9010` (Luhn sum 66)
  - Still redacted: Luhn-valid cards contiguous, spaced, hyphenated, Amex 4-6-5, behind `key=`, and beside an unrelated digit group.
  - Whole-value replacement is UNCHANGED (P-048): a true positive anywhere in a scrubbed value still replaces the whole value with the placeholder.
  - `redactions_applied` keeps its unit and gains no field. Its count drops by exactly the removed false positives.
  - No new or changed tracing emit.
- **Spec-master edits:** none (this chunk's /implement touched no master; wrap P2 owns the amendments below).
- **Counts / qualifiers moved:**
  - The `security` crate test suite went 39 → 54 (basis: `cargo nextest run -p security` → `54 tests run: 54 passed`; the RED-before run selected the 15 new cases with 39 skipped). The breakdown is +8 `scrubber_redacts_luhn_valid_card_forms` cases and +7 `scrubber_allows_non_card_digit_runs` cases; one recall case was replaced in place.
  - test-plan §4's security crate row states `crate suite 14 → 23` (`.andromeda/test-plan.md:366`; `grep -n 'crate suite' .andromeda/test-plan.md`), stale since before this chunk.
  - Workspace nextest is 2429/2429 (the gate log, `Summary … 2429 tests run: 2429 passed`).
  - Webview vitest is 848/848, 81 files (`npm run test --prefix pulse-app/ui`, run after the lockfile bump).
- **Dev-tool versions:** none — node v24.13.1 / npm 11.8.0 re-read on the dev host, unchanged.
- **Harness / gate surface:** none.
- **Cross-project / external claims:**
  - Conductor `c97f697` (read-only; basis: `git show` at /phase): `evidence/rm-capture-b2.txt:453-459` shows `workspace=[redacted: credit_card]` and the model's "Credit Card Issue". That is the measured origin.
  - Overseer relay `pc-overseer/relays/pulse-wrap-scrubber-2026-09-30.md` (measured 2026-09-30): Conductor's 2026-09-30 series ran against a `pulse-app` rebuilt from `fcc31b2` (proven by content) with 0 `[redacted: credit_card]` anywhere, and the data dir's name was absent from every capture.
  - The same relay (Conductor `:56`, Pulse `fcc31b2`): the real model (Llama 3.2 3B) surfaced the canary in 4 of 9 storms and dismissed the scenario's storm digest both times it emitted (parse `ok`, no incident).
  - CI runs:
    - ci#36671290813 on `fcc31b2`: 12/13 green, `supply-chain` red (`check:npm-supply-chain` `findings-red`, the five GHSAs above; `cargo audit` 0 vulnerabilities); boot green.
    - ci#36675962820 on `85e0736`: 13/13 green, wall 1380 s; secret-scan#36675962803 success. Basis: `ci.py conclusion --sha HEAD --wait 2400`, recorded in `evidence/operator-pass.md`.
- **Reverted / negative API facts:** none shipped.
  - Two temporary mutations were applied and reverted (Step 6 mutation check): Luhn forced true, and Luhn over the whole candidate instead of the window walk. Reverted with the Edit tool; a grep for their tokens reads 0.
- **Insufficient fixes (written, kept, not the remedy):** none. The downstream real-model non-surfacing is a separate defect, not a remainder of this one (relay; owned by a new route entry, P5).
- **Spec claims disproved by measurement:** none new this chunk.
  - The /phase premise corrections (the `<PROJECT>` block path, the ssn-arm involvement, the arm's shape living in the source comment rather than security-plan) were already recorded in `scope.md` at /phase P4 / P5 val-1.
  - The recall pin `4532-1234-5678-9010` being Luhn-invalid was recorded at P5 val-1.
- **Expected amendments (from plan):**
  - security-plan §Security Anti-Patterns → Logging — record the `credit_card` precision rule, the accepted recall trade, and that the measured harness-basename false positive is closed.
    - **carried** by the Schema / config bullet.
    - Site: `grep -n 'credit_card' .andromeda/security-plan.md` → 1 hit at `:428`, the "Uniform scrubber coverage" catalog enumeration. The other six masters have 0 hits for `credit_card|Luhn`, except test-plan with 1 (below).
  - test-plan §4 security crate row — re-sync the crate suite count.
    - **carried** by the Counts bullet (39 → 54 measured).
    - Site: `grep -n 'crate suite' .andromeda/test-plan.md` → 1 hit at `:366`.
- **Coverage of new surfaces:** no new external surface, hot-path op or UI element. One CHANGED behaviour at an existing boundary:
  - `security::scrubber` credit_card arm (a precision change at every scrub boundary) → validation n/a · instrumentation n/a (no emit added; `redactions_applied` unit unchanged) · PII redacted✓ (Luhn-valid cards still redact) · tests unit (`scrubber.rs` recall + false-positive rstest) + integ (`unit_digest_runtime_scrub.rs`, `markdown.rs` `assemble_report`) · a11y n/a · tokens n/a.

## Deviations from intent
- Code: none. Plan Steps 1–6 were executed as written, and the plan's 18-test count atom matched.
- The catalog tuple gained a third element (a private per-arm predicate) so that only the card arm's rule changes. This is structural and private; the seven other arms' regexes are untouched.
- Operator pass fired by the agent on the overseer's word ("Run the operator pass now, entries 11-14 in order"). The pre-CI commits and pushes are the operator's by construction; the agent performed them on that explicit word.
- The bindings clobber: /implement's `capability-drift` went red once because the default-features workspace nextest rewrote the worktree `pulse-app/ui/src/bindings/index.ts` without `mcp.*`. I restored it with `git checkout HEAD` (the staged copy was verified clean) instead of the `--features mcp-server` regen, which would have meant a second native feature-graph compile.
- Scope record (`gate.py scope` at P1: `scope: clean — changed 4 · listed 3 · recorded 1 … widening 1`):
  - widening: `pulse-app/ui/package-lock.json` — serves the CI read of the pushed head (Test Commands entry 14).
    - word: "Read the step output, find the cause, and fix it in-chunk (founder: no deferral; an exception only if no fixed version exists anywhere, with provenance and a closing condition, as GHSA-7pqw was)".
    - Given by the overseer, relaying the founder.

## Decisions & corrections
- FOUNDER (P4, 2026-09-30, clicked live): card check = "Luhn on windows" (whole separator groups, 13–19 digits). Rejected at P4:
  - Luhn at every digit offset (~95 % false Luhn-pass on a 19-digit run)
  - greedy-span Luhn (misses a card beside another group)
  - a separator-grouping grammar
  - entropy scoring
  - a third-party crate
- FOUNDER: boot smoke "Run it"; the pulse-app/4317 host constraint was released for this chunk's P3.
- FOUNDER via overseer (operator pass): fix the npm supply-chain red in-chunk, no deferral; an exception only when no fixed version exists anywhere (with provenance + closing condition).
- FOUNDER 2026-09-30 (relay), two new route entries before Conductor return, in this order:
  1. span-level redaction («да маскировать фрагмент»)
  2. real-model incident surfacing («Чинить в Pulse 0.3.0»; measure WHY first; Conductor's third v3-09 series BLOCKED-ON it)
- A sweep hazard in the hygiene P1 drive-path predicate: it fires on a SYNTHETIC test literal (`C:\Users\runner\…\rm-20260923-093840`) echoed by a panic message into committed evidence logs. The fix was to replace the literal in the logs with a labelled placeholder. The source literal in `scrubber.rs` does not trip hygiene: the P3 rust plane reads source differently.
- `npm audit`'s `fixAvailable` was not used as upgrade guidance (the Tier 2 rule). I checked the published versions with `npm view`, applied a targeted `npm update`, then re-measured the gate.
- Mid-session edits are reverted with anchored Edit calls (the mutation-check discipline); a grep for the mutation token confirmed each revert.

## Outcome
- Acceptance criteria, re-asserted against the diff:
  - (security) Allowed and byte-identical for `rm-20260923-093840` in path, key and `PROJECT:` forms, and for `1790699962319180900` — **met**: `scrubber_allows_non_card_digit_runs` 7/7; no other arm claims them.
  - (security) Luhn-valid forms redact as `credit_card`; catalog 8 arms in the same order — **met**: `scrubber_redacts_luhn_valid_card_forms` 8/8; the order is unchanged in the diff.
  - (security) Mutation check recorded — **met**:
    - RED-before: 7 fail, all "redacted as credit_card".
    - (a) Luhn neutralized reds exactly those 7.
    - (b) Collapsed windowing reds exactly `case_7`/`case_8`.
    - The record is `evidence/mutation-check.md` plus three logs.
  - (tests) `pii_scrub_closure` keeps the date-stamped digest and collapses the card digest to `[redacted:credit_card]` — **met** (2 tests).
  - (tests) `assemble_report` over `/tmp/rm-20260923-093840` gives `project_context == "workspace=/tmp/rm-20260923-093840"` — **met**.
  - (tests) workspace nextest exit 0 — **met** (2429/2429).
  - (arch) security public surface unchanged — **met**: new items are private `fn`/`type`/`const` only.
  - (arch) no manifest or lockfile change — **met for the Rust manifests**, which is what the criterion's gate asserts (the diff probe prints nothing). The npm lockfile moved under the operator's word (the scope record's widening, not a new dependency). The criterion cites arch §Inherited Defaults → Validation (no new dependency), which holds.
  - (obs) no new or changed emit; `redactions_applied` unchanged — **met** (the diff adds no `tracing::` call).
  - (tests) standard gates pass in order with capability-drift last — **met** after the bindings restore.
  - (smoke) P3 warm boot under an `rm-YYYYMMDD-HHMMSS` cwd and data dir — **met**, see `evidence/boot-smoke.md`:
    - 0 `app.panic.fatal`, 0 ERROR
    - digest.assemble.request / interpretation.prompt.assemble / interpretation.incident.created ×3 each
    - `workspace_root_basename = rm-20260930-065023`
    - PID-specific stop left ports closed.
  - (ci) `verdict: green` for the pushed head — **met**: `85e0736`, ci#36675962820, 13/13.
- Gates (implement, final state):

  | gate | verdict |
  |---|---|
  | `cargo fmt --check` | green |
  | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | green |
  | `cargo nextest run --workspace --profile ci -E 'test(/scrubber_redacts_luhn_valid_card_forms\|…/)'` | green (`18 tests run: 18 passed`) |
  | `cargo nextest run --workspace --profile ci` | green (2429) |
  | `git diff --name-only a08ae29… -- Cargo.lock ":(glob)**/Cargo.toml"` | green (no output) |
  | `git diff --name-only a08ae29… -- crates pulse-app xtask scripts` | recorded (roster = the 3 touchpoints at implement; +`pulse-app/ui/package-lock.json` after the operator pass) |
  | `cargo xtask capability-widening-check` | green |
  | `cargo xtask check:ingest-progress` | green |
  | `cargo xtask check:staged-artifacts` | green |
  | `cargo xtask capability-drift` | green on re-run (first run red from the bindings clobber, restored) |
  | `gate.py hygiene` (operator) | clean (after the evidence-log literal fix) |
  | `cargo xtask pre-push:linux` (operator) | green — tree `c1c0c14…` before push 1, tree `e6a178b…` before push 2 |
  | push (operator) | `a08ae29..fcc31b2`, then `fcc31b2..85e0736` |
  | `ci.py conclusion --sha HEAD --wait 2400` (operator) | round 1 `fcc31b2` red (supply-chain, npm advisories — fixed); round 2 `85e0736` green 13/13 (ci#36675962820) |

- Watches:
  - Linux boot silent death: 3 green runs, `dd5c700` (ci#36632205717), `fcc31b2` (ci#36671290813), `85e0736` (ci#36675962820). The watch's own retirement rule (3 greens) is MET.
  - "Met" is not "root cause proven": the run-wrapper added at `dd5c700` (the waiting subshell in `agent-run.sh boot`) removed the app's orphaning, so the original post-ready death may be masked rather than explained.
- Outcome basis:
  - The final state rests on the operator pass (Setup 4 commits `fcc31b2`, `85e0736`) and the final HEAD's CI run ci#36675962820, recorded in `evidence/operator-pass.md`.
  - Implement's P4 report (this conversation) is the basis for gates, mutations and smoke.
  - Operator directive between implement and this report: the npm fix (lockfile only).
- Process hygiene (implement P4 census, re-measured): `pulse-app.exe` pid 64616 terminated, `inject_demo.exe` terminated (timeout 25), gate cargo trees terminated. pre-push:linux ran inside the WSL distro and exited 0; its distro-side processes are `unmeasured — visible from the WSL distro`.
