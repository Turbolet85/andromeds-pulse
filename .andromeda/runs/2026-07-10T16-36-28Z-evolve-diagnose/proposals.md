# Evolve Diagnosis — andromeda-pulse · Epoch 3 — State honesty & legibility · 2026-07-10T16:36Z

> Founder-facing, evidence-backed, **obligation-free**: accept / reject / defer / modify any item
> with no mechanism-side consequence. Nothing here is applied, queued, or remembered. Every finding
> below is grounded in `.andromeda/friction-log.ndjson` (Epoch-3 slice); raw query outputs are the
> `q-*.json` twins beside this file.

## Mechanism health

- **196 records** — 126 `step` / 70 `friction` · **0 unparseable**.
- **Coverage: complete.** Every (skill, step) checkpoint fired for all 9 diagnosed chunks —
  phase 5×9=45 · implement 3×9=27 · wrap 5×9=45 · new-session orientation ×9. **Zero gaps** (no
  checkpoint silently failed to fire).
- **Untyped rate: 6/70 (8.6%)** — low; the typed vocabulary is catching most friction.
- **Problem-fact fill: 44/126 step records (35%)** carry a `problem` deviation-scan fact (53 facts
  total) — the scan is live throughout the diagnosed range.
- **Calibration boundary in range:** capture began **mid-Epoch-3** — earliest record
  `2026-07-05-anomaly-surfacing`. Epoch 3's first chunk `2026-07-01-live-only-service-truth`
  **predates capture** (0 records) — read as a boundary, not as "nothing occurred." The 9 recorded
  chunks are `2026-07-05` → `2026-07-10`.
- **Epoch shape note:** Epoch 3 is **frontend-only** (8 of 9 chunks zero-`.rs`-delta; one backend
  chunk, `constellation-severity-live-wiring`). This shape drives the two dominant patterns below.

---

## Proposals (typed patterns)

### P1 — implement/fix-loop + wrap/gates · `tooling.gate-deferral` — 13 cases · weight 14 (combined)
**Pattern:** On every zero-`.rs`-delta webview chunk, the expensive workspace Rust gates
(`clippy --workspace` / `nextest --workspace` / `capability-drift` / self-verify BOOT half) were
deferred at `/implement` and again honored-deferred at the wrap light gate — "re-run at the next
`.rs`-touching chunk" — 8 of 9 chunks, 29 total deferral impacts.

**Evidence:** ALL 13 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| anomaly-surfacing | fix-loop: 4 TS files, deferred clippy/nextest --workspace/self-verify boot/capability-drift | deferred 4 | q-typed.json |
| constellation-severity | fix-loop: backend chunk deferred self-verify release-build + a11y half (zero UI delta) | deferred 1 | q-typed.json |
| dropdown-layout-bug | fix-loop: zero-.rs webview, deferred clippy+nextest+capability-drift+self-verify boot | deferred 4 | q-typed.json |
| traces-auto-refresh | fix-loop: deferred clippy+nextest+capability-drift (continuing P-080 webview deferral) | deferred 3 | q-typed.json |
| self-explaining-empty | fix-loop: zero-.rs, deferred 5 gates (fmt/clippy/nextest/capability-drift/self-verify) | deferred 5 | q-typed.json |
| traces-layout-polish | fix-loop: zero-.rs, cold workspace gates deferred | deferred 1 | q-typed.json |
| floating-window | fix-loop: nextest RUN deferred — cold ~30-binary test-link on 26G disk + rlib-format mismatch | deferred 1 | q-typed.json |
| anomaly-surfacing | gates: **deferral CLOSED** — P-068 became Rust-touching, all 4 gates re-ran GREEN | deferred 0 | q-typed.json |
| constellation-severity | gates: deferred cold re-run — ran green at /implement same session (1727/1727) | deferred 2 | q-typed.json |
| dropdown-layout-bug | gates: honored the zero-.rs deferral — cold rebuild for a webview chunk is redundant | deferred 4 | q-typed.json |
| plain-language-status | gates: clippy+nextest deferred — ran green in implement P2 same session (1730) | deferred 2 | q-typed.json |
| traces-layout-polish | gates: honored zero-.rs deferral of the cold cargo gates | deferred 1 | q-typed.json |
| floating-window | gates: nextest RUN honored-deferred — changed-surface gates all re-ran green | deferred 1 | q-typed.json |

**Proposal:** This is largely the **source-delta-proportional rule working as designed** — a
frontend epoch has nothing new to re-compile-test, and several deferrals *did* close green at the
`.rs`-touching chunks (anomaly-surfacing, constellation-severity, plain-language). The residue worth
the founder's eye is the **environmental driver**: the wrap-time re-run is a *cold ~30-binary
test-link* that is "OOM/disk-risky on 26G free" and hits an "rlib-format mismatch forcing a cold
rebuild." The change-shape that would remove the cause lives in tooling, not the per-chunk project
call — e.g. a cached/incremental workspace-test target (or a `sccache`/`--no-run` link-reuse path)
so the re-run is cheap enough to never defer, plus freeing disk as a standing setup step. See **L1**
(the level read) and **L3** (the chronic-degrade read) — this pattern is the epoch's spine.

### P2 — implement/smoke · `tooling.harness-friction` — 6 cases · weight 15 (highest)
**Pattern:** The project's headful smoke harnesses (`cargo xtask self-verify`, `scripts/agent-run.ps1`)
boot the **stale compile-time-embedded frontend** and default to a `--release` cold rebuild, so they
cannot validate a fresh webview-only change — the operator hand-drove a "warm re-embed" manual boot
(npm build → `cargo build -p pulse-app` re-embed → launch debug binary → inject_demo → obs-log +
visual) in 6 of 9 chunks.

**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| dropdown-layout-bug | self-verify + agent-run.ps1 boot the STALE embedded frontend → can't validate a fresh webview change; manual re-embed required | dialogue 2 | q-typed.json |
| floating-window | plan listed `self-verify` but it boots --release (cold, costly on 26G) + can't verify cross-window docking; substituted warm-re-embed debug smoke | — | q-typed.json |
| plain-language-status | warm-re-embed FIRST run exited 132 (SIGILL) at teardown after ~68k obs lines; SECOND run clean exit 0 (known-benign teardown) | retries 1 | q-typed.json |
| traces-layout-polish | inject_demo runs a finite storm then exits + 60s query window → spans age out → 2 false "No traces yet" episodes | dialogue 2 | q-typed.json |
| anomaly-surfacing | `npm run lint:a11y` Windows Git-Bash quoting bug — inline `--rule` args don't survive shell/npm quoting (harness bug, not a code violation) | — | q-typed.json |
| constellation-severity | obs-extract acceptance wanted `incidents.list_active` `item_count` in the log, but it's REDACTED by the default-deny AllowList | — | q-typed.json |

**Proposal:** The recurring core (3 of 6) is that **the headful smoke harness can't see a fresh
webview build without a manual re-embed**, and its `--release` default is too heavy for the
inner-loop. A change-shape that removes the cause: a first-class **"warm re-embed webview smoke"**
mode in `xtask self-verify` / `agent-run` (debug binary, re-embed the freshly-built frontend, launch
+ health-poll + inject_demo) so the operator's recurring manual recipe becomes a supported harness
path. The tail cases are three smaller, individually-fixable harness bugs already logged as Epoch-4
CARRYs (lint:a11y quoting; inject_demo continuous-stream; obs AllowList vs the acceptance target).
This pattern **also fuels L2's override signature** (smoke self-verify overridden).

### P3 — wrap/curation · `ambiguity.filter-borderline` — 7 cases · weight 7
**Pattern:** Curation Filter-1 (dedup) and Filter-4 (confidence ~0.6) sat at a decision edge in 7 of
9 chunks — no halt, no wrong outcome, but a recurring judgment cost.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| legible-constellation | C1 (operator-visual-verify) at the Filter-1 dedup edge vs the same-day boot-smoke entry; judged distinct | — | q-typed.json |
| constellation-severity | single-source-parity near Filter-4 ~0.6 — "the formula's implicit BASE is unspecified" so +0.2 reads ambiguously | — | q-typed.json |
| dropdown-layout-bug | Filter-1 dedup edge: popover entry overlaps the 2026-06-30 fixed-height entry (same surface); judged KEEP (additive) | — | q-typed.json |
| traces-auto-refresh | candidate B (SC 4.1.3 announce) near Filter-4 ~0.65 with topical dedup overlap; kept Tier 2 | — | q-typed.json |
| plain-language-status | several near the 0.6 threshold — SIGILL-132 (~0.5) rejected, chrono/SystemTime (~0.6) kept | — | q-typed.json |
| traces-layout-polish | inject_demo aging-out near Filter-4 ~0.6 — valuable-but-single-session; kept Tier 3 | — | q-typed.json |
| floating-window | operator-visual-verify at the Filter-1 dedup edge vs the 2026-07-05 lesson; kept (cross-window is a new facet) | — | q-typed.json |
| —

**Proposal:** Two concrete sharpenings would remove most of this recurring judgment cost: (a)
**Filter-4's confidence heuristic has an unspecified implicit BASE** — the records literally note
"specific-technical-detail +0.2 alone reads as 0.2"; stating a base value (or worked example) in
`curation-tier-decision.md` would make the ~0.6 boundary mechanical. (b) The **Filter-1 dedup edge
recurs on the same axis** — "operator-visual-verify-for-layout" was weighed as dup-vs-distinct in 3
chunks; a tie-breaker line ("a new surface/facet on a known lesson = KEEP as additive") would settle
it. Low urgency (zero downstream impact) — this is judgment friction the guide could reduce, not a
defect.

### P4 — phase/take-up · `ambiguity.version-derivation` — 3 cases · weight 3
**Pattern:** A gitignored sibling `andromeda-pulse-0.4.0-incubator/` at the project root makes the
"highest `{project}-X.Y.Z` dir = active version" rule mis-rank to **0.4.0-incubator over the real
0.3.0** — resolved each time by falling back to master-route's last heading.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| constellation-severity | `-0.4.0-incubator` dir could make the highest-X.Y.Z heuristic pick 0.4.0; resolved via master last-heading | extra_reads 0 | q-typed.json |
| dropdown-layout-bug | sibling `-0.4.0-incubator` coexists with active 0.3.0; resolved by highest-clean rule + master has no 0.4.0 heading | extra_reads 1 | q-typed.json |
| traces-auto-refresh | two root version dirs; the literal highest-X.Y.Z rule points at `-incubator` while master last-heading identifies 0.3.0 | extra_reads 1 | q-typed.json |

**Proposal:** The most concrete, mechanical fix in the epoch. The active-version derivation rule
(shared by new-session / phase Setup) could **exclude non-clean-semver dirs** (`-incubator`,
suffixed) from the glob, or — stronger — **derive active-version from master-route's last
`## {project}-{version}` heading first** and use the dir glob only to confirm, inverting today's
"glob-then-confirm." Reinforced by the orientation-side untyped records (see the emerging cluster
under Playbook-extension candidates). Removing this makes every future session's take-up
deterministic against the incubator sibling.

### P5 — new-session/orientation · `tooling.health-false-red` — 3 cases · weight 3
**Pattern:** Health **Check 6 (architecture staleness)** fires false-red every session —
`architecture.md ~234h newer than CLAUDE.md` past the 24h threshold — because arch.md is
hand-amended per-chunk during active work while CLAUDE.md `@import`s it (the expected workflow).

**Evidence:** ALL 3 cases (all orientation runs; `chunk:null`) —

| session | what | impact | evidence |
|---|---|---|---|
| orientation | Check 6: arch.md ~234h newer than CLAUDE.md — benign (arch amended per-chunk + v3 migration) | — | q-typed.json |
| orientation | Check 6 warns arch.md newer by ~234h (>24h) — expected manual-arch-edit workflow | — | q-typed.json |
| orientation | Check 6 arch-staleness — normal chunk-driven /scope-arch amendments, not a setup-integrity problem | — | q-typed.json |

**Proposal:** Check 6's 24h staleness heuristic **assumes CLAUDE.md is regenerated whenever arch.md
changes** — false for this project's per-chunk arch-amend workflow. A change-shape that removes the
false-red: in `health-criteria.md`, either compare arch.md against its own `*-amendments.md` sidecar
mtime (staleness = arch body newer than its distillation *and* no amendment activity), or downgrade
Check 6 to informational when the sidecar shows recent per-chunk amendments. Purely a
health-check-calibration change (Check 6 is non-gatekeeping today, so this only removes noise).

### P6 — phase/validate · `contract.mechanical-check` — 4 cases · weight 5
**Pattern:** Two P5 mechanical checks WARN by construction on frontend chunks — Check 6 (plan
size-band: 70–84 lines < the 150 floor) and Check 7 (code-graph-consulted: no tree-query trace
because a TS/TSX surface is graph-not-applicable).

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| anomaly-surfacing | both soft checks WARN on a small webview chunk — size-band (70 lines) + code-graph (no Rust symbols) | — | q-typed.json |
| traces-auto-refresh | Check-7 soft-warns on absent tree-query — text excuses only cold-start, real reason is graph-not-applicable (TS surface) | — | q-typed.json |
| plain-language-status | Check-4 (UI-gate) true→WARN: P4 omitted `self-verify` for a UI+.rs chunk; added at P5 | iterations 1 | q-typed.json |
| traces-layout-polish | Check-7 (code-graph) + Check-6 (size, 84 < 150) both WARN — false positives; graph NOT-APPLICABLE for TS/TSX | — | q-typed.json |

**Proposal:** Checks 6 and 7 lack a **frontend/TS-surface exemption**. `graph-not-applicable` is
already a recognized calibration term (per `diagnosis-pass.md`); wiring it into
`validation.md` — "a plan whose modify-set is entirely webview/TS skips the code-graph-consulted
check and uses a lower size floor" — would stop the recurring by-construction WARN. The one genuine
signal in this group (plain-language: the UI-gate check correctly caught a missing `self-verify`) is
the check working — so the exemption should be scoped to Checks 6/7, not Check 4.

### P7 — wrap/route-resolve · `contract.carry-no-owner` — 3 cases · weight 3
**Pattern:** Operator CARRYs recur that have **no in-version markerless entry to pin to** — routed
ad hoc to a new Epoch entry, the handoff Notes, or a doc-comment.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| anomaly-surfacing | 3 CARRYs homed: #2 → P-077 (owner found); #1 (internal-scroll + wireframe) had no owner → new Epoch-3 entry | — | q-typed.json |
| legible-constellation | widget per-dot-labels follow-up (speculative, no natural owner) → routed to handoff Notes | — | q-typed.json |
| traces-auto-refresh | viz `next_cursor` keying CARRY (no in-version entry touches viz pagination) → left as a doc note | — | q-typed.json |
| —

**Proposal:** The CARRY mechanism has a clean home when an owning entry exists but **scatters
owner-less residuals** across three different sinks. A change-shape: an explicit **residual/backlog
bucket** in the working-route (a standing "unpinned carries" tail, or a `backlog.md`) so owner-less
CARRYs land in one auditable place instead of the handoff Notes where they can be lost. Note the live
symptom: Epoch-4's P-076 entry has already accreted a very long chain of headful-residual CARRYs —
the bucket would relieve that accretion. Low urgency; a tidiness/traceability improvement.

### P8 — phase/plan · `ambiguity.scope-question` — 3 cases · weight 9
**Pattern:** The plan step hit genuine scope ambiguity needing an AskUserQuestion round in 3 chunks
(popover token, label-collision approach, window-mechanism + content-reuse).

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| dropdown-layout-bug | popover bg token: design-correct `raised-2` contradicted the operator's written `--color-inset` intent → RESEARCH-CORRECTS-INTENT ask | dialogue 1 | q-typed.json |
| traces-layout-polish | 1 recommended-first ask on label collision-avoidance (pure-fn heuristic vs measure-based) — a genuine testability-vs-fidelity user call | dialogue 1 | q-typed.json |
| floating-window | 2 questions in 1 round (window mechanism front-vs-Rust; content-reuse wrapper-vs-refactor); both materials-leaned + recommended | dialogue 1 | q-typed.json |

**Proposal:** **No change recommended — this is the pipeline working as designed.** Each ask was a
genuine user decision (a spec-vs-intent conflict, a testability trade-off, a mechanism fork), each
single-round and recommendation-marked per the P4 AskUserQuestion discipline. Surfaced only for
completeness (it cleared the n≥3 threshold); the weight is dialogue, not defect. If anything, it
evidences that RESEARCH-CORRECTS-INTENT and recommended-first asks are firing correctly.

---

## Cross-step chains (starting heuristics)

**None surfaced.** Anchoring on suffering consumers found 2 `input.*` frictions (chrono dev-dep-only
assumption at plain-language/code; report-window under-specification at floating-window/fix-loop) and
1 `consumed[].quality=wrong` (dropdown-bug/smoke: agent-harness wrong for a webview chunk — the P2
harness-friction root). The within-chunk lineage join to a formally-`ok`-but-signalled producer
returned **0 chains**, and no (producer, artifact, consumer) shape recurred across ≥2 chunks. Honest
read: either the producers genuinely carried no quality signal, or the starting heuristics (no
transitive chains, no cross-step scoring) are too thin to catch Epoch 3's shape — a miss that feeds
the dogfood-calibration of these heuristics, per `diagnosis-pass.md`. (`q-chains.json` has the raw
join.)

---

## Level candidates (systemic-masked-as-project)

### L1 — deferred-forever / band-aid — 7 deferred facts · 15 deferral-open signals · 8 chunks
**Facts:** `resources`-nature deferrals recur at nearly every wrap `gates` step — e.g.
floating-window/gates "nextest RUN honored-deferred — cold ~30-binary test-link OOM/disk-risky on 26G
free"; dropdown-bug/gates "a cold rebuild for a webview-only chunk is costly"; +5 more (see
`q-level.json` `deferred.fact_detail`). Deferral-open signals appear in 8 of 9 chunks.
**Level hypothesis:** The *decisions* are correct at the project level (a zero-`.rs` chunk has
nothing to re-test), but the *recurring driver* is environmental — a **26G-free disk + a cold
~30-binary test-link + an rlib-format mismatch that forces cold rebuilds**. That resources-nature
constraint is being absorbed one-chunk-at-a-time by deferrals. Most close correctly at `.rs`-touching
chunks; the honest tail is **one genuinely-unclosed `.rs` deferral** — the `window.rs` label-enum
test-run from the final chunk — now riding into Epoch 4.
**Proposal:** A pipeline/environment fix rather than a standing per-chunk project call: (a) an
**incremental/cached workspace-test path** (link-reuse / `sccache` / `--no-run` staging) so the
re-run is cheap enough never to defer; and/or (b) **free-disk as a standing setup step** (the handoff
already references `scripts\free-disk.ps1`); and (c) an explicit **"deferred-gate closure" ledger
line** so the one carried `.rs` deferral is provably closed at Epoch-4's first `.rs` chunk rather
than assumed. This is the same root as **P1**; L1 is its systemic reading.

### L2 — override — 5 `solution:overridden` facts (2 on the same rule)
**Facts:** `route-resolve` trajectory-HALT **overridden twice** (legible-constellation +
constellation-severity: "the new-chunk-ahead entries would normally HALT+dialogue at P5; operator
pre-directed them"). Plus 3 single overrides: plan premise (dropdown-bug `--color-inset` falsified by
research), smoke self-verify (warm-re-embed substituted ×2).
**Level hypothesis:** The **P5 trajectory-HALT rule is miscalibrated for a standing operator
pre-direction** — when the operator has already directed the chunk-ahead edits in-session, the HALT
fires and is immediately overridden. The rule assumes trajectory edits are always a surprise needing
dialogue; here they were pre-authorized. (The smoke overrides are the P2 harness root, not a distinct
rule miscalibration.)
**Proposal:** A `route-resolve.md` refinement: recognize a **pre-authorized trajectory edit** (the
operator directed the new-chunk-ahead entry earlier in the same session) as an AUTO path with a
recorded note, rather than a HALT-then-override. Narrow and evidence-bounded (2 facts on one rule) —
surfaced for the founder's judgment on whether 2× warrants the rule change.

### L3 — chronic-degrade — `tooling.gate-deferral` (8 chunks) + `tooling.harness-friction` (6 chunks)
**Facts:** Both tooling frictions recur across the whole epoch **without ever halting** — soft_exit
total = 0, only 3 step-level halts in 126 records. `ok-degraded` outcome appears once.
**Level hypothesis:** These are exactly the degradations the halt policy *structurally never
surfaces* — each instance is individually small and rule-sanctioned, so nothing ever trips a halt,
yet they are the two most frequent friction types in the epoch. The gate-deferral (L1) and
harness-friction (P2) are the same two roots seen through the chronic-degrade lens: silent, constant,
never escalated.
**Proposal:** No new fix beyond P1/P2/L1 — L3 is the **confirmation that the epoch's top-2 frictions
are chronic, not incidental**. Its value is telling the founder these two deserve a
tooling-level fix precisely *because* the attended-halt model will never raise them on its own.

---

## Playbook-extension candidates (untyped patterns, F-4)

**None meet the F-4 threshold** (n≥3 untyped in-epoch, or n≥2 recurring from a prior epoch; no prior
epoch exists in the ledger). Two **emerging clusters** (n=2) to watch next epoch:

- **version-dir-layout at repo root (n=2 untyped, both orientation):** "working-route.md +
  verification-matrix.json live at repo-root `./andromeda-pulse-0.3.0/` (v3 migration), not under
  `.andromeda/` — the version-dir assumption cost a failed glob + a repo-wide find." **Strongly
  reinforces P4** (same v3-layout root cause, seen from orientation instead of take-up). If it
  recurs in Epoch 4 it clears the threshold — candidate type `contract.version-dir-layout`, draft
  criteria: *"orientation/take-up location probe assumed a `.andromeda/{version}/` subdir but the v3
  version dir is at the project root."*
- **intent/spec premise falsified by shipped code (n=2 untyped: premise-correction F8-sort +
  layout-templates-widget-vs-constellation):** spec/intent text lags the shipped code; the typed
  `contract.intent-divergence` (n=2) is the reinforcing signal. Emerging; candidate type
  `contract.spec-lags-code` if it recurs.

---

## Below threshold — no action

_Visible for the founder's eye; each below the F-2 proposal threshold._

- `reconcile/ambiguity.playbook-no-match` — n=2, w=6, 2 chunks (dialogue 2)
- `smoke/retry.smoke-reentry` — n=2, w=6, 2 chunks (retries 4; SIGILL-132 teardown re-launch)
- `route-resolve/ambiguity.trajectory-halt` — n=2, w=4, 2 chunks (dialogue 1) — pairs with L2
- `curation/ambiguity.tier-routing` — n=2, w=2, 2 chunks
- `smoke/tooling.subprocess-bounds` — n=2, w=2, 2 chunks
- `validate/contract.intent-divergence` — n=2, w=2, 2 chunks — reinforces the emerging spec-lags-code cluster
- `orientation/UNTYPED` — n=2, w=2 (extra_reads 4) — the version-dir cluster above
- `fix-loop/UNTYPED` — n=2, w=2
- Singletons (n=1): `smoke/process`, `fix-loop/tooling.environmental`, `code/input.conventions-gap`
  (chrono dev-dep), `fix-loop/input.plan-step-ambiguous` (report-window under-spec),
  `research/ambiguity.scope-boundary`, `research/UNTYPED` (premise-correction),
  `reconcile/contract.false-positive-proposal`, `plan/UNTYPED` (spec-vs-code drift),
  `validate/contract.matrix-claim`, `distill/contract.extract-format`,
  `distill/contract.binding-contradiction`, `fix-loop/ambiguity.scope-pressure`.

---

_End of diagnosis. Nothing above is applied or remembered; the founder takes it from here._
