# Session Handoff

**Last Updated:** 2026-08-26T19:08:00Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 30 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-26-l4-runtime-security-residuals): the L4 path inputs get a guard, and the argv prompt gets a bound`

## Position
- Done: **2026-08-26-l4-runtime-security-residuals** — the three product-consumed L4 path vars and the digest-derived `-p` argv prompt carry an enforced, stated guard. Confinement is **opt-in, not defaulted** — and the residual that leaves is stated, not implied.
- Next (first markerless): **Interpretation brief completeness** — it now carries the re-pinned `cargo audit` PREREQ, **next interval point 49** (47 was a ratified skip here; 48 skips too).
- Then: **Ingest consumer initiating freeze** · Halo State Pulse canvas disposition · Advisory backlog (+CARRY: `strict-path` adopt-or-drop) · npm advisory coverage · Diagnostics un-muting (+CARRY: the two full-path boot records) · Staged-bindings assertion · Metrics label surface.

## Work done
**The guard.** Always-on structural hardening on all three vars — reject a `..` component **before** canonicalizing (canonicalization erases `..`, so a post-check can never see it), bound the raw value at 4096, canonicalize, assert regular-file — plus **opt-in** confinement under the new `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, comparing **both sides canonicalized** per the `publish_workspace_key` precedent (Windows returns a `\\?\` prefix; a one-sided compare fails regardless of truth). A root that is SET but unresolvable **fails closed** — a typo must not disable the guard the operator asked for.

**Why opt-in and not the categorical rule:** the GGUF and `llama-cli.exe` live outside the data dir **by design**, so a data-dir default would reject every configuration that has ever run L4 here. The exception is therefore NARROWED with a truthful ground, not deleted — and the unconfined posture is now **announced once per boot** instead of being a silent convention.

**The bound.** The `-p` prompt is bounded before argv: 16 KiB ceiling + NUL/C0/C1 rejection **whitelisting `\n\r\t`**. The ceiling was derived, not invented — 154 real-model assemblies spanned 5,947–6,297 bytes, so it sits ~2.6× above the observed max and below the Windows `CreateProcess` 32,767 limit, making the guard fire before the OS does.

**Observability cost: one new leaf, one completed.** `interpretation.model.load.error` had **zero producers** — a pre-allocated declaration — and now has its first live emit (4 → 6 fields). `interpretation.model.allow_root` is new with its own exact leaf.

**Gates:** fmt · clippy all-features 0 warnings · **nextest 1988/1988 + 1 skip** (baseline 1963, +25 matching the tests added exactly) · capability-widening clean · check:ingest-progress PASS · capability-drift clean after the documented regen · deny bans/licenses/sources exit 0 · advisories designed-red at the same 8 IDs.

**Three mutation checks, all discriminating** — including one that reproduced a documented trap first-hand: deleting the new exact leaf left the resolver probe **passing vacuously** via the bare `interpretation` fallback while only the field-set pin caught it.

**Smoke: THREE legs** (fresh data dir each, 0 orphans, ports released). A: enforced root + model outside it → rejection with basename only, full path **0× across 11,251 lines**. B: root unset → `confinement=unconfined` WARN, exactly once. **C: the positive arm** — legitimate model inside the root → enforced, **0 rejections**. Leg C was added on operator directive and is the does-not-brick-the-real-host proof; without it that arm was unit-only.

## Drift resolved
**21 amendments across 4 masters · 0 escalations · 8 cascade leaf edits across 7 files · drift = 0.**
- **security-plan ×11** — the exception NARROWED (not deleted) at §Anti-Patterns → Input with its owner marked DISCHARGED; the `strict-path` mandate narrowed off this boundary at three sites; a NEW §Input Validation row for the argv boundary; §Threat Model's "no declared canonicalization" retired; the basename-only rule broadened from plugin paths to every product path, with the two pre-existing full-path boot records named as a carried exception.
- **arch ×3** — `ANDROMEDA_PULSE_L4_ALLOW_ROOT` registered; the MODEL_PATH unconfined clause retired; **the dev-vs-bundled binary-name split recorded** (operator directive 2).
- **obs-plan ×4** — the two leaves registered/completed in §8, dual-registered at §6, and §1's Vector 6 broadened on two axes it asserted too narrowly.
- **test-plan ×3** — both path-confinement rows narrowed to the opt-in form (an unset root makes a confinement test **vacuous**), plus a new `l4-path-guard-callsite-wiring-coverage` trigger.
- **Cascade:** CLAUDE.md `GENERATED:setup:warnings` · `rules/security.md` ×2 · `docs/security-summary.md` · `rules/observability.md` · `docs/obs-summary.md` · `docs/services/interpretation.md` · `docs/services/workspace-detector.md`. Verified-and-correctly-skipped: `docs/tests-summary.md`, `docs/stack.md`, `docs/conventions.md`, and `rules/testing.md` (hit sat in preserve-verbatim Session Additions **and was not stale**).

## Notes
- **An operator-supplied coordinate measured FALSE and was corrected before it landed.** Directive 2 asked for the arch fix on the ground that the confusion "has now cost two consecutive plans (P-077's deviation 1)". Re-derivation at HEAD: the predecessor's `plan.md:117` names `./target/debug/pulse-app.exe` **correctly**, and its deviation 1 concerns flat config. **One instance, not two.** The amendment landed on the corrected basis — the registry entry *invites* the error — rather than on a recurrence count that did not hold.
- **What this chunk did NOT do, stated plainly:** it did not make confinement the default. On a host that leaves `ANDROMEDA_PULSE_L4_ALLOW_ROOT` unset, an actor who can set the path vars can still point the product at an arbitrary file. That is a deliberate design choice, now announced once per boot rather than silent, and the specs say so at every site.
- **A real gap this chunk's own detectors caught:** the guard FUNCTIONS are pinned and mutation-checked, but neither production CALL SITE is — reverting `LlamaCliInference::new` or the prompt check leaves **every test green**. Minted as `l4-path-guard-callsite-wiring-coverage`.
- **Curation:** T1 1 (in-place additive extension) · T2 2 · T3 0; 3 rejected at Filter 1. CLAUDE.md unchanged at **156/200**.
- Audit trail: `.andromeda/runs/2026-08-26T18-40-57Z-wrap/` (+ the phase run dir).
- Last failed command: none.

## Deferred learnings
- **`covered-by-a-spec-master` has no curation disposition.** Filter 1's dedup sources are the three curation homes plus rule-file bodies — spec masters are absent, yet `architecture.md` is `@import`ed into every session, so an entry duplicating it is redundant at identical context cost. The dev-vs-bundled binary trap hit exactly that shape and was rejected by judgment, outside the documented filters.
- **The tiebreakers do not rank two Tier-2 files.** The layout-char entry fit `rules/security.md` by subject (unconditional load) and `rules/testing.md` by cost (path-scoped). Tiebreaker 2 says prefer lower cost; subject-fit said otherwise. Resolved on subject; the list only ranks ACROSS tiers.
- **Anchor-by-truncated-view cost two Edit failures this session** — one ambiguous anchor (3 matches) and one no-op (old == new), both from choosing an anchor off a `cut -c1-N` view instead of reading the line whole. Cheap individually; the pattern is the signal.
- Still open from prior wraps: the **deferral-destination generalization** (a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes).
- Standing: the **bindings clobber** recurred again (default-features nextest → capability-drift red → documented regen → clean). Owned by the "Staged-bindings assertion" entry.
