# Report — 2026-10-10-capability-record-re-based

**Chunk:** one current record for all 82 capability ids, each claimed or retired with its surface; the old gate reads it
**Date:** 2026-10-10T12:23:16Z
**Commits:** `923a0dad chore(2026-10-10-capability-record-re-based): operator pre-CI commit` (the one commit since the
last wrap's `279a477e`; `git log --format='%h %s' 279a477e..HEAD`)

## Changes (structured — detectors read this)
- **Files:** five outside the chunk folder and the run dirs (`gate.py scope`: `changed 5 · listed 5`, base
  `279a477e`): `docs/capability-record.json` (new, 1661 lines) · `xtask/src/capability_record.rs` (new, 802 lines) ·
  `xtask/src/main.rs` · `.github/workflows/ci.yml` (line 105, a step's display name) ·
  `pulse-app/tests/a11y_perf_workflow.rs` (lines 150-151, an assertion's failure message). Line figures are the
  new-text listing's.
- **Symbols / APIs:**
  - `cargo xtask verify:capability-matrix` keeps its name and changes what it reads. Before: the fixed path
    `docs/v0_2_0/capability-verification-matrix.json`, ids P-001…P-060, exit 0 clean and 1 on a violation, an absent or
    unparseable file falling into the generic `xtask error:` exit 1. Now: two fixed in-repo paths held as constants
    in the new module, `docs/capability-record.json` (`RECORD_PATH`) and `andromeda-pulse-0.4.0/working-route.md`
    (`ROUTE_PATH`); **exit 0 clean · 1 findings · 2 cannot-evaluate**; no path argument and no environment variable.
    Its one line reads `verify:capability-matrix: {clean|violations} ({n} ids: {c} claimed, {r} retired, {k}
    violation(s))`, the three counts read from the record, or `verify:capability-matrix: cannot-evaluate ({reason})`.
    The JSON event line (target `xtask.verify_capability_matrix`) and the report twin
    `target/capability-matrix/report.json` carry `state`, `capability_count`, `claimed_count`, `retired_count`,
    `violation_count`, `reason`; the twin adds `violations` and `generated_at`. The old twin's
    `verification_mode_counts` member is gone.
  - `xtask::capability_record` (new module, private to the xtask binary; `mod capability_record;` at
    `xtask/src/main.rs:10`): `pub fn evaluate(root) -> Verdict` (354-369) reads both files; `pub fn judge(record,
    route, root) -> Verdict` (300-352) is the pure verdict; `pub enum Verdict { CannotEvaluate(String),
    Read(Reading) }` (30-35) with `state` · `exit_code` · `line`; `pub struct Reading { ids, claimed, retired,
    findings }` (37-44). One caller of `evaluate`: `verify_capability_matrix` in `xtask/src/main.rs`, itself called
    once from `main()` (`:327`). The verb's function is no longer `async`.
  - The verdict's arms, in order. **cannot-evaluate:** the record absent, unreadable, not JSON or holding no
    `capabilities` array; the route absent or unreadable. **findings:** the id set not exactly P-001…P-082 once each
    (missing · duplicate · outside the range); a `disposition` that is neither `claimed` nor `retired`; the `legend`
    not equal to the module's three closed sets; on a claimed entry — `carried_by` absent, an element outside
    P-083…P-129, or empty with no `note`; `provisional` true with no `note`; no scenario; scenarios that are all
    `source-evidence`; an unknown scenario kind or verification mode; a file ref that does not exist or cannot be
    read; a `contains` anchor not found; an `external`, `manual` or `by-construction` scenario with no `note`; on a
    retired entry — a `scenarios`, `verification_mode` or `carried_by` key; `surfaces` empty or an unknown word;
    `removed_by` empty or a title no route entry carries; `kept_half_owner` naming such a title; `guard` absent, its
    `state` unknown, `runs` or `part` with an empty `by`, `part` or `none` with an empty `unrun`; a record with no
    claimed id. **clean** otherwise.
  - A route entry's title is its line without a leading `[marker] ` stamp, up to the first ` — `; header lines, `_`
    note lines and indented lines are not entries (`route_titles`, 137-152).
  - No TauRPC procedure, port, socket, environment variable or IPC route is added or changed.
- **Crates / modules:** one module added to the `xtask` crate (`capability_record`). No workspace member added or
  removed; no crate edge changes.
- **Dependencies:** none added, none bumped. `Cargo.toml`, `Cargo.lock`, `deny.toml` and `rust-toolchain.toml` are
  unchanged (the scope guard entry printed nothing against `279a477e`).
- **Schema / config:** `docs/capability-record.json` is a new committed data file with a stated form.
  - Header: `schema_version` (1) · `product` · `version` (`andromeda-pulse-0.4.0`) · `as_of` (`2026-10-10`) ·
    `supersedes` (three paths: `docs/v0_2_0/pulse-capability-spec.md`,
    `docs/v0_2_0/capability-verification-matrix.json`, `andromeda-pulse-0.3.0/verification-matrix.json`) · `legend`
    (three objects — `disposition`: `claimed` · `retired`; `surface`: `window` · `model` · `desktop` · `workspace` ·
    `training-export` · `corpus-encryption`; `guard`: `runs` · `part` · `none` — each word with one line of meaning).
  - `capabilities`: 82 entries in id order. Every entry: `id` · `title` · `disposition` · optional `note`.
    A claimed entry adds `carried_by` (0.4.0 requirement ids) · `verification_mode` · `scenarios` · optional
    `changed_form` · optional `provisional`. A retired entry adds `surfaces` · `removed_by` (working-route entry
    titles) · `guard` (`state` · `by` · `unrun`) · optional `kept_half_owner`.
  - **The form another project reads** (inputs#I2, inputs#I5, inputs#I6, inputs#I7; inputs#I11 item 6): the accepted
    set is every entry whose `disposition` is the string `claimed`; an entry whose `disposition` is `retired` says
    with what in `surfaces` and by which route entry in `removed_by`. Those three fields — `disposition`, `surfaces`,
    `removed_by` — are what Conductor's entry `Accepted capability set re-based` derives its set from; nothing else
    in the file is needed for it. A later change of those three fields' form is a change another repository reads.
  - Scenario kinds the gate knows: five with a file (`nextest-file` · `ui-test` · `a11y-spec` · `ci-script` ·
    `source-evidence`) and three with none (`by-construction` · `external` · `manual`; the last two are new). The
    old gate's `xtask-gate` kind is no longer known; no entry of either old record used it. Verification modes: the
    old seven plus `dynamic-external` and `manual`.
  - No config key, migration, violation schema or scrub shape changes.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - The gate's subject: 60 ids (P-001…P-060) → 82 ids (P-001…P-082), **36 claimed and 46 retired** (basis: the record
    probe `82 82 True 36 46 0`; the verb's line). Stated in the masters at `test-plan.md:535` ("all 60 P-001–P-060
    capabilities", the old path, "Future capability additions (P-061+) extend the matrix JSON") — search
    `verify:capability-matrix|capability-verification-matrix` over the seven masters and `.andromeda/registries/`:
    1 hit, `test-plan.md:535`; 0 in the other six and 0 in the key files. Leaves that restate it:
    `.claude/rules/testing.md:58` and `:107`, `.claude/docs/tests-summary.md:75` and `:84`,
    `.claude/rules/security.md:160` (a Session Addition), `.claude/docs/session-learnings.md:420` (history).
  - Guard states of the 46 retired entries: **6 none · 15 part · 25 runs** (basis: the second record probe,
    `6 15 25 46 4 36`). The plan first read 7 / 14 / 25; the operator's one rule moved P-076 to `part`
    (`evidence/operator-edit-plan.md`, inputs#I9, inputs#I10).
  - Four claimed entries carry `provisional: true`: P-030, P-040, P-048, P-050.
  - Workspace test count on the dev host: 2944 of 2944 (`cargo nextest run --workspace --profile ci`, implement's
    two firings); 36 of them are this chunk's (`capability_record::tests::*`, 36 distinct names in the run log).
- **Dev-tool versions:** none — no host tool was installed, upgraded or read changed.
- **Harness / gate surface:**
  - The xtask verb as above (input, exits, line, twin).
  - `.github/workflows/ci.yml:105`: the `lint-test` step's display name changed from
    `cargo xtask verify:capability-matrix (chunk #99 — P-001..P-060 scenario mapping)` to
    `cargo xtask verify:capability-matrix (reads the capability record — P-001..P-082, each claimed or retired)`.
    Its `run:` line, its place after `capability-widening-check` and every other line of the workflow are unchanged
    (the parsed-workflow probe: `1 True True True -`).
  - `pulse-app/tests/a11y_perf_workflow.rs:150-151`: the failure message of
    `ci_workflow_invokes_xtask_verify_capability_matrix`; its assertion is unchanged.
  - 36 new pins in `xtask/src/capability_record.rs` (`mod tests`, 371-802), over constructed records under
    `tempfile::TempDir`, one per arm, plus one that reads the committed record over the committed route
    (`the_committed_record_reads_clean_over_the_committed_route`, 525-536) and asserts the verb's clean line with
    `82 ids: 36 claimed, 46 retired`.
  - The gate now reads a file the wrap writes, `andromeda-pulse-0.4.0/working-route.md`: a route-resolve that
    renames, retires or splits an entry the record names reddens the verb and the committed-record pin until the
    record is corrected.
- **Cross-project / external claims:**
  - CI: `ci#38049792921` (pull-request event, attempt 1) on `923a0dad8ff40830104b9ffe1a9ae26b362a9706`,
    `verdict: green`, checks 7/7, six `ci` jobs each `success`; `secret-scan#38049792949` success. The `lint / test`
    job log of that run holds the verb's clean line once (`evidence/operator-pass.md`). The verdict was taken on
    the pre-CI commit; this wrap's commit follows it and changes one entry of the record (P-040, below).
  - Conductor (`../conductor`, `build/conductor-0.4.0` at `321dc8f2`): its route entry `Accepted capability set
    re-based` waits on this record (inputs#I2); its pin `contracts/pulse-capabilities.toml` names 82 ids
    (inputs#I5) and is read by membership (inputs#I6); its requirement v4-06 (inputs#I7); its copies of the two old
    records (inputs#I3, inputs#I4). Nothing of Conductor's was written.
  - `inputs.py verify`, 11 entries — `unchanged 8 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 ·
    n/a 3 · uncited 3 · unparsed 0` before this report existed:
    - I1 · `../additional/pc-overseer/relays/pulse-phase-capability-record-re-based-2026-10-10.md` · copy · unchanged
    - I2 · `../conductor:conductor-0.4.0/working-route.md` · pointer @321dc8f2 · unchanged
    - I3 · `../conductor:.andromeda/refs/capability-verification-matrix.json` · pointer @321dc8f2 · unchanged
    - I4 · `../conductor:.andromeda/refs/pulse-capability-spec.md` · pointer @321dc8f2 · unchanged
    - I5 · `../conductor:contracts/pulse-capabilities.toml` · pointer @321dc8f2 · unchanged
    - I6 · `../conductor:crates/conductor-core/src/capability_manifest.rs` · pointer @321dc8f2 · unchanged
    - I7 · `../conductor:conductor-0.4.0/requirements.md` · pointer @321dc8f2 · unchanged
    - I8 · the operator's four answers at the P4 dialog · copy (message) · n/a
    - I9 · the operator's first plan-edit directive, given at implement · copy (message) · n/a — cited here
      (inputs#I9); it read `UNCITED` before this report
    - I10 · the operator's second directive (six follower lines; the operator pass) · copy (message) · n/a — cited
      here (inputs#I10)
    - I11 · `../additional/pc-overseer/relays/pulse-wrap-capability-record-re-based-2026-10-10.md`, the wrap
      directive, snapped at this wrap · copy · unchanged — cited here (inputs#I11)
- **Reverted / negative API facts:** none. Considered and not built, by the plan's rejections: a path argument or
  environment variable naming the record; a workflow-inline reader; validating that a retired entry's `guard.by`
  files exist.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `test-plan.md:535` states the gate "validates `docs/v0_2_0/capability-verification-matrix.json`, which
    enumerates all 60 P-001–P-060 capabilities" and that additions from P-061 on "extend the matrix JSON". False
    from this chunk on: the gate reads `docs/capability-record.json`, 82 ids, two dispositions, exits 0 · 1 · 2
    (basis: `xtask/src/capability_record.rs` `RECORD_PATH`; the verb's line on the tree and in `ci#38049792921`).
  - Not a master: `docs/v0_2_0/capability-verification-matrix.json`'s own `purpose` line says it is "Validated by
    `cargo xtask verify:capability-matrix`". False from this chunk on. The file stays byte-identical here by the
    plan; its marking is the route's (the entry `Records say what the product is`).
  - The plan's entry-5 atom assumed the base CI step's parsed name ran to "scenario mapping)"; YAML reads the text
    after ` #` as a comment, so the parsed name ends at "(chunk" (`yaml.safe_load` of `ci.yml` at `279a477e`). Three
    step names of the base workflow end that way. Corrected in the plan by the operator edit; not a master claim.
  - `research.md` reads P-076 as "no proof runs". Superseded on this point by the operator's one rule (inputs#I9,
    inputs#I10): 54 pins under `xtask::webview_drive::` ran in implement's workspace run, so P-076 is `part`.
    research.md stays as written.
- **Expected amendments (from plan):**
  - test-plan §9 Capability verification matrix — the gate's input is `docs/capability-record.json`, all 82 ids, two
    dispositions, exits 0 · 1 · 2; the sentence on ids from P-061 on restated for the one record. **Carried**: the
    Symbols / APIs, Counts and Spec-claims bullets above. Site search: `verify:capability-matrix|
    capability-verification-matrix` — test-plan 1 hit (`:535`), 0 in its key files.
  - architecture §Occupied Resources → xtask CLI surfaces (dev/CI gates) — register `cargo xtask
    verify:capability-matrix` with its module, its two inputs, its exits, its arms and its line. **Carried**: the
    Symbols / APIs and Harness bullets above. Site search: the same pattern — architecture 0 hits (the verb was
    never registered there); the registry bullet is `architecture.md:261`; `ci-gates|quarantine-tracking` locates
    the sibling registrations at `:261` and in `.andromeda/registries/contracts/architecture/ci-cd-approach.md`.
  - P-117's ledger note — written at phase P5 through the ledger tool; not an amendment. Nothing to write here.
  - From the wrap directive (inputs#I11 item 6), not in the plan's list: where the masters list what an external
    reader may rely on, the record is named with its three fields. **Carried**: the Schema / config bullet "The
    form another project reads". Site search: `capability-record` — 0 hits in the seven masters and the key files;
    the masters' home for contracts an outside reader relies on is architecture §Standard Contracts
    (`architecture.md:100`).
- **Coverage of new surfaces:**
  - `cargo xtask verify:capability-matrix` (a dev/CI verb over two committed in-repo files) → validation
    mechanism✓ (every malformed or absent input is a finding or the cannot-evaluate exit; no caller-supplied path) ·
    instrumentation n/a (a dev tool; it prints one JSON event line, no product log) · PII n/a (its inputs are
    committed project documents; it prints ids, route titles and repo-relative refs, and the report twin's path) ·
    tests unit (36 pins) · a11y n/a · tokens n/a
  - `docs/capability-record.json` (a committed record read by the verb and by another project) → validation
    mechanism✓ (the verb, in CI `lint-test`) · instrumentation n/a · PII n/a · tests unit (the committed-record
    pin) + the two record probes · a11y n/a · tokens n/a

## Deviations from intent
- **The plan was edited between runs on the operator's word** (inputs#I9, inputs#I10; `evidence/operator-edit-plan.md`):
  entry 5's name atom, the P-076 table row, the guard-count atom, and six follower lines. /implement edits no plan
  on its own; the operator directed it as his hands.
- **P-076 is `part`, not `none`** (the operator's one rule). The record, the plan and the probe atom carry it;
  research.md does not and stays as written.
- **P-040's line was rewritten at this wrap** on the wrap directive (inputs#I11 item 3), after the pre-CI commit
  and its CI run: `carried_by` is `["P-084"]` (the plan's table says P-084, P-099); `changed_form` quotes the two
  clauses of the one P-084 sentence and says in so many words that no requirement sentence says the engine works
  with no reader at the door, that this is not claimed, and that the working-route entry `Door inside the engine's
  process` would have to state it; the `note` says P-099 is not listed because no sentence of it carries the form.
  The diff is that one entry (3 lines in, 4 out). The verb reads the edited record clean; the light gate re-runs
  the block over it.
- **Two verification-mode words were added**, `dynamic-external` (P-075) and `manual` (P-077), as the 0.3.0 ledger
  spells its methods; the plan named no mode for the four claimed 0.3.0 ids. P-067 and P-074 take
  `automated-nextest`.
- **The `xtask-gate` scenario kind was dropped**; no entry of either old record used it.
- **One arm beyond the plan's list**: a claimed entry whose scenarios are all `source-evidence` is a finding (the
  old gate's rule, without its notes escape).
- **P-074 carries a `changed_form`** the plan's table does not list: its text names the cadence → digest → model
  chain, which `requirements.md:101-102` says leaves.
- **The report twin** lost `verification_mode_counts` and gained `claimed_count`, `retired_count`, `reason`.
- **The old verb body was replaced by a scratchpad script** with count-1 asserts on its three anchors, not by the
  Edit tool.
- **Two claimed ids have no proof that runs in any gate**, as the plan directs ("the old record's own,
  unchanged"): P-040's one scenario is a webview unit test, and P-030's is the old by-construction argument for
  the original sentence; its note says the changed form has no proof yet.
- scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 5 · listed 5 · recorded 0`, base
  `279a477e`).

## Decisions & corrections
- **The operator, 2026-10-10 (inputs#I9):** entry 5's name atom ends at "(chunk"; P-076 is `part` by one rule for
  the guard word.
- **The operator, 2026-10-10 (inputs#I10):** the six follower lines follow the same edit; research.md is
  superseded on P-076 and stays as written; the operator pass, entries 20 to 25 around the pre-CI commit; a red
  boot job would have been a new reading (it read green).
- **The operator's wrap directive (inputs#I11):** P-117 is not claimed; the four provisional ids stay PROVISIONAL
  in the record and the handoff; P-040's line quotes the one sentence or says none does; the route pins the plan
  owes; an owner for the two old records; the record named where the masters list what an external reader relies
  on; stops at the route-resolve card and before the flip and the commit.
- **Sweep hazard found:** an unquoted YAML step name is cut at ` #` (a comment). A probe that prints a parsed step
  name never sees the text after it; `ci.yml` at the base holds three step names cut that way (`(chunk`).
- **Sweep hazard found:** a P5 baseline of a `new` probe that reads red for one reason cannot vouch for an atom
  that only prints once the chunk's edit exists (entry 5's name line was empty on the untouched tree).
- **Process:** the Bash guard refused a `cat` heredoc with a file target once and a leading `cd` into a
  subdirectory once; both were re-issued in the guard's form.
- **Evidence transcription:** one job timestamp was mistyped in `evidence/operator-pass.md` and caught by checking
  every copied line against its log before the record was closed.

## Outcome
**Acceptance criteria, each against the diff.**
- (record) 82 entries, P-001…P-082 once each, each `claimed` or `retired` — **met** (`82 82 True 36 46 0`).
- (record) every retired entry names a surface and a removing entry the route carries, and states its guard; every
  claimed entry names its carrying requirement or says why none does — **met**; the plan's figure for the second
  probe is `6 15 25 46 4 36` after the operator edit, and the record reads it.
- (record) no third disposition; the four PROVISIONAL entries are claimed, each `changed_form` opening "The
  original sentence no longer holds as written" — **met** (P-030, P-040, P-048, P-050).
- (gate) the verb exits 0 with `verify:capability-matrix: clean (82 ids: 36 claimed, 46 retired, 0 violation(s))`
  and validates proofs of claimed ids only — **met**.
- (gate) the verb's contract is stated in the module and the `about` string; `ci.yml` gains no inline reader —
  **met**. Its registration in the masters is this wrap's amendment.
- (tests) pins redden per arm; an absent record and an absent route read cannot-evaluate; one pin reads the
  committed record; step 0's red reading is in `evidence/red-before-green.md` (35 of 36 red against a
  pass-everything stub) — **met**.
- (tests) the workspace run returns 0 with no test newly ignored or deleted; the bindings equal the base — **met**.
- (security) `capability-drift` and `check:staged-artifacts` exit 0; the scope guard prints nothing; the workflow
  diff adds no `uses:`, `permissions:` or `run:` line — **met**.
- (obs, a11y, layouts) the scope guard prints nothing — **met**.
- (arch) no workspace member and no manifest changes; the two old records are byte-identical to the base — **met**
  (`git diff --quiet 279a477e HEAD -- docs/v0_2_0 andromeda-pulse-0.3.0`, exit 0).
- (ci) the pull-request run of the pushed tip reads `verdict: green`, its id recorded, and the `lint / test` log
  holds the clean line once — **met** for `923a0dad` (`ci#38049792921`).
- (process) the merge-base probe is the entry directly before the push — **met**.
- **P-117 is advanced and not claimed.** Shown here: one current record for all 82; the old gate passes while
  carrying no retired id's proof. Not shown: "the external harness's accepted set equals that record" — read in
  Conductor, after its entry runs.

**Gates** (by `run`; the verdicts are implement's firings and the operator pass's; the wrap's light gate re-runs
the block).
- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
- `git diff --name-only 279a477e… -- crates pulse-app xtask scripts docs …` (the scope guard) — green · exit 0, no
  output
- `python -X utf8 -c "import yaml; …print(' '.join(sorted(d['jobs'])))"` — green · last line the six job names
- `python -X utf8 -c "import subprocess, yaml; …"` (the parsed workflow against the base) — red at implement's
  first firing on the name atom alone; **green** after the operator edit: `lint-test/cargo xtask
  verify:capability-matrix (chunk`, `1 True True True -`
- `grep -c 'run: cargo xtask verify:capability-matrix' .github/workflows/ci.yml` — green · last line `1`
- `git diff 279a477e… -- .github/workflows/ci.yml | grep -c -E '^\+.*(uses:|…|run:)'` — green · exit 1, last line `0`
- the first record probe — green · `82 82 True 36 46 0`
- the second record probe — green · `6 15 25 46 4 36` (after the operator edit)
- `cargo xtask check:english-sources` — green · `"verdict": "clean"`
- `cargo xtask capability-widening-check` — green · exit 0
- `cargo xtask check:ingest-progress` — green · exit 0
- `cargo xtask check:staged-artifacts` — green · exit 0
- `cargo xtask capability-drift` — green · exit 0
- `cargo xtask verify:capability-matrix` — green · the clean line
- `cargo nextest run --workspace --profile ci` — green · 2944 of 2944, 0 skipped (two firings)
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet 279a477e… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green at implement and again on
  the committed tree in the operator pass: `"verdict": "green"`, `all-stages-ok`, head `923a0dad`
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (`leg = 'operator'`) — fired as written
  before the pre-CI commit: `hygiene: clean — read 61`
- the merge-base probe (`leg = 'operator'`) — green · exit 0, `history: unmoved`
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0`
  (`leg = 'operator'`) — green · `history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 279a477e→923a0dad`
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`
  (`leg = 'operator'`) — fired as written: `verdict: green · checks 7/7 · wall 1731 s`, `ci#38049792921`
- the `lint / test` job-log read (`leg = 'operator'`, `<id>` = 38049792921) — green · last line `1`
- the job list (`leg = 'operator'`, report-only) — **recorded**; the outcome it printed: six jobs of
  `ci#38049792921`, each `success` (boot smoke, supply-chain, lint / test, coverage gate, a11y, mcp-server tests);
  nothing to disposition.
- Smoke: skipped — no boot-path and no UI-surface change; the verb ran for real as its own entry.

**Watches:** none — the entry carried no freight.

**Outcome basis.** The operator pass ran: the verdicts above rest on its final state, the commit `923a0dad` and its
CI run `ci#38049792921`, recorded in `evidence/operator-pass.md`. Implement's report, given in this same
conversation, is the basis for what only it holds (the red-first reading, the deviations, its census). Between
implement and this report: two operator directives (inputs#I9, inputs#I10) that changed the plan and P-076, and
the wrap directive (inputs#I11) that changed P-040's line after the CI run. The conversation is present; the
chunk's evolve records were not read.

**Process hygiene.** Implement's census, measured against the process list after its gate run and again after the
operator pass: no `cargo`, `nextest`, `rustc`, `pulse-app`, `xtask` or `ci.py` process left running; this chunk
started no app and bound no port. Started by this wrap: the code-graph refresh (background), read at P4.

**Limits.** One CI run was read, attempt 1, on the pre-CI commit; only the `lint / test` job log was opened. The
P-040 edit postdates that run; it is covered by the local block alone until the wrap's commit is pushed and read.
The cannot-evaluate arms of the verb are held by pins, never by a run of the verb over an absent file (it takes no
path). No Conductor file was read again at this wrap beyond `inputs.py verify`'s hash comparison.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 279a477e (the parent of the oldest pre-CI commit 923a0dad) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 1 line(s) in 1 range(s)
added: 105
### docs/capability-record.json — new file · 1661 line(s)
- 1-1661 «{»
  - 6-10 «"supersedes": [»
  - 11-29 «"legend": {»
  - 30-1660 «"capabilities": [»
### pulse-app/tests/a11y_perf_workflow.rs — added 2 line(s) in 1 range(s)
added: 150-151
### xtask/src/capability_record.rs — new file · 802 line(s)
- 20-27 «pub const SURFACES: [&str; 6] = [»
- 30-35 @31 «pub enum Verdict {»
- 37-44 @39 «pub struct Reading {»
- 46-79 «impl Verdict {»
  - 47-53 «pub fn state(&self) -> &'static str {»
  - 55-61 «pub fn exit_code(&self) -> u8 {»
  - 63-78 @64 «pub fn line(&self) -> String {»
- 81-88 @82 «const FILE_KINDS: [&str; 5] = [»
- 92-102 «const MODES: [&str; 9] = [»
- 107-113 @108 «fn id_number(id: &str) -> Option<u32> {»
  - 110-112 «(digits.len() == 3 && digits.bytes().all(|byte| byte.is_ascii_digit()))»
- 115-117 «fn text<'a>(entry: &'a Value, key: &str) -> &'a str {»
- 119-131 @120 «fn words<'a>(entry: &'a Value, key: &str) -> Vec<&'a str> {»
  - 121-130 «entry»
- 133-135 «fn has_note(entry: &Value) -> bool {»
- 137-152 @140 «fn route_titles(route: &str) -> BTreeSet<&str> {»
  - 141-151 «route»
- 154-180 «fn legend_findings(legend: Option<&Value>, findings: &mut Vec<String>) {»
  - 155-158 «let Some(legend) = legend.and_then(Value::as_object) else {»
  - 159-163 «let sets: [(&str, &[&str]); 3] = [»
  - 164-176 «for (key, closed) in sets {»
  - 177-179 «if legend.len() != sets.len() {»
- 182-247 «fn claimed_findings(id: &str, entry: &Value, root: &Path, findings: &mut Vec<String>) {»
  - 183-197 «match entry.get("carried_by").and_then(Value::as_array) {»
  - 198-200 «if entry.get("provisional") == Some(&Value::Bool(true)) && !has_note(entry) {»
  - 202-204 «if !MODES.contains(&mode) {»
  - 206-210 «let scenarios = entry»
  - 211-214 «if scenarios.is_empty() {»
  - 215-240 «for scenario in scenarios {»
  - 241-246 «if scenarios»
- 249-298 «fn retired_findings(id: &str, entry: &Value, titles: &BTreeSet<&str>, findings: &mut Vec<String>) {»
  - 250-254 «for key in ["scenarios", "verification_mode", "carried_by"] {»
  - 256-258 «if surfaces.is_empty() {»
  - 259-263 «for surface in surfaces {»
  - 265-267 «if removing.is_empty() {»
  - 268-274 «for title in removing {»
  - 275-282 «if entry.get("kept_half_owner").is_some() {»
  - 283-286 «let Some(guard) = entry.get("guard").filter(|guard| guard.is_object()) else {»
  - 288-291 «if !GUARD_STATES.contains(&state) {»
  - 292-294 «if state != "none" && words(guard, "by").is_empty() {»
  - 295-297 «if state != "runs" && words(guard, "unrun").is_empty() {»
- 300-352 @302 «pub fn judge(record: &str, route: &str, root: &Path) -> Verdict {»
  - 303-305 «let Ok(doc) = serde_json::from_str::<Value>(record) else {»
  - 306-308 «let Some(capabilities) = doc.get("capabilities").and_then(Value::as_array) else {»
  - 315-340 «for entry in capabilities {»
  - 341-346 «for number in RECORD_IDS {»
  - 347-349 «if reading.claimed == 0 {»
- 354-369 @355 «pub fn evaluate(root: &Path) -> Verdict {»
  - 356-359 «let read = |relative: &str| {»
  - 360-363 «let record = match read(RECORD_PATH) {»
  - 364-367 «let route = match read(ROUTE_PATH) {»
- 371-802 @372 «mod tests {»
  - 379-384 «const ROUTE: &str = "# Working Route\n\n\»
  - 386-399 «fn legend() -> Value {»
  - 401-410 «fn claimed(id: &str) -> Value {»
  - 412-421 «fn retired(id: &str) -> Value {»
  - 423-432 @424 «fn valid_record() -> Value {»
  - 434-449 «fn root_holding(record: Option<&str>, route: Option<&str>) -> tempfile::TempDir {»
  - 451-454 «fn judge_record(record: &Value) -> Verdict {»
  - 456-469 «fn assert_cannot_evaluate(verdict: &Verdict, names: &str) {»
  - 471-486 @473 «fn only_finding(record: &Value) -> String {»
  - 488-494 «fn assert_finding(record: &Value, id: &str, says: &str) {»
  - 496-501 @497 «fn with_entry(index: usize, change: impl FnOnce(&mut Value)) -> Value {»
  - 503-507 «fn without_key(index: usize, key: &str) -> Value {»
  - 509-514 @510 «fn a_valid_record_reads_clean() {»
  - 516-523 @517 «fn the_counts_on_the_line_are_read_from_the_record() {»
  - 525-536 @526 «fn the_committed_record_reads_clean_over_the_committed_route() {»
  - 538-542 @539 «fn an_absent_record_cannot_be_evaluated() {»
  - 544-549 @545 «fn an_unreadable_record_cannot_be_evaluated() {»
  - 551-555 @552 «fn a_record_that_is_not_json_cannot_be_evaluated() {»
  - 557-561 @558 «fn a_record_with_no_capabilities_array_cannot_be_evaluated() {»
  - 563-567 @564 «fn an_absent_route_cannot_be_evaluated() {»
  - 569-577 @570 «fn a_missing_id_is_a_finding() {»
  - 579-587 @580 «fn a_duplicated_id_is_a_finding() {»
  - 589-597 @590 «fn an_id_outside_the_range_is_a_finding() {»
  - 599-603 @600 «fn an_unknown_disposition_is_a_finding() {»
  - 605-613 @606 «fn a_legend_that_is_not_the_three_closed_sets_is_a_finding() {»
  - 615-618 @616 «fn a_claimed_entry_without_carried_by_is_a_finding() {»
  - 620-624 @621 «fn a_carrying_requirement_outside_the_range_is_a_finding() {»
  - 626-636 @627 «fn an_empty_carried_by_needs_a_note() {»
  - 638-642 @639 «fn a_provisional_entry_needs_a_note() {»
  - 644-648 @645 «fn a_claimed_entry_with_no_scenario_is_a_finding() {»
  - 650-656 @651 «fn source_evidence_alone_is_no_proof() {»
  - 658-662 @659 «fn an_unknown_scenario_kind_is_a_finding() {»
  - 664-668 @665 «fn an_unknown_verification_mode_is_a_finding() {»
  - 670-676 @671 «fn a_dangling_file_ref_is_a_finding() {»
  - 678-684 @679 «fn a_missing_anchor_is_a_finding() {»
  - 686-699 @687 «fn a_scenario_with_no_file_needs_a_note() {»
  - 701-714 @702 «fn a_retired_entry_carries_no_proof_of_its_own() {»
  - 716-720 @717 «fn a_retired_entry_with_no_surface_is_a_finding() {»
  - 722-726 @723 «fn an_unknown_surface_is_a_finding() {»
  - 728-732 @729 «fn a_retired_entry_with_no_removing_entry_is_a_finding() {»
  - 734-738 @735 «fn a_removing_entry_the_route_does_not_carry_is_a_finding() {»
  - 740-751 @741 «fn a_title_is_read_past_its_marker_stamp_and_never_from_a_note_line() {»
  - 753-762 @754 «fn a_kept_half_owner_the_route_does_not_carry_is_a_finding() {»
  - 764-767 @765 «fn a_retired_entry_with_no_guard_is_a_finding() {»
  - 769-773 @770 «fn an_unknown_guard_state_is_a_finding() {»
  - 775-784 @776 «fn a_guard_that_runs_names_what_runs() {»
  - 786-794 @787 «fn a_guard_short_of_running_names_what_does_not_run() {»
  - 796-801 @797 «fn a_record_with_no_claimed_id_is_a_finding() {»
### xtask/src/main.rs — added 32 line(s) in 10 range(s)
added: 10 · 245 · 327 · 767-772 · 777-781 · 783-792 · 796-798 · 806-807 · 811-812 · 817
  - 784-791 «let counts = serde_json::json!({»
