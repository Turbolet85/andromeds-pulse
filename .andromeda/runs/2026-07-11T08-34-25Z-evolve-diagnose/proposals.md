# Evolve Diagnosis — andromeda-pulse · Epoch 3 — State honesty & legibility · 2026-07-11T08:34Z

> Founder-facing, evidence-backed, **obligation-free**: accept / reject / defer / modify any item
> with no mechanism-side consequence. Nothing here is applied, queued, or remembered (stateless
> re-run; prior diagnoses are not inputs). Every finding is grounded in
> `.andromeda/friction-log.ndjson` (Epoch-3 slice); raw query outputs are the `q-*.json` twins
> beside this file.

## Mechanism health

- **196 records** — 126 `step` / 70 `friction` · **0 unparseable**.
- **Coverage: complete.** Every (skill, step) checkpoint fired for all 9 diagnosed chunks —
  phase 5×9 · implement 3×9 · wrap 5×9 · orientation ×9 = 126 · **zero gaps**.
- **Untyped rate: 6/70 (8.6%)** · **problem-fact fill: 44/126 step records (35%)**, 53 facts — the
  deviation scan is live throughout the diagnosed range.
- **Calibration boundary:** capture began **mid-Epoch-3** (earliest `2026-07-05`); the epoch's
  first chunk `2026-07-01-live-only-service-truth` **predates capture** (0 records) — a boundary,
  not "nothing occurred." Diagnosed range: 9 chunks, `2026-07-05` → `2026-07-10`.
- **Epoch shape:** **frontend-only** (8/9 chunks zero-`.rs`-delta; one backend chunk,
  `constellation-severity-live-wiring`) — this shape drives most patterns below.

---

## Proposals (typed patterns)

### P1 — implement/fix-loop + wrap/gates · `tooling.gate-deferral` — 13 cases (7+6)
**Pattern:** Every zero-`.rs` webview chunk deferred the expensive workspace Rust gates
(`clippy`/`nextest --workspace`/`capability-drift`/self-verify BOOT half) at `/implement` and again
honored-deferred them at the wrap light gate — 8/9 chunks, 29 deferral impacts.
**Evidence (all 13):** anomaly-surfacing (fix-loop def 4 / gates **CLOSED green** when it became
Rust-touching), constellation-severity (fix 1 / gates 2, green same-session), dropdown-bug (fix 4 /
gates 4), traces-auto-refresh (fix 3), self-explaining-empty (fix 5), traces-layout-polish (fix 1 /
gates 1), plain-language (gates 2, green same-session), floating-window (fix 1 / gates 1 — the one
genuine `.rs` deferral riding forward). Full rows in `q-typed.json`.
**Proposal:** Largely the source-delta-proportional rule working as designed (several closed green at
`.rs` chunks). The residue for the founder's eye is the **environmental driver** — a cold ~30-binary
test-link "OOM/disk-risky on 26G free" + an rlib-format mismatch forcing cold rebuilds. Change-shape
lives in tooling: a cached/incremental workspace-test target so the re-run never needs deferring,
plus free-disk as a standing step. See **L6** (deferred-forever) — this is its typed face.

### P2 — implement/smoke · `tooling.harness-friction` — 6 cases · weight 15 (highest)
**Pattern:** The headful smoke harnesses (`cargo xtask self-verify`, `agent-run.ps1`) boot the
**stale compile-time-embedded frontend** and default to a `--release` cold rebuild, so they cannot
validate a webview-only change — a manual "warm re-embed" boot was substituted in 6/9 chunks.
**Evidence (all 6):** dropdown-bug (self-verify boots stale embed, dialogue 2), floating-window
(--release can't verify cross-window docking), plain-language (warm-boot SIGILL-132 teardown, retry
1), traces-layout-polish (inject_demo finite storm + 60s window → 2 false empties, dialogue 2),
anomaly-surfacing (`lint:a11y` Windows quoting bug), constellation-severity (obs `item_count`
REDACTED by the AllowList). `q-typed.json`.
**Proposal:** The recurring core is that **the smoke harness can't see a fresh webview build without
a manual re-embed** and its `--release` default is too heavy for the inner loop. Change-shape: a
first-class **"warm re-embed webview smoke"** mode in `self-verify`/`agent-run`. The tail three are
individually-fixable harness bugs already logged as Epoch-4 CARRYs. This is the typed face of **L2**.

### P3 — wrap/curation · `ambiguity.filter-borderline` — 7 cases
**Pattern:** Curation Filter-1 (dedup) and Filter-4 (confidence ~0.6) sat at a decision edge in 7/9
chunks — no wrong outcome, a recurring judgment cost.
**Evidence (all 7):** legible-constellation (Filter-1 dedup edge), constellation-severity ("the
formula's implicit BASE is unspecified"), dropdown-bug (Filter-1 overlap, KEEP additive),
traces-auto-refresh (~0.65 + topical dedup), plain-language (several near 0.6), traces-layout-polish
(~0.6 single-session gotcha), floating-window (Filter-1 edge, cross-window is a new facet).
`q-typed.json`.
**Proposal:** (a) State **Filter-4's implicit confidence BASE** (or a worked example) in
`curation-tier-decision.md` — the records literally note "+0.2 alone reads as 0.2." (b) Add a
**Filter-1 tie-breaker** ("a new surface/facet on a known lesson = KEEP as additive") — the same edge
recurred 3×. Low urgency (zero downstream impact).

### P4 — phase/take-up · `ambiguity.version-derivation` — 3 cases
**Pattern:** A gitignored sibling `andromeda-pulse-0.4.0-incubator/` at the project root makes the
"highest `{project}-X.Y.Z` dir = active version" rule mis-rank to **0.4.0-incubator over 0.3.0** —
resolved each time via master-route's last heading.
**Evidence (all 3):** constellation-severity, dropdown-bug (extra_reads 1), traces-auto-refresh
(extra_reads 1). `q-typed.json`.
**Proposal:** The most concrete, mechanical fix in the epoch: **exclude non-clean-semver dirs**
(`-incubator`) from the active-version glob, or **derive active-version from master's last heading
first** and use the dir glob only to confirm (inverting today's glob-then-confirm). Reinforced by
**L4** (the orientation-side band-aid theme). Removing it makes every session's take-up deterministic.

### P5 — new-session/orientation · `tooling.health-false-red` — 3 cases
**Pattern:** Health **Check 6 (arch staleness)** fires false-red every session — `architecture.md
~234h newer than CLAUDE.md` past the 24h threshold — because arch.md is hand-amended per-chunk while
CLAUDE.md `@import`s it.
**Evidence (all 3):** three orientation runs, all the same benign Check-6 warning. `q-typed.json`.
**Proposal:** Check 6 assumes CLAUDE.md is regenerated whenever arch.md changes — false for the
per-chunk arch-amend workflow. Change-shape: in `health-criteria.md`, compare arch.md against its
`*-amendments.md` sidecar mtime (or downgrade Check 6 to informational when the sidecar shows recent
amendments). Non-gatekeeping today, so this only removes noise.

### P6 — phase/validate · `contract.mechanical-check` — 4 cases
**Pattern:** Two P5 checks WARN by construction on frontend chunks — Check 6 (plan size-band: 70–84
lines < 150 floor) and Check 7 (code-graph-consulted: no tree-query trace, TS/TSX surface is
graph-not-applicable).
**Evidence (all 4):** anomaly-surfacing (both), traces-auto-refresh (Check 7), plain-language
(Check 4 UI-gate correctly caught a missing self-verify — the one true signal), traces-layout-polish
(both). `q-typed.json`.
**Proposal:** Wire the recognized **`graph-not-applicable`** term into `validation.md` — "a plan whose
modify-set is entirely webview/TS skips the code-graph-consulted check and uses a lower size floor."
Scope the exemption to Checks 6/7, **not** Check 4 (which worked).

### P7 — wrap/route-resolve · `contract.carry-no-owner` — 3 cases
**Pattern:** Operator CARRYs recur with **no in-version markerless entry to pin to** — scattered to a
new Epoch entry, the handoff Notes, or a doc-comment.
**Evidence (all 3):** anomaly-surfacing (internal-scroll CARRY → new entry), legible-constellation
(per-dot-labels → handoff Notes), traces-auto-refresh (viz `next_cursor` → doc note). `q-typed.json`.
**Proposal:** An explicit **residual/backlog bucket** (a standing "unpinned carries" tail, or
`backlog.md`) so owner-less CARRYs land in one auditable place. Live symptom: Epoch-4's P-076 entry
has accreted a very long CARRY chain. Tidiness/traceability; low urgency.

### P8 — phase/plan · `ambiguity.scope-question` — 3 cases · weight 9
**Pattern:** Genuine scope ambiguity needing an AskUserQuestion round in 3 chunks.
**Evidence (all 3):** dropdown-bug (popover token: RESEARCH-CORRECTS-INTENT), traces-layout-polish
(label-collision approach — testability-vs-fidelity), floating-window (window mechanism +
content-reuse, 2 in 1 round). `q-typed.json`.
**Proposal:** **No change recommended — the pipeline working as designed.** Each ask was a real user
decision, single-round, recommendation-marked. Surfaced only because it cleared the threshold; the
weight is dialogue, not defect.

---

## Cross-step chains (starting heuristics)

**None surfaced.** 2 `input.*` frictions (chrono dev-dep at plain-language/code; report-window
under-spec at floating-window/fix-loop) and 1 `consumed.quality=wrong` (dropdown-bug/smoke —
agent-harness wrong for a webview chunk, the P2/L2 root). The within-chunk lineage join to a
formally-`ok`-but-signalled producer returned **0 chains**; no (producer, artifact, consumer) shape
recurred across ≥2 chunks. Honest read: the starting heuristics (no transitive chains, no scoring)
are too thin for Epoch 3's shape — a miss that feeds the dogfood-calibration of these heuristics.
(`q-chains.json`.)

---

## Level candidates (systemic-masked-as-project)

_Stage 4 ran two passes. **Pass A** clustered all 40 `workaround/removed-cause` facts by the
obstacle each routes around (band-aid detection, independent of typed correlates) → 5 themes at
threshold. **Pass B** ran the form signatures. Full detail in `q-level.json`._

### L1 — band-aid · **the raw-twin save contract is heavier than the (already-clean) agent returns** — 11 facts / 7 chunks · **no typed correlate**
**Obstacle routed around:** phase-distill's "save-raw-return-then-strip" + wrap-reconcile's "7 per-doc
`.raw-fanout-{doc}.md`" twin discipline.
**Facts (process/workaround, all 11):** distill wrote `{specialty}.md` then `cp`-d to `.raw-*` (or
skipped the twin) in legible-constellation, constellation-severity, traces-auto-refresh,
plain-language, self-explaining-empty, traces-layout-polish; reconcile consolidated the 7
`.raw-fanout-{doc}.md` into one file in legible-constellation, constellation-severity, dropdown-bug,
traces-auto-refresh, self-explaining-empty. The recorded reason is invariant: *"returns were already
clean (raw==stripped) / all 7 proposals:[] — nothing to strip or parse."*
**Level hypothesis:** This is the flagship Pass-A catch — **it left no typed friction at all**, only
deviation-scan facts, because the workaround is smooth. The contract assumes distiller/detector
returns arrive dirty (preamble/trailing to strip) and per-doc-separable; in practice Explore agents
return clean, header-first, and (for reconcile) empty. The cause lives in the **mechanism's
raw-twin contract**, not the project.
**Proposal:** A `fan-out.md` / `amendment-flow.md` refinement: make the per-doc raw twin
**conditional** — save a raw twin only when the return needed stripping (raw≠stripped) or carried a
proposal; otherwise a single consolidated audit file with the verdicts is the sanctioned artifact.
That would turn the epoch's single most frequent deviation into the documented path.

### L2 — band-aid · **the default smoke harness can't validate a fresh webview build** — 8 facts / 7 chunks
**Obstacle:** `scripts/agent-run.sh` + `cargo xtask self-verify` (boot the stale embedded frontend;
`--release` cold default). **Facts (process/workaround):** warm-re-embed manual boot substituted in
constellation-severity, dropdown-bug, traces-auto-refresh, plain-language (×2), self-explaining-empty,
traces-layout-polish, floating-window.
**Level hypothesis:** The systemic reading of **P2** — the same obstacle, seen as a band-aid: a
mechanism/harness default that no webview chunk can use, worked around identically every time (now
codified in a user memory-directive + `frontend.md`). Cause lives in the harness.
**Proposal:** As P2 — a first-class **warm-re-embed webview smoke** mode; L2 is the evidence that it
is not incidental but the standing path for every UI chunk.

### L3 — band-aid · **the recursive-delete guard blocks `rm -rf` in smoke setup** — 3 facts / 3 chunks
**Obstacle:** the sandbox destructive-command / recursive-delete deny rule. **Facts
(environment/workaround):** `rm -rf` of the temp data dir was permission-denied → restructured to
mkdir-only on a fresh unique subdir, in dropdown-bug, traces-auto-refresh, self-explaining-empty.
**Level hypothesis:** An environment constraint absorbed at the project level each chunk (the fix is
already encoded in a `testing.md` 2026-07-05 "split-into-granular-safe-steps" note).
**Proposal:** Bless the pattern in the smoke recipe/harness — **temp data dirs use a fresh unique
subdir per run and never delete** (mkdir-only), so the guard is never hit. Small, removes a recurring
environment workaround.

### L4 — band-aid · **the v3 version-dir-at-repo-root layout** — 3 facts / orientation + take-up
**Obstacle:** the active-version / version-dir location assumption. **Facts (environment/process):**
orientation's `.andromeda/{version}/*` glob missed the repo-root `./andromeda-pulse-0.3.0/` (×2,
needing a repo-wide find); take-up's highest-`X.Y.Z` rule nearly picked the `-incubator` sibling.
(Plus the `removed-cause` observation: the untracked `-0.4.0-incubator/` had to be gitignored to keep
a commit chunk-scoped.)
**Level hypothesis:** The systemic reading of **P4** — the v3 migration relocated the version dir but
the location probes (orientation health-check + take-up derivation) still assume the pre-migration
`.andromeda/` subdir. Cause lives in the shared derivation rule.
**Proposal:** As P4 (master-heading-first derivation + exclude `-incubator`), applied to **both**
`health-criteria.md` and `promotion.md`/Setup so orientation and take-up agree. Reinforced by the
emerging untyped cluster (below).

### L5 — band-aid · **the full a11y sweep is too heavy to re-run on byte-identical inputs** — 3 facts / 3 chunks
**Obstacle:** the fix-loop / light-gate "run the full `test:a11y` chain" (~1min: lighthouse / pa11y /
aggregator / regression). **Facts (process|resources/workaround):** legible-constellation ran
`verify:contrast` + the targeted p11 spec instead of the full chain; self-explaining-empty and
traces-layout-polish did NOT repeat the sweep at the P7 light gate (byte-identical ui/dist, green at
P2).
**Level hypothesis:** A resources/process obstacle absorbed at the project level. Distinct from L6
(different tool: a11y sweep, not cargo). Note this is largely **the source-delta-proportional rule
applied correctly** to a11y — the band-aid framing asks only whether the *re-run cost itself* is the
recurring obstacle.
**Proposal:** Make the a11y sweep's **byte-identical-input skip explicit** in the light-gate rule
(gate on a ui/dist hash: unchanged since the green P2 run → record "a11y sweep unchanged, skipped"
rather than a per-chunk workaround). Low urgency; formalizes what already happens correctly.

### L6 — deferred-forever · workspace-cargo gate deferrals — 7 deferred facts · 15 signals · 8 chunks
**Facts:** `resources`-nature deferrals at nearly every wrap `gates` step (floating-window "nextest
RUN honored-deferred — cold ~30-binary test-link OOM/disk-risky on 26G"; +6 more, `q-level.json`).
**Level hypothesis:** Mostly by-design (a frontend chunk has nothing to re-test; several closed green
at `.rs` chunks), but the recurring **driver is environmental** (26G disk + cold link + rlib
mismatch), and it now carries **one genuine unclosed `.rs` deferral** (the `window.rs` label-enum
test-run) into Epoch 4. Same root as P1.
**Proposal:** An incremental/cached workspace-test path + free-disk-as-setup (pipeline/environment,
not per-chunk project deferral) + an explicit **deferred-gate closure ledger line** so the carried
`.rs` deferral is provably closed at Epoch-4's first `.rs` chunk.

### L7 — override · route-resolve trajectory-HALT overridden — 2 facts on one rule (5 overrides total)
**Facts:** `route-resolve` trajectory-HALT overridden in legible-constellation + constellation-severity
("new-chunk-ahead entries would normally HALT+dialogue at P5; operator pre-directed them"). (Plus 3
single overrides: plan premise falsified, smoke self-verify ×2 — the P2/L2 root.)
**Level hypothesis:** The **P5 trajectory-HALT rule assumes trajectory edits are always a surprise** —
here they were pre-authorized in-session, so the HALT fires and is immediately overridden.
**Proposal:** A `route-resolve.md` refinement recognizing a **pre-authorized trajectory edit** as an
AUTO-with-note path rather than HALT-then-override. Narrow (2 facts, one rule) — surfaced for the
founder's judgment on whether 2× warrants it.

### L8 — chronic-degrade · gate-deferral (8 chunks) + harness-friction (6 chunks) recur without halting
**Facts:** Both recur across the whole epoch with **soft_exit total = 0** and only 3 step-halts in
126 records. **Level hypothesis:** exactly the degradations the attended-halt model structurally
never surfaces — the two most frequent friction types, silent and constant. **Proposal:** no new fix
beyond P1/P2/L1/L2/L6 — L8 is the **confirmation** that the epoch's top frictions are chronic, so
they warrant a tooling-level fix precisely because a halt will never raise them.

---

## Playbook-extension candidates (untyped patterns, F-4)

**None meet the F-4 threshold** (n≥3 untyped in-epoch; no prior epoch in the ledger). Two **emerging
clusters** (n=2) to watch next epoch:
- **version-dir-layout at repo root** (2 untyped orientation records) — **reinforces P4 + L4**;
  candidate type `contract.version-dir-layout` if it recurs in Epoch 4.
- **intent/spec premise falsified by shipped code** (2 untyped: F8-sort premise-correction +
  layout-templates-widget-vs-constellation; the typed `contract.intent-divergence` n=2 reinforces) —
  candidate type `contract.spec-lags-code` if it recurs.

---

## Below threshold — no action

_Visible for the founder's eye only._

**Typed groups below threshold (20):** `reconcile/ambiguity.playbook-no-match` (n2,w6) ·
`smoke/retry.smoke-reentry` (n2,w6) · `route-resolve/ambiguity.trajectory-halt` (n2,w4 — pairs with
L7) · `curation/ambiguity.tier-routing` (n2) · `smoke/tooling.subprocess-bounds` (n2) ·
`validate/contract.intent-divergence` (n2 — reinforces the spec-lags-code cluster) ·
`orientation/UNTYPED` (n2 — the version-dir cluster) · `fix-loop/UNTYPED` (n2) · plus 12 singletons
(`smoke/process`, `fix-loop/tooling.environmental`, `code/input.conventions-gap`,
`fix-loop/input.plan-step-ambiguous`, `research/ambiguity.scope-boundary`, `research/UNTYPED`,
`reconcile/contract.false-positive-proposal`, `plan/UNTYPED`, `validate/contract.matrix-claim`,
`distill/contract.extract-format`, `distill/contract.binding-contradiction`,
`fix-loop/ambiguity.scope-pressure`).

**Pass-A singleton workarounds (12, below the band-aid theme threshold):** research skipped
code-graph-first (value-flow question) · validate LINK→MINT cap (P-079) · code combined 2 plan fns ·
code integration-file proof tests · fix-loop `cargo build --tests` to de-race rlib/OOM (the L6
environmental root) · distill `cp` cd-d into run dir → cwd-persist append fail · code chrono dev-dep
→ SystemTime · fix-loop `fmt --check | tail` masked exit · curation `cat >>` vs `.tmp`+rename ·
reconcile wireframe prose edited but ASCII sketch left · fix-loop Report hosting restructured
(plan-gap) · gates gitignored `-incubator` (removed-cause, folded into L4).

---

_End of diagnosis. Nothing above is applied or remembered; the founder takes it from here._
