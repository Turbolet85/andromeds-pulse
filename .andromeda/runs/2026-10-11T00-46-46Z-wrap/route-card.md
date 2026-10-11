# Route-resolve card — 2026-10-10-door-inside-the-engine-s-process

**Outcome (written after the operator's words, the pc overseer, 2026-10-11; the chunk's inputs#I8 and inputs#I9).**
Section 1: the removal entry `Stdio sidecar retired` minted as proposed, freight a to e. Section 2: none of the
three dispositions as leaned — option 3, an entry of its own, `Stdio agent reaches the door`, minted directly before
the removal entry and carrying P-092, its form a boundary widening its own P4 brings to the operator; its wording
was proposed at the stop before the flip and written on the operator's word there. Section 4: items 4 to 8, 10, 12
and 13 carried as leaned; item 9 corrected in place with a dated tag. Section 5: no split. The card below stands as
presented.

Presented to the operator at Phase 5 of this wrap, second window. Nothing below is written to the working route, the
capability record or any other file until the operator's word. Line numbers are `working-route.md`'s as it stands.

Read for this card: `route.py markerless` (70 markerless of 84 entries), `route.py epoch` (Epoch 1: entries 19,
markerless 5, frozen complete 13, pending 1), `route.py pins` (55 freight blocks, every one `CARRY:`; no `PREREQ:`,
no `BLOCKED-ON:`, no `WATCH:`), the frozen line 37 and the entries at 39, 98, 100, 128, 130, 132, 134 whole,
`docs/capability-record.json` (P-036, P-039, P-040 and every `removed_by` value).

## 1. The removal entry — minted directly after line 37, ahead of line 39 (the operator's direction, inputs#I5 item 4)

```
Stdio sidecar retired — sidecar, its double gate, the window's side of it and its CI job leave; its tests read through the door; two ways in until then (P-092)
```

Title and 25-word scope are a proposal. It carries P-092, whose text says the door "replaces the stdio sidecar". No
requirement is added and no ledger entry minted.

Freight proposed on it, one `CARRY:` each:

- **a — what the cut left** (record: this chunk's plan, Implementation notes; its report). The sidecar process: the
  `andromeda-pulse-mcp` binary and its `[[bin]]`, `feature_gate.rs`, `tracing_setup.rs`, the feature `mcp-server`,
  the `rmcp` dependency with its `deny.toml` entries, `ANDROMEDA_PULSE_MCP_ENABLED`, the crate's two subprocess tests
  and the four `pulse-app` tests that spawn the binary, rewritten to read through the door. What the engine does for
  it: `publish_workspace_key_for_sidecar` and `run/workspace-key`, whose only reader is the sidecar. The window's
  side: `mcp_router.rs` and its test, the five wiring sites of `main.rs`, the tray item, the three `mcp.*` procedure
  pins with the bindings, `Settings.mcp_server_enabled` and the `ready` check of the same name, the Settings switch,
  the "Send to agent" button and its hook, the a11y spec and helpers. CI: the `mcp-test` job, and the feature on the
  `boot` job's release build with its pin.
- **b — not this entry's:** `cargo xtask smoke:external-resolve`. Line 39, `Scenario legs driven against the console
  engine`, already owns moving its write to the door, in its own words ("the external write going through the door").
- **c — what the door still says in the sidecar's voice** (found at this chunk, report items 1 to 3): its
  `initialize` answer names the server `andromeda-pulse-mcp`; the tool descriptions speak of "the in-memory buffer"
  and of "the main app's in-memory registry … on next launch"; the tools' argument structs read past an undeclared
  member, so the door's own check is the only refusal — once the sidecar is gone one of the two forms is chosen
  (security-plan names this entry as owner of that gap; the operator's word, 2026-10-11).
- **d — the capability record:** P-039 names two webview proofs of the "Send to agent" button
  (`use-mcp-delivery.test.ts`, `SendToAgentButton.test.tsx`), which leave here; a removal that deletes a file a
  claimed entry names re-points that scenario in the same chunk. P-036 and P-039 also name
  `pulse-app/tests/e2e_p3_mcp_incident_tools.rs`, an in-process test of the dispatcher that stays valid while the
  tools crate stays.
- **e — masters:** test-plan §1's three sidecar pending rows (`workspace-key-cross-process-coverage`,
  `mcp-incident-read-back-cross-process-coverage`, `security-vector-coverage: MCP response body redaction`) and §6
  scenario P3's residual — what they hold once their subject is gone; the "two ways in" sentences this wrap wrote
  (architecture [MCP Server Surface], security-plan's door vector); and the "MCP server status change" notification
  design-system and layout-templates name, which was never built (a wording correction, not a removal).

## 2. The stdio gap (inputs#I5 item 5)

After the removal a same-host agent that speaks over stdio has no first-class way to the socket; this project ships
no stream relay, and whether a general-purpose one is installed on this host was not measured. Dispositions:

1. **A `CARRY:` on `Door reachable from another host` (line 100) — the lean.** That entry already states "where a
   party's credential lives on the developer's machine", which is the same question: how an agent on the developer's
   machine reaches the door. The gap stands from the removal until that entry, Epoch 4.
2. A `CARRY:` on the removal entry: it then must close the gap before the sidecar leaves, which needs a way in that
   does not exist yet.
3. An entry of its own, between the two.

A new command word of the engine (a bridge for stdio callers) is a boundary widening and is not chosen at a card.

## 3. The capability record's P-040 line, read against the sentence the plan stated (inputs#I5 item 6)

- The plan (Goal): "the engine starts, detects and reports with no reader at the door."
- The record, `changed_form`: "Claimed in a changed form: the engine starts, detects and reports with no reader at
  the door." The same sentence, word for word.
- `carried_by` reads P-084 and P-092; the one scenario is `nextest-file`
  `pulse-app/tests/integration_engine_boot.rs` with the anchor `an_incident_forms_with_the_door_listening_and_no_reader`;
  `provisional` is true and the note names the operator's answers of 2026-10-10 and that the founder's word is owed.
  `disposition`, `surfaces` and `removed_by` keep their form. `cargo xtask verify:capability-matrix` read 82 ids,
  36 claimed, 46 retired, 0 violations at implement.
- The record names ten route entries in `removed_by`; `Door inside the engine's process` is not one of them and is
  named only in P-040's prose. Minting the removal entry renames, retires and splits none of the ten, so the record
  needs no correction at this wrap. P-040 is not retired.

## 4. Found and not owned — dispositions, each with a lean (inputs#I5 item 8)

| # | What | Lean |
|---|---|---|
| 1–3 | the door's `initialize` name; the tool descriptions; the argument structs | the removal entry (freight c) |
| 4 | a failed tool call reads `tool failed` whatever failed: "no such incident" and a fault look the same | `CARRY:` on `One incident whole through the door` (line 130) |
| 5 | with no buffer connection every tool call answers `-32603`, the corpus tools included | `CARRY:` on `Door admission lifecycle and bounds` (line 98) |
| 6 | the socket file outlives the engine and is replaced at the next boot | `CARRY:` on `One place on a node` (line 41), beside the lock file |
| 7 | that the interrupt ends a real running query is unshown (test-plan §1 `door-timeout-interrupt-real-query-coverage`) | `CARRY:` on line 98 — the plan and the pending row already name it |
| 8 | the window app reads `ANDROMEDA_PULSE_DOOR_RESOLVE` and uses nothing of it | `CARRY:` on `Window retired` (line 52) |
| 9 | `rules/security.md` Session Additions 2026-05-12 says `mcp-server` is "the only optional crate"; it is no longer optional | correct that one clause in place with a dated `[corrected 2026-10-11: …]` tag — a curation correction on this chunk's measurement; the alternative is to carry it uncorrected |
| 10 | on a system that is not Unix no door is seated | `CARRY:` on `Other operating systems retired from the code` (line 64) |
| 11 | the stdio gap | section 2 |
| 12 | NEW at this wrap: the idle and time-bound arms of `unit_door.rs` wait on the real clock (about 30 s and 5 s a run; no `start_paused`, no `time::pause`), where test-plan §11 says never real time without injection | `CARRY:` on line 98, which owns the bounds: it moves them to injected time or says why a real socket cannot |
| 13 | NEW: what line 98 still holds. This chunk built three of its four clauses (store-only reads, answers bounded in size and time, the resolve write named and switchable) | leave the line's words; a `CARRY:` saying what stands built and what remains: admission and revocation by command, and items 5, 7, 12 |

## 5. Epoch 1's size

Epoch 1 holds 19 entries and will hold 20 with the removal entry (6 markerless). Lean: no split, as at the last
card; the epoch closes at `Detection baseline through the console engine`.

## 6. Nothing to pin, read and found empty

- No gate deferral was taken, so no `PREREQ:`. No `WATCH:` stands. No `BLOCKED-ON:` on this chunk's line or on any
  markerless entry; no `gated` record.
- The two `CARRY:` blocks on this chunk's frozen line are consumed: P-126 is proven on the runner, and the P-040
  either-or is settled by the stated sentence. The frozen line is not touched here; the flip's compaction strips it.
- The citation sweep left no row in the route.
- No route entry is renamed, retired or reordered.
