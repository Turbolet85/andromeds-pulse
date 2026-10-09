# Cascade step 2 — the sweep and its dispositions

**The search.** `cascade.py sweep` over `cascade-patterns.toml`, 13 patterns, each with a known-positive control
that fired on the pre-pass masters (baseline `277d65db`): `some-runs` · `four-of-seven` · `app-life` · `three-invoc`
· `step-order` · `ready-on-status` · `handshake-gap` · `before-webview` · `no-adapter-ci` · `dev-null` · `boot-poll`
· `p129-owner` · `harness-status`. The set was derived from all 36 amendments of the pass (34 proposals, 2 raised)
before the first run: the retired wording (counts, the 0.57 s to 1.40 s figure, "three invocations"), the retired
mechanisms (ready on the status verdict alone; the step order boot, status, cleanup; the app stopped before the
webview's request) and the tokens the claims were named by.

**Not looked for:** the claims' restatements in the four sidecars and in chunk artifacts (never swept by
construction); a restatement of the readiness mechanism that uses none of "verdict alone", "until running-healthy",
"readiness poll" or `harness:status`.

**The listing.** First run: `total (13 patterns) · 75 rows over 17 files` · `per class · new 10/4 · standing 33/8 ·
leaf 28/7 · curation 4/2 · base 0/0`. After the leaf re-derivations, the same call: `total (13 patterns) · 66 rows
over 17 files` · `per class · new 10/4 · standing 33/8 · leaf 19/7 · curation 4/2 · base 0/0`. The dispositions below
are of the first listing's rows; the second listing was read for what still stands.

## Masters and key files

| Rows | Disposition |
|---|---|
| `new` 10 rows (architecture 180, 217, 242, 243, 250; security-plan 447; test-plan 100; the key file `5-command-implementation.md` 5, 7) | this pass's own text — the retired figures named as history ("under the former step order …"), or the new mechanism; no change |
| `standing edited`, architecture 180 · 250 (`four-of-seven`, `app-life`, `no-adapter-ci`, `dev-null`, `boot-poll`, `harness-status`) | amended lines; each remaining match read by offset: the history clause this pass wrote, the `frame_cause` cause tokens (true), the pre-build's "timed readiness poll" (true), the ps1 failed-poll sentence (true, the line it names still prints) — no change |
| `standing edited`, architecture 217 · 242 · 243 | amended lines; the remaining `harness:status` matches name the `status` verb's child and the pid-file reader — true; no change |
| `standing edited`, security-plan 395 · 447 | amended lines; 395's `boot` polls "the spawn record" (true); 447's figures are the history clause; no change |
| `standing`, security-plan 222 `handshake-gap` | read at offset 131: "the 2026-08-23 measured gap is CLOSED" — another gap (a scrub-coverage one); a true claim sharing the token; no change |
| `standing edited`, obs-plan 132 · 445 · 545 · 559 | amended lines; the four-logs count, the 0.57 s to 1.40 s figure, the back-to-back step order and the before-the-webview stop are kept as dated history of the former step order; the cause-token list is true; no change |
| `standing edited`, test-plan 66 · 134 · 135 | amended lines; 66 names `harness:status` as the `status` verb's verdict (true); 134's earlier widenings are history; 135's token list is true; no change |
| `standing`, the key file `5-command-implementation.md` 4 · 5 · 17 | 4 and 5 amended; 17 is the `status` verb's own Command body — true; no change |
| `standing`, the key file `per-chunk-gate-discipline.md` 40 | read at offset 2709: the `status` verb delegates to `harness:status` — true; no change |
| `standing`, the key file `pid-file.md` 5 | "the latter is `harness:status`'s `ended`" — true but no longer the whole set: **amended** to name the three verbs that report the record |
| `standing`, the key file `pid-file.md` 6 · `status-endpoint-shape.md` 3 | the `status` verb reads through `harness:status`; the verb's output is that verdict — true; no change |
| `three-invoc` | 0 rows after the pass; its control fired on the pre-pass architecture 249 — a statement about this pattern |

## Leaves (re-derived, cascade step 3)

| Rows | Disposition |
|---|---|
| `.claude/rules/observability.md` 64 · 87 · 99 | re-derived: the `ui.webgpu.adapter` CI witness, which harness leg reads which verdict, the CI frame line |
| `.claude/rules/verification-harness.md` 22 | re-derived: the `boot` bullet (the `harness:ready` poll, the closed gap, the failure line, the wrapper's streams); a `harness:settled` bullet added to the xtask list |
| `.claude/rules/verification-harness.md` 24 · 29 · 59 · 89 | the `status` verb, the status-shape note, the pid lifecycle, the `frame_cause` tokens — true; no change |
| `.claude/rules/security.md` 17 | re-derived: the harness-only state-file class with its new readers, files and variable read |
| `.claude/docs/obs-summary.md` 80 · 68 | re-derived. Row 68 (the same harness-legs sentence as observability.md 87) was in the first listing; the orchestrator read that listing through two partial views that skipped it, and the second run's rows, read whole for leaves, showed it |
| `.claude/docs/tests-summary.md` 12 | re-derived: the `boot` row; 14 (the `status` row) true, no change |
| `.claude/docs/security-summary.md` 43 | re-derived |
| `.claude/docs/commands.md` 86 | re-derived; two xtask lines added after 100; 88 and 100 true, no change |
| `CLAUDE.md` | no row; its `GENERATED:setup:warnings` carve-out sentence recomputed from the amended security-plan §Input (the class's new members named). The module and overview lines name xtask's verbs selectively and never named `harness:status`; recomputed, no change |

## Curation homes and judgment bases

| Rows | Disposition |
|---|---|
| `.claude/rules/testing.md` 218 (`dev-null`) | a shell redirect in a recipe — not the claim; no change |
| `.claude/docs/session-learnings.md` 2554 · 2561 · 2563 (`harness-status`) | an entry about the `cargo xtask` alias — true; no change |
| `playbook.md`, `drift-base.md` | 0 rows |
