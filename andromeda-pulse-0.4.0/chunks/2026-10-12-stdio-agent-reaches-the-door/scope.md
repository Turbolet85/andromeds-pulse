# Scope — Stdio agent reaches the door

**Marker:** `2026-10-12-stdio-agent-reaches-the-door` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:39`):** "Stdio agent reaches the door — an agent on the developer's machine speaking
stdio reaches the door's tools with no sidecar; its form is a widening brought at P4 (P-092)"
**Chunk base:** `d3e694d2` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
Since `2026-10-10-door-inside-the-engine-s-process` the console engine `andromeda-pulse-engine run` answers the nine
tools on a door inside its own process: an owner-only Unix-domain socket at `{data dir}/run/door/sock`, one JSON-RPC
2.0 line per request. An agent on the developer's machine that speaks MCP over stdio (it starts a command and talks
to that command's standard input and output) cannot use that door today: the project ships no way between stdio and
the socket, and the only stdio way in is the sidecar `andromeda-pulse-mcp`, a second process built on what this
version removes. This chunk gives a stdio agent a way to the door's tools that needs no sidecar, so that the next
entry, `Stdio sidecar retired`, can remove the sidecar without leaving the agent with nothing.

## What the chunk builds (the entry's own words)
- **An agent on the developer's machine speaking stdio reaches the door's tools.** The agent starts something and
  speaks to it over stdio; what it reaches is the door, and what answers is the running engine from its own stores.
- **With no sidecar.** The way does not pass through `andromeda-pulse-mcp` or anything it is built on (the
  credential store open, the published workspace key, the parsed incident).
- **Its form is a widening brought at P4.** The form is not chosen; P4 lays out the forms and the operator answers.
- **P-092** is the capability the entry names; it is advanced, not expected to be claimed (its acceptance holds
  legs owned by later entries — read at P4 from the ledger).

## The entry's freight (one `CARRY:`, folded; route.py pins lists one block for line 39)
- **CARRY:** "the form of this way is not chosen — it is a boundary widening that this entry's own P4
  brings to the operator, and nothing was chosen at the card that minted it; its acceptance words are that an agent
  reads through the door and no sidecar exists, so Stdio sidecar retired follows this entry and the sidecar does not
  leave before this way exists (the operator's word at 2026-10-10-door-inside-the-engine-s-process's route-resolve
  card, the pc overseer, 2026-10-11); as built at that chunk the door speaks over a Unix-domain socket only, this
  project ships no stream relay between stdio and that socket, and whether a general-purpose one is installed on the
  dev host was not measured (record: that chunk's plan, Implementation notes)". Folded as: the form is P4's dialog;
  the sidecar stays in this chunk. "This project ships no stream relay between stdio and that socket": verified at
  `d3e694d2` — the console grammar is `run` | `version` alone (`pulse-app/src/console.rs:16-42`) and no door client
  exists outside the tests (research, Graph impact).
- [premise-corrected: measured at P3 — `command -v socat` → `/usr/bin/socat`, `socat version 1.8.1.3`; `nc`, `ncat`,
  `netcat`, `busybox`, `websocat` absent] A general-purpose stream relay IS installed on the dev host: `socat`. Whether
  the `ubuntu-22.04` runner has it is unmeasured. The CARRY's "was not measured" stood when written; it no longer does.

## What the directive adds (inputs#I1 the invocation, inputs#I2 the phase directive — each closed at P3)
Closure at P3: every bullet below is verified as the path the plan takes. The surface statements were read against
architecture §Standard Contracts → Engine door (`:157`) and §Occupied Resources → Network ports (`:181`, the door's
PROVISIONAL sentence). "Conductor is not touched" was read in this repository alone: the sidecar names Conductor as a
process that spawns it (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:72`), and the leaned form leaves both the
sidecar and the door's form unchanged. Conductor's own repository was not read. The merge-base probe and the shared
ports stand in test-plan §3 and the handoff.
- **Stop at P4 with the forms, and at P5 for the operator's yes** (inputs#I1). At P4 the forms are laid
  out with their alternatives — for example a stdio bridge inside the engine binary, a separate thin bridge program,
  the agent dialling the socket itself — each with what it adds as a new process, file, command word or place on
  disk, and a lean; then stop (inputs#I2 item 2).
- **The answer at P4 is PROVISIONAL** (inputs#I2 item 2): the founder's word on the door's surface is still
  owed (the door is PROVISIONAL: a Unix-domain socket under `run/`, owner only by three layers, always on), so this
  chunk builds on that surface without widening it.
- **The door's surface does not move** (inputs#I2 item 3): no new listening socket, port, credential, tool
  or argument that reaches a path; the nine tools answer as they do. A form that needs any of these is a separate
  widening and is said so at P4.
- **The stdio sidecar stays** (inputs#I2 item 4); its removal is `Stdio sidecar retired`. At P5 the plan says
  what the stdio agent and the sidecar each reach, so nothing is reachable through one and not the other by accident.
- **Conductor is not touched** (inputs#I2 item 5): it reads Pulse as at 0.3.0; its read through the door is
  its own later entry. If this chunk changes what a Conductor run sees, stop at P4 and say where.
- **Acts that may meet the auto-mode classifier** (a directory delete, a CI-job cut) are listed at P5 so
  they are prepared before implement (inputs#I2 item 6).
- **A merge-base probe before every push** (inputs#I2 item 7; test-plan §3 states it already).
- **Ports 4317 / 4318** are shared with Conductor's builder on this host: ask before a run that binds them
  (inputs#I2 item 8; the handoff states it already).
- **At P5:** what `planlint` check 4 listed, P-092's concretization shown against its acceptance, the
  plan card, then stop for the yes (inputs#I2 item 9).
- `origin/main` is `178ebac5` (inputs#I2 item 1, measured by the operator); verified at P3 by `git ls-remote origin
  refs/heads/main` → `178ebac56e26…`. That it is PR #41 merged by the founder rests on the directive alone.

## Boundaries
- No change to the door's socket, its modes, its owner check, its bounds, its methods or its nine tools;
  no new listener of any kind.
- The sidecar `andromeda-pulse-mcp`, its double gate, its tests and its CI job stay as they are.
- The window app is not changed (it seats no door).
- Conductor's contract (architecture §Standard Contracts → Engine door) is read, not changed.

## CI read at Setup (5a)
- `d3e694d2` (the last wrap's flip and HEAD): **green**, 7/7 checks, wall 1716 s — `ci#38103099502` and
  `secret-scan#38103099479`, both pull_request, completed/success. Nothing to disposition.
