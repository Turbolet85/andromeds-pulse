# Report — 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts

**Chunk:** the L4 hardware probe reads GPU-present on a host whose libcuda.so lives directly under /usr/lib
**Date:** 2026-10-04
**Commits:** `7fc5fa2 chore(2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts): operator pre-CI commit, for the run this chunk's verdict reads` (since `last_wrap` 2026-10-04T15:04:55Z; basis `git log --format='%h %s' 03fb097..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `crates/interpretation/src/hardware.rs` · `crates/interpretation/Cargo.toml` · `xtask/src/source_lint.rs` ·
  `xtask/src/staged_gate.rs` · `Cargo.lock` (derived) — basis `git diff --name-only 03fb097 -- crates pulse-app xtask Cargo.lock`
  (5 paths); `gate.py scope` clean — changed 5 · listed 5 · recorded 0.
- **Symbols / APIs:** in `crates/interpretation/src/hardware.rs`, all PRIVATE (no new `pub` item, `HardwareProfileDetector`'s
  public surface unchanged):
  - `const CUDA_PROBE_DIRS: &[&str]` = `usr/lib/x86_64-linux-gnu` · `usr/local/cuda/lib64` · `usr/lib` · `usr/lib64`
    (root-relative; the first two are the pre-chunk paths);
  - `const CUDA_PROBE_NAMES: &[&str]` = `libcuda.so` · `libcuda.so.1`;
  - `fn cuda_driver_present_under(root: &Path) -> bool` — true when any `root/dir/name` exists (`Path::exists`, follows
    symlinks, so a dangling link reads absent);
  - all three carry `#[cfg_attr(not(target_os = "linux"), allow(dead_code))]` — no blanket allow; on Linux the arm's call
    keeps them live;
  - `detect_gpu_present`'s Linux arm is now exactly `cuda_driver_present_under(Path::new("/"))` (8 candidates, any present →
    GPU-present). Its only caller stays `detect_real` (`hardware.rs`, code-graph `calls` row, research.md §Graph impact).
    The macOS / Windows arms, `detect_real`, `profile_label`, `HardwareProfile`, the override and the three
    `interpretation.hardware.detect` emit sites are byte-unchanged.
  - No IPC method, endpoint, port, env var, capability, topic or table added or changed.
- **Crates / modules:** none added or removed; `interpretation` changed (above).
- **Dependencies:** `tempfile.workspace = true` added to `crates/interpretation` `[dev-dependencies]` — a dev-dep only, the
  package was already in the lock (`git diff 03fb097 -- Cargo.lock | grep -c '^+name = '` → 0; the lock gains one dependency
  line under `interpretation`).
- **Schema / config:** none.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** workspace nextest 2601 → 2614 (+13: 8 rstest cases + 5 pins; basis the gate entry
  `cargo nextest run --workspace --profile ci` → `2614 tests run: 2614 passed, 0 skipped`, and stage 5 of the operator
  pass the same). `interpretation` + `xtask` 340 → 353. Docs stating a workspace test count: the handoff only (P6
  rewrites it). `hardware.rs` co-located tests 10 → 16 test functions (`grep -c '#\[test\]\|#\[rstest::rstest\]'` → 16), 10 → 23 run cases (the rstest fn expands to 8).
- **Dev-tool versions:** none.
- **Harness / gate surface:** two comment lines `// andromeda:walks-tree` — one in the `#[cfg(test)] mod tests` of
  `xtask/src/source_lint.rs`, one in that of `xtask/src/staged_gate.rs` (founder ruling D-3(a), registry U40, relayed by
  the overseer 2026-10-04). Comment only; the gate tool's run header now reads `walk-class rust 2
  (xtask/src/source_lint.rs, xtask/src/staged_gate.rs)`. No xtask verb, CI step or verdict shape changed.
- **Cross-project / external claims:** CI run ci#37220563721 (+ secret-scan#37220563681) on `7fc5fa2` — `verdict: green ·
  checks 13/13 · wall 1645 s` (`ci.py conclusion --sha HEAD --wait 2400`; recorded in `evidence/operator-pass.md`). The
  verdict was taken on `7fc5fa2`; this wrap's commit adds to it.
- **Reverted / negative API facts:** none (Step 6's three mutations were one-shot controls, applied and restored; not a
  shipped surface).
- **Insufficient fixes (written, kept, not the remedy):** none. Scope note, not a defect of this chunk: on this host L4 real
  mode now routes to `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` (`-ngl 99`); no Linux llama.cpp CUDA binary exists on the host
  yet (overseer measurement 2026-10-04, wrap directive), so a real-model run here needs that binary — owned by the route
  entry "L4 interpretation names its triggering cue" (see Decisions).
- **Spec claims disproved by measurement:** `.claude/docs/services/interpretation.md:26` states the GPU probe is
  "dlopen-shaped"; it is an `exists()` presence check with no `dlopen` anywhere in `hardware.rs` (measured at P3, research.md
  §Sweep; re-read at HEAD). A LEAF doc, not a master — disposition: the cascade's leaf re-derive of
  `docs/services/interpretation.md` at this wrap. No master states the probe's path set (`grep -c libcuda` over the seven
  masters → 0 each).
- **Expected amendments (from plan):**
  - architecture §Occupied Resources → Environment variables — register the product-consumed
    `ANDROMEDA_PULSE_HARDWARE_PROFILE` override (`hardware.rs:26` `ENV_HARDWARE_PROFILE_OVERRIDE`, read at `new()`; accepts
    the four profile labels snake- or kebab-case; invalid → WARN on `interpretation.hardware.detect` + real detection) —
    not carried by a change of this chunk (the var is unchanged), its fact stated here: `grep -c HARDWARE_PROFILE
    .andromeda/architecture.md` → 0; the registry block starts at `architecture.md:220` (`Environment variables
    (reserved at arch level)`), siblings `:229` (`…_LLAMA_CUDA_BIN_PATH`) and `:236` (`…_L4_DETERMINISTIC`).
  - security-plan §Input Validation → the `CLI / env var inputs` row (`security-plan.md:138`) — the same registration;
    `grep -c HARDWARE_PROFILE .andromeda/security-plan.md` → 0. The var's validation: `trim` + lowercase + a closed match
    over four labels (`parse_profile_override`), anything else rejected to real detection; never logged by value.
  - test-plan §4 `interpretation crate` "What unit tests cover" bullet (`test-plan.md:385`, currently the citable-evidence
    prompt surface only) — extend to the hardware-probe candidate-set pins: the six `cuda_probe_` pins (8-case rstest over
    dirs × names, empty root, unprobed names/dirs, the Arch symlink chain, a dangling link, the exact-set pin), TempDir
    fixtures per test-plan.md:614; `grep -n 'interpretation crate' .andromeda/test-plan.md` → 1 hit.
- **Coverage of new surfaces:**
  - `cuda_driver_present_under` (private Linux CUDA presence probe over compile-time root-relative paths) → validation n/a
    (no external input; no env, no subprocess, no dlopen — the source probe over added non-comment lines → 0) ·
    instrumentation n/a (no new emit; the existing `interpretation.hardware.detect` record carries `gpu_available`
    unchanged; the print-site probe → 0) · PII n/a (no path logged) · tests unit (6 pins / 13 cases; RED 8/13 before the
    widening; mutation-checked) · a11y n/a · tokens n/a

## Deviations from intent
- Step 6(c)'s mutation ran `cargo clippy -p interpretation --all-targets --all-features -- -D warnings`, not the workspace
  form the plan names — the dead-code finding is local to that crate's lib target, so both forms reach the same error
  (RED: three `never used` errors). Recorded in `evidence/mutation-checks.md`.
- The Linux arm's comment is ASCII ("Heuristic - false negatives OK"); the pre-chunk comment carried a non-ASCII dash.
  `check:english-sources` clean.
- scope record: none — `gate.py scope` clean, 0 recorded (P1 read: changed 5 · listed 5 · excluded 35).

## Decisions & corrections
- Candidate set "dirs × names" — ruled at P4 by the overseer, founder-delegated, 2026-10-04 (plan Provenance).
- Walk-class marks — founder ruling D-3(a), registry U40, relayed by the overseer 2026-10-04. Plan Implementation notes
  record, for the operator's ruling, a genuine real-tree walker the ruling did not name:
  `pulse-app/tests/unit_observability_allowlist_sweep.rs` reads `CARGO_MANIFEST_DIR/src` (the dead-test ratchet) and is
  NOT marked — out of this chunk's directive; open for the operator.
- Operator directive (overseer, founder-delegated, 2026-10-04): implement verified against the tree; run the operator pass
  22–32; stop before the wrap.
- Wrap directive (overseer, measured 2026-10-04 ~19:30): `AI-Model/` is restored in the tree from the backup disk
  (`Llama-3.2-3B-Instruct-Q4_K_M.gguf`, sha256 `6c1a2b41…28ff`, equal to
  `.andromeda/runs/2026-05-24T19-00-12-step0-spike/spike-result.md:7`), plus `experiments/` and
  `andromeda-pulse-0.4.0-incubator/`, all gitignored; what still blocks the entry "L4 interpretation names its triggering
  cue" (`working-route.md:162`) is a Linux llama.cpp CUDA binary — none on this host yet (founder desk act pending).
  Carry that as the entry's block wording at the route resolve.
- Sweep hazard: none new.

## Outcome
- Acceptance re-asserted against the diff:
  - (tests) each of the 8 dirs × names candidates reads present; empty root and `libcudart.so` / `libcuda.so.2` /
    `opt/cuda/libcuda.so` read absent — MET (`cuda_probe_` entry green, collection proof green, `evidence/red-before-green.md`
    8 failed / 5 passed on the base set, incl. `case_5_usr_lib_so`).
  - (tests) the Arch symlink chain reads present, a dangling link absent — MET.
  - (tests) the production set is pinned exactly; mutations (a) 4 RED, (b) 5 RED, (c) clippy RED (the arm-wiring guard
    holds) — MET (`evidence/mutation-checks.md`).
  - (arch) the dev-host routing reading flips — MET: `l4-decision-probe: INCONCLUSIVE - ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH is
    unset` exit 2 (base read `…_CPU_BIN_PATH…`), produced in this run after its example build.
  - (security) no subprocess / dlopen / env read or write / new print site in added lines; no new lockfile package — MET.
  - (arch) confined to the four touchpoints + the derived lock line; no new pub item / procedure / env var / capability /
    topic; capability-drift and check:staged-artifacts clean; bindings byte-identical to the base — MET.
  - (tests) both walk-class marks present — MET.
  - (tests) the full standard gate set green in order, workspace 2614 measured (predicted 2614) — MET.
  - (ci) CI read of the pushed HEAD `verdict: green`, run ci#37220563721 — MET.
- Gates (implement run `.andromeda/runs/2026-10-04T17-17-21Z-implement`, block 32 entries · green 21 · red 0 · not-run 11):
  `cargo fmt --check` green · `cargo clippy --workspace --all-targets --all-features -- -D warnings` green ·
  `cargo nextest list … 'test(/cuda_probe_/)'` green (all six contains atoms) · `cargo nextest run … 'test(/cuda_probe_/)'`
  green (13 run, 13 passed) · `… 'package(interpretation) | package(xtask)'` green (353) · `cargo build -p pulse-app
  --example l4_decision_probe` green · the `l4_decision_probe --arms A0 --n 1` reading green (exit 2 · contains `…CUDA_BIN_PATH
  is unset`) · the scope-guard `git diff --name-only 03fb097…` green (no output) · the lockfile `+name =` probe green
  (exit 1, 0) · the print-site probe green (exit 1, 0) · the subprocess/env probe green (exit 1, 0) · `git grep -l -F
  'andromeda:walks-tree' -- xtask/src` green · `cargo xtask check:english-sources` green · `capability-widening-check` ·
  `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` · `verify:capability-matrix` green ·
  `cargo nextest run --workspace --profile ci` green (2614) · the mcp-server bindings regen green · the bindings base
  `git diff --quiet` green. No `defer` entry; no deferral. The operator entries (hygiene, six native pre-push stages,
  regen + close re-fired after stage 5, push, CI read) were driven by hand: all exit 0 / atoms held, recorded in
  `evidence/operator-pass.md`. Smoke: skipped — no boot-path or UI-surface touchpoint; the detector → routing equality ran
  as the e2e gate entry.
- Watches: none folded.
- Outcome basis: the operator pass ran — its final HEAD `7fc5fa2` (the one pre-CI commit, `git log 03fb097..HEAD`) and that
  HEAD's CI run ci#37220563721 green, recorded in `evidence/operator-pass.md`; implement's P4 report (this conversation)
  for the gate block, RED and mutation readings.
- Process hygiene: implement's census — cargo/nextest/clippy runs terminated; `l4_decision_probe` exited 2 (binds no
  port); no pulse-app / model launched; 4317/4318 not listening. The operator pass started cargo, npm and one
  `ci.py` poller, all ended (the background tasks reported completion). Re-measured at this wrap's start: no cargo / rustc /
  nextest process from this chunk's runs remained.
