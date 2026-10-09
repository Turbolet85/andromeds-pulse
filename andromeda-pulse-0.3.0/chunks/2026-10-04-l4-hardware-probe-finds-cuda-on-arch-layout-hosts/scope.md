# Scope — The L4 hardware probe finds CUDA on Arch-layout hosts

**Marker:** `2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts` · **Version:** andromeda-pulse-0.3.0 ·
**Taken up:** 2026-10-04

## Intent (working entry, verbatim title + hint)
The L4 hardware probe finds CUDA on Arch-layout hosts — a host whose `libcuda.so` lives directly under `/usr/lib`
reads GPU-present.

## What this chunk builds
- The Linux arm of `detect_gpu_present` (`crates/interpretation/src/hardware.rs`) recognises a CUDA driver library
  installed directly under `/usr/lib` (the Arch / Omarchy layout), so `HardwareProfileDetector` reads GPU-present on
  such a host instead of falling to a CPU profile.
- The two layouts the arm already probes (`/usr/lib/x86_64-linux-gnu/libcuda.so` — Debian/Ubuntu multiarch;
  `/usr/local/cuda/lib64/libcuda.so` — the CUDA toolkit) keep reading GPU-present: the change widens the probe, it
  never narrows it.
- A host with no CUDA driver library at any probed location still reads GPU-absent (the safe default the arm's own
  comment names: a false negative reclassifies to a CPU profile, never the reverse).
- The probe's location set becomes testable without the host's real filesystem (the current arm reads fixed absolute
  paths, so a unit test can only observe the host it runs on) — a pure seam the production arm calls over the real
  paths, plus a pin on the production candidate set itself. (Verified at P3: no existing test calls
  `detect_gpu_present` or `detect_real` — `hardware.rs:203-283`; the seam's form is a P4 fork, research.md §Open
  questions.)
- Which file name(s) to probe: the unversioned `libcuda.so` is the name the arm uses today; the driver's runtime
  soname is `libcuda.so.1` (on this host both exist, `/usr/lib/libcuda.so -> libcuda.so.1 -> libcuda.so.610.57.04`).
  Whether the soname is also probed is a P4 fork. (Verified at P3 as an open question: no host reachable here shows
  a soname-only install, so it is decided at P4, not measured.)
- `/usr/lib64` — on this host a symlink to `/usr/lib` (Arch); on Fedora/RHEL layouts a real directory (unmeasurable
  here). Whether the widening names it is the same P4 fork; the entry itself names only `/usr/lib`. (Verified at P3:
  `/usr/lib64 -> lib` on this host.)

- **Ruled at P4 (2026-10-04):** the candidate set is every pair of four directories (the two existing ones,
  `/usr/lib` and `/usr/lib64`) and two names (`libcuda.so` and `libcuda.so.1`) — "dirs × names", the overseer,
  founder-delegated.
- **Folded at P4 by founder ruling D-3(a) (registry U40, relayed by the overseer, 2026-10-04):** one comment line
  carrying the token `andromeda:walks-tree` in the test module of each of `xtask/src/source_lint.rs` and
  `xtask/src/staged_gate.rs`, so the gate tool's walk-class line marks those tree-walking tests. Comment only.
  Measured at P4: both files' tests walk TempDir / fixture-repo trees, never the real repository.

## Boundaries
- Linux arm only. The macOS (`true`) and Windows (`nvcuda.dll`) arms are not touched.
- The probe stays a filesystem-presence heuristic over system-level paths — no `dlopen`, no `nvidia-smi` subprocess,
  no model or URL probing (the fn's own security note: "only system-level signals"); no new dependency.
- The `ANDROMEDA_PULSE_HARDWARE_PROFILE` override, the profile vocabulary (`gpu_primary` / `gpu_fallback` /
  `cpu_primary` / `cpu_fallback` / `unknown`), `profile_label`, the `ModelLoadEvent` broadcast and the TauRPC /
  bindings surface are unchanged. No new env var, port, procedure, capability, table or log target. (Verified at P3:
  the change is confined to `hardware.rs`, plus at most a dev-dep line in its `Cargo.toml`.)
- No path logged in full (security rule: basename only, if anything) — the probe logs no path at all. (Verified at
  P3: the three `interpretation.hardware.detect` emit sites, `hardware.rs:51,60,115`, carry `gpu_available` (a bool)
  and bounded labels only. The target has its own exact allowlist leaf at `observability.rs:2022`.)
- Not in scope: the real L4 model's availability on this host (a founder desk act; the next entry owns the
  measurement that needs it), the llama.cpp CUDA vs CPU binary selection (`ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH`)
  beyond whatever reads the detected profile today.

## Surfaces touched
- `crates/interpretation/src/hardware.rs` — `detect_gpu_present` (Linux arm, the `libcuda.so` candidate array at
  `:147-150` + the `.any(..exists())` at `:151-152` at HEAD `03fb097`; the hint's `:147-152` at `ffb62f0` — the file
  is unchanged between the two) and its doc comment / tests.
- Consumers read the profile through `HardwareProfileDetector` only (verified at P3, code graph:
  `pulse-app/src/main.rs:509`, the `hardware_profile.rs:14` re-export, `l4_decision_probe.rs:489`,
  `integration_real_llama_cli.rs:95`; none needs an edit). On a host that flips to GPU-present, the downstream
  changes are: the model tier is unchanged (`Primary`); L4 real mode reads `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` /
  `-ngl 99` instead of the CPU var (`llamacli_inference.rs:444-453`); the Tier-2 cadence stops being skipped
  (`coordinator.rs:154-165`); and `model.current_profile` / `diagnostics.snapshot` read `gpu_primary`. A host with
  no probed library is unchanged.
- [premise-corrected: no master states the probe's path set; the one doc hit mis-describes the mechanism] Docs:
  `.claude/docs/services/interpretation.md:26` calls the GPU probe "dlopen-shaped", but it is an `exists()` presence
  check. It is a leaf doc, re-derived at wrap. No arch / security / obs / test master names `libcuda` (P3 grep).

## Folded freight (from the working entry)
- **CONTEXT (verbatim):** "`detect_gpu_present`'s Linux arm (`crates/interpretation/src/hardware.rs:147-152` at
  `ffb62f0`) probes only `/usr/lib/x86_64-linux-gnu/libcuda.so` and `/usr/local/cuda/lib64/libcuda.so`; on the
  Omarchy (Arch) dev host neither exists and the library is `/usr/lib/libcuda.so` (→ `libcuda.so.1`), so the L4
  hardware profile reads GPU-absent there (measured at 2026-10-04-linux-launch-stays-up-on-nvidia-wayland P3 and
  re-checked at its wrap); placed before the Rust-floor entry on the overseer's proposal at that wrap — the founder
  may move it".
  - Coordinates re-verified at take-up (2026-10-04, HEAD `03fb097`): the array still names exactly those two paths
    (`hardware.rs:148-149`); `ls` reads both absent on this host; `/usr/lib/libcuda.so -> libcuda.so.1 ->
    libcuda.so.610.57.04` present; `/usr/lib64 -> lib`; `ldconfig -p` resolves `libcuda.so` and `libcuda.so.1` to
    `/usr/lib` (x86-64) and `/usr/lib32` (32-bit).
- **Causal claim (measured-marked on the entry; kept verbatim):** "so the L4 hardware profile reads GPU-absent there
  (measured at 2026-10-04-linux-launch-stays-up-on-nvidia-wayland P3 and re-checked at its wrap)". VERIFIED at P3
  (2026-10-04, HEAD `03fb097`): with every override and bin var unset, `l4_decision_probe --arms A0 --n 1` exits 2
  with `INCONCLUSIVE - ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH is unset`. The detector reads GPU-absent, so with 32 cores
  it reports `CpuPrimary` and routes to the CPU binary.
- **Placement:** before "The declared Rust floor matches the code", on the overseer's proposal (2026-10-04); the
  founder may move it. Not a scope obligation.

## Operator facts carried (handoff, 2026-10-04)
- Any run that launches pulse-app, a window or the model needs a slot from the operator first (ports 4317/4318 are
  shared with conductor-builder). This chunk's proof needs no launch: unit pins, plus the dev-host
  `l4_decision_probe` routing reading, which binds no port and needs no model (verified at P3).
- The real L4 model is absent on this host; this chunk does not need it (GPU presence is a filesystem fact).

## CI verdict read at Setup (5a)
- `03fb097` (the last wrap's flip = HEAD): **green** — checks 13/13, wall 1717 s; ci#37211780437 completed/success,
  secret-scan#37211780392 completed/success. Nothing to fold.
