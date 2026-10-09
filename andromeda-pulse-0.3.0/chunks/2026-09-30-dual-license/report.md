# Report — 2026-09-30-dual-license

**Chunk:** Dual license — MIT OR Apache-2.0: LICENSE-MIT + LICENSE-APACHE, every Cargo and npm manifest
**Date:** 2026-09-30T07:44:36Z
**Commits:** `7229bab chore(2026-09-30-dual-license): operator pre-CI commit` (since last_wrap 2026-09-30T06:38:38Z;
chunk base `1dfca7412b0026cfbb5221ebf460bb5665983f9e`)

## Changes (structured — detectors read this)
- **Files:** new `LICENSE-MIT` · new `LICENSE-APACHE` · new `xtask/src/license_check.rs` · `Cargo.toml` ·
  `pulse-app/ui/package.json` · `pulse-app/ui/package-lock.json` · `.github/workflows/update-channels.yml` ·
  `xtask/src/main.rs` · `pulse-app/tests/distribution_manifests.rs` (companion, scope-recorded). Basis:
  `git diff --name-only 1dfca74 -- . ':(exclude).andromeda' ':(exclude)andromeda-pulse-0.3.0'` (+ the 3 new files).
- **Symbols / APIs:** four test-only fns in the new `#[cfg(test)] mod license_check` (xtask binary crate):
  `license_texts_present_at_root` · `license_workspace_value_is_dual_and_inherited` ·
  `license_npm_manifest_and_lock_root_agree` · `license_channel_manifests_carry_dual_license`, plus private helpers
  `workspace_root` · `read` · `section_lines` · `workspace_members`. No public fn, IPC method, TauRPC procedure,
  endpoint, port, env var or xtask verb added or changed (the `mod` is `#[cfg(test)]`; no `Cmd` arm).
- **Crates / modules:** changed `xtask` (one test-only module). No crate added or removed; the 16 workspace members are
  unchanged.
- **Dependencies:** none added or bumped. `Cargo.lock` unchanged vs `1dfca74` (gate `git diff --name-only 1dfca74… --
  Cargo.lock deny.toml pulse-app/tauri.conf.json pulse-app/ui/npm-policy.json` printed nothing). The witness reads
  TOML as text and JSON via the existing `serde_json` dep.
- **Schema / config:** project license value is now the SPDX expression `MIT OR Apache-2.0`:
  - `Cargo.toml [workspace.package] license = "MIT OR Apache-2.0"` (was `"MIT"`); all 16 members inherit via
    `license.workspace = true`, no per-crate override (basis: the witness parses the `[workspace] members` list and
    asserts each manifest).
  - `pulse-app/ui/package.json` `"license": "MIT OR Apache-2.0"`; the lockfile ROOT entry `packages[""]` carries the
    same value — hand-edited, one line, no regen; the dependency entry at `node_modules/@asamuzakjp/css-color` untouched.
  - Generated channel manifests in `update-channels.yml`: Homebrew formula `license any_of: ["MIT", "Apache-2.0"]`
    (was `license "MIT"`); Scoop manifest `"license": "MIT|Apache-2.0"` (was `"license": "MIT"`). No action pin,
    `permissions:` block or step structure changed.
  - Tauri bundles: no `tauri.conf.json` edit; `bundle.license` defaults to the Cargo value (tauri-utils 2.9.0
    `config.rs:1633`, research).
  - Root texts: `LICENSE-MIT` (`Copyright (c) 2026 Turbolet85`, 25 lines) and `LICENSE-APACHE` (Apache License 2.0
    verbatim, 201 lines, the appendix template line left standard), byte-identical to Conductor `cdb7082`
    (md5 `0d9a205d780c21cbd7c042a596b7c446` / `1836efb2eb779966696f473ee8540542`), staged `i/lf w/lf`.
  - `deny.toml` unchanged: it sets no `private` key, so cargo-deny's default already license-checks all 16 own crates
    (research mutation probe: a scratch config without `"MIT"` rejected all 16, cargo-deny 0.20.2); both `MIT` and
    `Apache-2.0` sit in `[licenses] allow` (`deny.toml:80-81`), so the new expression passes (`bans ok, licenses ok,
    sources ok`).
- **Spec-master edits:** none during the chunk (all three expected amendments land at this wrap).
- **Counts / qualifiers moved:** workspace nextest 2429 → 2433 tests (+4, the witness; basis: implement gate logs,
  `2433 tests run: 2433 passed`). No master states a workspace or xtask test count (`grep -n -E '2429|2433|\| *`?xtask'
  test-plan.md` → 0 count hits; §4 has no xtask row). No other documented count moved — verified.
- **Dev-tool versions:** none — cargo-deny, nextest, npm unchanged this chunk.
- **Harness / gate surface:** no xtask verb, CI step or verdict shape added. The witness rides the standard
  `cargo nextest run --workspace` gate (and CI's lint-test job). The plan ran `cargo xtask capability-drift` BEFORE the
  default-features workspace nextest, then the `--features mcp-server` `emit_taurpc_bindings` regen as the last
  cargo-adjacent step, then a base-named probe `git diff --quiet 1dfca74… -- pulse-app/ui/src/bindings/index.ts`
  (exit 0 in both full implement runs) — an ordering that departs from test-plan §3's "`capability-drift` runs LAST"
  (`test-plan.md:304`, `:320`), per the overseer's founder-delegated directive of 2026-09-30 (not a founder ruling).
- **Cross-project / external claims:**
  - Conductor repo `D:/dev/projects/conductor` @ `cdb7082`: `LICENSE-MIT` / `LICENSE-APACHE` blobs read via
    `git cat-file blob`, md5 above, 0 CR bytes.
  - CI: `ci#36682995161` + `secret-scan#36682995178` on sha `7229bab71249…` (the pre-CI commit) — `verdict: green ·
    checks 13/13 · wall 1487 s` (`ci.py conclusion --sha HEAD --wait 2400`). This wrap's own commit adds to that tree
    (report, specs, state) with no source change.
  - `cargo xtask pre-push:linux` (WSL): `"verdict": "green"`, `reason: all-stages-ok`; stages script-modes 179 ms ·
    npm 17 095 ms · clippy 10 046 ms · test 24 265 ms · ci-gates 554 ms (sum 52.1 s); **invocation wall time 107.6 s**
    (task launch 07:16:34.8Z → exit 07:18:22.4Z) against the recorded 9-min warm baseline; cache 24.5 GB of its
    42.9 GB cap, not cleaned. The operator states the WSL VM runs capped at 20 GB since 2026-09-30 (operator's
    statement, not measured here). Its `tree` `1fa77a60…` equals `7229bab`'s tree minus only
    `evidence/operator-pass.md` (written after the run; measured by a scratch index `read-tree 7229bab` → `rm --cached`
    that file → `write-tree`).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - Chunk-artifact claim (research.md `## Files to modify` / plan Codebase touchpoints): the modify set was complete for
    the manifest change. Measured false by the first full implement gate run — `pulse-app/tests/distribution_manifests.rs
    ::update_channels_workflow_scoop_manifest_required_keys` pinned the substring `"license": "MIT"`, which
    `"MIT|Apache-2.0"` does not contain (workspace nextest `2432 passed, 1 failed`). Recorded in the report, no
    amendment owed (research is immutable; the fix rode as a scope-recorded companion).
- **Expected amendments (from plan):**
  - architecture §Established Decisions — new [License] decision → **carried**: Schema / config bullet (value, texts,
    manifests). Sites: `grep -n -i license architecture.md` → 1 hit (`:241`, the npm gate's "license" check — a
    different subject); `grep -c -w MIT architecture.md` → 0. No existing license statement; the decision is new.
  - security-plan §Dependency Security → CI integration — cargo-deny license-checks own crates; own expression in the
    allowlist → **carried**: Schema / config bullet (`deny.toml` sub-bullet). Sites: `grep -n -i license
    security-plan.md` → 5 hits (`:216` the CI-integration bullet — the target; `:217`, `:221`, `:251`, `:256` — gate
    wiring / npm channel / bootstrap phases, none states whether own crates are checked).
  - test-plan §3 Per-chunk gate discipline — the ordering note → **carried**: Harness / gate surface bullet. Sites:
    `grep -n capability-drift test-plan.md` → `:304` (`# LAST — see the ordering note below`) and `:320` (the ordering
    note), plus `:639` / `:686` (CI step order — ci.yml unchanged, not a target).
- **Coverage of new surfaces:**
  - `xtask license_check` (test-only module) → validation n/a · instrumentation n/a · PII n/a · tests unit (4, RED at
    base 4/4, GREEN after, mutation-checked 4/4) · a11y n/a · tokens n/a
  - root `LICENSE-MIT` / `LICENSE-APACHE` (static text) → all n/a · tests unit (`license_texts_present_at_root`)

## Deviations from intent
- One companion edit outside research's lists: `pulse-app/tests/distribution_manifests.rs` — its Scoop required-keys
  pin moved `"license": "MIT"` → `"license": "MIT|Apache-2.0"`. Justification: the test pins the changed artifact's
  DATA, so the manifest change reddens it by construction; the pin keeps asserting the Scoop key is present.
- Scope record (`gate.py scope`: `clean — changed 9 · listed 8 · recorded 1 (companion 1 · mechanical 0 · in-intent 0 ·
  widening 0)`):
  - companion — `pulse-app/tests/distribution_manifests.rs` · serves `.github/workflows/update-channels.yml` · self
- `xtask/src/main.rs`: the `#[cfg(test)] mod license_check;` sits in the alphabetical slot (after `ingest_progress`),
  not "beside the existing mod list :9-20" literally — within the listed file.

## Decisions & corrections
- Operator (take-up): MIT OR Apache-2.0, holder Turbolet85, Conductor `cdb7082` shape; diff-shaped probes name the
  chunk base (W182). The gate order — `capability-drift` BEFORE the default-features workspace nextest — is the
  OVERSEER directive (founder-delegated), 2026-09-30, not a founder ruling; it answers the every-chunk recurrence of
  `bindings/index.ts` being rewritten before `capability-drift` reads it. Ratified at this wrap into the playbook
  (rule :74 superseded, the order appended as a routine rule).
- Founder at P4 via overseer: the witness is an xtask test module, not a `check:license` verb.
- Operator (this wrap): record the pre-push:linux wall time (above) and carry the P-072 parse warning for the pipeline.
- Sweep hazard: a research "tests that PIN the changed artifact's DATA" sweep keyed on the manifest's own file names
  misses a test that pins a GENERATED artifact's text by substring — `distribution_manifests.rs` reads
  `update-channels.yml` and matched `"license": "MIT"` as a substring, so the pinned token was the old VALUE, not the
  file name. The pattern that finds it: grep the tests tree for the old value literal (`grep -rn '"license": "MIT"'
  pulse-app/tests xtask crates`).
- Host: the project's PreToolUse guard blocks a `cat >> file <<EOF` append to an evidence document; evidence appends
  go through the Edit tool.
- Pipeline (recorded, not actioned): `matrix.py show` prints `UNPARSED: P-072 — legacy notes placement
  (verification.notes); top-level notes is the convention` at `andromeda-pulse-0.3.0/verification-matrix.json:161`
  (P-072 is not this chunk's; logged as a `contract.grammar-irregularity` friction record at implement).

## Outcome
- Acceptance, re-asserted against the diff:
  - Witness RED at base (4/4 failing, Step 1) and GREEN after (`4 tests run: 4 passed`) — met
    (`evidence/witness-readings.md`).
  - Mutation check: (a) `LICENSE-APACHE` deleted, (b) `Cargo.toml` → `"MIT"`, (c) lock root → `"MIT"`, (d) Scoop →
    `"MIT",` — each reddened exactly its own test, restores sha256-verified, final GREEN — met.
  - `[workspace.package] license = "MIT OR Apache-2.0"`, every member inherits, no override — met.
  - Root texts byte-identical to Conductor `cdb7082`, LF — met (md5 gate; `i/lf w/lf`).
  - npm manifest + lock root agree; no other lockfile line changed — met (the diff is one line; `Cargo.lock` probe and
    `npm-policy.json` unchanged).
  - Homebrew / Scoop native dual syntax — met.
  - `cargo deny check bans licenses sources` exit 0; `deny.toml` unchanged — met.
  - `check:npm-supply-chain` exit 0; `npm-policy.json` unchanged — met.
  - Standard gate set in the directed order — met.
  - No coverage gate owed (xtask excluded; no threshold edited) — met by construction.
  - CI green on the pushed head, run id named — met: `ci#36682995161` on `7229bab`.
  - No capability claimed — met (`matrix.py show`: claimed 0).
- Gates (implement run 2, the final state; outcome words from `gate.py run`):
  - `cargo fmt --check` green · `cargo clippy --workspace --all-targets --all-features -- -D warnings` green ·
    `cargo nextest run --workspace --profile ci -E 'test(/license_check::/)'` green (`4 tests run: 4 passed`) ·
    `md5sum LICENSE-MIT LICENSE-APACHE` green · `cargo deny check bans licenses sources` green ·
    `cargo xtask check:npm-supply-chain` green · `git diff --name-only 1dfca74… -- Cargo.lock deny.toml …` green (no
    output) · `git diff --name-only 1dfca74… -- . ':(exclude)…'` recorded (roster: `.claude/session-handoff.md`,
    `update-channels.yml`, `Cargo.toml`, `distribution_manifests.rs`, `package-lock.json`, `package.json`,
    `xtask/src/main.rs`) · `cargo xtask capability-widening-check` green · `check:ingest-progress` green ·
    `check:staged-artifacts` green · `capability-drift` green · `cargo nextest run --workspace --profile ci` green
    (2433/2433; red in run 1 — the companion pin above, fixed) · mcp-server `emit_taurpc_bindings` regen green ·
    `git diff --quiet 1dfca74… -- pulse-app/ui/src/bindings/index.ts` green · `npm run lint` / `typecheck` / `test
    --prefix pulse-app/ui` green.
  - Operator legs (fired by the agent on the overseer's word, `evidence/operator-pass.md`): `gate.py hygiene` →
    `hygiene: clean` · `cargo xtask pre-push:linux` → `"verdict": "green"` · clean-tree guard + `git push origin
    chore/migrate-pulse-to-v3` → `1dfca74..7229bab` · `ci.py conclusion --sha HEAD --wait 2400` → `verdict: green ·
    checks 13/13`.
  - Smoke: skipped — no boot-path / UI-surface change.
- Watches: none folded.
- Outcome basis: the operator pass ran — `7229bab` (the only commit since `1dfca74`) and its CI run `ci#36682995161`
  recorded in `evidence/operator-pass.md`; implement's P4 report (this conversation) for the witness readings and the
  companion fix.
- Process hygiene: implement census — none started beyond the gate tool's own entries; no `pulse-app` / `cargo` /
  `node` / driver process running after P4 (PowerShell `Get-Process`). The pre-push WSL run exited 0.
