# Scope — Linux launch stays up on NVIDIA + Wayland

**Marker:** `2026-10-04-linux-launch-stays-up-on-nvidia-wayland` · **Version:** andromeda-pulse-0.3.0 · **Taken up:** 2026-10-04

## Intent (working entry, verbatim title + hint)
Linux launch stays up on NVIDIA + Wayland — the default Linux launch survives on an NVIDIA GPU under a Wayland
compositor.

## What this chunk builds
Two halves, in order, per the founder ruling 2026-10-04 ("measure the cause first, then ship a default, detect and
fall back, or document"):

1. **Measure the cause.** On the dev host (NVIDIA GA102 / RTX 3090 · Hyprland · WebKitGTK 2.52.6 · native Wayland),
   establish by real-display launches whether the Gdk `Error 71 (Protocol error) dispatching to Wayland display`
   death at the default posture REPRODUCES, how reliably, and which lever(s) prevent it — enough to say whether the
   DMA-BUF renderer is the CAUSE or only a LEVER that hides another one. The measurement is the chunk's first
   deliverable and its result decides half 2.
2. **Ship the remedy the measurement supports** — exactly one of the ruling's three shapes:
   - **a default** — the Linux launch sets the measured lever before GTK/WebKit initialise, on every Linux host;
   - **detect and fall back** — the lever is applied only where the measured trigger condition holds (e.g. an NVIDIA
     driver under a Wayland session). The detection signal is still unchosen: it is the P4 fork, decided from the
     measurement in research.md (fixed-literal candidates seen on this host: `/sys/module/nvidia`,
     `/proc/driver/nvidia`; session: `WAYLAND_DISPLAY`/`XDG_SESSION_TYPE`);
   - **document** — if no in-process remedy is sound, the limitation and the workaround are stated where a Linux user
     meets them.
   Which shape ships is a P4 fork, decided from the half-1 result, never from the hypothesis.

## Boundaries
- Linux only. macOS and Windows launch paths are unchanged.
- A value the user (or a harness) has ALREADY set for the lever's env var is honoured, never overwritten — verified as
  a consequence of arch §Cross-cutting Patterns → Config management (process env outranks built-in defaults).
- No port, TauRPC procedure, capability JSON or corpus change — verified: the change is additive (one call at the head
  of `main()`, one emit after `observability::init`; research.md §Graph impact). If the remedy is a product env
  read/write, it is a new entry in arch §Occupied Resources (env vars) — a wrap amendment, not a phase edit.
- Not in scope: the Halo glow layer, WebGPU frame budgets, and any non-NVIDIA or X11-session failure (none has been
  observed). A measurement that turns one up is recorded and routed, not absorbed.

## Surfaces touched (closed against the code at P3)
- `pulse-app/src/main.rs` — [premise-corrected: "before the Tauri builder runs" is not early enough; `main()` builds
  the multi-thread tokio runtime at `main.rs:285`, whose worker threads exist before `observability::init` at `:293`]
  the env lever must be the FIRST statement of `main()`, ahead of the tokio runtime build, for the edition-2024
  `set_var` precondition to hold.
- `pulse-app/src/observability.rs` — verified pattern: one bounded boot record naming the chosen posture (closed
  label, no path, no env value beyond the var NAME), behind its own exact allowlist leaf, emitted AFTER
  `observability::init` (deferred emit: no subscriber exists at the head of `main()`).
- A new `pulse-app/src/render_posture.rs` module + `pub mod` in `pulse-app/src/lib.rs` (research.md §New files).
- [premise-corrected: there is no README at the repo root] a "document" shape has no user-facing doc home in the repo
  today; `.claude/docs/` leaves are agent-facing and wrap re-derives them.

## Folded freight (from the working entry)
- **CONTEXT (verbatim premise):** "S2's default posture died ~1.5 s in on Gdk `Error 71 (Protocol error) dispatching
  to Wayland display` (NVIDIA GA102 · Hyprland · WebKitGTK 2.52.6 · native Wayland — one host, one launch); a relaunch
  with `WEBKIT_DISABLE_DMABUF_RENDERER=1` stayed up for the whole P-075 round (measured at
  2026-10-02-incident-events-readable-through-mcp, `evidence/round-result.md`); whether the DMA-BUF renderer is the
  cause or only the lever is unmeasured and no confining mechanism is identified".
  - Coordinate re-verified at HEAD: `andromeda-pulse-0.3.0/chunks/2026-10-02-incident-events-readable-through-mcp/evidence/round-result.md:43-51`
    states the death, the host, and the `WEBKIT_DISABLE_DMABUF_RENDERER=1` relaunch.
  - Overseer-relayed second coordinate (2026-10-04 phase note), re-verified: Conductor
    `conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/evidence/round-ledger.md:30-34` at Conductor
    `f5076ad`. **It is the SAME launch** (pulse-app pid 590369, "exited on its own (the Wayland crash)", ledger :133),
    observed from the Conductor side — **n = 1 failing launch, not two independent observations.**
- **[premise-corrected: measured at P3, 7 real-display arms — the default died 3/3 at ~2.0 s, exit 1; each of four
  single-variable levers kept it up 60 s] causal claim, marker kept verbatim:** "whether the DMA-BUF renderer is the
  cause or only the lever is unmeasured" → the DMA-BUF renderer is a LEVER, not the cause: `__NV_DISABLE_EXPLICIT_SYNC=1`,
  `GDK_BACKEND=x11` and Mesa's EGL vendor each prevent the death as well, and the common factor of all four is
  removing NVIDIA's EGL explicit-sync path (egl-wayland2) on a native Wayland surface (research.md §Host measurement;
  matches Linger#135 on driver 610.57.04 + WebKitGTK 2.52.6). Which protocol message the compositor rejects is
  unmeasured; exposure on other hosts is unmeasured; n = 1 per lever.
- **Mechanism (verified at P3, was an inferred hypothesis):** GTK3's Wayland backend `_exit(1)`s after logging a
  dispatch protocol error — `libgdk-3.so.0` `_gdk_wayland_display_queue_events`: `wl_display_dispatch_pending` < 0 →
  `g_log` "Error %d (%s) dispatching to Wayland display." → `_exit(1)`. That is why no `app.exit` record exists for the
  death (`_exit` is a stated blind spot of `app.exit`).
- **Founder ruling 2026-10-04** (relayed by the pc overseer): measure the cause first, then ship a default, detect and
  fall back, or document.
- **watch:** CI `boot smoke (ubuntu-22.04)` reached `boot: ready`, then ended `exit 1` with no `app.exit` and no
  `app.panic.fatal` record before `agent-run.sh status` (run ci#37189514735 attempt 1 on a073722; the re-run of the
  failed job was green; cause not established; first red in 25 runs) (0/3; since
  2026-10-04-corpus-key-creation-is-race-free). Observation only — no criterion, task or gate.

## Measurement constraints (operator facts, 2026-10-04 phase note)
- A real-display slot is available while the founder is at the desk; **the agent ASKS the operator for it and never
  launches on its own.** Ports 4317/4318 are shared with conductor-builder (handoff Notes) — same rule.
- overseer1's Xvfb recipe (`env -u WAYLAND_DISPLAY GDK_BACKEND=x11 xvfb-run …`) avoids Wayland, so it CANNOT measure
  this defect; it is admissible only as an X11 control arm, never as the witness.
- The code graph is live on this host again (operator); the handoff's "refresh reads STALE" note is superseded for
  this session — P3 verifies before relying on it.

## CI verdict read at Setup (5a)
- `ffb62f0` (the last wrap's flip = HEAD): at Setup **verdict not yet available** — ci#37192549178 in progress (12 of
  13 checks running at read; oldest `lint / test (ubuntu-22.04)` 166 s), secret-scan#37192549168 completed/success.
  Re-read at P3: **green** — checks 13/13, wall 1115 s (ci#37192549178 completed/success). Nothing to fold; the
  watch's retirement count is the wrap's to record.
