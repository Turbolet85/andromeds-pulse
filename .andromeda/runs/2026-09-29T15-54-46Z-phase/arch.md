# arch extract

## Relevance
relevant: arch owns the CI/CD approach, the CI task-runner decision, the formalized xtask/agent-run CLI registry the pre-push check joins, and the CARRY E registration target.

## Constraints
- Per architecture §Established Decisions [CI Task Runner], shared CI/dev-loop logic goes in `cargo-xtask` (`just` and `cargo-make` were rejected as extra DSL surface). Per §Infrastructure Patterns → CI/CD approach, "all shared CI logic that needs Rust lives in the `xtask` crate so contributors can run identical commands locally". So the WSL pre-push check (D) must be an xtask verb or ride the existing `scripts/agent-run.{sh,ps1}` driver pair. It must not be a free-standing script. Its name and placement are still P4 decisions.
- architecture §Occupied Resources → xtask CLI surfaces applies the 2026-08-25 formalized-CLI-contract rule. Every dev/CI verb gets registered with an exit-code contract (0 green · 1 red · 2 cannot-evaluate), named verdict arms and one machine-readable verdict on stdout. Any new pre-push verb falls under this rule. The registry today names only `smoke:hue-shift` among the scenario legs. CARRY E adds `smoke:gap-resume` and `smoke:external-resolve` beside it. Research has to check whether those two verbs actually exist in `xtask/src/main.rs` at HEAD.
- architecture §Occupied Resources → xtask CLI surfaces (the `scripts/agent-run.{sh,ps1}` contract) sets these rules:
  - `boot` pre-builds `cargo build --bin pulse-app --release`, then `cargo build -p xtask`, and spawns the binary by path.
  - ci.yml's Linux-only "Boot pulse-app smoke" step stays GATING (`continue-on-error` dropped).
  - The three invocations run inside one `xvfb-run` and share `ANDROMEDA_PULSE_DATA_DIR`.
  - Every job exports that variable to `$GITHUB_ENV` from a step right after harden-runner, because `runner.*` is unavailable in workflow- and job-level `env:`.

  Every job created by the split (A) inherits that last requirement. Whether the release-profile build in boot and the separate `release 26` CI step compile the same artifact is research's question (C).
- Per architecture §Conventions → Feature flags and §Cross-cutting Patterns → Feature-gate hygiene, `mcp-server` stays off by default. Removing the `mcp build` duplicate (C) must not turn the feature on by default. Per §Cross-cutting Patterns → Webview IPC capability policy, `capability-drift` diffs `EXPECTED_PROCEDURES` against both the worktree and the staged bindings. Any change to how the mcp-feature build runs must still leave `mcp.*` in the bindings shape that the drift gate reads in whichever job it lands.
- architecture §Infrastructure Patterns → Build system pins these gates:
  - Lint: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
  - Typecheck: `tsc --noEmit`.
  - The distribution release profile: `lto = "thin"`, `codegen-units = 1`, `strip = true`.

  Speed work may reorganise where these gates run. It must not redefine them or change the distribution release profile.
- architecture §Infrastructure Patterns → CI/CD approach describes `ci.yml` as a single matrix chain: `fmt → clippy → xtask test → cargo build --workspace (release profile smoke)`. After the split (A) that text describes something that no longer exists. The update is owed as a wrap amendment; phase and implement do not edit arch.
- Per architecture §Occupied Resources → Network ports and → Environment variables (`ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `_HTTP_PORT`), and §Cross-cutting Patterns → Test-time telemetry injection, any boot leg the WSL check runs binds `127.0.0.1` only. Where the fixed `:4317`/`:4318` would collide (host constraint), it moves ports through the registered override variables. Per §Occupied Resources → Process / service identity, the dev-build binary it launches is `target/{profile}/pulse-app` in the distro's own target dir, never `target/{profile}/andromeda-pulse`.

## Patterns to follow
- The formalized-CLI-contract shape shared by `check:npm-supply-chain`, `harness:status`, `check:staged-artifacts` and `smoke:hue-shift` (architecture §Occupied Resources → xtask CLI surfaces). Its parts: exit arms 0/1/2, named verdict arms, one pretty-JSON verdict on stdout, and an optional report twin under `target/<name>/`.
- Wire a gate into ci.yml as a plain named `run:` step with no new third-party action, following the `check:staged-artifacts` precedent (architecture §Occupied Resources → xtask CLI surfaces).
- Driver pairs change in lockstep: any new verb or flag on `scripts/agent-run.sh` gets its `agent-run.ps1` twin (architecture §Occupied Resources → xtask CLI surfaces, agent-run contract).
- `capability-drift` stays the gate that runs last in its job, because it folds in the staged assertion and relies on that slot (architecture §Occupied Resources → xtask CLI surfaces, `check:staged-artifacts` entry).
- Agent-driven harness ergonomics: deterministic invocation, machine-parseable output, schema-stable contract (architecture §Cross-cutting Patterns → Development Style). The pre-push verdict should be readable by an agent without scraping the log.

## Anti-patterns to avoid
- No ad-hoc pre-push script outside the xtask / agent-run surfaces, and no new task-runner DSL (architecture §Established Decisions [CI Task Runner]).
- Do not make `mcp-server` a default feature to collapse builds (architecture §Conventions → Feature flags: "default features are minimal").
- No boot leg binds `0.0.0.0` or any non-loopback interface (architecture §Occupied Resources → Network ports).

## Contract bindings
- arch §Occupied Resources → xtask CLI surfaces ↔ tests: test-plan registers `smoke:gap-resume` / `smoke:external-resolve` / `smoke:hue-shift`, and arch registers only the last. CARRY E closes this at wrap. Implement makes sure the xtask verbs exist.
- arch agent-run contract (`boot`/`status`/`cleanup`, `harness:status` verdict) ↔ obs + tests: the Linux boot smoke step and its failure diagnosis are the WATCH instrument. Moving the step into its own job must keep the step intact.
- arch §Cross-cutting Patterns → Webview IPC capability policy ↔ security: `capability-drift` / `check:staged-artifacts` and the mcp-feature bindings shape must survive the rearrangement into parallel jobs.
- arch §Infrastructure Patterns → CI/CD approach ↔ security §Supply chain + CI: every new job needs SHA-pinned actions, `harden-runner` as its first step, and workflow-level `permissions: contents: read`.
- arch §Occupied Resources → Environment variables ↔ security path-env carve-out: a new harness-only variable (e.g. a WSL distro or tool locator) is registered as harness-only, not consumed by the production binary, and guarded under the carve-out shape.

## Acceptance criteria contributions
- The new pre-push surface has an exit-code contract (0/1/2), named verdict arms and one machine-parseable stdout verdict, and exists as an xtask verb or an agent-run verb, so wrap can register it (per architecture §Occupied Resources → xtask CLI surfaces; §Established Decisions [CI Task Runner]).
- `smoke:gap-resume` and `smoke:external-resolve` are real xtask `Cmd` registrations at the chunk's HEAD, so the wrap amendment that lists them beside `smoke:hue-shift` describes existing verbs (per architecture §Occupied Resources → xtask CLI surfaces).
- Every CI job that runs the agent-run harness exports `ANDROMEDA_PULSE_DATA_DIR` via `$GITHUB_ENV` from a step (never via `runner.*` in workflow- or job-level `env:`). The Linux "Boot pulse-app smoke" step stays gating, with its three invocations inside one `xvfb-run` (per architecture §Occupied Resources → xtask CLI surfaces, agent-run contract).
- The production binary reads no new environment variable. Any variable the chunk adds is registered as harness-only (per architecture §Occupied Resources → Environment variables).
