# Operator relay: a 0-pending wrap, Pulse (2026-10-06) — route adaptation after Conductor's fourth series

From the pc overseer (founder-delegated). Facts are `measured 2026-10-06` unless marked `hypothesis:`.

## 1. Context correction (what happened since your last wrap)

- Conductor ran its fourth pre-registered real-model series for its `v3-09` against Pulse `5f77859` tonight (three drives,
  one sitting, 20:06Z-20:31Z; Conductor `fa6a374`). It was the first reading of the shipped `gemma-4-E4B-it-Q4_K_M` at prompt
  `v2.5` on retry naming. **Verdict: NOT MET, 2 of 3.**
- All three rank-1 statements name the retry storm — the facet `TRIGGER_FRAMING_INSTRUCTION` asks for. They split on the
  service: d1 "Retry storm due to service instability on Conductor or Conductor-Canary", d2 "A retry storm is actively
  occurring in the Conductor service", d3 "Retry storm: Conductor-Canary experiencing a retry storm" (Conductor
  `conductor-0.3.0/chunks/2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09/evidence/rm-capture-d{1,2,3}.txt`,
  `:421`, `:424`, `:426`). Conductor's rule wants the triggering service as a whole word; d3 attributed the storm to the canary.
- In the same captures every canary digest (6 of 6) and every scenario digest (3 of 3) surfaced.
- Your tree: `target/release/pulse-app` and `andromeda-pulse-mcp` were rebuilt at `5f77859` by the Conductor builder
  (22:00 and 22:02 local) on the operator's grant; nothing else in this repository was touched, and no process is left.

## 2. Adaptation item, for your disposition

**FOUNDER RULING 2026-10-06 ~22:50, live, his own word** (the overseer put three options; he picked the first, «думаю а»):
Conductor's `v3-09` is neither relaxed nor deferred — Pulse is fixed, then Conductor runs a fifth series. Rejected by him:
relaxing Conductor's rule to the retry facet alone; deferring `v3-09` at Conductor's version close.

So Pulse owes one new route entry. The overseer's working title is **"the L4 rank-1 hypothesis names the triggering cue's
service"**; the wording is yours. Measured basis for its CONTEXT:

- The digest's trigger line names the signal and no scope: `crates/triage/src/digest/assembler.rs:680`
  (`TRIGGER_LINE_PREFIX` `:608`); Pulse's own test expects that line to read TRIGGER: Retry storm (`:1186`).
- `TRIGGER_FRAMING_INSTRUCTION` (`crates/interpretation/src/prompt.rs:90-94`) obliges rank 1 to name that signal "in the
  TRIGGER line's own words". Nothing instructs the service.
- The service reaches the model only on the cue line (`scope_id=…`) and the `SERVICES` rows.
- `hypothesis:` d3's "Conductor-Canary" came from a `CORPUS MATCHES` line of an earlier canary incident. Supporting, not
  proving: d1's own narrative says "Prior context indicates a similar 'Retry storm: Conductor-Canary …' incident was active
  3 minutes ago" (`rm-capture-d1.txt:417`). The phase measures it; do not build on it unmeasured.

The HOW (the scope on the TRIGGER line, in the instruction, or both) is the phase's to settle by measurement. Its acceptance
is a pre-registered real-model reading, and every model run waits for daytime or the founder's word: only GPU runs are
barred at night on this host, everything else may run at any hour.

## 3. Pinning: anchors only

- The new entry is the FIRST take-up: the head of the markerless tail, ahead of "L4 generation records render unredacted".
  Reason: Conductor's fifth series and its version close are BLOCKED-ON the sha that ships it. The placement is the
  overseer's; the founder can move it.
- Everything already in the tail keeps its order behind it.

## 4. Also at this wrap

- The `U35` door your 2026-10-04 setup re-run handed to "a 0-pending /andromeda-wrap-session": take it now.
- `pwsh` 7.6.6 is installed on this host since tonight (`/usr/bin/pwsh`), so the `.ps1` latency-grader CARRY on
  "pre-push:linux runs natively on Linux" no longer waits on the founder; correct that CARRY's wording if it says so.
- A curation candidate your own setup re-run of tonight named: the second seed bullet under Session Learnings in CLAUDE.md
  still says a new TauRPC procedure needs a `pulse-app/capabilities/` JSON entry, while the Critical Warnings block and
  `security.md` say that entry does not exist in this project. Yours to confirm against the tree and correct.
