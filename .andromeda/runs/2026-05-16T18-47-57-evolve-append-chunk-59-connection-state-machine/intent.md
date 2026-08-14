# Intent — append-chunk-59-connection-state-machine

**Captured:** 2026-05-16T18:47:57Z
**Source:** /andromeda-evolve --allow-route-append (slug derived from new-session dashboard recommendation + docs/v0_2_0/pulse-v0_2_0-route.md prospective chunk #59)

## User intent (combining session context)

The /andromeda-new-session dashboard at session 71 start recommended
Path A — continue pulse v0.2.0 evolution by registering the next chunk
in route §2 Epoch 9 via /andromeda-evolve --allow-route-append. User
invoked with the flag set + no further input (per "work without
stopping for clarifying questions" instruction).

The next chunk per `docs/v0_2_0/pulse-v0_2_0-route.md` (Phase 1 —
Connection awareness) is:

### #59 — Connection state machine

- **Depends on:** nothing (parallel-safe with #57, #58)
- **Capabilities enabled:** P-001 (Receiver Lifecycle State),
  P-002 (Last-Span-Ago), P-003 (Receiver Failure Surface),
  P-004 (Orthogonal Health Domains)
- **Distillation layer:** L1b (state tracker, hot-path updated)
- **Crates touched:** `crates/ingest/` (new module inside)
- **TauRPC delta:** +1 procedure `connection.current_state()`
- **Broadcast topics delta:** +1 `pulse://stream/connection-state`
- **Workspace deps delta:** none
- **Arch registry delta:** +1 broadcast topic, +1 TauRPC procedure
- **Specialist plan touches:** arch (reconcile with existing health
  IPC — document orthogonality: health=subsystem state,
  connection=data flow), test-plan (state transition coverage),
  obs-plan (heartbeat tick for connection module)
- **Summary:** LastIngestTracker atomic Instant updated in ingest
  hot path. Background poller (1-2s tick) emits state changes
  (Listening / Receiving / Idle / Stalled / ReceiverFailed) to
  broadcast. Receiver-task panic path connects to state machine via
  existing panic hook.

## Form

**Form 1** — chunk append to existing epoch (Epoch 9 — Foundation v0.2.0,
which currently holds only #57 + #58, both done in sessions 68 + 70
respectively).

## Grounding (per Check 8.6)

Concrete trigger: the v0.2.0 capability spec + distillation architecture
+ pre-existing route plan at `docs/v0_2_0/pulse-v0_2_0-route.md` identifies
#59 as the next dependency-driven step (Phase 1 — Connection awareness).
Implementation has not started; this Type 7 amendment registers the chunk
in route so /andromeda-phase + /andromeda-implement have a target.
