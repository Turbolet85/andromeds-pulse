# Adaptation record — 0-pending wrap (session 37)

**Wrap:** 2026-08-23T11:52:00Z · `chore/migrate-pulse-to-v3` · master-route `pending` = 0
**Path:** Setup step 6 no-op path + P5 route-resolve (operator-requested adaptation)
**Scope touched:** `andromeda-pulse-0.3.0/working-route.md` markerless tail only

---

## Item 1 — priority reorder (trajectory) · APPLIED

**Operator request.** Swap the first two markerless entries so *Webview self-verify on the Windows
host* sits ahead of *Integration UX e2e test*.

**Stated rationale.** P-076's acceptance names "a tauri-driver e2e test" as its proof method, and the
self-verify entry is the one that installs and proves that driver on this host; in today's order the
head requires a tool its successor provides.

**Premise verified at HEAD before applying** (per the 2026-08-21 verify-at-HEAD discipline — the
relayed/authored claim is re-derived first-hand, never taken on trust):

| claim | source read | result |
|---|---|---|
| P-076's acceptance names a tauri-driver e2e test | `andromeda-pulse-0.3.0/verification-matrix.json` → `P-076.verification.acceptance` | **CONFIRMED, verbatim**: "A tauri-driver e2e test drives the full path under deterministic-L4 and asserts Traces render, incident creation, and an Investigate result" |
| the self-verify entry stands that driver up | working-route.md, the entry's own `SCOPE:` | **CONFIRMED**: "stand up that driver, then the UI legs of the entries below become AGENT legs rather than operator legs — this entry is their prerequisite, which is why it sits ahead of them" |
| today's order inverts that | working-route.md markerless positions | **CONFIRMED**: the entry claiming to sit ahead sat at position 2, behind P-076 at position 1 |

**Finding beyond the request.** The displaced entry's own text already asserted the correct order
("which is why it sits ahead of them") while the file contradicted it. The claim is unenforced prose:
route-resolve's dependency-reorder rule fires only when a *chunk outcome* surfaces a dependency, and
no pass re-reads the standing tail for order-vs-declared-dependency contradictions. Recorded as
friction `contract.structural-blind-spot`.

**Applied.** Markerless positions 1 ↔ 2. Positions 3–9 untouched, byte-identical.

---

## Item 2 — annotation migration · APPLIED

Per route-resolve §Operator-requested adaptation, existing annotations move with the edit, and a
reorder ahead of the previous head re-pins its next-entry PREREQs onto the new head, origins preserved.

| annotation | before | after |
|---|---|---|
| P-076's 13 `CARRY:` | Integration UX e2e test | **unchanged, all 13** — travelled with their entry |
| `PREREQ: re-check cargo audit` | Integration UX e2e test | **migrated** to Webview self-verify (new head); origin `2026-08-15-corpus-key-persistence` preserved |
| `NOTE:` naming the PREREQ's departure | stale — dated 2026-08-21, no target named | re-dated 2026-08-23, target named |

Annotation counts on all seven other markerless entries: unchanged.

---

## Item 3 — `cargo audit` pin #9 → #10, probe point 37 · FIRED IN FULL FORM

This wrap is session 37 (`session_count` 36 → 37) = the ratified every-3rd-wrap probe point
(25/28/31/34/**37**). Fired in FULL form, **exit status read directly, never through a pipe**.

| leg | exit | result |
|---|---|---|
| `cargo audit` | **1** | `error: error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244` |
| `cargo deny check advisories` (named overlap) | 1 | designed-RED at exactly the **8 owned IDs** — 0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258 |
| `cargo deny check bans licenses sources` (pass/fail half) | **0** | `bans ok, licenses ok, sources ok` |

Signature reproduced **byte-identically**, overlap unshifted → **probe-auto-satisfy: probe unchanged,
5th consecutive**. Basis re-verified rather than echoed: the upstream RustSec load failure is
unclearable by any repo change, which holds a fortiori here — this wrap added and bumped **zero**
dependencies (no `Cargo.toml` / `Cargo.lock` delta, verified). **Next probe point: 40.**

**Deviation from a literal reading of the request, surfaced deliberately.** The request said pin #9
migrates "untouched". Its text read *"the NEXT wrap is point 37 and fires the probe in FULL form"* —
and this wrap **is** 37 and fired it. Migrating those bytes verbatim would have left the pin
self-contradictory at rest, so the status line was re-authored (points → `25/28/31/34/37`, next → 40,
pin #9 → #10) while **basis, signature, origin and closing condition are unchanged**. Route-resolve
requires the basis be re-verified at each pin and the outcome recorded as `probe unchanged, {N}th
consecutive`, so an update was owed regardless. Raised in the wrap report for correction if the
intent was byte-freeze.

Raw probe output: `audit-probe.txt` · `deny-advisories.txt` · `deny-bls.txt` (this directory).

---

## Integrity verification

Route file compared against the pre-edit snapshot (`working-route.before.md`, this directory):

- line count 101 → **101**
- entry count 41 → **41**
- **all 32 `[{marker}]`-frozen lines byte-identical** — the freeze contract is intact
- 9 markerless entries → 9; positions 3–9 byte-identical
- exactly **2** content-changed lines — the two swapped entries

## Not done here

No P1 report, no P2 fan-out, no P7 gate/flip sequence — the no-op path runs neither, because there is
no chunk to attribute work to. Master-route is untouched (0 pending; wrap's only master write is the
`pending → complete` flip, which needs a pending record). Curation ran and produced **zero** candidates:
the session's only content is evolve telemetry (excluded from curation scope by contract) plus this
directive, already recorded in the route itself.
