# Codebase Research — 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts

## Scope
- **Depth:** moderate · **Reads:** 7 (hardware.rs full; observability.rs `:2018-2036`; main.rs `:495-560`;
  llamacli_inference.rs `:430-470`; coordinator.rs `:140-165`; l4_decision_probe.rs `:1-48` + `:447-500`;
  interpretation `Cargo.toml`) · **Globs/Greps:** 9 · **Graph queries:** 2 (rust plane)
- **Harness rules consulted:** none — no live leg in this chunk (the dev-host routing evidence is a run of the
  already-built `l4_decision_probe` example, which binds no port and launches no app, window or model)
- **Platform issues consulted:** none — no runner-only bullet (CI `03fb097` green 13/13) and no CI-reading entry
  outside the operator leg

## Files inspected
- `crates/interpretation/src/hardware.rs` (full, 283 lines @ `03fb097`) — `detect_gpu_present` (`:131-167`), Linux arm
  `:141-153`: a two-element literal array (`:148-149`) checked with `Path::exists` (`:151-152`). It is private and
  has one caller, `detect_real` (`:102-124`), which maps `gpu_available` → `GpuPrimary`, else cores ≥ 4 →
  `CpuPrimary`, else `CpuFallback`. The `#[cfg(test)] mod tests` (`:203-283`, 10 tests) cover
  `parse_profile_override`, `profile_label`, `with_profile` and `available_cpu_cores`. **NO test calls
  `detect_gpu_present` or `detect_real`** (derivation: reading `:203-283`; the code graph's only `calls` row for
  `detect_gpu_present` is `detect_real` at `:102`). So the probe's candidate set is unguarded today.
- `crates/interpretation/Cargo.toml` — a LIBRARY crate with no `[lib] test = false`, so a co-located `mod tests` in
  `hardware.rs` runs under nextest. `rstest` is already a dev-dep. `tempfile` is NOT a dev-dep of this crate
  (derivation: reading the manifest's `[dev-dependencies]`, which holds `rstest` + `tokio` only).
- `pulse-app/src/observability.rs:2022-2034` — `interpretation.hardware.detect` has its OWN exact leaf
  `{profile, detection_latency_ms, gpu_available, cpu_core_count, profile_detection_decision_recorded,
  override_source}`. The three emit sites (`hardware.rs:51`, `:60`, `:115`) emit no path. `detection_latency_ms`
  is in the leaf but no site emits it (pre-existing, unrelated; recorded, not owned).
- `pulse-app/src/main.rs:509-517` — the boot detector is `HardwareProfileDetector::new()`. Its profile feeds
  `tier_for_profile` (`model_router.rs:31-37`: GpuPrimary and CpuPrimary both map to `ModelTier::Primary`) and
  `LlamaCliInference::new(tier, profile, …)`. The same `Arc` is passed on at `:542` (`model.current_profile`),
  `:912` and `:1395` (the cadence coordinator and diagnostics).
- `pulse-app/src/llamacli_inference.rs:444-453` — `binary_target_for_profile`: GpuPrimary/GpuFallback →
  (`ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`, `-ngl 99`, `"cuda"`); CpuPrimary/CpuFallback/Unknown → (`…_CPU_BIN_PATH`,
  `-ngl 0`, `"cpu"`).
- `crates/triage/src/cadence/coordinator.rs:154-165` — a Tier-2 cycle is SKIPPED when the profile is `CpuPrimary`
  ("tier2 skipped: cpu-primary profile disables acceleration"). A `GpuPrimary` host runs Tier-2.
- `pulse-app/examples/l4_decision_probe.rs:447-500` — the dev probe resolves `HardwareProfileDetector::new()` →
  `binary_target_for_profile` before any generation. With the bin vars unset it exits 2 printing
  `l4-decision-probe: INCONCLUSIVE - {env_name} is unset` (format string at `:448`). `{env_name}` names the
  routed binary var, so the probe reports the detected tier's routing with no model, no port and no app.

## Graph impact (rust plane; trace `.andromeda/runs/2026-10-04T16-59-02Z-phase/tree-query-2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts.json`)
- **`detect_gpu_present`** — 1 caller: `detect_real` (`hardware.rs:102`). It is private, so the change has no
  cross-crate blast radius and no signature change.
- **`HardwareProfileDetector`** — refs outside its own file: `pulse-app/src/main.rs:509` (boot) ·
  `pulse-app/src/hardware_profile.rs:14` (re-export) · `pulse-app/examples/l4_decision_probe.rs:49,489` ·
  `pulse-app/tests/integration_real_llama_cli.rs:39,95`. All construct through `new()`/`default()`, none through
  a seam this chunk would add, so none needs an edit.

## Patterns detected
- **Fail-safe probe default** (`hardware.rs:143-146`, `:157-159`): a false negative reclassifies to a CPU profile.
  The widening only adds candidates, so that default is untouched.
- **Parameter injection over ambient state** (test-plan §4 triage precedent, `now_nanos`): the analogue here is a
  pure fn over a root (or a candidate list plus an `exists` predicate). It is reachable at unit grain without any
  host path.
- **Bounded-label proof without a launch** (`l4_decision_probe.rs:448`): the `{env_name} is unset` verdict names
  `…_CPU_…` or `…_CUDA_…`. That is the end-to-end detector → routing reading, with no title or path printed.

## Conventions to follow
- **Test location**: co-located `#[cfg(test)] mod tests` in `hardware.rs`, which runs (library crate; see the
  manifest above).
- **Host independence**: no pin may read the host's real `/usr/lib*`. The dev host has the driver; the CI
  ubuntu-22.04 / macOS / Windows runners do not (test-plan §9 matrix). A test exercising the Linux arm directly is
  `cfg(target_os = "linux")`-gated (precedent: `crates/corpus` lock-dir pin). A pure helper over an injected root or
  list runs on every OS.
- **Fixtures**: synthetic layouts. `tempfile` would need a new dev-dep line in `crates/interpretation/Cargo.toml`
  (a dev-dep, so not shipped; `tempfile` is already in the workspace graph). The alternative, an injected `exists`
  predicate over a literal path set, needs no new dev-dep. Which one is P4's call.
- **ASCII-only source** (`check:english-sources`): new doc comments and literals stay Cyrillic-free.

## Equality the plan rests on (verified at HEAD)
- **On this host, HEAD's detector yields `CpuPrimary`, which routes to the CPU binary.** Measured 2026-10-04 (P3):
  `env -u ANDROMEDA_PULSE_HARDWARE_PROFILE -u …_LLAMA_CUDA_BIN_PATH -u …_LLAMA_CPU_BIN_PATH -u …_MODEL_PATH
  ./target/debug/examples/l4_decision_probe --arms A0 --n 1` → exit 2,
  `l4-decision-probe: INCONCLUSIVE - ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH is unset`. The binary predates HEAD's last
  commit, but none of `hardware.rs` / `llamacli_inference.rs` / the probe changed since (`git log -1` on the three
  files → `69f0b93`). The host has 32 cores (`nproc`), so the no-GPU arm is `CpuPrimary`.
- **The fix's equality**: with `/usr/lib/libcuda.so` present (as here), `detect_gpu_present` → `true` →
  `GpuPrimary` → `binary_target_for_profile` → `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` / `-ngl 99`, and the same probe
  run prints `… _LLAMA_CUDA_BIN_PATH is unset`. The deciding control is the candidate set in the Linux arm
  (`hardware.rs:147-150`).
- **Downstream consequences on a host that flips** (all designed behaviour for a GPU host, recorded so the plan
  states them):
  - the model tier is unchanged (`Primary` either way);
  - L4 real mode reads the CUDA bin var instead of the CPU one;
  - the Tier-2 cadence stops being skipped (`coordinator.rs:154-165`);
  - `model.current_profile` / `diagnostics.snapshot` report `gpu_primary`;
  - an `l4-latency-p99` run is graded on the gpu-primary row.
  On a host with no probed library (every CI runner), nothing changes.

## New files to create
- none

## Files to modify
- `crates/interpretation/src/hardware.rs` — widen the Linux arm's candidate set (the entry's `/usr/lib/libcuda.so`, plus whatever P4's fork adds), make the probe testable through a pure seam, and add the pins
- `crates/interpretation/Cargo.toml` — the `tempfile` dev-dep (P4 leaned the TempDir fixture form per test-plan.md:614)
- `xtask/src/source_lint.rs` — one `andromeda:walks-tree` comment line in its test module (founder ruling D-3(a), U40, relayed at P4)
- `xtask/src/staged_gate.rs` — one `andromeda:walks-tree` comment line in its test module (same ruling)
- derived `Cargo.lock` by `cargo build` — the interpretation entry's dependency list gains `tempfile`; no new package

## Sweep (named for the plan)
- `libcuda` over `*.md` / `*.rs` (`grep -rn 'libcuda\|nvcuda' --include=*.md .` and the `.rs` read above): code
  hits are only `hardware.rs:143-149`. Doc hits needing a wrap look: `.claude/docs/services/interpretation.md:26`
  states "The GPU probe is per-platform and **dlopen-shaped**". That is false at HEAD: it is an `exists()` presence
  check, with no `dlopen` anywhere in `hardware.rs`. It is a leaf doc, re-derived at wrap. No arch / security / obs
  / test master states the probe's path set (the grep returned no master hit).
- `ANDROMEDA_PULSE_HARDWARE_PROFILE` over the masters (`grep -n HARDWARE_PROFILE .andromeda/architecture.md
  .andromeda/security-plan.md` → 0 lines): the PRODUCT-consumed override (`hardware.rs:25,47`) is registered in
  neither arch §Occupied Resources → Environment variables nor security-plan §Input Validation's CLI / env var row.
  This is a pre-existing registry gap the chunk does not create and does not change. Precedents
  (2026-06-28-deterministic-env-gated-l4-mode, 2026-08-16-baseline-family-reachability,
  2026-08-25-demo-injector-formalized-api-surface-retire) closed the same shape as routine wrap registration.

## Open questions
- Which candidates beyond the entry's `/usr/lib/libcuda.so` does the widening name: the `libcuda.so.1` soname
  (present here beside the unversioned symlink), `/usr/lib64` (a symlink to `/usr/lib` here; a real directory on
  Fedora/RHEL layouts, unmeasurable on this host), or neither? → blocks: plan-decision
- Seam form: a pure fn over an injectable ROOT (production passes `/`; tests a `tempfile::TempDir` with real files
  and symlinks — one new dev-dep), or over a candidate list plus an `exists` predicate (no new dev-dep; symlink
  chains not exercised)? → blocks: plan-decision (it decides whether `crates/interpretation/Cargo.toml` is
  touched)
