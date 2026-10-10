# Fan-out results — 2026-10-10-boot-smoke-s-self-end-closed

Seven doc-agents, one batch, each sent the amendment-flow prompt verbatim with its detectors (19 detector slots over
19 drift-base entries: architecture 2 · security-plan 4 · design-system 2 · layout-templates 2 · test-plan 3 ·
obs-plan 4 · a11y-plan 2) and, for the four masters with keyed contracts, its render. Entities: each return was read
whole as it arrived and holds no `&lt;`, `&gt;` or `&amp;`; no scripted probe ran, the returns reaching this window
as messages and not as files. Each proposal's `rationale` is abridged here to the report bullet it cites; `change`,
`sidecar`, `basis` and `dependent-of` are as returned (a `change` longer than its claim is cut at `…` after the
claim). Coordinates were checked against the report's bullets and its `## New text, by line` section: every one is
the report's. **Rejected for a source the report does not carry: 0.**

## Verdicts

- **architecture** — 7 proposals (2 primaries, 5 dependents), all `warning`. No commentary stripped.
- **security-plan** — 5 proposals (1 primary, 4 dependents), all `escalate` (the detector's declared severity).
  Stripped: a comment block saying D-security-auth, -deps and -logging read no drift; that D-security-input found
  no unvalidated boundary and its drift is three reader enumerations that do not name the new reader; "Severity
  below is the detector's declared `escalate`, not a finding of a validation gap"; and that `xlib_threads::init`
  was considered and not proposed (no input from outside the binary, no security-plan claim retired).
- **design-system** — `proposals: []`. Commentary stripped; raw twin `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Nothing stripped.
- **test-plan** — 5 proposals (1 primary, 3 dependents, 1 further primary), all `warning`. Stripped: a comment
  block saying D-tests-framework and D-tests-obs-harness read no drift, and one note "for the orchestrator's eye":
  §4 and the §9 `lint-test` row do not mention that `unit_xlib_threads` loads a host library, and the report did
  not state whether a host without it fails or skips (raised below as O1).
- **obs-plan** — 6 proposals (4 primaries, 2 dependents), all `warning`. Stripped: a comment block saying
  D-obs-instrumentation, -stack and -pii read no drift; that the defect's status sits at §7 Process-end cause with
  §10 restating it; and that §9's log-file row was swept and left standing (its `ci#38019133294` reading is dated
  history and its description of `boot-series.json` still holds).
- **a11y-plan** — `proposals: []`. Commentary stripped; raw twin `.raw-fanout-a11y-plan.md`.

## architecture

- **A1** · D-arch-resources · warning · §Occupied Resources → xtask CLI surfaces, the `harness:boot-series` row, the
  exit-code clause. change: replace "1 `self-ended` (a boot's settle verdict read `ended`)" with the two ways: a
  settle verdict of `ended`, or since this chunk a boot with no settle verdict whose exit record is an end the app
  made itself (any record but `signal 15 (TERM)` and `signal 9 (KILL)`; none stays `other`); such an entry keeps
  `verdict`, `app_exit_record`, `windows_settled` null and carries `ended` and `exit_witness` read from the boot's
  own data dir (the pid from `run/andromeda-pulse.spawn`, the record through `harness_status::{end_file, read_ended,
  read_pid}`, the label through `harness_witness::label`, its second production caller), both through the 48-byte
  bound; member sets, `cycle` labels, verdict set, exit codes unchanged. rationale: Schema / config; Symbols; Spec
  claims disproved. basis: `.andromeda/architecture.md:258`; `xtask/src/harness_series.rs:285-294`, `378-389`.
  → **apply** (check 1: "Accurate this-chunk addition", routine; check 5: expected amendment 1).
- **A2** · dependent-of D-arch-resources · same row, the `cycle` / ordinal 1 clause. change: ordinal 1 is listed from
  the smoke's `logs/harness-settled.json` when the top data dir holds one, else from the smoke's own exit record and
  witness label when that record is an own end (a top dir whose record is the verb's own `TERM` is not listed);
  out of the counts either way. basis: `architecture.md:258`; `harness_series.rs:391-405`. → **apply** (as A1).
- **A3** · dependent-of D-arch-resources · same row, the "A measured limit:" sentence. change: retire it to a dated
  reading (until this chunk … counted `other`, as measured on `ci#38019133294`); since this chunk counted `ended`
  with its exit record and witness label; the new limit: that read never ran on a runner (all 24 boots of
  `ci#38026637514` reached ready), read on a real process by the local repair leg and pinned per arm; the
  `timed-out` path never ran live. basis: `architecture.md:258`. → **apply** (as A1).
- **A4** · dependent-of D-arch-resources · §Occupied Resources → Filesystem locations, the `run/andromeda-pulse.spawn`
  + `run/andromeda-pulse.exit` entry. change: add `harness:boot-series` to the readers — for a boot that took no
  settle verdict, in its own data dir and in the top data dir for the smoke, the pid from the spawn record and the
  end through the same bounded readers, then that dir's `logs/exit-witness.jsonl` through `harness_witness::label`;
  nothing of `run/` copied, two bounded labels leave. basis: `architecture.md:218`. → **apply** (as A1).
- **A5** · dependent-of D-arch-resources · §Infrastructure Patterns → CI/CD approach (key file). change: "a boot whose
  settle verdict reads `ended` fails the job" becomes a boot that ended by itself fails the job, by either way; the
  step, `--count 7`, `if: always()` and the three workflow tests unchanged. basis:
  `registries/contracts/architecture/ci-cd-approach.md:3`. → **apply** (as A1).
- **A6** · D-arch-decisions · warning · §Stack and Technologies. change: add one row, the Xlib thread
  initialisation (Linux, product start path): `libX11.so.6` reached at run time through the already-direct `libc`
  (`dlopen` `RTLD_NOW`, `dlsym` `XInitThreads`, one call, handle never closed), no Cargo dependency, manifest or
  lockfile change; `pulse_app::xlib_threads::init()` the SECOND statement of `main()` after the render-posture step
  and before the runtime; the five-value return unused and unlogged; inert where the library or symbol is not
  found; no environment variable, log record, thread or outside input; one call site, pinned; why (`tao` 0.35.0's
  device-event thread; Xlib 1.7.5 against 1.8.13; 12 of 16 against 0 of 24) … basis: `pulse-app/src/main.rs:282`;
  `pulse-app/src/xlib_threads.rs:19-28`, `30-37`, `39-65`, `67-71`; `architecture.md:242` (not amended). → **apply**
  (check 1: routine by "Accurate this-chunk addition", and the operator's recorded direction settles it: inputs#I3
  "through the libc crate it already depends on", inputs#I4 for the place; check 5: expected amendment 2). The row
  carries the leaving owner `Window retired` (inputs#I5 item 3). Not read as a boundary widening: no input crosses;
  the call resolves, by its soname, a library the binary already links.
- **A7** · dependent-of D-arch-decisions · §Infrastructure Patterns → CI/CD approach (key file). change: "the cause is
  not closed and that red is read by its per-boot verdicts" becomes the closed status with the readings:
  `ci#38022477393` (5 of 8, 12 of 16 over the two runs) and `ci#38026637514` (success on attempts 1, 2, 3; 24
  boots, 0 self-ended); the series stays a gating step and a self-end in it is a new reading, not an expected red.
  basis: `ci-cd-approach.md:3`. → **apply** (check 5: expected amendment 3; inputs#I5 item 2).

## security-plan

Every proposal here: detector D-security-input · severity `escalate` · rationale: Symbols / APIs ("New callers of
existing readers"), Schema / config, Coverage of new surfaces (`validation mechanism✓`), Expected amendments (the
three sites; classification stays PROVISIONAL) · basis `xtask/src/harness_series.rs:378-389`.

- **S1** (primary) · §Security Anti-Patterns → Input, the exit-witness arm's item (d). change: add that
  `harness:boot-series` also reads, for a series boot with no settle verdict and for the smoke's top data dir when it
  holds no `logs/harness-settled.json`, that boot's own data dir through the existing bounded readers (the pid from
  the spawn record through `read_pid`, the exit record through `read_ended`'s grammar, the witness file through
  `harness_witness::label`) and lists the exit record and one closed witness label as `ended` / `exit_witness` in
  `boot-series.json`, both through the 48-byte bound; no member, label, kept file or witness line kind added,
  nothing new read from inside the app's process, no environment value and no path; classification stays
  PROVISIONAL; limit: never ran on a runner.
- **S2** (dependent) · same section, the arm's item (c). change: the bounded witness reader has two production
  callers (`harness:settled`, `harness:boot-series`) and its one closed label leaves as the settle verdict's eighth
  member and, for a boot with no settle verdict, as that boot's `exit_witness` in `boot-series.json`; label set
  unchanged.
- **S3** (dependent) · same section, the boot-recorder STATE FILES class. change: `harness:boot-series` added to the
  readers of the spawn record (`read_pid`, not the pid file) and the exit record (`read_ended`); "read only by the
  harness" stands.
- **S4** (dependent) · §Input Validation, the `CLI / env var inputs` row's fourth harness-only boundary. change: the
  witness reader stands behind `harness:settled`'s member and behind `harness:boot-series`'s per-boot
  `exit_witness`; the series sentence gains the before-ready read through `read_pid`, `read_ended` and the bounded
  reader, both values through `bounded_label`.
- **S5** (dependent) · §Threat Model Summary → Attack surface → CLI input (Trust boundary). change: the series verb's
  read set gains "a boot's own spawn record, exit record and witness file through the existing bounded readers when
  that boot took no settle verdict"; "No `pulse-app` code reads any of them" and PROVISIONAL stand.

→ **S1 to S5: escalate — HALT for the operator's word** (check 1). The playbook alone reads them routine by actual
class: rule "An ESCALATE-severity detector … fires on a finding OUTSIDE the class that severity exists to guard"
(both its conditions hold: the report's Coverage row reads `validation mechanism✓`, so no unvalidated boundary; the
actual class, a reader enumeration brought to what the chunk shipped, is governed by "Accurate this-chunk
addition"), and the P5-approved expected amendment 6 names the change itself. They are staged to escalate on the
operator's wrap directive, inputs#I5 item 7: "A proposal a detector grades `escalate` halts at the card for my word.
The witness record's classification stays PROVISIONAL." Recommended: apply all five as written (descriptive; one
dependent group, applied whole or not at all).

**Resolved at the escalation card, 2026-10-10.** Three options were put (apply all five · apply, and mark for the
founder · decline for now). The answer: "Apply, and mark for the founder", word: "the second reader belongs to the
same PROVISIONAL item as the record itself - one thing awaits his word, not two." — the operator (the pc overseer),
2026-10-10, given here. → **S1 to S5: applied**, each amended site saying the second reader is part of the same
PROVISIONAL item as the witness record. One change from the proposals on that word: S3's addition to the
boot-recorder state files class points to the exit-witness arm's item (d) for its classification, so the read sits
under one item and not under the classified state-files class beside it.

No playbook rule is proposed: the class already has its rule (the escalate-severity detector outside its guarded
class), and the halt came from the wrap directive, not from a missing rule.

## test-plan

- **T1** · D-tests-coverage · warning · §1 Pending coverage triggers →
  `harness-cleanup-verdict-and-boot-spawn-shell-coverage`. change: a "Narrowed" clause: 25 unit pins (was 16); the
  verdict no longer under-reads a before-ready boot (the read, the own-end rule, ordinal 1); the `ci#38019133294`
  reading stays as the measurement before the change; the legs now (repair, close, host, `ci#38026637514` ×3); in
  "Owed" drop the before-ready record (discharged at the unit tier plus the repair leg), state the never-on-a-runner
  limit; the witness-arm branches and a live `timed-out` reading stay owed; the row stays open. basis:
  `.andromeda/test-plan.md:134`. → **apply** (routine; expected amendment 4b).
- **T2** · dependent · §9 Pipeline structure → Boot smoke (harness). change: replace the two sentences ("a boot whose
  settle verdict reads `ended` fails the job. A measured limit: …") with the own-end rule in both ways, the null
  members, ordinal 1, unchanged shape; keep the `ci#38019133294` reading and add `ci#38026637514`. basis:
  `test-plan.md:496`. → **apply** (routine; expected amendment 4a).
- **T3** · dependent · §9, the build-failure conditions, the Linux boot-smoke bullet. change: restate the `self-ended`
  parenthesis in both ways. basis: `test-plan.md` (no line given; the bullet is line 509 by this wrap's read). →
  **apply** (routine).
- **T4** · dependent · §3 → 5-command implementation, the `boot` label's Readiness signal. change: replace its closing
  two sentences with the closed status and the measured chain (no read failed; the second thread; the library
  builds read from Ubuntu's packages; `errno` 11 stale; the call second in `main`; `ci#38026637514` ×3); not traced:
  the step inside Xlib; the witness-library / series contribution clause stays "Not measured"; P-129 … basis:
  `registries/contracts/test-plan/5-command-implementation.md:5`. → **apply** (routine; expected amendment 4c), with
  one correction at apply: the body says P-129's ref is written and its flip is the wrap's coverage step, never
  "met (`planned → implemented`)" as a standing status.
- **T5** · D-tests-coverage · warning · §1 Pending coverage triggers → `render-posture-main-placement-coverage`.
  change: a "Narrowed" clause: a committed source-order pin over `main()` now exists
  (`main_calls_it_second_after_the_render_posture_step_and_before_the_runtime`); still owed: `emit_posture` follows
  `observability::init` exactly once. basis: `pulse-app/tests/unit_xlib_threads.rs:70-80`. → **apply** (routine,
  "Accurate this-chunk addition": the pin is this chunk's). Read at this wrap, as the proposal asked: the pin
  compares the first non-comment line of `main()`'s body whole against the render-posture statement, the second
  against the call, and requires the runtime build after both, so the apply-is-first and before-the-runtime halves
  are pinned; the emit half is not.

## obs-plan

- **B1** · D-obs-defect-narrative · warning · §7 Error classes captured → Process-end cause. change: "The end is
  named, not closed; its owner is the route's closing entry." becomes the closed status (the call second in `main`,
  inert where not found, no log record; `ci#38026637514` ×3, 24 boots, 0 self-ended, against 12 of 16); the
  self-end sentence in the past tense; no owner pointer remains; the builds published before keep the defect on a
  host whose Xlib is older than 1.8, the founder's. basis: `pulse-app/src/main.rs:282`;
  `pulse-app/src/xlib_threads.rs:39-65`. → **apply** (routine; expected amendment 5a; disproved claim 4).
- **B2** · same paragraph. change: "Not measured: why the X connection's read failed, and the runner's own library
  build …" becomes the measured chain (no read failed; a second thread inside an Xlib without thread
  initialisation; one capture, the variation shows the cause; the library builds read from Ubuntu's packages); not
  traced: the step inside Xlib. basis: report, Spec claims disproved 1-2; `p3-measurements.md`. → **apply**
  (routine; disproved claims 1-2).
- **B3** · same paragraph. change: `errno` 11 is what the `end` line records and is a stale value, not a member of
  the ending call; the witness line's shape sentence stands. → **apply** (routine; disproved claim 3).
- **B4** · §10 Performance budgets → the WebGPU canvas frame row. change: widen "(a series boot whose settle verdict
  reads `ended` fails the job)" to both ways. basis: `xtask/src/harness_series.rs:285-294`, `378-389`. → **apply**
  (routine).
- **B5** · dependent · same row. change: add the both-steps-pass reading as measured (`ci#38026637514` ×3, the frame
  line present in each boot job log). → **apply** (routine; expected amendment 5b). The line's wording is quoted from
  `evidence/operator-pass.md`: `ci-gates: perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter
  (no_navigator_gpu)`, read in each of the three attempts.
- **B6** · dependent · §10 CI gates → the perf-budget bullet. change: cite the both-steps-pass reading with the series
  step in place beside the pre-series `ci#37979648967`. → **apply** (routine; as B5).

## Raised by the orchestrator

- **O1** · test-plan §4 Unit strategy ("Two unit binaries shell out of Rust …") and §9 Pipeline structure → the
  `lint-test` row (the workspace tests' host needs). The workspace tests need the host's `libX11.so.6` since this
  chunk: `unit_xlib_threads`'s found arm loads it and a host without it fails that pin, never skips it; no setup
  step; green in the `lint / test` job of `ci#38026637514` attempt 1. From the test-plan agent's stripped note; the
  fact was added to the report's Harness / gate surface bullet after the fan-out. → **apply** (check 5 raise,
  routine: "Accurate this-chunk addition").

## Checks 2 to 6

- **2 Cross-contradiction:** none. A1 to A3, T2, T3, B4 and A5 restate one rule in one direction; A7, B1, T4 state one
  closed status with the same readings.
- **3 Intent-consistency:** the report does not diverge from the entry or the plan's acceptance criteria; the four
  deviations carry the operator's word; the scope record holds no line.
- **4 Absence needs evidence:** no proposal claims an absence or a caught-all. The line profile of each master a hit
  is dispositioned in is read before its apply (`architecture.md:258` is 26 656 B, `security-plan.md:138` and `:395`
  about 10.6 KB each): those sites are edited by anchored replacement of the exact sentence, never from a clipped
  view.
- **5 Expected amendments:** all nine entries are matched (A1-A5, A6, A7, T1, T2, T4, B1-B3, B5-B6, S1-S5); O1 is
  raised beside them. No `claim false` row in `citation-dispositions.md`.
- **6 Disproved claims:** obs-plan §7's four → B1, B2, B3; the architecture and test-plan "measured limit" and
  `self-ended` wording → A1, A3, T2, T3 (and A5, B4); the plan's mutation forecast → a chunk artifact, reported in
  the report and `evidence/mutation-checks.md`, no edit. Each DISPOSED.
