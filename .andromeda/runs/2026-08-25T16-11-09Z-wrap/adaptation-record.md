# Adaptation Record — 0-pending operator-requested wrap

**When:** 2026-08-25T16:11:09Z · **Session:** 43 · **Branch:** `chore/migrate-pulse-to-v3`
**Path:** Setup step 6 — 0 pending master records; tree dirty only with expected-transient bookkeeping
(`session-handoff.md`, `.andromeda/friction-log.ndjson`), which the session-state contract classes as
git-CLEAN here. No chunk in flight, so no P1 report and no P2 fan-out ran.

Two operator items, both dispositioned below.

---

## Item 1 — LIFT "Demo injector formalized + api-surface retire" (P-077) to first markerless

**Disposition: APPLIED.** Trajectory class (priority reorder), which route-resolve halts on — the operator's
invocation carried the ruling, the rationale and the scope hint, which IS that dialogue per route-resolve
§Operator-requested adaptation. No further round was needed and none was invented.

**Move:** tail-last → first markerless position, ahead of `Halo State Pulse canvas disposition`. Everything
else in the tail keeps its order, as instructed.

New markerless order:

1. Demo injector formalized + api-surface retire *(lifted)*
2. Halo State Pulse canvas disposition
3. Advisory backlog
4. npm advisory coverage
5. Diagnostics un-muting + harness-truth sweep
6. Staged-bindings assertion
7. Metrics label surface

**Annotations moved with the edit** (route-resolve: annotations travel, origins preserved):

- `PREREQ: re-check cargo audit` — re-pinned from the Halo entry onto the lifted entry, because the lift made
  the injector the first markerless entry and that PREREQ was pinned as the *next entry's* obligation. Origin
  (`2026-08-15-corpus-key-persistence`) and ratification (pin #15, 2026-08-16) preserved verbatim; the pin text
  updated to record this wrap's discharge (Item 2).
- The five existing `CARRY:` annotations rode the entry unchanged, with ONE augmented: the
  CONTINUOUS-UNIQUE-STREAM carry is now marked load-bearing rather than optional, because a finite ~600-batch
  storm cannot hold a sustained scenario across the L2→L3 20–60s window plus the L4 queue that the new scope
  requires.

**Scope hint folded in** (operator ruling, verbatim in substance): the verification leg is the FIRST live
REAL-model chain proof — injector-driven sustained scenario under real L4 (not det-L4), an incident forms, and
the operator judges the interpretation brief (ManualCheck class); det-L4 remains the deterministic arm.

**Rationale, verified first-hand at this wrap** rather than relayed (the verify-at-HEAD discipline — the
operator cited three closed blockers and one commit SHA; each was re-derived before it entered durable route
text):

| Cited blocker | Verified at |
|---|---|
| bootstrap env-override | `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`, `crates/triage/src/cue/thresholds.rs:12` |
| workspace-key alignment | `2961f4e` = `feat(2026-08-14-workspace-key-alignment)`, dated 2026-08-15 |
| observer starvation | closed by the seq/identity family — `8e9856c` (log-records), `d4b432b` (metrics-points) |

All three reproduced. No correction to the operator's premise was needed.

### One discrepancy surfaced, not silently absorbed

P-077's verification-matrix entry currently reads `method: by-construction` with acceptance *"inject_demo.rs is
tracked and builds; the integration UX e2e (P-076) uses it to drive telemetry"* — and **P-076 is already
`verified`**, so that acceptance is satisfiable **without ever running the real model**. The operator's scope
hint RAISES the bar to a live ManualCheck chain proof.

The matrix was **not** written here: this wrap claimed no capability, and concretizing an acceptance belongs to
the claiming chunk at promotion (verification-matrix-contract §Acceptance lifecycle). The discrepancy is
recorded as a `NOTE:` on the lifted entry so `/andromeda-phase` must confront it at promotion rather than take
the cheaper path that is already open.

---

## Item 2 — `cargo audit` PREREQ, interval point 43, IN FULL FORM

**Disposition: DISCHARGED IN FULL FORM.** The operator's instruction that the count moved when this wrap was
inserted is correct and was followed — the probe was run here, not deferred to the Halo entry's wrap.

| Element | Result |
|---|---|
| Probe | `cargo audit` under cargo-audit **0.22.2** |
| Exit | **1**, read DIRECTLY (`$?` captured immediately, never through a pipe) |
| Basis | `error: error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244` — **byte-identical** to the recorded basis; upstream; the DB still will not load |
| Overlap (pass/fail half) | `cargo deny check bans licenses sources` → **bans ok, licenses ok, sources ok**, exit 0 |
| Overlap (designed-red half) | `cargo deny check advisories` → exit 1 at the **same 8** owned upgradeable IDs: 0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258 — **set unchanged** |
| Deferral status | Continues (it ends only when `cargo audit` loads). Compact form holds — basis unchanged. |
| Next interval point | **46** |

### Two counting rules measured at this probe

**(a) Error BLOCKS are not the ID count.** The advisories run emitted **10 error blocks for 8 distinct IDs**,
because a crate present at two lockfile versions raises one block per version — quick-xml's 0194 and 0195 each
appeared twice, at `Cargo.lock:509` and `:510`. A block count reported as an owned-set size manufactures a
false "the set grew from 8 to 10" alarm. Enumerate distinct `RUSTSEC-` ids.

**(b) An interval POINT belongs to the wrap that actually occurs.** Inserting a wrap moves the count, so a
point owed "at the next entry's wrap" falls on the inserted wrap instead. This generalizes the operator's own
instruction and is now written where the discipline lives, so the next inserted wrap does not re-derive it.

Both rules were applied to the spec masters, not left as prose here — see below.

---

## Documents written

| File | Write |
|---|---|
| `andromeda-pulse-0.3.0/working-route.md` | the lift + annotation moves (markerless tail only) |
| `.andromeda/security-plan.md` | §Dependency Security probe clause: session-43 discharge, next point 46, counting rules (a) + (b) — the body said "next at session 43", which this wrap's own probe falsified |
| `.andromeda/security-plan-amendments.md` | sidecar entry recording the above with its Why |
| `.claude/rules/security.md` | leaf cascade of the same correction |
| `.claude/docs/session-learnings.md` | Tier 3 curation entry (see below) |

### Route-file integrity check (post-edit)

| Invariant | Before | After |
|---|---|---|
| Total lines | 106 | 104 (−4 tail region, +2 at the lift) |
| Frozen `[marker]` lines | 37 | 37 — **byte-identical set**, diffed |
| Markerless entries | 7 | 7 |
| `   ↓` separators | 40 | 40 |
| Frozen lines touched by the diff | — | **0** |

This check earned its place: an anchored edit removing the PREREQ from the end of the Halo line also
swallowed the following line's break, merging the next separator onto it. The separator count (39 against an
expected 40) caught it; a second edit restored the break. The prose read correctly the whole time.

---

## Curation

- **Tier 3 ×1** → `.claude/docs/session-learnings.md`: an anchored edit ending at a line terminus can swallow
  the next line's break, so structured-line ledgers need a structural invariant check after editing, not a
  prose re-read. (Tier-2 fallback found no matching rule scope → demoted per the fallback chain.)
- **2 candidates routed OUT of curation** into the spec master instead — the two counting rules above. Their
  proper home is the probe clause they correct, and the same wrap was already writing that body; curating them
  as well would have minted a duplicate of a fact just written to the source the leaf derives from.
- Filters: 0 dup · 1 task-specific rejected (the operator's trajectory ruling — route content, not a learning)
  · 0 conflicts · 0 deferred. CLAUDE.md unchanged at 155/200.

---

## Not done on this path (by design)

No P1 report, no P2 detector fan-out, no P4 code-graph refresh, no P7 gates — there was no chunk. No master
record was written (nothing to flip). No verification-matrix write (no capability claimed). The light gate
does not apply; the two `cargo deny` invocations above were run as the PREREQ's overlap evidence, not as a
chunk gate.
