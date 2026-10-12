# Fan-out results — 2026-10-10-capability-record-re-based

Seven doc-agents, one batch, each given its doc, the report, its keyed-contract render where the doc carries one,
and its detectors (2 + 4 + 2 + 2 + 3 + 4 + 2 = 19, the drift base's 19). Entity probe on the two carrying returns:
0 entities. Proposals: 4 (architecture 2 · test-plan 2). Rejected for a source the report does not carry: 0.

## Verdicts

- **architecture** — 2 proposals (below). D-arch-decisions: no drift.
- **security-plan** — `proposals: []`. Stripped commentary: no external-input surface, no auth or key code, no
  dependency, no scrub or log boundary touched; 0 occurrences of the gate or either record in the doc; a note that
  the record retires ids against `corpus-encryption` and `training-export`, which the doc describes as live, and
  that this is the removing entries' to amend, not this chunk's. Raw twin: `.raw-fanout-security-plan.md`.
- **design-system** — `proposals: []`. Stripped commentary: no UI rendered, both new surfaces `tokens n/a`; no
  status claim of the doc touched by a report bullet; a note, from one read of the record, that the glow layer's
  "DEFERRED to the next version" wording and its residual pointer were stale before this chunk and that the entry
  `Window retired` owns the rewrite. Raw twin: `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped commentary: no surface or region added; no status claim
  touched; a note that the doc cites P-063, P-068, P-070, P-080, P-082 as provenance on surfaces a removing entry
  will take. Raw twin: `.raw-fanout-layout-templates.md`.
- **test-plan** — 2 proposals (below). D-tests-framework and D-tests-obs-harness: no drift.
- **obs-plan** — `proposals: []`. Stripped commentary: no hot path, no telemetry dependency, no user-data logging,
  no §10 narrative touched; 0 hits for nine patterns in the doc and its eight key files; a note that the doc
  names P-025, P-027, P-045, P-047 as observables and the report gives no per-id disposition for them. Raw twin:
  `.raw-fanout-obs-plan.md`.
- **a11y-plan** — `proposals: []`. Stripped commentary: no interactive element, no violation-schema change; 0
  mentions of the gate or the records in the doc and three key files. Raw twin: `.raw-fanout-a11y-plan.md`.

## architecture — the return

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates)"
    change: "Append a registration to the bullet: `cargo xtask verify:capability-matrix` — the capability-record gate implemented at `xtask/src/capability_record.rs` (chunk 2026-10-10-capability-record-re-based; registered per the same formalized-CLI-contract rule; a module private to the xtask binary — `evaluate(root) -> Verdict` reads both files, `judge(record, route, root) -> Verdict` is the pure verdict, one caller `verify_capability_matrix` in `xtask/src/main.rs`). Inputs: two fixed in-repo paths held as constants, `docs/capability-record.json` (`RECORD_PATH`) and `andromeda-pulse-0.4.0/working-route.md` (`ROUTE_PATH`); no path argument and no environment variable. Contract: exit 0 clean · 1 findings · 2 cannot-evaluate; one line `verify:capability-matrix: {clean|violations} ({n} ids: {c} claimed, {r} retired, {k} violation(s))`, the three counts read from the record, or `verify:capability-matrix: cannot-evaluate ({reason})`; one JSON event line (target `xtask.verify_capability_matrix`) and the report twin `target/capability-matrix/report.json`, both carrying `state`, `capability_count`, `claimed_count`, `retired_count`, `violation_count`, `reason`, the twin adding `violations` and `generated_at`. Arms: cannot-evaluate — the record absent, unreadable, not JSON or holding no `capabilities` array, or the route absent or unreadable; findings — the id set not exactly P-001…P-082 once each, a `disposition` neither `claimed` nor `retired`, a `legend` not equal to the module's three closed sets, on a claimed entry (`carried_by` absent, an element outside P-083…P-129, or empty with no `note`; `provisional` true with no `note`; no scenario; scenarios all `source-evidence`; an unknown scenario kind or verification mode; a file ref that does not exist or cannot be read; a `contains` anchor not found; an `external`, `manual` or `by-construction` scenario with no `note`), on a retired entry (a `scenarios`, `verification_mode` or `carried_by` key; `surfaces` empty or an unknown word; `removed_by` empty or a title no route entry carries; `kept_half_owner` naming such a title; `guard` absent, its `state` unknown, `runs` or `part` with an empty `by`, `part` or `none` with an empty `unrun`), a record with no claimed id; clean otherwise. A route entry's title is its line without a leading `[marker] ` stamp, up to the first ` — `; header lines, `_` note lines and indented lines are not entries. Run by ci.yml's `lint-test` job after `capability-widening-check`; it reads a file the wrap writes, so a route-resolve that renames, retires or splits an entry the record names reddens the verb and the committed-record pin until the record is corrected. Before this chunk it read the fixed path `docs/v0_2_0/capability-verification-matrix.json`, ids P-001…P-060, exit 0 clean and 1 on a violation, an absent or unparseable file falling into the generic `xtask error:` exit 1. Held by 36 pins in the module's `mod tests`, one of which reads the committed record over the committed route. It adds no TauRPC procedure, port, socket, environment variable or IPC route."
    sidecar: "architecture §Occupied Resources → xtask CLI surfaces: registered `cargo xtask verify:capability-matrix` (module `xtask/src/capability_record.rs`, inputs `docs/capability-record.json` + `andromeda-pulse-0.4.0/working-route.md`, exits 0 · 1 · 2, its arms, its line and report twin) — chunk 2026-10-10-capability-record-re-based."
    rationale: "The report's Changes (Symbols / APIs; Harness / gate surface) re-base the verb onto two fixed inputs with a new 0 · 1 · 2 exit contract, a new verdict line, a changed report twin and a new module; its Expected amendments name this exact section and state the site search found 0 architecture hits (\"the verb was never registered there\"). The registry bullet carries every sibling formalized-CLI-contract verb (`check:npm-supply-chain`, `check:staged-artifacts`, `ci-gates`, `quarantine-tracking`, …) but not this one, so the gate surface the chunk landed is unregistered. Keyed contract `§Infrastructure Patterns → CI/CD approach` was read and needs no edit: it names `lint-test`'s gates only as \"the xtask gates\" and states nothing about this verb's input, ids or exits."
    basis: ".andromeda/architecture.md:261 (the registry bullet, as the report locates it); xtask/src/capability_record.rs:354-369 (`evaluate`), 300-352 (`judge`), 137-152 (`route_titles`); xtask/src/main.rs:10 and :327; .github/workflows/ci.yml:105"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts"
    change: "Add a contract bullet: **Capability record — the form another project reads**: `docs/capability-record.json` (`schema_version` 1; header `product` · `version` · `as_of` · `supersedes` · `legend`; `capabilities`, 82 entries P-001…P-082 in id order, each `id` · `title` · `disposition` · optional `note`) is the one current record of the capability ids. The accepted set is every entry whose `disposition` is the string `claimed`; an entry whose `disposition` is `retired` says with what in `surfaces` (closed words `window` · `model` · `desktop` · `workspace` · `training-export` · `corpus-encryption`) and by which working-route entry in `removed_by` (entry titles). Those three fields — `disposition`, `surfaces`, `removed_by` — are what Conductor's route entry `Accepted capability set re-based` derives its set from, and nothing else in the file is needed for it; a later change of those three fields' form is a change another repository reads. The record is validated by `cargo xtask verify:capability-matrix` in ci.yml's `lint-test` (§Occupied Resources → xtask CLI surfaces) and supersedes `docs/v0_2_0/pulse-capability-spec.md`, `docs/v0_2_0/capability-verification-matrix.json` and `andromeda-pulse-0.3.0/verification-matrix.json`."
    sidecar: "architecture §Standard Contracts: added the capability record `docs/capability-record.json` as a contract an external reader (Conductor) relies on, named with its three fields `disposition` · `surfaces` · `removed_by` — chunk 2026-10-10-capability-record-re-based."
    rationale: "The report's Schema / config bullet lands a new committed data file with a stated form and a sub-bullet \"The form another project reads\" (inputs#I2, I5, I6, I7; I11 item 6): Conductor derives its accepted set from `disposition`, `surfaces`, `removed_by`. Its Expected amendments carry the wrap directive that the record be named with its three fields where the masters list what an external reader may rely on, state the site search `capability-record` found 0 hits in the seven masters and the key files, and name architecture §Standard Contracts as that home. A cross-repository contract the chunk landed is therefore absent from the registry sections."
    basis: ".andromeda/architecture.md:100 (§Standard Contracts, as the report locates it); docs/capability-record.json:6-10 (`supersedes`), 11-29 (`legend`), 30-1660 (`capabilities`)"
# D-arch-decisions: no drift. The report's Dependencies bullet reads "none added, none bumped" with Cargo.toml, Cargo.lock, deny.toml and rust-toolchain.toml unchanged; Crates / modules adds one module to the existing `xtask` crate, no workspace member and no crate edge; no TauRPC procedure, port, socket, env var or IPC route. The verb stays inside the locked [CI Task Runner] `cargo-xtask` decision and follows the registry's exit 0 · 1 · 2 formalized-CLI-contract shape.
# Sweep: both proposals add registrations and retire no architecture claim — `verify:capability-matrix|capability-verification-matrix|capability-record` has 0 hits in architecture.md and in the four keyed-contract files, so no dependent-of proposal exists. Read and left alone: §Existing Scopes names `docs/v0_2_0/pulse-capability-spec.md` ("capability spec P-001 through P-060+") as a supporting document of the `pulse-v0_2_0-route` scope; the report keeps the old records byte-identical and assigns their marking to the route entry `Records say what the product is`, so it is not a change of this chunk.
```

## test-plan — the return

```yaml
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Capability verification matrix (CI step, runs after capability-drift)"
    change: >-
      Replace the paragraph: `cargo xtask verify:capability-matrix` (name unchanged) reads two fixed in-repo files, with no path argument and no environment variable — `docs/capability-record.json`, the one current record of all 82 ids P-001–P-082, each `claimed` or `retired` (36 claimed · 46 retired at chunk 2026-10-10-capability-record-re-based), and `andromeda-pulse-0.4.0/working-route.md` — and exits 0 clean · 1 findings · 2 cannot-evaluate (record absent, unreadable, not JSON or without a `capabilities` array; route absent or unreadable). It validates proofs of claimed ids only: at least one scenario and not all `source-evidence`, a known scenario kind and one of nine verification modes (the old seven plus `dynamic-external` and `manual`), every file ref existing and every `contains` anchor found, a `note` on each `by-construction` / `external` / `manual` scenario; a retired id carries no scenario and names its surfaces, the working-route entry title that removes it, and its guard state (`runs` · `part` · `none`). Drop the "P-040 default-off" by-construction example (P-040's one scenario is a webview unit test). `docs/v0_2_0/capability-verification-matrix.json` is superseded and no longer read by the gate. Replace the "P-061+ extend the matrix JSON" sentence: ids P-061–P-082 live in the same record; the id set is closed at P-001…P-082 in `xtask/src/capability_record.rs` (a missing, duplicate or out-of-range id is a finding), so a change to the set lands in the module and the record in the same chunk; and because the gate reads the working route, a route-resolve that renames, retires or splits an entry the record names reddens the verb and the xtask pin `the_committed_record_reads_clean_over_the_committed_route` (in every `cargo nextest run --workspace`) until the record is corrected.
    sidecar: >-
      2026-10-10-capability-record-re-based — §9 capability gate re-based: `verify:capability-matrix` now reads `docs/capability-record.json` (82 ids, claimed/retired, 36/46) plus `andromeda-pulse-0.4.0/working-route.md`, exits 0/1/2; the old claim (validates `docs/v0_2_0/capability-verification-matrix.json`, all 60 P-001–P-060, "P-061+ extend the matrix JSON", P-040 as the by-construction example) retired as disproved by measurement.
    rationale: >-
      Report "Spec claims disproved by measurement" names `test-plan.md:535` as false from this chunk on (the gate reads `docs/capability-record.json`, 82 ids, two dispositions, exits 0 · 1 · 2), and "Expected amendments (from plan)" lists exactly this §9 edit as Carried by the Symbols / APIs, Counts and Spec-claims bullets. Symbols / APIs gives the two constants `RECORD_PATH` / `ROUTE_PATH`, the exits and the arms; Schema / config gives the scenario kinds and the two added modes; Counts gives 60 → 82, 36 claimed · 46 retired; Harness / gate surface gives the route dependency and the committed-record pin (525-536); Deviations gives P-040's one scenario being a webview unit test. This gate is the test-plan's capability → proof coverage gate, so it is filed under D-tests-coverage. Sweep for other occurrences of the retired claim in test-plan and its seven key files (`verify:capability-matrix`, `capability-verification`, `capability matrix`, `verification matrix`, `60 capabilit`, `P-060`, `P-061`, `pulse-capability-spec`, `capability-record`): one hit, the paragraph at `:535` under its heading; 0 in the key files — matching the report's own site search (1 hit, 0 in key files). No dependent proposal is owed. The heading keeps its wording (the verb keeps its name; "runs after capability-drift" is not changed by the report — `ci.yml:105` changed a display name only).
    basis: ".andromeda/test-plan.md:535 · xtask/src/capability_record.rs:300-352 (judge), 354-369 (evaluate), 92-102 (MODES), 525-536 (committed-record pin)"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers (documented gaps)"
    change: >-
      Add a row `verify-capability-matrix-verb-glue-coverage` — NEW 2026-10-10-capability-record-re-based: the verdict is pinned in-crate (36 co-located pins in `xtask/src/capability_record.rs` `mod tests`, one per arm over constructed records under `tempfile::TempDir`, plus the committed-record pin asserting the clean line `82 ids: 36 claimed, 46 retired`; 35 of 36 read red against a pass-everything stub), but the verb function `verify_capability_matrix` in `xtask/src/main.rs` gained no pin at that chunk — the JSON event line (target `xtask.verify_capability_matrix`), the report twin `target/capability-matrix/report.json` (`state` · `capability_count` · `claimed_count` · `retired_count` · `violation_count` · `reason` · `violations` · `generated_at`; `verification_mode_counts` gone) and the process exit taken from `Verdict::exit_code`. The verb itself was read on one arm only, clean / exit 0 (locally and in the `lint / test` log of `ci#38049792921`); its cannot-evaluate exit 2 never ran through the verb, which takes no path. Owed: a pin over the twin's member set and the verb's exit per state (a seam passing the root, or a pin on the pure `counts` value).
    sidecar: >-
      2026-10-10-capability-record-re-based — pending coverage trigger `verify-capability-matrix-verb-glue-coverage` opened: the verdict arms are unit-pinned (36), the verb's event line, report twin and non-clean exits in `xtask/src/main.rs` are not.
    rationale: >-
      §2 puts isolated module logic at the unit tier, and the plan's own precedent rows record glue that ships with a one-arm live reading in place of a per-run pin (`discovery-observer-wiring-coverage`, `exit-hook-main-composition-coverage`). The report's Counts bullet says all 36 of this chunk's tests are `capability_record::tests::*`; Harness / gate surface lists no pin on the verb function, only the unchanged CI-wiring assertion `ci_workflow_invokes_xtask_verify_capability_matrix`; the new-text listing shows `xtask/src/main.rs` gaining only the module line, the call sites and the verb body (10 · 245 · 327 · 767-817, the twin's counts at 784-791); Deviations records the twin's member change; Limits states the cannot-evaluate arms are "held by pins, never by a run of the verb over an absent file". Inference to confirm at apply: that no pin predating this chunk reads the twin or the event line — the report states only what this chunk added. The module arms themselves are NOT drift (unit tier met, co-located `mod tests`, nextest).
    basis: "xtask/src/main.rs:784-791 (the twin's counts) · xtask/src/capability_record.rs:371-802 (mod tests, the 36 pins)"
# D-tests-framework: no drift — the report's runner is `cargo nextest run --workspace --profile ci` (2944 of 2944), the 36 new tests are co-located `#[cfg(test)] mod tests` using `tempfile::TempDir`, and Dependencies lists none added or bumped; matches §2 / §4.
# D-tests-obs-harness: no drift — the report changes no 5-command verb, status shape or log format (its "Harness / gate surface" bullet is the xtask gate, one CI step display name at ci.yml:105 and a failure message at a11y_perf_workflow.rs:150-151); the seven §3 key files hold 0 mentions of the verb or either record, so no key file is touched.
```

## Validate — dispositions

1. **architecture · D-arch-resources · §Occupied Resources → xtask CLI surfaces** — **apply.** Check 1: the
   playbook's "Accurate this-chunk addition" (routine); it is also the plan's second expected amendment (check 5).
   Every coordinate it cites is the report's or its last section's. Applied re-derived from the report, shorter
   than the proposal's `change`.
2. **architecture · D-arch-resources · §Standard Contracts** — **apply.** Check 1: "Accurate this-chunk addition"
   (routine), and the operator's recorded direction settles it (the wrap directive, inputs#I11 item 6: the record
   named with its three fields where the masters list what an external reader relies on). Not a boundary widening:
   no channel, input class or crossing changes; a committed file's form is stated. Coordinates are the listing's
   rows (`docs/capability-record.json` 6-10, 11-29, 30-1660).
3. **test-plan · D-tests-coverage · §9 → Capability verification matrix** — **apply.** Check 5: the plan's first
   expected amendment; check 6: it disposes the report's first disproved claim (`test-plan.md:535`). Routine.
4. **test-plan · D-tests-coverage · §1 → Pending coverage triggers** — **apply.** Check 1: "Accurate this-chunk
   addition" (routine): the row records what the report states — the 36 pins are the module's, the verb's glue in
   `xtask/src/main.rs` has none, the verb ran on its clean arm only. Check 4 (an absence claim): the proposal's
   own flagged inference, that no earlier pin reads the twin or the event line, rests on research.md ("The
   function has no test: `git grep` for `verify_capability_matrix` and `capability-matrix` under `xtask/src` finds
   only `main.rs` itself") and on this chunk adding none there.

Check 2 (cross-contradiction): none; the four edit four sections. Check 3 (intent): the report's deviations are
each justified by an operator's word (inputs#I9, inputs#I10, inputs#I11) or stay inside the entry's intent; the
scope record holds no line. Check 5: both entries of the plan's list are proposed; the directive's third is
proposed; no row of `citation-dispositions.md` reads `claim false`. Check 6, the report's four disproved claims:
`test-plan.md:535` → proposal 3 · the old gate record's `purpose` line → the route (a pin on `Records say what the
product is`, P5) · the plan's entry-5 atom → corrected in the plan by the operator edit, and curation (P3) · research
on P-076 → the report's entry, research unedited.

Escalations: 0.

Notes that are no proposal and no drift of this chunk, carried to the route-resolve card: the masters still
describe as live what the record retires (design-system's glow-layer wording, layout-templates' provenance ids,
security-plan's corpus encryption and training export, obs-plan's four capability-named observables); each
rewrite belongs to the entry that removes the surface.
