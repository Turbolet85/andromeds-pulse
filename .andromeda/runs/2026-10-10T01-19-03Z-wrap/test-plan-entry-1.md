
## 2026-10-09-pre-push-check-native-on-linux — the pre-push:linux paragraph describes the native check and its first native readings
**Section:** §3 → Per-chunk gate discipline (the `cargo xtask pre-push:linux` paragraph of the key file)
**Change:** The paragraph says the verb runs on the Linux dev host itself since this chunk; was "it requires a Windows host with the WSL `Ubuntu` distro … it cannot run on the dev host", syncing a distro clone. Now:
- Linux alone, `not-linux` elsewhere. The WSL requirement, the hand-run of the six stages from 2026-10-03 and the residual carried from 0.3.0 are kept as one dated history sentence; the port is built as P-103.
- Order: host guard → two pins from the repository (`pins-unreadable`) → `HOME` (`home-unset`) → provisioning probes → the per-run area `target/pre-push/run/`, reset once provisioning has passed → `head` and `tree` through a temporary index → the stages in the working tree.
- Children get a cleared environment plus `HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR` and, for `npm` only, `PUPPETEER_CACHE_DIR`; no session bus, runtime dir or display variable, pinned by set equality.
- The `test` stage's rewrite of the tracked bindings is put back as found (`restore-failed:bindings`; not on a signal). `ci-gates` runs over the data dir recreated holding only a seed log.
- The verdict has six members, `{verdict, reason, head, tree, stages[{name, ok, ms}], missing[]}`; was eight. The nine reasons are listed.
- A missing piece is `cannot-evaluate` / `provisioning-missing`, named in `missing[]`: `rust:{channel}`, `rust:clippy`, `cargo-nextest`, `node:{pin} (found {version})`, `tool:npm`, `tool:git`, `tool:cc`, `tool:python3`; was the apt list, `jq`, `xvfb-run` and one apt remediation line.
- Readings: the 2026-09-29 and 2026-10-01 greens are labelled readings of the WSL form. New, native, on the dev host: green 2026-10-10 on the working tree (90 s) and on the committed tree `cb8cc4dc` (89 s warm, `npm` 66.9 s, `test` 18.1 s, 2836 tests, load average 4.61 before and 11.32 after). A timing under that load, never a budget; a cold `target/` was not measured.
- The credential-store legs are expected to clean-skip inside the `test` stage: stated, not measured.
- Fragility: Node is the first `node` on the caller's PATH; on the dev host a user-level `mise` Node 24 beside the default Node 26. Was the other project's Node install inside the distro.
**Why:** The chunk rebuilt the verb natively and measured it green on the dev host twice. A native timing is one reading on a shared host, so it is written with its load and never as a budget.
**Kept:** "an xtask verb because the 5-command `agent-run` discipline admits no sixth verb"; "NOT a CI step"; the stage list; "it binds no port and starts no `pulse-app`".
**Ref:** .andromeda/runs/2026-10-10T01-19-03Z-wrap/
