# Route-resolve card — 2026-10-10-capability-record-re-based

Prepared at P5 and held for the operator's word (the wrap directive, inputs#I11 item 8). Nothing below is written to
`andromeda-pulse-0.4.0/working-route.md` yet. Every edit is an annotation appended to a markerless entry; no title, no
order and no frozen line moves, so the capability gate's reading of the route is unchanged by them.

## State read

- `route.py cursor`: records 92 · complete 91 · pending 1 (this chunk) · gated 0. Next: `working-route.md:29`,
  `Console engine entry point`.
- `route.py pins`: 26 freight blocks on the markerless tail, none on this chunk's frozen line (`:27`); nothing to
  migrate. No gate deferral, no watch, no block on this chunk.
- `route.py epoch`: Epoch 1 holds 18 entries (markerless 9 · frozen complete 8 · pending 1).

## A. The pins the plan owes (inputs#I11 item 4), each as measured

1. **`Window's gates retired`** (`:48`) —
   `CARRY: the capability record retires 21 ids that no gate fully guards until their surface leaves (docs/capability-record.json, guard; record: 2026-10-10-capability-record-re-based's report) — 6 with no running proof (P-025, P-026, P-064, P-065, P-066, P-068) and 15 with a part of one (P-020, P-024, P-028, P-029, P-054, P-060, P-063, P-069, P-070, P-071, P-076, P-078, P-080, P-081, P-082): no CI step, xtask verb or pre-push stage runs the webview unit tests, the headful verbs self-verify and webview-drive, the latency script or the real-model leg (measured at that chunk's P3)`
2. **`Display-only computation retired`** (`:56`) —
   `CARRY: the capability record names this entry as owner of the kept-half code of three ids retired with the window (kept_half_owner; record: 2026-10-10-capability-record-re-based's report) — P-002's last-span tracker and P-003's receiver-failure state in crates/ingest/src/connection.rs, and P-058's diagnostics snapshot; no 0.4.0 requirement text carries them (the operator's per-id reading, 2026-10-10)`
3. **`Incident is the engine's own record`** (`:65`) —
   `CARRY: the capability record names this entry as owner of P-023's acknowledge cool-down code in crates/triage/src/incident/registry.rs (kept_half_owner; record: 2026-10-10-capability-record-re-based's report) — its only trigger was the window's IPC route incidents.acknowledge, and the door keeps one write, resolving (P-093)`
4. **`Records say what the product is`** (`:79`) — the owner of the two old records and the 0.3.0 ledger's 22 ids
   (inputs#I11 item 5), and of the false `purpose` line —
   `CARRY: three records are superseded by docs/capability-record.json since 2026-10-10-capability-record-re-based and stand byte-identical until this entry marks them history — docs/v0_2_0/pulse-capability-spec.md, docs/v0_2_0/capability-verification-matrix.json (its purpose line still says cargo xtask verify:capability-matrix validates it, false since that chunk) and andromeda-pulse-0.3.0/verification-matrix.json for P-061 to P-082; Conductor holds its own copies of the first two (.andromeda/refs/, read at 321dc8f2), and architecture §Standard Contracts says they stand until the route retires or re-homes them`

## B. P-117's third clause (inputs#I11 item 2)

`Version close on Linux` (`:184`) carries the leg: its line reads "capability record and external harness's accepted
set agree (P-117)". It does not say what is read against what. Proposed, so the clause cannot be forgotten at the
close —
`CARRY: P-117's third clause is read here — Conductor's accepted set (contracts/pulse-capabilities.toml, after its entry Accepted capability set re-based) against the ids whose disposition is claimed in docs/capability-record.json (36 at 2026-10-10-capability-record-re-based, where the first two clauses were shown and P-117 stayed unclaimed)`

## C. P-040 (inputs#I11 item 3) — for the operator's word

The record's P-040 line was rewritten at this wrap. It now quotes the one requirement sentence that carries the
changed form as claimed (independence from a model, P-084: "the engine starts, detects and reports with no model
present and none configured" and, in the same sentence, "a large model reads through the door (P-092) and is not part
of the engine"), and says in so many words that no requirement sentence says the engine works with no reader at the
door, that this is not claimed, and that the entry `Door inside the engine's process` would have to state it.
`carried_by` is `["P-084"]`; P-099, named beside it at P4, is dropped because no sentence of it carries the form. It
stays `provisional`.

Two things here are a reading, not a measurement: that P-084's sentence is "the one", and that the door's entry is the
one that would have to state the rest. If the entry stands, proposed on `Door inside the engine's process` (`:35`) —
`CARRY: the capability record's P-040 line (PROVISIONAL, the founder's) names this entry as the one that would have to state that the engine starts, detects and reports with no reader at the door; no requirement sentence says it (record: 2026-10-10-capability-record-re-based's report)`

## D. Questions with a lean

1. **Three of the fifteen are model ids** (P-020, P-054, P-060): their unrun proof is the real-model leg and the
   latency script, which leave with `Local model retired` and `Cadence coordinator retired`, not with the window's
   gates. Lean: keep the one carry on `Window's gates retired`, as answered at P4 and as the plan says.
2. **The new test-plan pending trigger** `verify-capability-matrix-verb-glue-coverage` (the verb's event line, twin
   and non-clean exits have no pin). Lean: no route pin; it stands in test-plan §1 like its siblings.
3. **Epoch 1 has grown to 18 entries** via insertions. A boundary would restore the diagnose and audit cadence.
   Your word at the last wrap was no split. Lean: no split.

## E. Noted, no edit proposed

- The masters still describe as live what the record retires: design-system's glow-layer wording (its residual
  pointer was stale before this chunk), layout-templates' provenance ids, security-plan's corpus encryption and
  training export, obs-plan's capability-named observables. Each rewrite belongs to the entry that removes the
  surface (`Window retired` already says the design, layout and a11y masters state no interface).
- The gate now reads this file. A later route-resolve that renames, retires or splits one of the ten entries the
  record names corrects the record in the same wrap, or the verb and its pin read red.
- No requirement was added, no residual's premise was measured, no gated record exists.

## The operator's word, and what was written

The operator, 2026-10-10, given in the wrap session at this card, verbatim: "card read. 1 - yes, the five pins in A
and B as worded. 2 - yes: the P-040 line stands as you rewrote it, and the pin in C goes in; its words say that entry
plan states the missing sentence or the id is retired there, nothing in between. 3 - one rule: an id is carried on the
entry that removes its surface, so P-020, P-054 and P-060 are named on the entry that retires the model, and the carry
on the window gates entry names the rest; say the two counts. 4 - no pin. 5 - no split. Then go on and stop before the
flip and the commit with git status --short."

Written to `andromeda-pulse-0.4.0/working-route.md`, seven `CARRY:` blocks appended to seven markerless entries (the
tail's freight blocks 26 to 33, none `INDETERMINATE`; `cargo xtask verify:capability-matrix` reads clean over the
route after them):

- `:35` `Door inside the engine's process` — C, with the word's either-or: the entry's plan states the missing
  sentence, or P-040 is retired in the record at that entry, nothing in between.
- `:48` `Window's gates retired` — A1 as changed by answer 3: 18 ids carried there (6 with no running proof, 12 with
  a part), and it says 3 are on `Local model retired`.
- `:56` `Display-only computation retired` — A2 as worded.
- `:65` `Incident is the engine's own record` — A3 as worded.
- `:67` `Local model retired` — new by answer 3: P-020, P-054 and P-060, 3 there and 18 on `Window's gates retired`.
  The record's removing entry for P-060 is `Cadence coordinator retired`; its unrun proof, the latency script, is
  named on the model's entry as the word directs, and the carry says so.
- `:79` `Records say what the product is` — A4 as worded.
- `:184` `Version close on Linux` — B as worded.

Not written: no pin for the test-plan pending trigger (answer 4); no epoch split (answer 5).
