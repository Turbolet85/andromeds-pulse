
## 2026-10-10-engine-harness-proofs-completed — the readiness verb reads ready only over this run's own bound records
**Section:** §Occupied Resources → xtask CLI surfaces (`cargo xtask harness:ready`; the `scripts/agent-run.{sh,ps1}` entry) · → Filesystem locations (the `run/andromeda-pulse.pid` bullet) · → Environment variables (`ANDROMEDA_PULSE_LOGFILE`)
**Change:**
- Was "`ready` = the status verdict `running-healthy` plus a TCP connection accepted on each OTLP port", receiver labels `accepting` | `refusing`; now `ready` needs both receivers `accepting`, and the labels are three: `accepting` (the connection accepted AND, after the last `app.boot.engine` record, an INFO record of that receiver's bind target, `app.boot.otlp.grpc.bind` / `app.boot.otlp.http.bind`, with `bind_address` `127.0.0.1:` and that port), `foreign` (accepted, no such record: a failed-bind ERROR record, another port, the other receiver, a record before the last boot record, no boot record), `refusing` (not accepted).
- The verb probes both ports, reads the log family once, takes the program by `harness_status::program_of` and still passes the status through `for_program`; `read_program` is `harness:status`'s. Verdict words, exits and the six members unchanged; both programs.
- `agent-run.sh boot` prints a second failure line when the last verdict held `foreign` (`a receiver port accepts but this run's log holds no bound record for it: …`); both lines can print. `agent-run.ps1` is not edited and matches `refusing` alone.
- The pid-file bullet and the script entry restate the decision with the bound-record condition.
**Why:** An accepted loopback connection does not say whose listener accepted it: with another process on the port the verb read `ready` and the boot verb printed `boot: ready`. Fixed at the verb, for both programs, on the operator's word at the plan dialog (the pc overseer, 2026-10-10).
**Kept:** For the window program the `foreign` reading is pinned at the decision and not read live. The ps1 script's missing `foreign` line stays with the route entry that retires the other operating systems.
**Ref:** .andromeda/runs/2026-10-10T19-50-04Z-wrap/
