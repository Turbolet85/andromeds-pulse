
## 2026-10-09-pre-push-check-native-on-linux — the pre-push:linux row describes the native check
**Section:** §Occupied Resources → xtask CLI surfaces, the `cargo xtask pre-push:linux` row
**Change:** The row describes a verb that runs on the Linux dev host itself; was "the WSL Linux pre-push verb … Windows host only", every call going through `wsl.exe` into a distro clone. Now:
- Linux alone; any other system reads `cannot-evaluate` / `not-linux`.
- The verdict document has six members, `{verdict, reason, head, tree, stages[{name, ok, ms}], missing[]}`, pinned by set equality, none carrying an environment value or a path; was eight, with `remediation` and `cache{bytes, cap, cleaned}`.
- Reasons, complete: `all-stages-ok` · `not-linux` · `pins-unreadable` · `home-unset` · `provisioning-missing` · `run-dir-unusable` · `tree-unreadable` · `stage-failed:{stage}` · `restore-failed:bindings`; `sync-mismatch` is gone.
- Order of a run: host guard → two pins from the repo (the Rust channel, ci.yml's Node major) → `HOME` → provisioning probes → per-run area → `head` and `tree` → six stages in the working tree, output on stderr.
- Every probe and stage child: a cleared environment plus `HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`, `PUPPETEER_CACHE_DIR` (the `npm` stage only), `GIT_INDEX_FILE` (the tree-id git calls only); no session bus, runtime dir or display variable.
- `missing[]` names: `rust:{channel}`, `rust:clippy`, `cargo-nextest` (the last two probed only when the channel is listed), `node:{pin} (found {version})` with `none` / `unreadable`, `tool:npm`, `tool:git`, `tool:cc`, `tool:python3`. The apt-list pin and the one `sudo apt-get install` line are gone; the verb installs nothing.
- The per-run area `target/pre-push/run/` is reset once provisioning has passed, so a `provisioning-missing` run rewrites only the report twin. The clone, its 40 GiB cap and the binary-patch sync are gone.
- The `test` stage's rewrite of the tracked bindings is put back as found; `restore-failed:bindings` when it cannot; the restore does not run on a signal.
- "reads no new env var" is retired: the verb reads `HOME` and `PATH` by value and prints or writes neither.
- Node is the first `node` on the caller's PATH; on the dev host a user-level `mise` Node 24 (v24.21.0, npm 11.19.0, as measured there) beside the default Node 26. The other project's Node install is no longer read.
**Why:** The chunk rebuilt the verb natively and measured it green on the dev host twice. The reset-after-provisioning order, the `home-unset` reason and the `missing[]` names are implement's deviations, accepted for these rows by the operator (the pc overseer, 2026-10-10).
**Kept:** "Dev-host only — not wired into CI"; the three exits; the six stage names and their order; the `check:english-sources` row's mention of the `source-lint` stage.
**Ref:** .andromeda/runs/2026-10-10T01-19-03Z-wrap/
