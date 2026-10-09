# Fan-out results — 2026-10-04-declared-rust-floor-matches-the-code

Seven Explore doc-agents, one parallel batch; prompts substituted from `amendment-flow.md` and self-checked for
unsubstituted `{…}`. Detector counts sent: arch 2 · security-plan 4 · design-system 2 · layout-templates 2 ·
test-plan 3 · obs-plan 4 · a11y-plan 2 = 19 = the drift-base's 19 entries. `{contracts_line}` dropped for every doc:
`registry.py contracts` read `NOT MIGRATED` (arch · tests · obs · a11y) or `n/a` (security · design · layout).
Entity probe on every return: 0 HTML entities. No raw twin: every `proposals: []` return carried only trailing
`#` commentary, which stripping removed (substance below), and both proposal-carrying returns parsed.

## Verdicts
- **architecture** — 4 proposals (D-arch-decisions ×1 primary + 3 `dependent-of`).
- **security-plan** — `proposals: []`. Stripped commentary: all four detectors clean (no new external input, no
  dependency, keychain change test-only, no scrub shape changed); it noted the plan's expected amendment at
  `security-plan.md:470` falls outside its detectors and is owed through the expected-amendments channel.
- **design-system** — `proposals: []`. Stripped: no UI; 0 hits for the floor tokens.
- **layout-templates** — `proposals: []`. Stripped: no surface; 0 hits.
- **test-plan** — 2 proposals (D-tests-framework, D-tests-coverage). D-tests-obs-harness clean (no harness change).
- **obs-plan** — `proposals: []`. Stripped: no hot path, no logging change, §10 untouched; corpus-key mentions at
  `obs-plan.md:418,:506,:526,:528` are WARN targets / allowlist leaves, not the lock file.
- **a11y-plan** — `proposals: []`. Stripped: no interactive element, no schema change; 0 hits.

## Parsed proposals and dispositions

### architecture
1. `D-arch-decisions` · warning · §Stack and Technologies → Primary language / runtime row (`architecture.md:14`) —
   declared `rust-version = "1.95"` equals the pin, set by the dependency graph (28 packages declare 1.95.0); own-code
   bound ≥ 1.89; witnessed by `declared_floor_equals_the_pinned_channel`; drop the "declared 1.85 is stale, owned by
   the route entry" clause.
   → **apply** (check 1: playbook "Accurate this-chunk addition", routine — every named value is in the report's
   Changes; also the P5-approved expected amendment naming this change).
2. `D-arch-decisions` · dependent-of D-arch-decisions · §Established Decisions → [Primary Language] (`:47`) — the
   header's "code floor rustc ≥ 1.89" restates 1.89 as the floor.
   → **apply** (same rule; expected amendment).
3. `D-arch-decisions` · dependent-of · §Inherited Defaults → Language / runtime (`:355`) — same retired claim.
   → **apply** (same rule; expected amendment).
4. `D-arch-decisions` · dependent-of · §Standard Contracts → `app_info` example (`:103`) — `"rust_version": "1.84.0"`
   → `"1.95"` (the field is `env!("CARGO_PKG_RUST_VERSION")`, the declared string verbatim; report Symbols / APIs).
   → **apply** (same rule; expected amendment).

### test-plan
5. `D-tests-framework` · warning · §4 Unit Test Strategy → Framework (Rust crates) (`test-plan.md:344`) — the
   stale-floor parenthetical becomes declared `1.95` == the pin, dependency-set, witnessed.
   → **apply** (routine, same rule; expected amendment).
6. `D-tests-coverage` · warning · §4 Unit Test Strategy → corpus crate (`:383`) — add the skip-arm no-residue witness
   `corpus_key_skip_arm_leaves_no_lock_file`; crate suite 87 → 88.
   → **apply** (routine, same rule; expected amendment).

## Validate (six checks)
- **Re-derivation tell:** none — every `basis` names a line of the doc itself; `health.rs:360`, the 28-package
  dependency count and the 87 → 88 count are all carried by the report.
- **1 Playbook:** proposals 1–6 → "Accurate this-chunk addition" (`playbook.md:106`), routine; its only exclusion (a
  REVERSAL of a locked decision) does not hold — [Primary Language] Rust 2024 stands, the floor is restated.
- **2 Cross-contradiction:** none (each proposal edits a distinct section).
- **3 Intent-consistency:** the entry and scope item 1 named "≥ 1.89"; the report declares 1.95. Justified — P3
  research measured the dependency floor and the scope's inferred-scope block records it (closed at P3); the plan's
  Constraints reject 1.89. Intent was incomplete and is already amended in `scope.md`. Scope record: none
  (`gate.py scope` clean, 0 recorded).
- **4 Absence needs evidence:** the test-plan agent's "no dependent occurrence" claim cites its grep and dispositions
  its other hits (`:627` httpmock MSRV, `:378`/`:414` other "floor" senses); my P1 sweep agrees
  (`grep -n 'rust-version\|1\.85\|1\.89\|trails the pin'`: arch `:14,:47,:355` + `:103` for `1\.84`; test-plan `:344`;
  security-plan `:470`; 0 elsewhere). The cascade sweep below re-checks it over all seven masters.
- **5 Expected-amendments reconciliation:** arch ×4 → proposals 1–4 · test-plan ×2 → proposals 5–6 ·
  **security-plan §Security Anti-Patterns → Universal (`:470`) — no detector proposed it → raised by the orchestrator,
  routine** (the report substantiates it: Schema / config + the witness; the 1.85.0 Edition-2024 minimum stays true,
  the declared floor 1.95 and its witness are recorded beside it) · leaves (`CLAUDE.md`, `docs/stack.md`,
  `docs/security-summary.md`, `docs/services/corpus.md`) → cascade step 3, not proposals.
- **6 Disproved-claims disposition:** (a) "≥ 1.89 as THE floor" → DISPOSED by proposals 1–3 and 5; (b) plan gate 8's
  unreachable exit atom → a falsified CHUNK-ARTIFACT claim: DISPOSED as "recorded in the report, no amendment owed"
  (the light gate skips it on the overseer's directive).

**Escalations: 0.** Staged to apply: 6 proposals + 1 orchestrator-raised (security-plan `:470`).
