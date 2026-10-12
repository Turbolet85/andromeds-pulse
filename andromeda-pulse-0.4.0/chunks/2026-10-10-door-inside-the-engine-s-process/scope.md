# Scope — Door inside the engine's process

**Marker:** `2026-10-10-door-inside-the-engine-s-process` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:37`):** "Door inside the engine's process — tools read the engine's stores, owner
only, own host, call records allowlisted; stdio sidecar, its double gate leave (P-092, P-093)"
**Chunk base:** `303fa9a8` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
Today the only way anything outside the engine's process reads a finding is the stdio sidecar: a second process
whose four telemetry tools read their own empty in-memory database, and which is built on what this version removes
(the credential store, the resolver, the incident it parses, the model's renderer, the workspace key) (intent F6,
OBSERVED). The intent rules that the door lives inside the engine's process and replaces the sidecar as soon as the
console engine exists, ahead of those removals, so that the later chunks change the door, which stays, and never the
sidecar, which goes (F6, EXPECT; §6 Order, ruled). The console engine exists since
`2026-10-10-console-engine-entry-point`. This chunk builds the door's first form and removes the sidecar.

## What the chunk builds (the entry's own words)
- **A door inside the engine's process.** The engine itself answers calls from outside its process; no second
  process stands between a caller and the engine's stores.
- **Tools read the engine's stores.** What a tool returns comes from the stores the running engine holds, not from
  a database of the door's own.
- **Owner only, own host.** Until another host can reach the door it answers on the engine's own host, and only to
  the engine's owner (the first `CARRY:` below gives the owner half to P-126).
- **Call records allowlisted.** Each call leaves a record in the engine's own log, and that record's name is on the
  log allowlist.
- **The stdio sidecar and its double gate leave.** The compile-time feature and the runtime variable that gate the
  sidecar go with it.

## What the directive adds (inputs#I1 — each bullet closed at P3; the four open forks go to the P4 dialog)
- **The boundary questions come first, as ONE dialog at P4, before any design is settled** (inputs#I1 item 1): what
  the door listens on (a socket of which kind, where, with what mode and owner); how "owner only" and "own host" are
  enforced and not merely configured (P-126: another account on the same host is refused); what a call may reach.
  Each option says what it opens that is closed today. The operator's answer is PROVISIONAL; the founder's own word
  is owed on the surface itself, and the plan says so. Verified at P3 as the path the plan takes; what each question
  rests on was measured (research, Patterns detected): the masters hold no door surface and no way to tell one
  local account from another; `run/` under the data dir is mode 755 today; a socket address holds 108 bytes and the
  longest measured data dir leaves 3 to spare; the peer's account is readable on a Unix-domain connection with no
  new dependency; a second account can be driven on the hosted runner and is unmeasured on this host.
- **What the requirement fixes, not re-decided here** (inputs#I1 item 2): the door is inside the engine's process;
  own host only until another host can reach it; it reads the engine's stores and nothing else on the node (not the
  token, not configuration secrets, not files outside those stores); every answer is bounded in size and in time;
  the one write it keeps, resolving an incident, is named as a write and can be switched off. Admission, its
  lifecycle and encryption in transit begin when another host can reach the door, and this chunk builds none of
  them. Verified at P3 against the intent's F6b and P-093's requirement line. The later entries are `Door admission
  lifecycle and bounds` (`working-route.md:98`: a party admitted and revoked by command) and `Door reachable from
  another host` (`working-route.md:100`: admitted parties only, encrypted in transit, where a party's credential
  lives). One overlap was read: `:98`'s own words also name "store-only reads; answers bounded in size and time;
  resolve write named, switchable". This chunk builds those for the own-host form, as the directive says; what
  `:98` then still holds is the operator's to say at a wrap.
- **The door's form is a contract with another project** (inputs#I1 item 3): "Conductor reads the engine through
  this door from its first check and never through the sidecar (its intent, measured in this pair's record)" — the
  relay's own reading, closed against the snapshot alone; Conductor's intent was not read here. In this repository
  the sidecar's own comment names Conductor as a caller that spawns it
  (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:72`), and architecture's Standard Contracts hold no entry for a
  door. The plan states the one form an outside reader depends on: how it connects, what a call and an answer look
  like, what an error looks like. A form only this repository's tests can use is half the entry.
- **Everything that stands on the sidecar today is listed and dispositioned** (inputs#I1 item 4). Verified and
  enumerated at P3 (research, "What stands on the sidecar today"): 75 tracked code files name it, in five groups:
  the sidecar process; what the engine does for it; the window's side; CI and one harness leg; the capability
  record. The plan gives each a disposition: removed here, or which entry removes it and why it cannot go now.
- **Size** (inputs#I1 item 6) `[premise-corrected: measured at P3 (research, Size)]`: the entry is two pieces of
  unequal size. The door alone is 4 new files and 12 modified (research's two lists). The sidecar's removal reaches about 60
  more files across the crate, `pulse-app`, three other crates, `xtask`, the workflow and its pins, 20 webview
  files and the a11y suite, and brings the webview gates, the a11y job and the procedure-set close into the gate
  block. The removal's rewritten tests read through the door, so the door answers first. P4 proposes the cut; the
  route is the operator's to change at a wrap.
- **Which tools the door answers at this chunk** `[premise-corrected: read at P3 in crates/mcp-server/src/tools.rs]`:
  the sidecar answers nine tools through one dispatcher whose inputs are a buffer connection, a query state and the
  corpus with a workspace key, all of which `engine_boot::start` builds. Handed the engine's own, it answers the
  same nine from the stores the running engine holds; nothing in it is tied to stdio. The widened forms stay with
  their entries: filters by service, trace and text with aggregates (`Door telemetry queries`,
  `working-route.md:128`), one incident whole (`:130`), the curated snapshot as that code's one consumer (`:132`),
  the engine's own state (`:134`). Which of the nine the door answers now is the dialog's third question.
- **What a reader outside the process uses today besides the sidecar** `[premise-corrected: read at P3 in xtask/src]`:
  the harness verbs and the engine-cycle checks read the engine's log files and its pid and exit records; none of
  them calls the sidecar. One leg does: `cargo xtask smoke:external-resolve` spawns the sidecar to make its resolve
  write (dev host only, in no workflow), and `Scenario legs driven against the console engine`
  (`working-route.md:39`) owns moving that write to the door. No harness verb moves onto the door in this chunk: the
  readiness and status verdicts are member-pinned contracts, and their move belongs to `Engine end-to-end gate
  reachable` (`:45`) and `Detection baseline through the console engine` (`:47`).

## Decided at P4 (the operator's answers, the pc overseer, 2026-10-10 — inputs#I3; each PROVISIONAL, the founder's own word owed on the surface)
- **The cut.** This chunk builds the door and does not remove the sidecar. The entry's last clause, "stdio sidecar,
  its double gate leave", moves to a removal entry the operator mints at this chunk's wrap, directly after this one;
  its rewritten tests read through the door built here. For that one interval the engine has two ways in besides
  telemetry, the door and the sidecar, and the plan states what forbids a third.
- **The endpoint.** A Unix-domain stream socket under the data dir's `run/`, in an owner-only directory, file mode
  0600. No port, no place outside the data dir, no variable to find it. Calls are the sidecar's own JSON-RPC 2.0
  lines, one per line. The plan says how an agent that speaks over stdio reaches a socket today and which entry
  owns giving it a first-class way. No command word of the engine is added here.
- **Owner only.** Three layers: the owner-only directory, the socket's mode, and the peer's account read from the
  kernel on every accepted connection and compared with the engine's own; any other account, root included, is
  refused. A real leg is built: a committed arm connects as a second account on the hosted runner. The workflow
  step that runs a client as a second account is part of what the founder's word is owed on, and it runs nothing
  but that client.
- **Reach.** The nine tools the sidecar answers, read from the engine's own stores. The resolve write is on by
  default, the requirement's own reading (named as a write, can be switched off), and is switched off by one
  variable; both arms of the switch are pinned, and the call record of a write says it was a write.

## Capabilities (`verification-matrix.json`, all three `planned`, unclaimed)
- **P-092.** Acceptance: "From the first step in which a console engine exists, every reading from outside its
  process goes through a door inside that process: the engine's own end-to-end gate and the external harness at each
  theme read through it, no stdio sidecar exists, and no later removal has a sidecar to change; an agent on the
  developer's machine reads through it the telemetry the engine holds, filtered by service, time range, trace and
  text, with aggregates, one incident whole with its evidence and the incidents like it from the past, and the
  curated snapshot of recent telemetry; the test harness and the status command use the same door." Seven route
  entries name it: this one and `working-route.md:39`, `:100`, `:128`, `:130`, `:132`, `:167`. So this chunk
  advances it and does not claim it.
- **P-093.** Acceptance: "Before the engine accepts connections from another host the door answers on the engine's
  own host only; once it can be reached from another host it answers only an admitted party, a party is admitted and
  revoked in a stated way and a revoked party is refused; where an admitted party's credential lives on the
  developer's machine is stated, outside any repository and any log; the door's traffic is encrypted in transit; a
  request for the token, a configuration secret or a file outside the stores is refused; an answer too large or too
  slow is cut at a stated bound; what it returns was scrubbed at ingest; resolving an incident is named as a write
  and can be switched off." `:98` and `:100` name it too; the admission and encryption clauses are theirs. PROVISIONAL
  in the intent (F6b, the overseer's) until the founder's own word.
- **P-126.** Acceptance: "Before the door can be reached from another host, a call by the engine's owner on the
  engine's host is answered and a call by another account on the same host is refused." This entry alone names it.
  Its ledger note records it PROVISIONAL (it rests on P-093 and P-118).
- At P5 the plan shows what of each acceptance this chunk shows, and claims nothing that was not shown (inputs#I1
  item 7).

## Folded freight (two blocks on `working-route.md:37`; `route.py pins` lists two rows for the line)
- **CARRY 1** (whole): "this entry's owner-only is P-126's — until another host can reach the door, another account
  on the engine's host is refused". Folded as the P-126 bullet above and the boundary dialog's enforcement question.
- **CARRY 2** (whole): "the capability record's P-040 line (PROVISIONAL, the founder's) names this entry for what no
  requirement sentence says, that the engine starts, detects and reports with no reader at the door — this entry's
  plan states that sentence, or P-040 is retired in the record at this entry, nothing in between (the operator's
  word, 2026-10-10; record: 2026-10-10-capability-record-re-based's report)". Re-verified against the record:
  `docs/capability-record.json` holds P-040 as `claimed`, `carried_by` P-084, `provisional: true`, and its
  `changed_form` says "No requirement sentence says in so many words that the engine works with no reader at the
  door; that is not claimed, and the working-route entry \"Door inside the engine's process\" would have to state
  it." The directive repeats the either-or (inputs#I1 item 5): the plan states, as a requirement-level sentence with
  a check, that the engine starts, detects and reports with no reader at the door, or says the record retires P-040
  at this chunk's wrap. Nothing in between. P-040 is an id of the capability record, not of this version's matrix
  (`matrix.py show --id P-040`: no such id), so whichever arm is taken changes the record and its gate
  (`cargo xtask verify:capability-matrix`), not a matrix claim.

## Boundaries
- This entry adds the engine's first way in besides telemetry. A listening socket other than the two OTLP
  receivers, a new place on disk, a new credential, or anything reachable from another host is a boundary widening:
  it goes to the operator in the P4 dialog, the answer is PROVISIONAL, and the founder's own word is owed on the
  surface (inputs#I1 item 1).
- Nothing this chunk builds is reachable from another host, and no admission, admission lifecycle or encryption in
  transit is built here (inputs#I1 item 2).
- Test and harness legs bind claimed ports, never 4317 or 4318 (inputs#I1 item 7). Ports 4317 and 4318 are shared on
  this host: the operator is asked before any run that binds them.
- The record's form is a contract with Conductor (handoff; architecture §Standard Contracts): a change to the form
  of `disposition`, `surfaces` or `removed_by` is said to the operator before it lands. A removal that deletes a
  test file a claimed record entry names re-points that scenario in the same chunk.
- The spec masters are read-only at phase; what this chunk changes in them is the wrap's.

## Process items carried by the directives
- The plan lists the merge-base probe directly before its push entry (inputs#I1 item 7).
- At P5: what `planlint` check 4 listed, what of P-092, P-093 and P-126 this chunk shows, the plan card, and a stop
  for the operator's `yes` (inputs#I1 item 8; inputs#I2).

## Second fold source — the CI verdict read at Setup 5a
- One sha since the last wrap's flip: `303fa9a8` (the wrap's own commit; the flip is in it). `ci#38092921384` on it
  read **in progress** at take-up (checks 7/7 listed, 6 running, the oldest the boot smoke at 192 s);
  `secret-scan#38092921387` completed, success. The verdict was not yet available at take-up or at P4 (1 running,
  the coverage gate at 1164 s), and it was not folded or read as green then.
  **Read again at P5, after the run closed: `verdict: green`, checks 7/7, wall 1451 s** (`ci#38092921384`
  completed, success). No red and no `not green` is carried into this chunk.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py cursor`: no annotation-position line; `route.py pins`: two
  `CARRY:` rows for line 37 and no other).
