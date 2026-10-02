# Fan-out results — 2026-09-30-dual-license (wrap 2026-09-30T07-44-36Z)

Entity probe over all seven returns: `entities=0` (no `&lt;` / `&gt;` / `&amp;`; `<chunk-base>` arrived literal).

## Verdicts
- architecture — 1 proposal (D-arch-decisions). Stripped: an out-of-scope note that §Design Philosophy (:4) states
  "twelve library crates … fourteen workspace members total" against the 16 of §Occupied Resources (:202).
- security-plan — `proposals: []` (4 detectors). Stripped: per-detector reasons + a note naming the expected
  §Dependency Security → CI integration amendment as the plan's, not a detector's. Raw twin `.raw-fanout-security-plan.md`.
- design-system — `proposals: []` (2 detectors). Stripped: per-detector reasons. Raw twin `.raw-fanout-design-system.md`.
- layout-templates — `proposals: []` (2 detectors). Stripped: per-detector reasons. Raw twin `.raw-fanout-layout-templates.md`.
- test-plan — 2 proposals (D-tests-framework primary + one `dependent-of`). Stripped: reasons for D-tests-coverage /
  D-tests-obs-harness (no drift), non-target sites :639 / :686 (ci.yml order, unchanged), a detector-fit note (the
  §3 change filed under D-tests-framework whose check names §2/§4).
- obs-plan — `proposals: []` (4 detectors). Stripped: per-detector reasons. Raw twin `.raw-fanout-obs-plan.md`.
- a11y-plan — `proposals: []` (2 detectors). Stripped: per-detector reasons. Raw twin `.raw-fanout-a11y-plan.md`.

## Proposals + dispositions

### A1 — architecture · D-arch-decisions · warning
- section: §Established Decisions
- change: append a **[License]** entry — SPDX `MIT OR Apache-2.0`, set once at `Cargo.toml [workspace.package]`, all 16
  members inherit (no override); npm manifest + lock root carry it; root `LICENSE-MIT` (`Copyright (c) 2026 Turbolet85`)
  + `LICENSE-APACHE` (verbatim, Conductor `cdb7082`, LF); Homebrew `license any_of: ["MIT", "Apache-2.0"]` + Scoop
  `"license": "MIT|Apache-2.0"`; Tauri `bundle.license` inherits the Cargo value; cargo-deny license-checks own crates;
  pinned by the test-only `xtask` `license_check` module.
- basis: architecture.md:73 (insertion after the last entry); report Schema / config + Expected amendments.
- **Disposition: APPLY — routine** (playbook `Accurate this-chunk addition`, :106; check 5 expected-amendment 1 matched).
  Applied text re-derived from the report, not pasted.

### T1 — test-plan · D-tests-framework · warning (primary)
- section: §3 Per-chunk gate discipline — the ordering note (:320)
- change: `capability-drift` runs BEFORE the default-features workspace nextest; the `--features mcp-server`
  `emit_taurpc_bindings` regen is the last cargo-adjacent step; a base-named `git diff --quiet` bindings probe closes;
  the rest of the paragraph (check:staged-artifacts, the staged read) unchanged.
- **Disposition: APPLY — routine** (playbook :106; the operator directive of 2026-09-30 recorded in scope/plan settles
  it; check 5 expected-amendment 3 matched). Scope bound: the base-identity close is what was measured, for a chunk
  adding no procedure. **Cross-edge:** playbook rule :74 states the retired LAST ordering → ESCALATION E1 (judgment
  base, propose→approve).

### T2 — test-plan · D-tests-framework · warning · dependent-of D-tests-framework
- section: §3 standard gate set code block (:296-306)
- change: reorder to fmt → clippy → widening → ingest-progress → staged-artifacts → capability-drift (comment `# BEFORE
  the workspace nextest`) → nextest --workspace → mcp-server regen → base-named bindings probe → npm gates.
- **Disposition: APPLY — routine**, atomically with T1.

### S1 — security-plan · raised by the orchestrator (Validate check 5, expected-amendment 2)
- section: §Dependency Security → CI integration (:216)
- change: record that cargo-deny's `licenses` check covers the workspace's own 16 crates (`deny.toml` sets no `private`
  key; default not ignored; research mutation probe 16/16 at `1dfca74`, cargo-deny 0.20.2) and that the project's own
  expression `MIT OR Apache-2.0` is inside `[licenses] allow`.
- **Disposition: APPLY — routine** (the report substantiates it: Schema / config `deny.toml` sub-bullet; playbook :106).

### A2 — architecture · raised by the orchestrator (the architecture agent's stripped note, verified first-hand)
- section: §Design Philosophy (:4)
- change: "twelve library crates … fourteen workspace members total: twelve library crates + …" → fourteen library
  crates / sixteen workspace members, deferring to §Occupied Resources (:202) as the canonical list (as :4 itself says).
- basis: architecture.md:4 read whole (487 chars); the report's Crates bullet "the 16 workspace members are unchanged".
- **Disposition: APPLY — routine** (playbook `pre-existing reality falsifies a doc claim; impl correct; one artifact`,
  :54 — no impl half). Not this chunk's drift in origin; applied because the report carries the fact and the fix
  completes in one artifact.

## Escalations
- **E1** — `.andromeda/playbook.md:74` (routine-ORDERING rule) states "`cargo xtask capability-drift` runs LAST in the
  P7 light gate — after every regenerating op, including the light gate's own `nextest --workspace`". T1/T2 retire that
  ordering in test-plan §3 per the operator directive; this wrap's own light gate runs the plan's block in the operator
  order (`--only` forbidden). A judgment-base hit → propose → approve → append, never a direct edit.
  - **Resolved WITH the operator (AskUserQuestion): "Supersede + append (Recommended)"** — operator note: the gate order
    is the OVERSEER directive (founder-delegated), 2026-09-30, not a founder ruling; it fixes the every-chunk recurrence
    of `bindings/index.ts` being rewritten before `capability-drift` reads it. Applied: `playbook.md:76` note prefixed
    `SUPERSEDED 2026-09-30 (by the gate-order rule of 2026-09-30-dual-license) —`, the rest verbatim; a new routine
    rule appended stating the order, its scope (base-identity close measured for a procedure-free chunk only) and its
    provenance.

## Drift = 0
A1 · A2 · T1 · T2 · S1 applied; E1 resolved and applied; 0 open.
