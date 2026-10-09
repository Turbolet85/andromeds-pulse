# Adaptation record — the 2026-10-09T09-59-54Z 0-pending wrap

An operator-requested route adaptation that tunes the andromeda-pulse 0.4.0 route written the same day. Master at
Setup: 83 records, 83 complete, 0 pending, 0 gated; the tree carried only bookkeeping (the friction log, the
handoff's session-end stamp). HEAD `39edd11` equalled its upstream `origin/build/andromeda-pulse-0.4.0`. The cards
were shown to the operator before anything was written to the route, the requirements or the ledger; his four
answers are quoted below. The wrap stopped before its commit at his word, for his read of the tree.

## Whose word

- **The founder's own, relayed verbatim** by the pc overseer in
  `pc-overseer/relays/pulse-wrap-0pending-adapt-0.4.0-2026-10-09.md` §2 (outside this repo), by dialog, 2026-10-09:
  - the route approved as it stands, to be tuned after: «Мы можем попробовать оставить как есть и в догонку 0 пендиг
    врапом поднастроить что нам надо» (also in `.andromeda/runs/2026-10-09T09-17-27-route/review-feedback-1.md`);
  - what else leaves in 0.4.0 — the question: «Что ещё убираем из Pulse в 0.4.0? Можно выбрать несколько; что не
    выбрано — остаётся.»; his picks, all three: «Хост плагинов на WASM (Рекомендую убрать)», «Определение рабочей
    папки (Рекомендую убрать)», «Выгрузка для дообучения (Рекомендую убрать)»;
  - raw telemetry's term — his pick: «7 дней (Рекомендую)», whose option text says the term is configurable and a
    ceiling on the file's size stands beside it, at which the oldest goes first.
- **The operator's, given at this wrap's card, 2026-10-09**, verbatim:
  1. "Confirmed for 0.4.0: one engine watches one product, incidents live in its one store told apart by cue
     identity, nothing replaces the workspace key; add that a sender token is not an incident key in this version.
     Operator word."
  2. "Three entries, your order."
  3. "P-107 as a capability with its entry, as you wrote it; operator word, PROVISIONAL like the rest of C; your one
     note on P-088 is what I meant."
  4. "Owner of the red main: a CARRY on the Foundation entry that already edits the workflow, Windows and macOS CI
     legs retired; pre-push check native on Linux, so the repair lands in Epoch 1; if a letter makes the
     supply-chain entry the only lawful owner use yours and say why."
- **PROVISIONAL** (`gate-contract.md` §Scope, *Whose word*): the five controls of the relay's item C are the
  operator's own, no finding of the intent states them, and the fork is the founder's. They are recorded on the
  operator's word and stand until the founder's own word, which supersedes them by rule. The pc overseer brings
  them to him in a batch. They add controls; none widens what crosses a hardened boundary, so the playbook's
  boundary-widening rule did not fire and nothing halted on them.

## What the relay stated, re-derived at HEAD `39edd11` before it entered a record

| claim | reading at this wrap |
|---|---|
| the route holds 48 markerless entries in 8 epochs; requirements P-083…P-103 | holds: `route.py cursor` and `epoch` at Setup; `matrix.py coverage` read `verified 0/21 · unclaimed 21` |
| `review-feedback-1.md` holds these items as handed to the operator | holds: read whole |
| the plugin host is `crates/plugins`, the `wasmtime` dependency, `plugins-examples/` | holds: the crate and the folder exist (three examples); `wasmtime` is a workspace dependency (`Cargo.toml:140`) declared by `crates/plugins` and `pulse-app`; `ui-bridge` and `pulse-app` depend on `plugins` |
| workspace detection is `crates/workspace-detector`; incidents are keyed by the workspace it detects | holds: the registry's cooldown key is `(kind, scope, workspace)` (`crates/triage/src/incident/registry.rs:206`, `:250`); boot resolves one key from the detector and filters, stamps and persists by it (`pulse-app/src/main.rs:762-821`, `digest_runtime.rs:107-111`); the MCP sidecar reads the same key from a published file (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:77-78`); `mcp-server`, `ui-bridge` and `pulse-app` depend on the crate |
| the training export is `pulse-app/src/training_export.rs` and the `storage.export_for_training` route | holds: the file exists; the route is at `pulse-app/src/storage_router.rs:67`, `:157`; its test is `pulse-app/tests/e2e_p95_export_for_training.rs` |
| critical paths P4 (plugins) and P7 (workspace) | holds: `test-plan.md:77` and `:80` |
| P-091 says "kept for days" with no number | holds: `matrix.py show --id P-091` |
| three security suggestions stand deferred | holds: `merge-decisions.md:17`, `:23`, `:25` |
| the security master reads "Compliance triggers: None" | holds: `security-plan.md:105-107`; its basis there is that telemetry stays on the local machine |
| run `37907730264` on `60ef43c` failed, 11 of 12 green; job `113745162405`; the log's three lines | holds: `gh run view` (event `push`, branch `main`); the job log at its lines 657-660 |
| the same tree passed 12 of 12 as a pull-request run, `37904682919` on `0e45d58` | holds: `gh run view` (event `pull_request`, 12 of 12 success) |
| the workflow's top-level `permissions:` is `contents: read` alone; the step is `rustsec/audit-check` v2.0.0 at `:494` | holds: `ci.yml:8-9`; the step is `:493-496`; `ci.yml:3-6` runs on every pull request and on a push for `main` alone |
| `hypothesis:` on a push the step tries to create a check run and on a pull request it does not try | REFINED: it tries on both and is denied on both. The pull-request run's log reads `##[error]Unable to publish audit check! Reason: HttpError: Resource not accessible by integration` and the step passes; the push run's log reads the bare error and the step fails |
| `hypothesis:` the 8 unmaintained and 2 unsound findings are new and the step fails wherever it has something to report | FALSIFIED: the green pull-request run read the same advisory database (commit `7eebec69`, 1296 advisories) and printed the same "Found 8 unmaintained, 2 unsound". On HEAD's pull-request run `37914412856` the audit step ended success; that run was still in progress at this wrap (11 jobs settled, none failed) and its job log was not readable while the job ran |
| the phase letter's Setup 5a reads every commit since the last master flip, and a red that does not intersect the chunk halts at P1 | holds: `andromeda-phase/SKILL.md:137-158`; `references/promotion.md:64` — three answers (take up with the red recorded as unowned · the operator mints the owning chunk · fold into this chunk on his word). The base is `f18c631`, so `60ef43c` is among the shas read. No arm reads an owner pinned on another entry: the pin answers that halt in one line and does not prevent it |

Not measured: whether the audit step attempts the check run when it has no finding to report.

## Items and dispositions

**A. Three removals** (the founder). Three capabilities in Theme 0 and three entries in Epoch 2, after "Local model
retired" and before "Supply-chain gate re-based on the smaller graph", so all three precede "Detection parity after
the removals":

- P-104 "The plugin host is gone" — entry "WASM plugin host retired".
- P-106 "The training export is gone" — entry "Training export retired".
- P-105 "Workspace detection is gone" — entry "Workspace detection retired". What incidents are keyed by is the
  operator's answer 1, written into the requirement, the acceptance (one clause: the same storm under two tokens
  forms one incident) and a CARRY on the entry with the measured coordinates.
- CARRY on "End-to-end paths re-driven through the engine": P4 and P7 are named for retirement, not re-driven.
- CARRY on "Supply-chain gate re-based on the smaller graph": the departed set includes the WebAssembly runtime and
  the job's Cranelift-only assertion step.
- Ledger note on P-083: the reader note in `requirements.md` that 0.3.0's P-079 workspace key "stays" is superseded.

**B. Seven days** (the founder). No capability: a dated ledger note on P-091, a CARRY on "Telemetry store on disk"
(the term, that it is configurable, the size ceiling) and a CARRY on "Disk store measured under load" (whether a
week fits; the ceiling's figure is measured, not ruled). P-091's acceptance text is unchanged; its claiming chunk
concretizes it.

**C. Five controls** (the operator, PROVISIONAL):

1. Channel termination and key custody — CARRY on "Network OTLP receiver behind a token"; ledger notes on P-087 and
   P-088 (the key is held like the token).
2. The door encrypted in transit — CARRY on "Door admission and bounds".
3. The door's admission lifecycle — a second CARRY on the same entry; one ledger note on P-093 covers 2 and 3.
4. How the engine reaches a node — P-107 "The engine reaches a node" (Theme 1, method `manual`) and the entry
   "Engine delivered to a node" in Epoch 3, before "Theme 1 checked by the external harness".
5. The personal data of the real service — CARRY on "Real service watched for days"; ledger note on P-101. The
   security master is not edited.

**D. The red on `main`** (the operator's answer 4). Owner: a CARRY on "Windows and macOS CI legs retired; pre-push
check native on Linux" (Epoch 1), carrying the run and job ids, the denied call, and that a pull-request run of the
same step does not witness the repair. No letter makes another entry the only lawful owner: a `CARRY:` on a specific
entry is an owner by `route-resolve.md` §Drift = 0 and by the wrap's own P7.1 list, so the operator's entry was
used. `main` stays red until a repaired tree is pushed to it.

## What was written

- `andromeda-pulse-0.4.0/requirements.md`: four capability lines, each one anchored Edit (P-104, P-105, P-106 after
  P-085; P-107 after P-090). No other byte moved. Its header still reads "P-083…P-103" and "the three removals of
  Theme 0", and its reader note on P-079 still stands: this path may add capability lines only.
- `andromeda-pulse-0.4.0/verification-matrix.json`: four entries added through `matrix.py add` (ids minted by the
  tool, payloads `add-*.json`), born `planned`, unclaimed; ten dated notes through `matrix.py note` (texts
  `note-P-*.md`). `matrix.py coverage` reads `verified 0/25 · unclaimed 25`. The ledger and the requirements hold no
  Cyrillic, so each note cites the founder's pick in English and points here for his words.
- `andromeda-pulse-0.4.0/working-route.md`: four entries inserted, ten `CARRY:` blocks pinned on nine entries. 52
  markerless entries of 52; the first is still "Corpus encryption at rest retired" (`:11`); `route.py pins` reads
  all ten blocks. No frozen line exists. Epoch 2 holds 9 entries and Epoch 3 holds 8; neither reaches the split
  nudge.

## Not run on this path

No report, no fan-out, no gates, no master write, no code-graph refresh, no citation sweep, no consolidation, no
amendment: the masters describe the tree as it is, and each removal's drift belongs to its chunk's wrap. No `gated`
record exists, so there was no premise to re-verify. No residual line was appended (the ledger holds 0 `open`
lines). Curation ran on this conversation: four candidates, none written — the pull-request-passes-while-push-fails
reading of the audit step (0.6 exactly, rejected; the route's CARRY carries it), the GitHub repository's spelling
(below threshold; it rides the handoff's host note), the ASCII-only ledger convention (paraphrased from implicit
behaviour) and the operator's choice of owner (task-specific). No source file was touched; no merge, tag, release
or version bump.
