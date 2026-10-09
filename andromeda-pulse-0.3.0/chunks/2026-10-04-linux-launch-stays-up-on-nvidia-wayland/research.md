# Codebase Research — 2026-10-04-linux-launch-stays-up-on-nvidia-wayland

## Scope
- **Depth:** deep (the boot region + a live host measurement) · **Reads:** 14 · **Globs/Greps:** 22 · **Live launches:** 7 (operator-granted real-display slot, 2026-10-04 11:51–11:56 local)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (body §5-command discipline · §Status endpoint shape · §PID file · §Scenario legs, plus Session Additions 2026-08-28 ×2 [scenario-not-gate; window outlasts threshold] · 2026-08-29 [second source before a negative product claim] · 2026-08-30 [cross-invocation `$$` state]); 4 additions applied to the measurement design (a fresh data dir per arm; a 60 s window well past the ~2 s death; a second source per arm, stderr AND the JSON log; teardown judged by pid + port probes, not the kill's exit).
- **Platform issues consulted:**
  - `WebKitGTK "Error 71 (Protocol error) dispatching to Wayland display" NVIDIA` → itsMattGuenther/Linger#135 (fetched): reporter host NVIDIA RTX 4090 · open driver **610.57.04** · Hyprland (Omarchy) · system WebKitGTK **2.52.6** · native Wayland — default crashes with Error 71; `__NV_DISABLE_EXPLICIT_SYNC=1` launches; `=0` crashes. Stated mechanism: "NVIDIA's driver syncs frames with the compositor using explicit sync. Newer WebKitGTK and that path disagree, and the Wayland connection is closed with a protocol error." An AppImage bundling an older WebKitGTK did not crash.
  - omacom/omarchy#9716 (fetched; OPEN, unmerged): sets `WEBKIT_DISABLE_DMABUF_RENDERER=1` in `default/hypr/nvidia.lua` on the NVIDIA driver branches; claims "forcing Mesa's EGL vendor on the same machine makes both work with the DMA-BUF renderer still enabled, which isolates the fault to NVIDIA's EGL driver". The host's distro has NOT shipped this.
  - tauri-apps/tauri#10702 (fetched): open, labelled `status: upstream`; no maintainer default.
  - v2.tauri.app/develop/debug/linux-graphics (fetched): recommends `__NV_DISABLE_EXPLICIT_SYNC=1` ("often fixes the Wayland Error 71 crash without a performance cost") and `WEBKIT_DISABLE_DMABUF_RENDERER=1` ("at the cost of the faster rendering path"); "set them in `main()` before the webview is created so users do not have to"; "Only ship an unconditional override like this if you have verified your app is affected". Tauri sets neither by default.

## Host measurement (half 1 of the founder ruling — measured, not inferred)
Host: NVIDIA GA102 (RTX 3090, PCI 10de:2204) · `nvidia-open-dkms` / `nvidia-utils` 610.57.04 · Hyprland 0.56.2 · `webkit2gtk-4.1` 2.52.6 · `gtk3` 3.24.52 · `egl-wayland` 1.1.22 + `egl-wayland2` 1.0.2 (`/usr/share/egl/egl_external_platform.d/09_nvidia_wayland2.json` outranks `10_nvidia_wayland.json`) · mesa 26.2.2 · kernel 7.2.5 · `XDG_SESSION_TYPE=wayland`, `WAYLAND_DISPLAY` and `DISPLAY` both set (`pacman -Q`, `/proc/driver/nvidia/version`).

Binary: `target/debug/pulse-app` rebuilt at HEAD `ffb62f0` (`cargo build --bin pulse-app`, 14.6 s). Each arm: a fresh `mktemp -d` data dir, exactly ONE env variable changed, stdout+stderr captured, polled 1 s up to 60 s, then SIGTERM by pid (KILL after 10 s if needed), ports + process census re-probed. `:4317`/`:4318` re-checked free before every arm (operator condition); final census 0 `pulse-app`/WebKit processes, 0 listeners.

| arm | variable | ended | rc | t | stderr | `app.exit` | windows navigated | `ui.webgpu.adapter` |
|---|---|---|---|---|---|---|---|---|
| A1 | none (default) | died | 1 | 2.0 s | `Gdk-Message: … Error 71 (Protocol error) dispatching to Wayland display.` | 0 | none recorded | none recorded |
| A2 | none | died | 1 | 2.0 s | same line | 0 | none | none |
| A3 | none | died | 1 | 2.0 s | same line | 0 | none | none |
| B | `WEBKIT_DISABLE_DMABUF_RENDERER=1` | alive → TERM | 143 | 60.7 s | no Error 71 | 1 | 4/4 (`compact-widget`, `main`, `findings`, `report`) | `no_navigator_gpu` ×2 |
| C | `__NV_DISABLE_EXPLICIT_SYNC=1` | alive → TERM | 143 | 60.7 s | no Error 71 | 1 | 4/4 | `no_navigator_gpu` ×2 |
| D | `GDK_BACKEND=x11` (XWayland control) | alive → TERM | 143 | 60.7 s | `Failed to create GBM buffer of size 960x540: Invalid argument` ×2 | 1 | 4/4 | `no_navigator_gpu` ×2 |
| E | `__EGL_VENDOR_LIBRARY_FILENAMES=/usr/share/glvnd/egl_vendor.d/50_mesa.json` | alive → TERM | 143 | 60.7 s | `MESA-EGL: warning: egl: failed to create dri2 screen` | 1 | 4/4 | `no_navigator_gpu` ×2 |

In every arm the OTLP ports bound before the end (`ports_seen_up=1`), 0 `app.panic.fatal`, 0 ERROR records.

**What the measurement establishes (and its limits):**
- The default-posture death REPRODUCES deterministically on this host: 3/3 at ~2.0 s, exit 1 (n = 3; the 2026-10-02 round's n = 1 is now n = 4 with it).
- Four single-variable levers each prevent it (n = 1 each). Their common factor is that each removes NVIDIA's EGL explicit-sync path on a native Wayland surface: no DMA-BUF EGL surface (B), implicit sync (C), no Wayland surface at all (D), no NVIDIA EGL (E). **So `WEBKIT_DISABLE_DMABUF_RENDERER` is a LEVER, not the cause**; the cause is consistent with NVIDIA's EGL explicit-sync path (egl-wayland2) under WebKitGTK 2.52 on native Wayland, which matches the Linger#135 account on this exact driver and WebKitGTK version. The protocol violation is raised by the compositor; which protocol message violates the rule was NOT captured (no `WAYLAND_DEBUG` arm) — stated as unmeasured.
- D and E stay ALIVE on a DEGRADED render path (GBM buffer allocation failures; Mesa cannot drive the NVIDIA device), so they are measurement controls, never remedies.
- WebGPU is absent (`no_navigator_gpu`) under B, C, D and E alike, so no lever costs WebGPU here; WebKitGTK 2.52 exposes no `navigator.gpu` on this host. The default arms died before any adapter record, so the default posture's own adapter outcome is unrecorded.
- Measured on ONE host. Exposure on other NVIDIA + Wayland hosts (other compositors, egl-wayland without egl-wayland2, older WebKitGTK) is UNMEASURED; no confining mechanism beyond the four-lever pattern is identified.

## Files inspected
- `pulse-app/src/main.rs` (270–330, 1100–1130, 1560–1576) — `main()` builds the multi-thread tokio runtime at `:285` (worker threads spawn at `build()`), enters it, then `resolve_data_dir` `:292`, `observability::init` + `hold_log_guard` `:293`, `install_exit_hook` `:294` (spawns `pulse-exit-reporter`), signal listener `:296`, `window::emit_boot_spans` `:299`; `tauri::Builder::default()` `:1112`; `run_return` `:1574`.
- `pulse-app/src/window.rs` (40–102) — `emit_boot_spans` emits `app.boot.webview.init {webview_backend}` (compile-target label `GTKWebKit` on Linux), `app.boot.gpu.check {wgpu_backend}`, `app.boot.tray.init {tray_api}`.
- `pulse-app/src/lib.rs` (1–40) — the `pub mod` list a new module joins.
- `crates/interpretation/src/hardware.rs` (128–162) — `detect_gpu_present` Linux arm probes only `/usr/lib/x86_64-linux-gnu/libcuda.so` and `/usr/local/cuda/lib64/libcuda.so`; on this Arch host libcuda is `/usr/lib/libcuda.so`, so the L4 probe reads GPU-absent here (side finding, outside this chunk).
- `/usr/lib/libgdk-3.so.0` (gtk3 3.24.52, disassembled) — see Patterns detected.
- `scripts/agent-run.sh` (23–130) — `boot` captures the app's stdout+stderr to `$DATA_DIR/logs/boot.log` (`:79`), so the Gdk line lands there under the harness.
- `.github/workflows/ci.yml` (`:444` comment, apt `xvfb` rows) — the Linux `boot` job runs under one `xvfb-run` (X11), so CI never takes the native-Wayland path.
- `andromeda-pulse-0.3.0/chunks/2026-10-02-incident-events-readable-through-mcp/evidence/round-result.md:43-51` and Conductor `conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/evidence/round-ledger.md:30-34,133` at Conductor `f5076ad` — the same single launch (pid 590369), now superseded by the 7-arm measurement above.

## Graph impact (rust plane built this phase: 8613 nodes / 41898 edges, 72 s)
- **emit_boot_spans** — 1 caller: `main()` @ `pulse-app/src/main.rs:299` — the boot-record emit site a posture record sits beside.
- **observability::init** — callers `main()` @ `pulse-app/src/main.rs:293` plus three tests (`pulse-app/tests/e2e_p003_panic_hook_propagation.rs:51`, `pulse-app/tests/integration_exit_cause_record.rs:57`, `pulse-app/tests/perf_budget_samples.rs:115`) — unchanged signature; no threading.
- **install_exit_hook** — `main()` @ `pulse-app/src/main.rs:294` + 4 re-exec tests in `pulse-app/tests/integration_exit_cause_record.rs` — unchanged.
- **detect_webview_backend** — `emit_boot_spans` @ `pulse-app/src/window.rs:89` + `pulse-app/tests/unit_window_shell.rs:14,38` — unchanged.
- The change is ADDITIVE: one new call at the head of `main()` and one emit after `observability::init`; no existing signature changes, so there is no caller set to thread. Trace: `.andromeda/runs/2026-10-04T09-36-17Z-phase/tree-query-2026-10-04-linux-launch-stays-up-on-nvidia-wayland.json`.

## Patterns detected
- **GDK `_exit(1)` on a Wayland dispatch error** (`libgdk-3.so.0` `_gdk_wayland_display_queue_events`, offsets `0x5a42b`–`0x5a48e`): `wl_display_dispatch_pending` < 0 → `errno` → `g_log_structured_standard` with the format `Error %d (%s) dispatching to Wayland display.` → `mov $0x1,%edi; call _exit`. errno 71 = EPROTO (the compositor sent a fatal protocol error). This is why the death leaves no `app.exit`: `_exit` runs no `atexit` handler, matching obs-plan §7's stated unloggable set. The scope's mechanism hypothesis is VERIFIED.
- **Nothing sets these levers today**: `grep -rn -E 'set_var|WEBKIT_|GDK_BACKEND|__NV_|DMABUF' pulse-app/src crates/*/src` → 0 production hits (every `set_var` hit is inside a test module); the registry sources `wry-0.55.0`, `tao-0.35.0`, `tauri-2.11.0`, `tauri-runtime-wry` → 0 hits for `WEBKIT_DISABLE|__NV_DISABLE_EXPLICIT_SYNC|GDK_BACKEND`.
- **Env-layer boot posture shape** (`ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`, `ANDROMEDA_PULSE_L4_ALLOW_ROOT`): trimmed read, a closed outcome, announced once per boot, never a panic — the shape a launch posture copies (arch-history 2026-08-16, 2026-08-26).
- **Deferred emit**: a decision taken before the sink exists must be carried as a value and emitted after `observability::init` (no `tracing` subscriber exists at the head of `main()`).

## Conventions to follow
- **Edition-2024 `set_var` precondition**: `std::env::set_var` is `unsafe`; its precondition (no other thread reads/writes the environment concurrently) holds ONLY before `tokio::runtime::Builder::new_multi_thread().build()` at `pulse-app/src/main.rs:285`. The lever write must be the first statement of `main()`, not merely before `observability::init`. A `// SAFETY:` comment names that ordering.
- **Child inheritance**: WebKitGTK renders in child processes (`WebKitWebProcess`, `WebKitNetworkProcess` — seen as `pulse-app` children in Conductor's ledger :124); a parent `setenv` before they spawn reaches them through the inherited environment. Whether an in-process `set_var` (vs a shell-exported variable, which arms B/C used) reaches NVIDIA's EGL at load time is the equality the GREEN leg must measure — not assumed.
- **Pure decision function, injected inputs** (test-plan §3 `classify` pattern; testing.md §11 Mocking): the posture choice takes the preset value and the OS as parameters; tests never mutate process env.
- **pulse-app tests live in `pulse-app/tests/*.rs`** (`[lib] test = false`; the dead-lib-src ratchet is a flat zero for lib sources, `main.rs` exempt) and the new lib module carries no `#[test]`.
- **Allowlist**: a new `app.boot.*` target needs its own EXACT leaf (no bare `app` key exists — `grep -A1 'by_target.insert(' pulse-app/src/observability.rs` lists 15 `app.*` keys, none bare), guarded in `pulse-app/tests/` by exact-resolve + two-way field-set equality + a fallback discriminator.
- **ASCII-only sources** (`cargo xtask check:english-sources`).

## New files to create
- `pulse-app/src/render_posture.rs` — the Linux launch-posture decision (pure fn over the preset value + OS) and the head-of-`main()` apply step returning the decision for the deferred emit.
- `pulse-app/tests/unit_render_posture.rs` — the decision pinned per arm with injected inputs.
- `pulse-app/tests/unit_observability_allowlist_render_posture.rs` — the posture record's exact-leaf guard.

## Files to modify
- `pulse-app/src/lib.rs` — `pub mod render_posture;`
- `pulse-app/src/main.rs` — apply the posture as the first statement of `main()`; emit the posture record after `observability::init`.
- `pulse-app/src/observability.rs` — the posture record's exact allowlist leaf in `AllowList::production()`.

## Open questions
- Which remedy shape ships (founder ruling: a default, detect and fall back, or document) and which lever (`__NV_DISABLE_EXPLICIT_SYNC=1` vs `WEBKIT_DISABLE_DMABUF_RENDERER=1`) → blocks: plan-decision (P4 fork; the file lists above hold for the two in-process shapes — a "document"-only outcome drops all six and has no README to land in, since none exists at the repo root).
- Does an in-process `set_var` at the head of `main()` reach NVIDIA's EGL in the UI process and the WebKit children the same way the shell-exported variable did in arms B/C? → blocks: implementation-scope (the GREEN leg on the operator slot measures it; a NO falls back to the documented shape).
