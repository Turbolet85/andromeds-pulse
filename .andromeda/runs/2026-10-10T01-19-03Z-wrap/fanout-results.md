# Fan-out results — 2026-10-09-pre-push-check-native-on-linux

Seven doc-agents, one batch. Each return was taken from the agent's own hand-back by script, entity-decoded
(no return changed under the decode; the `entities=0` probe read 0 on all seven), and parsed. 19 proposals:
architecture 9 · security-plan 3 · test-plan 7 · obs-plan 0 · design-system 0 · layout-templates 0 · a11y-plan 0.
Rejected for a source the report does not carry: 0.

## architecture — 9 proposal(s)

Verdict: 9 proposals parsed. Stripped: comment lines stripped: D-arch-decisions no drift; D-arch-resources drift inside one registry row plus two unregistered env-var rows; the sweep found every hit of the retired wording in that row, bar the check:english-sources row's mention of the source-lint stage, which stays true.

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates) → the `cargo xtask pre-push:linux` row, sentence «Windows host only: every distro call is `wsl.exe -d Ubuntu --exec /usr/bin/env -i HOME=… PATH=…` (plus the stage's data dir), so nothing of the host environment crosses.»"
    change: >-
      Linux host only: the host guard (`host_supported(std::env::consts::OS)`) makes any other system read `cannot-evaluate` / `not-linux`. Every probe and stage child is spawned with a CLEARED environment plus exactly `HOME` (the developer's home) · `PATH` (`{HOME}/.cargo/bin`, then the directory of the first `node` file found in an absolute directory of the verb's own PATH — left out when there is none — then `/usr/local/bin:/usr/bin:/bin`) · `ANDROMEDA_PULSE_DATA_DIR` (`target/pre-push/run/data` under the repository root, absolute) · for the `npm` stage only `PUPPETEER_CACHE_DIR` (`target/pre-push/run/puppeteer`, absolute) · for the three git calls that compute `head` and `tree` only `GIT_INDEX_FILE` (`target/pre-push/run/index`); `DBUS_SESSION_BUS_ADDRESS`, `XDG_RUNTIME_DIR`, `DISPLAY` and `WAYLAND_DISPLAY` are not in the set (pinned by set equality), so no stage reaches an OS credential store.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — xtask CLI surfaces, `pre-push:linux` row: "Windows host only … wsl.exe" → Linux host only (host guard, `not-linux`), stage children under a cleared, constructed environment.
    rationale: >-
      Report Changes → Symbols/APIs "The host guard: the verb evaluates on Linux alone … any other system reads cannot-evaluate / not-linux", "Removed: the constants WSL, DISTRO … the Linux struct and its wsl.exe command wrapper", and "Environment variables" (the cleared set, by set equality); Counts "Where the check can run: a Windows host with the WSL Ubuntu distro → the Linux dev host"; Spec claims disproved 1 names this row ("Windows host only") as measured false by two green runs on the dev host.
    basis: ".andromeda/architecture.md:250 (the retired sentence) · xtask/src/pre_push.rs:134-138 (host_supported) · xtask/src/pre_push.rs:349-368 (stage_env)"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates) → the `cargo xtask pre-push:linux` row, lead clause «`cargo xtask pre-push:linux` — the WSL Linux pre-push verb implemented at `xtask/src/pre_push.rs` (chunk 2026-09-29-ci-wall-time-and-round-trips; registered per the same formalized-CLI-contract rule)»"
    change: >-
      `cargo xtask pre-push:linux` — the native Linux pre-push verb implemented at `xtask/src/pre_push.rs` (chunk 2026-09-29-ci-wall-time-and-round-trips; runs on the Linux dev host itself since chunk 2026-10-09-pre-push-check-native-on-linux, which removed the `wsl.exe` hop and its distro clone; registered per the same formalized-CLI-contract rule).
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — xtask CLI surfaces, `pre-push:linux` row lead: "the WSL Linux pre-push verb" → the native Linux pre-push verb.
    rationale: >-
      Same retired claim (the check runs through WSL) restated in the row's lead. Report header: "the surviving stages run on the dev host; the wsl.exe hop and its distro clone leave"; Outcome 1: "The two files that held the hop contain no `wsl` in any letter case"; Harness / gate surface: the verb and its `about` text changed.
    basis: ".andromeda/architecture.md:250 · xtask/src/main.rs:263 (the `about` string)"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates) → the `cargo xtask pre-push:linux` row, sentence «Its pins are read from the repo — the `rust-toolchain.toml` channel, ci.yml's `node-version` major, ci.yml's first `apt-get install` list — and any missing tool or package reads `cannot-evaluate` with `missing[]` named and one `sudo apt-get install` remediation line for the apt-installable ones (the verb never escalates).»"
    change: >-
      Its two pins are read from the repo — the `rust-toolchain.toml` channel and ci.yml's `node-version` major (either unreadable reads `pins-unreadable`) — and a missing piece reads `cannot-evaluate` / `provisioning-missing` with `missing[]` naming it: `rust:{channel}` (the pinned channel absent from `rustup toolchain list`; no install hint), `rust:clippy` and `cargo-nextest` (`cargo +{channel} clippy --version` / `cargo +{channel} nextest --version`, probed only when the channel is listed, because a `cargo +{channel}` call against an absent channel could install it), `node:{pin} (found {version})` (`none` when no `node` answered, `unreadable` when the output is not a short version token), `tool:npm`, `tool:git`, `tool:cc`, `tool:python3` (`python3` else `python` printing `Python 3`). The verb installs nothing and prints no install command; the ci.yml apt-list read, the `dpkg-query` probe, `apt:{package}`, `tool:jq`, `tool:xvfb-run` and the `sudo apt-get install` line are gone.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — xtask CLI surfaces, `pre-push:linux` row: provisioning is native (two repo pins, per-tool probes, `missing[]` names); the apt-list pin and the apt remediation line retired.
    rationale: >-
      The apt list and the `sudo apt-get install` line are the distro mechanism restated. Report Changes → "The provisioning check, native" (the required pieces and their `missing[]` names; "Gone: the read of ci.yml's apt list, the dpkg-query probe, apt:{package}, tool:jq, tool:xvfb-run, the one sudo apt-get install line. The verb installs nothing and prints no install command"); "The order of a run" (two pins; either unreadable is `pins-unreadable`); Removed: `apt_packages`, `installed_packages`, `remediation`; Deviation 3 (accepted by the operator).
    basis: ".andromeda/architecture.md:250 · xtask/src/pre_push.rs:402-413 (Probes) · xtask/src/pre_push.rs:437-441 (channel_listed) · xtask/src/pre_push.rs:477-489 (found_version) · xtask/src/pre_push.rs:491-495 (provisioning_stop)"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates) → the `cargo xtask pre-push:linux` row, sentence «It syncs a distro clone `~/andromeda-pulse-pre-push` to HEAD plus the working tree as one binary patch through a temporary `GIT_INDEX_FILE` (the real index untouched), verified by tree-id equality (`sync-mismatch` → red), caps the clone's `target/` at 40 GiB, and runs `script-modes` · `source-lint` (`cargo xtask check:english-sources` in the clone; …) · `npm` · `clippy` · `test` · `ci-gates` in order — six stages — stopping at the first failure.»"
    change: >-
      It runs in the working tree, in this order: host guard → the two pins → `HOME` → the provisioning probes → the per-run area `target/pre-push/run/` (`data/`, `puppeteer/`, the temporary index — the verb's own; it touches nothing else under `target/pre-push/` but the report twin, and never `~/.cache/puppeteer`) removed and created again, AFTER the provisioning check passes, so a `provisioning-missing` run rewrites only the report twin → `head` and `tree` through the temporary `GIT_INDEX_FILE` → `script-modes` (`git ls-files -s scripts/agent-run.sh`, green on mode `100755`) · `source-lint` (`cargo xtask check:english-sources`; since chunk 2026-10-01-real-model-incident-surfacing) · `npm` (`npm ci`, then `npm run build`, in `pulse-app/ui`) · `clippy` · `test` (`cargo xtask test`) · `ci-gates` (the data dir removed and created again holding only the seed log, then `cargo xtask ci-gates`) in order — six stages, each output on stderr — stopping at the first failure. The `test` stage runs inside `restoring`, which puts the tracked `pulse-app/ui/src/bindings/index.ts` back as it found it whichever way the stage ended (a file absent before is removed again; a failed restore makes the stage red, `restore-failed:bindings`); the restore runs when the stage's command returns, not on a signal, so a verb killed mid-`test` leaves the bindings rewritten. No clone, no `target/pre-push/index`, no `target/pre-push/tree.patch`, no 40 GiB cap, no `sync-failed:{step}` / `sync-mismatch` reason.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — xtask CLI surfaces, `pre-push:linux` row: the distro-clone sync, its tree-id check and the 40 GiB cap retired; stages run in the working tree over the per-run area `target/pre-push/run/`, reset after provisioning; the bindings restore registered.
    rationale: >-
      Three uses of "clone" in this sentence restate the retired WSL mechanism (report Expected amendment 1 counts them). Report Changes → Removed (`CLONE_DIR`, `CACHE_CAP_BYTES`, `sync` and the patch file, `cache`), "Filesystem" (per-run area; no longer written: `target/pre-push/index`, `target/pre-push/tree.patch`, the clone with its 40 GiB cap), "The order of a run", "The stages, native" ("each run in the working tree"), "The bindings restore", reasons "Gone: … sync-failed:{step}, sync-mismatch"; Deviation 2 (reset after provisioning, accepted by the operator into the amended rows).
    basis: ".andromeda/architecture.md:250 · xtask/src/pre_push.rs:863-879 (reset_empties_the_per_run_area_and_nothing_beside_it) · xtask/src/pre_push.rs:881-894 (six_stages_in_the_registered_order) · xtask/src/pre_push.rs:535-546 (the restore)"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates) → the `cargo xtask pre-push:linux` row, sentence «Cross-project fragility: its Node 24 comes from the Viola repo's `~/.local/viola-node/bin` install in the distro (the distro's apt ships Node 22 / npm 9, as measured 2026-09-29) — if that install goes, the verb reads `cannot-evaluate`, never green.»"
    change: >-
      Node provisioning (the cross-project fragility is retired — the Viola repo's Node install inside the WSL distro is no longer read by anything in this repository): the stage Node is the first `node` found in an absolute directory of the verb's own PATH, and its major must equal ci.yml's `node-version` pin, else the verb reads `cannot-evaluate` (`provisioning-missing`, `node:{pin} (found {version})`), never green. On the dev host Node 24 is a user-level `mise install node@24` (v24.21.0 with npm 11.19.0, installed 2026-10-10) beside the host default v26.8.2 that `command -v node` still resolves, so the green readings start the verb with the Node 24 directory first on PATH.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — xtask CLI surfaces, `pre-push:linux` row: the Viola-repo/distro Node dependency retired; Node comes from the verb's own PATH and must match ci.yml's major (dev host: mise Node 24 beside default v26.8.2).
    rationale: >-
      "install in the distro" restates the retired WSL mechanism and names a dependency the verb no longer has. Report Changes → Dev-tool versions ("Node v24.21.0 with npm 11.19.0 INSTALLED 2026-10-10T00:40Z by mise install node@24 … beside the host's default v26.8.2 … still what command -v node resolves … The Viola repository's Node install inside the WSL distro is no longer read by anything in this repository"); Environment variables (the PATH rule); Outcome 1 ("started with Node 24 first on PATH") and 3 (no Node on PATH → `provisioning-missing`, `node:24 (found none)`, exit 2).
    basis: ".andromeda/architecture.md:250 · xtask/src/pre_push.rs:333-347 (stage_path) · xtask/src/pre_push.rs:328-331 (dir_holding)"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates) → the `cargo xtask pre-push:linux` row, sentence «Contract: exit 0 green · 1 red · 2 cannot-evaluate; one pretty-JSON verdict on stdout `{verdict, reason, head, tree, stages[{name, ok, ms}], missing[], remediation, cache{bytes, cap, cleaned}}` plus a twin at `target/pre-push/report.json`.»"
    change: >-
      Contract: exit 0 green · 1 red · 2 cannot-evaluate; one pretty-JSON verdict on stdout `{verdict, reason, head, tree, stages[{name, ok, ms}], missing[]}` — six members, pinned by set equality in a test; no member carries a value of an environment variable or a path — plus a twin at `target/pre-push/report.json`. Reasons, complete: `all-stages-ok` · `not-linux` · `pins-unreadable` · `home-unset` · `provisioning-missing` · `run-dir-unusable` · `tree-unreadable` · `stage-failed:{stage}` · `restore-failed:bindings`.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — xtask CLI surfaces, `pre-push:linux` row: verdict document 8 → 6 members (`remediation` and `cache{bytes, cap, cleaned}` left); the complete reason set registered.
    rationale: >-
      Report Changes → Schema / config ("Members now, by set equality in a test: verdict, reason, head, tree, stages[{name, ok, ms}], missing[] — six. remediation and cache{bytes, cap, cleaned} left with the apt line and the clone. No member carries a value of an environment variable or a path"); Counts "The verdict document's members: 8 → 6 (architecture §Occupied Resources → xtask CLI surfaces … print[s] the eight-member form)"; "The verdict's reasons, complete" (four new, five gone); the three exits and `Verdict` enum kept.
    basis: ".andromeda/architecture.md:250 · xtask/src/pre_push.rs:937-950 (the_verdict_document_has_exactly_six_members)"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates) → the `cargo xtask pre-push:linux` row, sentence «It binds no port, starts no `pulse-app` and reads no new env var.»"
    change: >-
      It binds no port, starts no `pulse-app`, and adds no TauRPC procedure, capability or tracing target; it READS `HOME` and `PATH` from its own environment by value to build the stage environment (an unset or empty `HOME` reads `cannot-evaluate` / `home-unset`) and SETs `PUPPETEER_CACHE_DIR` for the `npm` stage — neither value is printed or written (§Environment variables).
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — xtask CLI surfaces, `pre-push:linux` row: "reads no new env var" retired; the verb reads `HOME` and `PATH` by value and sets `PUPPETEER_CACHE_DIR` for one stage.
    rationale: >-
      Report Spec claims disproved 2: "'It … reads no new env var' (.andromeda/architecture.md:250, the same row). The native verb reads HOME and PATH by value and sets PUPPETEER_CACHE_DIR for one stage"; Changes → Environment variables; "No port, no TauRPC procedure, no capability, no tracing target"; Deviation 1 (`home-unset`, accepted by the operator).
    basis: ".andromeda/architecture.md:250 · xtask/src/pre_push.rs:153-155 (the `HOME` read) · xtask/src/pre_push.rs:361-366 (the `npm` arm of stage_env)"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables (reserved at arch level) — NEW row, beside the other harness-only rows"
    change: >-
      `PUPPETEER_CACHE_DIR` — harness-only, SET by `cargo xtask pre-push:linux` (`xtask/src/pre_push.rs`; chunk 2026-10-09-pre-push-check-native-on-linux) into its `npm` stage child's environment only, as the absolute `target/pre-push/run/puppeteer` under the repository root, so the stage never touches `~/.cache/puppeteer`; no product code reads it. The same verb SETs `GIT_INDEX_FILE` (`target/pre-push/run/index`) for the three git calls that compute `head` and `tree` only, and `ANDROMEDA_PULSE_DATA_DIR` (`target/pre-push/run/data`, absolute) for every probe and stage child.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — Environment variables: `PUPPETEER_CACHE_DIR` registered (harness-only, set by `pre-push:linux` for its `npm` stage; not product-read).
    rationale: >-
      An env var the chunk lands that the registry does not hold. Report Changes → Environment variables ("for the npm stage only, PUPPETEER_CACHE_DIR (target/pre-push/run/puppeteer, absolute) … a variable no product code reads; grep -r PUPPETEER_CACHE_DIR over the seven masters and .andromeda/registries/: 0 hits") and Filesystem ("never ~/.cache/puppeteer"); Expected amendment 2: "0 hits in any master or key file, so the row is new".
    basis: "xtask/src/pre_push.rs:361-366 (the `npm` arm of stage_env) · xtask/src/pre_push.rs:349-368 (stage_env)"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables (reserved at arch level) — NEW row, beside the `DISPLAY` · `DBUS_SESSION_BUS_ADDRESS` · `XDG_RUNTIME_DIR` system-variable row"
    change: >-
      `HOME` · `PATH` — SYSTEM variables, NOT `ANDROMEDA_PULSE_*` inputs, read harness-only and by value by `cargo xtask pre-push:linux` (`xtask/src/pre_push.rs`; chunk 2026-10-09-pre-push-check-native-on-linux) to build the stage environment: `HOME` unset or empty reads `cannot-evaluate` / `home-unset`; `PATH` is searched, absolute entries only, for the first directory holding a `node` file, and that directory alone is carried into the stage PATH between `{HOME}/.cargo/bin` and `/usr/local/bin:/usr/bin:/bin`. No value of either is printed or written (the verdict document and its twin hold no environment value and no path); every probe and stage child otherwise starts from a cleared environment, with `DBUS_SESSION_BUS_ADDRESS`, `XDG_RUNTIME_DIR`, `DISPLAY` and `WAYLAND_DISPLAY` absent by set equality.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux — Environment variables: `HOME` · `PATH` registered as system variables read harness-only by `pre-push:linux` (tool-locating read; no value printed or written).
    rationale: >-
      The verb's own environment reads are unregistered. Report Changes → Environment variables ("The verb READS two from its own environment, by value, to build the stage environment, and prints or writes neither: HOME and PATH"); "The order of a run" (`home-unset`); Deviations 1 and 4 (unset `HOME`; the PATH search skips a non-absolute entry), accepted by the operator; Schema / config ("No member carries a value of an environment variable or a path"); Expected amendment 2.
    basis: "xtask/src/pre_push.rs:153-155 (the `HOME` read) · xtask/src/pre_push.rs:333-347 (stage_path) · xtask/src/pre_push.rs:328-331 (dir_holding)"
```

**Validate.** All nine: **apply** (check 1, playbook: an accurate this-chunk change inside an existing registry row and
two rows the chunk's own variables need; the plan's expected amendments 1 and 2, approved at phase P5, name the change).
Every coordinate each proposal cites is a row of the report's `## New text, by line` section (checked row by row;
`535-546` is a span from one row's first number to another's last). Applied as written text re-derived from the
report, not pasted:
- proposals 1 to 7 (the `pre-push:linux` row, sentence by sentence) landed as ONE rewrite of that row;
- proposal 5's Node version is written in the host-tool form, `as measured on the dev host at …`;
- proposals 8 and 9 landed as two new rows of Environment variables, `HOME` · `PATH` first.
Check 4: the row is one 19 KB line, read by offset (the row at chars 14472 to 16257 of the pre-pass line; the
`check:english-sources` mention at 6472), never from a clipped view.

## security-plan — 3 proposal(s)

Verdict: 3 proposals parsed. Stripped: comment lines stripped: D-security-auth, D-security-deps, D-security-logging no drift; D-security-input one coverage gap at three sites, the surface validated per the report and not yet registered.

```yaml
proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the harness-only class, after the `2026-10-09-boot-smoke-s-early-exit-found-and-closed` (a)-(c) sentence)"
    change: >-
      Add: "**The same class since chunk `2026-10-09-pre-push-check-native-on-linux` (classified routine harness evidence, not a boundary widening — the operator's reading at that chunk's wrap directive, inputs#I2 item 6):** `cargo xtask pre-push:linux` (`xtask/src/pre_push.rs`; an xtask verb, never product-consumed; it evaluates on Linux alone, any other system reads `cannot-evaluate` / `not-linux`) (a) reads `HOME` and `PATH` from its own environment BY VALUE, only to build the stage environment, and prints or writes neither — `HOME` unset or empty reads `cannot-evaluate` / `home-unset`, and the `PATH` search for the first `node` file takes absolute directories only; (b) spawns every probe and stage child with a CLEARED environment plus exactly `HOME`, `PATH` (`{HOME}/.cargo/bin`, the found `node` directory when there is one, `/usr/local/bin:/usr/bin:/bin`), `ANDROMEDA_PULSE_DATA_DIR` (`target/pre-push/run/data`, absolute), `PUPPETEER_CACHE_DIR` for the `npm` stage only (`target/pre-push/run/puppeteer`; read by no product code) and `GIT_INDEX_FILE` for the three git calls that compute `head` and `tree` only — `DBUS_SESSION_BUS_ADDRESS`, `XDG_RUNTIME_DIR`, `DISPLAY` and `WAYLAND_DISPLAY` are outside the set (pinned by set equality), so no stage reaches an OS credential store; (c) writes only its per-run area `target/pre-push/run/` (`data/`, `puppeteer/`, the temporary index; removed and created again after the provisioning check passes) and the report twin `target/pre-push/report.json`, whose six members (`verdict`, `reason`, `head`, `tree`, `stages[]`, `missing[]`; the set pinned by a test) carry no value of an environment variable and no path; it installs nothing and prints no install command; stage output goes to stderr as the tools' own text and passes no scrubber. The product binary's reads are unchanged."
    sidecar: "2026-10-10 · chunk 2026-10-09-pre-push-check-native-on-linux · §Security Anti-Patterns → Input: harness-only class gains the native `pre-push:linux` verb — by-value read of `HOME` / `PATH`, cleared child environment with a pinned variable set, per-run area under `target/pre-push/run/`; operator-classified routine harness evidence (inputs#I2 item 6), not a widening."
    rationale: >-
      Report Changes → Symbols / APIs → Environment variables + Filesystem: the verb now reads `HOME` and `PATH` by value and spawns children with a cleared environment plus a fixed set; Expected amendment 5 names this exact section as carried and records 0 hits for `pre-push` and `PUPPETEER` in security-plan, so the harness-only class enumeration (tool-locator vars, boot-recorder state files, the two harness-written log files, the three presence-read system variables) no longer covers every harness input. The boundary itself is validated per the report (Coverage of new surfaces: host guard, pins, provisioning check, an unmet pin is `cannot-evaluate`; `home-unset`; non-absolute PATH entries skipped), so this is a missing registration, not an unvalidated boundary. Severity is the detector's; the classification the escalation would ask for is already on record as the operator's (inputs#I2 item 6).
    basis: "xtask/src/pre_push.rs:349-368 (stage_env) · xtask/src/pre_push.rs:153-155 (the `HOME` read) · xtask/src/pre_push.rs:333-347 (stage_path) · xtask/src/pre_push.rs:134-138 (host_supported)"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the `CLI / env var inputs` row (How cell, the harness-only readers after `xtask::harness_ready`)"
    change: >-
      Add: "A third harness-only reader, `cargo xtask pre-push:linux` (`xtask/src/pre_push.rs`, chunk `2026-10-09-pre-push-check-native-on-linux`; not product-consumed): the system `HOME` and `PATH` are read by xtask alone, by value, to build the stage environment, and neither is printed or written — `HOME` unset or empty reads `cannot-evaluate` / `home-unset`; a non-absolute `PATH` entry is skipped in the search for `node`; the two pins come from the repository (the Rust channel from `rust-toolchain.toml`, the Node major from ci.yml), either unreadable reads `pins-unreadable`; a missing required tool or an unmet pin reads `cannot-evaluate` / `provisioning-missing` and names it in `missing[]`, never green; the found Node version is printed only as a short version token, else `none` / `unreadable`. Every probe and stage child gets a cleared environment plus a pinned set (see §Security Anti-Patterns → Input), and the verdict document carries no value of an environment variable and no path."
    sidecar: "2026-10-10 · chunk 2026-10-09-pre-push-check-native-on-linux · §Input Validation CLI / env var row: third harness-only reader registered (`pre-push:linux`: `HOME` / `PATH` by value, `home-unset`, `pins-unreadable`, `provisioning-missing`)."
    rationale: >-
      Same claim restated: this row enumerates the harness-only readers (`ANDROMEDA_PULSE_MSEDGEDRIVER_PATH`, then "a second harness-only reader, `xtask::harness_ready`") with each one's validation. Report Changes → Symbols / APIs (the order of a run, the provisioning check, Environment variables) adds a third reader whose validation the row does not state; left unamended, the row and the Anti-Patterns paragraph would disagree on the class's members.
    basis: "xtask/src/pre_push.rs:153-155 · xtask/src/pre_push.rs:477-489 (found_version) · xtask/src/pre_push.rs:491-495 (provisioning_stop)"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → `CLI input (env vars + binary launch)` → Trust boundary (the harness-only carve-out list)"
    change: >-
      Extend the carve-out list so it reads: "the harness-only carve-out — tool-locator vars, the `agent-run.{sh,ps1}` `.spawn` / `.exit` boot-recorder state files, the two harness-written `logs/harness-settled.json` / `logs/xvfb.log` files, the xtask-only presence read of `DISPLAY` / `DBUS_SESSION_BUS_ADDRESS` / `XDG_RUNTIME_DIR` (chunk `2026-10-09-boot-smoke-s-early-exit-found-and-closed`), and the `pre-push:linux` xtask verb's by-value read of `HOME` / `PATH` with its per-run area under `target/pre-push/run/` (chunk `2026-10-09-pre-push-check-native-on-linux`), none product-consumed — stands per §Security Anti-Patterns → Input."
    sidecar: "2026-10-10 · chunk 2026-10-09-pre-push-check-native-on-linux · §Threat Model Summary CLI-input trust boundary: harness-only carve-out list gains the `pre-push:linux` verb's `HOME` / `PATH` read and per-run area."
    rationale: >-
      Third site carrying the same enumeration: the CLI-input trust boundary lists the harness-only carve-out's members by name and points to §Security Anti-Patterns → Input. Report Changes → Environment variables + Filesystem add a member (Expected amendment 5); a single-site apply would leave this list one member short of the section it cites.
    basis: "xtask/src/pre_push.rs:349-368"
    dependent-of: D-security-input
```

**Validate.** All three: **apply**. The detector's severity is `escalate`. Check 1: the finding is a missing registration
of a harness reader the report shows validated (host guard, `home-unset`, absolute PATH entries only, pins,
provisioning), outside the class that severity guards — the playbook's rule for an escalate-severity detector firing
outside its class, with the report substantiating the actual class. The boundary-widening question (the playbook's
never-routine class, judged by its subject) is answered by the operator's recorded word for this wrap, inputs#I2 item 6:
the verb's tool-locating read of `PATH` and `HOME` and its per-run area under `target/` are routine harness evidence,
not a widening. It is recorded in the body and the sidecar as the pc overseer's own reading as operator, not the
founder's word, and is named again on the route-resolve card. The plan's expected amendment 5 names the section. The
primary and its two dependents landed together.

## design-system — proposals: []

Verdict: no drift. Stripped: comment lines stripped: both detectors not violated; 0 hits in the doc for the claims the chunk moved.

Raw twin (an empty return that stripping changed): `.raw-fanout-design-system.md`.

## layout-templates — proposals: []

Verdict: no drift. Stripped: comment lines stripped: both detectors no drift; 0 hits in the doc for the claims the chunk moved.

Raw twin (an empty return that stripping changed): `.raw-fanout-layout-templates.md`.

## test-plan — 7 proposal(s)

Verdict: 7 proposals parsed. Stripped: comment lines stripped: D-tests-coverage clean, D-tests-obs-harness clean, D-tests-framework violated (one primary and six occurrences of the retired claim).

```yaml
proposals:
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      In the `cargo xtask pre-push:linux` paragraph, replace the host clause ("it requires a Windows host with the WSL
      `Ubuntu` distro (`wsl.exe`), so since the dev host moved to Omarchy Linux (2026-10-03) it cannot run on the dev
      host ... carried to the next version as a residual") with: since chunk
      `2026-10-09-pre-push-check-native-on-linux` the verb runs on the Linux dev host itself, with no `wsl.exe` hop and
      no distro clone; it evaluates on Linux alone (`host_supported(std::env::consts::OS)`), any other system reading
      `cannot-evaluate` / `not-linux`; that chunk built the residual carried from 0.3.0 (P-103). Keep as dated history
      that between the 2026-10-03 host move and that chunk the WSL form could not run on the dev host and its six
      stages were run natively by hand under the founder ruling (green re-run, 2575/2575). "An xtask verb because the
      5-command `agent-run` discipline admits no sixth verb" and "NOT a CI step" stay.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux: §3 Per-chunk gate discipline, pre-push:linux host clause — "requires a
      Windows host with WSL / cannot run on the dev host" retired; the verb is native on Linux (host guard
      `not-linux`), residual P-103 built.
    rationale: >-
      Report "Spec claims disproved by measurement" 1 names this sentence and measures it false: two green runs of the
      verb on the dev host (Outcome 1; Gates: exit 0, 90.25 s, and again on the committed tree `cb8cc4dc`). Report
      "Counts / qualifiers moved": "Where the check can run: a Windows host with the WSL `Ubuntu` distro → the Linux
      dev host". Report Symbols/APIs: the host guard and the removed `WSL` / `DISTRO` / `CLONE_DIR` constants. Expected
      amendment 3, carried.
    basis: ".andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32 · xtask/src/pre_push.rs:134-138"

  - detector: D-tests-framework
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      In the same paragraph, replace "It syncs a distro clone to HEAD plus the working tree (one binary patch through a
      temporary index, verified by tree-id equality) and runs the Linux-reachable gates in order" and the `source-lint`
      stage's "in the clone" with: the six stages (same names, order and commands) run in the working tree, output to
      stderr, first failure stops. A run goes host guard → the two pins read from the repository (Rust channel from
      `rust-toolchain.toml`, Node major from ci.yml) → `HOME` read → the provisioning probes → the per-run area
      `target/pre-push/run/` (`data/`, `puppeteer/`, the temporary index) removed and created again, AFTER the
      provisioning check passes, so a `provisioning-missing` run rewrites only the report twin → `head` and `tree`
      (three git calls under `GIT_INDEX_FILE`) → the stages. Every probe and stage child is spawned with a cleared
      environment plus exactly `HOME`, `PATH` (`{HOME}/.cargo/bin`, the directory of the first `node` on the verb's own
      PATH, `/usr/local/bin:/usr/bin:/bin`), `ANDROMEDA_PULSE_DATA_DIR` (`target/pre-push/run/data`) and, for `npm`
      only, `PUPPETEER_CACHE_DIR`; `DBUS_SESSION_BUS_ADDRESS`, `XDG_RUNTIME_DIR`, `DISPLAY` and `WAYLAND_DISPLAY` are
      absent, pinned by set equality. The `test` stage runs inside `restoring`, which puts the tracked
      `pulse-app/ui/src/bindings/index.ts` back as found whichever way the stage ended (a failed restore reds the stage,
      `restore-failed:bindings`); the restore runs when the command returns, not on a signal, so a verb killed
      mid-`test` leaves the bindings rewritten. `ci-gates` runs over the data dir removed and created again holding only
      the seed log.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux: §3 pre-push:linux mechanism — distro-clone sync and binary patch
      retired; stages run in the working tree under a constructed environment, per-run area `target/pre-push/run/`,
      bindings restored around the `test` stage.
    rationale: >-
      Report Symbols/APIs: `sync` and its patch file removed; "The order of a run"; "Environment variables" (cleared
      environment, set pinned by set equality); "Filesystem" (`target/pre-push/index`, `target/pre-push/tree.patch` and
      the clone `~/andromeda-pulse-pre-push` no longer written); "The stages, native"; "The bindings restore".
      Deviation 2 (reset after provisioning) accepted by the operator, to be carried into the amended rows.
    basis: "xtask/src/pre_push.rs:349-368 (stage_env) · :333-347 (stage_path) · :896-909 (restore pin) · :863-879 (reset pin)"
    dependent-of: D-tests-framework

  - detector: D-tests-framework
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      In the same paragraph the verdict document reads `{verdict, reason, head, tree, stages[{name, ok, ms}], missing[]}`
      — six members, pinned by set equality; `remediation` and `cache{bytes, cap, cleaned}` are gone; no member carries
      an environment value or a path. Exits unchanged (0 green · 1 red · 2 cannot-evaluate), twin unchanged
      (`target/pre-push/report.json`). State the reasons complete: `all-stages-ok` · `not-linux` · `pins-unreadable` ·
      `home-unset` · `provisioning-missing` · `run-dir-unusable` · `tree-unreadable` · `stage-failed:{stage}` ·
      `restore-failed:bindings`.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux: §3 pre-push:linux verdict document 8 → 6 members (`remediation`,
      `cache{…}` removed); reason vocabulary restated (`not-windows`, `wsl-missing`, `distro-missing`, `sync-failed`,
      `sync-mismatch` gone; `home-unset`, `run-dir-unusable`, `tree-unreadable`, `restore-failed:bindings` new).
    rationale: >-
      Report "Schema / config": six members by set equality in a test, `remediation` and `cache{…}` "left with the apt
      line and the clone". Report "Counts / qualifiers moved": "The verdict document's members: 8 → 6 ... the test-plan
      key file `per-chunk-gate-discipline.md` ... print[s] the eight-member form". Report "The verdict's reasons,
      complete". Deviation 1 (`home-unset`) accepted by the operator.
    basis: "xtask/src/pre_push.rs:937-950 (the_verdict_document_has_exactly_six_members)"
    dependent-of: D-tests-framework

  - detector: D-tests-framework
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      In the same paragraph, replace "A pinned tool or package missing (the toolchain channel, ci.yml's Node major and
      apt list, `jq` / `git` / `xvfb-run` / `cc`) reads `cannot-evaluate` with the one apt remediation line, never
      green" with: a missing piece reads `cannot-evaluate` / `provisioning-missing` and is named in `missing[]`, never
      green — `rust:{channel}` (the pinned channel absent from `rustup toolchain list`, no install hint),
      `rust:clippy` and `cargo-nextest` (probed only when the channel is listed, since a `cargo +{channel}` call
      against an absent channel could install it), `node:{pin} (found {version})` (the Node major against ci.yml's
      pin; `none` when no `node` answered, `unreadable` when the output is not a short version token), `tool:npm`,
      `tool:git`, `tool:cc`, `tool:python3` (`python3` else `python` printing `Python 3`). The verb installs nothing
      and prints no install command. "It binds no port and starts no `pulse-app`" stays.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux: §3 pre-push:linux provisioning check — ci.yml apt list, `dpkg-query`,
      `tool:jq`, `tool:xvfb-run` and the one apt remediation line retired; native probes and their `missing[]` names
      stated.
    rationale: >-
      Report "The provisioning check, native" lists the required pieces and what is gone ("the read of ci.yml's apt
      list, the `dpkg-query` probe, `apt:{package}`, `tool:jq`, `tool:xvfb-run`, the one `sudo apt-get install` line");
      Removed symbols `apt_packages`, `installed_packages`, `remediation`. Outcome 3 measured it live:
      `provisioning-missing`, `missing: ["node:24 (found none)", "tool:npm"]`, exit 2. Deviation 3 (the `missing[]`
      names) accepted by the operator.
    basis: "xtask/src/pre_push.rs:477-489 (found_version) · :491-495 (provisioning_stop) · :437-441 (channel_listed)"
    dependent-of: D-tests-framework

  - detector: D-tests-framework
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      In the same paragraph, replace the closing "Fragility: its Node 24 is the Viola repo's `~/.local/viola-node/bin`
      install in the distro ..." with: Fragility: the stage PATH takes the directory of the first `node` on the verb's
      own PATH, and on the dev host Node 24 (`v24.21.0`, npm `11.19.0`) is a user-level `mise install node@24`
      (2026-10-10) beside the host default `v26.8.2`, which is what `command -v node` still resolves — so the verb is
      started with Node 24 first on PATH, and a run whose first `node` does not meet ci.yml's major reads
      `cannot-evaluate` (`node:24 (found …)`). The Viola install inside the WSL distro is no longer read by anything
      in this repository.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux: §3 pre-push:linux fragility note — the Viola distro Node install
      retired; Node 24 is a user-level mise install beside the host default v26, the verb needs it first on PATH.
    rationale: >-
      Report "Dev-tool versions": Node `v24.21.0` with npm `11.19.0` installed by `mise install node@24` beside the
      default `v26.8.2`, "still what `command -v node` resolves"; "The Viola repository's Node install inside the WSL
      distro is no longer read by anything in this repository". Outcome 1: green "started with Node 24 first on PATH";
      Gates: `env PATH="$HOME/.cargo/bin:/usr/bin:/bin" cargo xtask pre-push:linux` exit 2, `node:24 (found`.
    basis: "xtask/src/pre_push.rs:328-331 (dir_holding) · :333-347 (stage_path)"
    dependent-of: D-tests-framework

  - detector: D-tests-framework
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      In the same paragraph, mark the two existing "Measured green" readings (2026-09-29, 5/5 stages; 2026-10-01, 6/6
      stages ×2) as readings of the WSL form, and add the native reading beside them: measured green natively
      2026-10-10 on the committed tree `cb8cc4dc` (6/6 stages, 89 s warm — `npm` 66.9 s, `test` 18.1 s, 2836 tests;
      load average 4.61 before, 11.32 after: a timing under that load, not a budget), after a first green on the
      working tree at implement (90.25 s).
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux: §3 pre-push:linux readings — the 2026-09-29 and 2026-10-01 greens
      labelled as WSL-form readings; native green 2026-10-10 on cb8cc4dc added (89 s warm, a timing under load, not a
      budget).
    rationale: >-
      Report expected amendment 3: "The native reading is a new dated reading beside the WSL ones: green 2026-10-10 on
      `cb8cc4dc`, six stages, 89 s warm (`npm` 66.9 s, `test` 18.1 s, 2836 tests), load average 4.61 before and 11.32
      after; a timing under that load, not a budget". Gates: 90.25 s at implement, 89 s in the operator pass. Without
      the label the cold/warm WSL timings read as timings of the verb as it now is.
    basis: ".andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32"
    dependent-of: D-tests-framework

  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy (What unit tests cover → corpus crate)"
    change: >-
      In the corpus-crate bullet, the clean-skip clause "including any run under `env -i`, which drops
      `DBUS_SESSION_BUS_ADDRESS` with `XDG_RUNTIME_DIR` (measured at that chunk's native pre-push stage 5)" reads:
      including any run whose environment carries neither `DBUS_SESSION_BUS_ADDRESS` nor `XDG_RUNTIME_DIR` — measured
      under `env -i` at that chunk's by-hand native stage 5, run while the WSL form of `pre-push:linux` could not run
      on the dev host; since chunk `2026-10-09-pre-push-check-native-on-linux` that stage is the verb's own `test`
      stage, whose constructed environment holds neither variable (pinned by set equality), where the clean-skip is
      stated, not measured (the stage's log cannot show a passing test's output). The `--success-output immediate` /
      `lacks [skip]` rule that follows is unchanged.
    sidecar: >-
      2026-10-09-pre-push-check-native-on-linux: §4 corpus crate — "native pre-push stage 5" re-anchored: the by-hand
      `env -i` stage is now the verb's own `test` stage under a constructed environment; clean-skip there stated, not
      measured.
    rationale: >-
      Report expected amendment 4, carried: "test-plan §4 Unit Test Strategy, the corpus crate's 'native pre-push stage
      5' clause ... The stage is now the verb's own `test` stage under a constructed environment; that the
      credential-store legs clean-skip there is stated, not measured by this chunk". Report "Environment variables":
      the four session variables "are not in the set (pinned by set equality), so no stage reaches an OS credential
      store". The clause names a by-hand stand-in for a verb the primary change makes native, so it is the second
      site of the retired claim in the body.
    basis: ".andromeda/test-plan.md:227 · xtask/src/pre_push.rs:616-621 (SESSION_VARIABLES)"
    dependent-of: D-tests-framework
```

**Validate.** All seven: **apply** (check 1, playbook: an accurate this-chunk change; the plan's expected amendments 3 and 4
name both sites). Coordinates checked against the report's last section. Proposals 1 to 6 landed as one rewrite of the
key file's paragraph (`.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32`, two anchored edits
around the unchanged middle of the stage list); proposal 7 landed in §4's corpus bullet. `registry.py check`: 0 defects.

## obs-plan — proposals: []

Verdict: no drift. Stripped: comment lines stripped: all four detectors hold; one note for the orchestrator to weigh, not proposed - the section 10 frame row and CI gates say the boot job's ci-gates line reads its adapter cause on a run that reaches the settle verdict, and the red run printed a settle verdict (ended) and no ci-gates line was read for it.

Raw twin (an empty return that stripping changed): `.raw-fanout-obs-plan.md`.

## a11y-plan — proposals: []

Verdict: no drift. Stripped: comment lines stripped: both detectors no drift; 0 hits in the doc and its ten key files for the claims the chunk retires.

Raw twin (an empty return that stripping changed): `.raw-fanout-a11y-plan.md`.

## Raised by the orchestrator (Validate checks 5 and 6)

- **Check 5, expected amendments:** all five entries of the plan's list are matched by proposals (1: architecture
  1 to 7 · 2: architecture 8 and 9 · 3: test-plan 1 to 6 · 4: test-plan 7 · 5: security-plan 1 to 3). No row of
  `citation-dispositions.md` reads `claim false`.
- **Check 6, disproved claims:** 1 (the check cannot run on the dev host) — matched by architecture 1 and test-plan 1.
  2 (reads no new env var) — matched by architecture 7. 3 (the boot job's `ci-gates` line on every run that reaches
  the settle verdict) — raised by the orchestrator on the obs-plan detector's note, after measuring it: the red run's
  job reads its `ci-gates` step `skipped` and its log holds 0 `ci-gates` lines. **apply**, routine (a doc claim a
  measurement disproved; the implementation is as it should be and no other half exists): obs-plan §10's frame row
  and CI gates bullet, test-plan §1's WebGPU trigger row. The neighbouring log-side claims were read and hold.
- **Check 2:** no two proposals edit one section in opposing directions; architecture, test-plan and security-plan
  state the same facts of the verb.
- **Check 3:** the report's six deviations are accepted by the operator's recorded word; the scope record holds no line.

## Totals

22 amendments applied (19 proposed + 3 raised) in architecture, test-plan (body and one key file), obs-plan and
security-plan · 1 escalate-severity group resolved on the operator's recorded word · 0 rejected · 6 sidecar entries.
