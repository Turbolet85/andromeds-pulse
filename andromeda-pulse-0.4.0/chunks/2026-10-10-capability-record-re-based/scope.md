# Scope — Capability record re-based

**Marker:** `2026-10-10-capability-record-re-based` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:27`):** "Capability record re-based — one current record for all 82 ids, each
claimed or retired with its surface; the old gate reads it (P-117)"
**Chunk base:** `279a477e` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
82 capabilities stand as claimed in two records that describe the product of 0.2 and 0.3: P-001…P-060 in
`docs/v0_2_0/pulse-capability-spec.md` with a gate over `docs/v0_2_0/capability-verification-matrix.json`, and
P-061…P-082 in `andromeda-pulse-0.3.0/verification-matrix.json`. Many of them are about the window, the local model
or the desktop distribution, which this version removes (intent R11). This chunk writes ONE current record that
gives every one of the 82 ids exactly one of two dispositions — still claimed by the engine as it will stand, or
retired with a named surface — and makes the old gate read that record, so that it carries no retired id. The
record is also what another project reads: the external harness (Conductor) derives its accepted set from it.

## What the chunk builds
- **One current record for all 82 ids.** One file, P-001…P-082, each id present exactly once.
- **Two dispositions and no third** (the entry: "each claimed or retired with its surface"; inputs#I1 item 4).
  An id the engine keeps in a changed form is claimed, and the record names the 0.4.0 requirement that carries
  the changed form (P-083…P-129); "kept for now" is not a disposition. Closed at P3: every claimed id but one
  has such a requirement to name (research, "Claimed, the reading not in doubt"); P-077 has none and says so.
- **A retired id names its surface** — the window, the model or the desktop, as P-117's requirement lists them.
  `[premise-corrected: read at P3 over all 82 texts and the route]` The three words do not cover the record:
  (a) none of the 82 is about the desktop's distribution, so that surface has no member; (b) the tray, the updater
  and the notification plugins leave with `Window retired` (`working-route.md:50`), so a tray id is a window id
  here; (c) the cadence and the digest are the model's surface (P-084: "with everything that served it");
  (d) four ids are retired by a ruled removal that is none of the three — P-043 and the key half of P-079 (the
  workspace, P-105), P-046 (the training export, P-106), P-049 (corpus encryption, P-085). Which surface word
  those four take is a fork, brought at P4.
- **A retired id names the route entry that removes its surface** (inputs#I1 item 3), by the entry's title. Closed
  at P3: an entry of Epochs 2 and 3 exists for every surface named (`Window retired`, `Window's gates retired`,
  `Display-only computation retired`, `Local model retired`, `Incident is the engine's own record`,
  `Digest retired`, `Cadence coordinator retired`, `Workspace detection retired`, `Training export retired`,
  `Corpus encryption at rest retired`). The plan says, per retired id, what guards the capability between this
  chunk and that entry. `[premise-corrected: measured at P3]` "Its tests still run" is false for a class of
  them: no workflow step, xtask verb or pre-push stage runs the webview unit tests (`vitest`), and none runs the
  headful or dev-host graders. Seven retired ids have no proof that runs today and fourteen have a part of one
  (research, as corrected at P4). They went to the operator at P4.
- **The record's form is stated for a reader outside this project** (inputs#I1 item 2): the plan names the one
  file and which fields say "claimed" or "retired", and with what. Closed at P3 against the reader: Conductor's
  accepted set is one flat list of ids in `contracts/pulse-capabilities.toml` (inputs#I5), read by list membership
  (inputs#I6), so a per-id disposition field gives its entry the set with no second derivation.
- **The old gate reads the record.** `cargo xtask verify:capability-matrix` (`xtask/src/main.rs:774`, the CI
  `lint-test` step at `ci.yml:105-106`) passes while carrying no retired id. Verified at P3: the gate demands
  exactly P-001…P-060 present once each in `docs/v0_2_0/capability-verification-matrix.json`
  (`main.rs:917-923`), prints `{n}/60 capabilities` (`:958`), and on the untouched tree reads
  `clean (60/60 capabilities, 0 violation(s))`, exit 0. `[premise-corrected: test-plan §9 and the entry's own
  words]` Its reading of P-061…P-082 is not open: the record is one file for all 82, the gate reads that file,
  and test-plan §9's standing rule is that ids from P-061 on extend the file the gate validates. So the gate
  holds all 82 ids to one disposition each and validates the scenarios of the claimed ones.
  `[premise-corrected: read at P3]` The gate has no arm for an absent or unreadable input (it exits 1 through a
  generic error) and no test of its own; the masters' pattern for a gate over a committed file is exit 0 pass ·
  1 findings · 2 cannot-evaluate, each arm pinned.
- **The classification itself.** `requirements.md:84-103` holds the route's reading of the 22 ids P-061…P-082,
  marked there as "the founder's to correct" and as not this record; no reading of P-001…P-060 existed. Read at
  P3 over all 82 texts, the tree and the route: 24 claimed and 39 retired with the reading not in doubt, 19 in
  doubt (research, "The 82 ids"). The doubtful ids are brought at P4 as ONE dialog, grouped, each group with what
  claiming and what retiring would mean for the version; an id whose fate turns on what the product is, not on
  where its code lives, is said so in its option — that one is the founder's (inputs#I1 item 5). The technical
  fork is the operator's where it is not obvious.
- **The two old records after this chunk** stay where they are, byte-identical. Closed at P3: the working entry
  `Records say what the product is` (`working-route.md:79`, P-116) owns marking the documents of 0.2–0.3 as
  history, and Conductor holds its own copies of both (inputs#I3, inputs#I4). The new record names the two as
  what it supersedes. The old gate record's `purpose` line, which says the gate validates it, is false from this
  chunk on; that sentence is P-116's entry's to mark.

## Decided at P4 (the operator's answers, the pc overseer, 2026-10-10 — inputs#I8)
- **The real surface is named.** The record's surface words are a closed set: window · model · desktop ·
  workspace · training-export · corpus-encryption. The desktop word stands because P-117 names it and has no
  member. P-043 is retired with the workspace, P-046 with the training export, P-049 with corpus encryption, P-079
  with the window and the workspace; each names the entry that removes its code.
- **A split id is claimed only where the text of a 0.4.0 requirement carries its kept half**, read word by word;
  an id no text carries is retired with the window, and the code of its kept half gets its owner entry named in
  the record, never left to P-121. Read so: claimed are P-001 and P-004 (P-098; P-004 with P-091), P-027 and
  P-067 (P-109), P-042 (P-112, P-105), P-036 and P-044 (P-110, P-092), P-075 (P-102). Retired are P-002 (no text
  carries a last-span tracker), P-003 (no text carries the receiver's own failure) and P-058 (no text carries the
  pipeline's operational metrics; P-122 carries two other figures); their kept-half code is owned by
  `Display-only computation retired`. The same rule gives P-023's cool-down code its owner,
  `Incident is the engine's own record`.
- **The four ids that turn on what the product is are claimed in a changed form — PROVISIONAL, the founder's own
  word owed** (the operator brings them to him). Each record line says plainly that the original sentence no
  longer holds as written and quotes the form it is claimed in: P-030 (P-099, P-102), P-040 (P-084, P-099),
  P-048 (P-088, P-093, P-085 — scrubbing before storing), P-050 (P-106, P-093). An id whose changed form no
  requirement text carries is said so at P5, never stretched.
- **The ids with no running proof are recorded as such, and the wrap pins a `CARRY:`** on `Window's gates retired`
  naming them.
- **The count that follows:** 36 claimed, 46 retired, 82 in all.

## The other side — the external harness
- Verified at P3: "measured today in Conductor at `321dc8f` (`build/conductor-0.4.0`): it holds its own COPIES of
  the two old records (`.andromeda/refs/capability-verification-matrix.json`,
  `.andromeda/refs/pulse-capability-spec.md`), and its route has the answering entry at
  `conductor-0.4.0/working-route.md:56`" (inputs#I1 item 1). Conductor's HEAD is `321dc8f` on
  `build/conductor-0.4.0`, pushed; line 56 reads "Accepted capability set re-based — after Pulse's Capability
  record re-based: accepted set, classification, coverage record, references re-derived; no retired capability
  named; old pin stands meanwhile (v4-06)" (inputs#I2); both copies are committed there (inputs#I3, inputs#I4).
  The spec copy is byte-equal to this tree's. `[premise-corrected: diffed at P3]` The matrix copy differs from
  this tree's in three `notes` strings and in nothing else; every id, title, category, mode and scenario is equal.
- Verified at P3: nothing this chunk writes breaks Conductor today. Its pin is its own file (inputs#I5) and names
  all 82; its requirement v4-06 says its side stays unbuilt until this record is written (inputs#I7). P-117's
  third clause — "the external harness's accepted set equals that record" — cannot be read until Conductor's
  entry runs (inputs#I1 item 1).

## Capability
- **P-117** (`verification-matrix.json#P-117`, `planned`, unclaimed). Acceptance: "One current record lists all 82
  capabilities, each either still claimed by the engine or retired with the window, the model or the desktop; the
  old gate passes while carrying no retired id; the external harness's accepted set equals that record."
  Verified at P3: this chunk can show the first two clauses. The third waits on Conductor's entry, and the working
  entry `Version close on Linux` (`working-route.md:184`) also names P-117 ("capability record and external
  harness's accepted set agree"). So the capability is advanced here and not claimed; P5 says what of P-117 this
  chunk shows and what waits for Conductor, and claims nothing that was not shown (inputs#I1 items 1 and 7).

## Boundaries
- No surface is removed here. The window, the model, the desktop distribution, their code and their tests stand
  after this chunk exactly as before it; the entries of Epochs 2 and 3 remove them.
- No capability P-083…P-129 is re-worded, and the 0.4.0 ledger `andromeda-pulse-0.4.0/verification-matrix.json`
  is written only through its tool at P5.
- No file of Conductor's is written. Its accepted set, classification, coverage record and references are its own
  entry's work.
- The spec masters are read-only at phase; what this chunk changes in them is the wrap's.

## Process items carried by the directive (inputs#I1 items 6 and 7)
- The plan lists the merge-base probe directly before its push entry.
- At P5: what `planlint` check 4 listed, P-117's concretization against its acceptance, the plan card, and a stop
  for the operator's `yes`.

## Folded freight
- none — the entry carries no freight block (`route.py pins`: no row for `working-route.md:27`).

## Second fold source — the CI verdict read at Setup 5a
- One sha since the last wrap's flip: `279a477e` (the wrap's own commit; the flip is in it).
  `ci#38046702090` on it read **in progress** at take-up (checks 7/7 listed, 6 running, the oldest the coverage
  gate at 191 s) and again twice at P3 (2 running at the last read); `secret-scan#38046702082` completed, success.
  At take-up and at P3 the verdict was not yet available, and it was not folded or read as green then.
  **Read once more at P5, after the run closed: `verdict: green`, checks 7/7, wall 1699 s** (`ci#38046702090`
  completed, success; attempt 1). No red and no `not green` is carried into this chunk.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py blocked`: 0 blocks on a pending, gated or markerless line).
