
## 2026-10-10-engine-harness-proofs-completed — harness registrations: the taken-port refusal, the readiness verb's read of bind records, the pinned word set, the cycle engine's lock file
**Section:** §Input Validation → the CLI / env var inputs row (`harness:ready`; the `agent-run.sh` word set; `harness:engine-cycle`) · §Security Anti-Patterns → Input (the console-engine harness paragraph, items (a) and (c); the exit-witness arm, item (a))
**Change:**
- `harness:engine-cycle`: was seven closed refusal labels; now eight, `port-taken` added — a cycle port already accepting a loopback connection, read before the `HOME` read, nothing built or started, no data dir made.
- `harness:ready`: it probes both resolved ports, reads the log family once as JSON lines (a non-JSON line skipped), and each receiver member is one of three closed labels, `accepting` · `foreign` · `refusing`; labels alone leave the verb. The harness paragraph names the read as joining its readers, with no classification of its own. `agent-run.sh boot` prints one more failure line on `foreign` (two loopback ports, nothing else); `agent-run.ps1` not edited.
- The `agent-run.sh` word set: was "no committed shell-level test"; now pinned by committed shell-level tests in `xtask/tests/agent_run_shell.rs` (bare `boot`, `status engine` and bare `status`, any other word → usage, exit 2, no data dir). The ps1 script has no such word.
- The cycle verb, item (c): was "it writes only its per-cycle data dir"; now the verb itself writes only that, while the engine it starts also writes the content-free corpus-key lock file to the system temp dir (no runtime dir in the child set) and takes the passphrase fallback after opening it; pinned on a spawned engine.
- The exit-witness arm's "no committed shell-level test" is scoped to clause (d) of the shell-coverage row, whose cleanup, boot failure-reason and word-arm clauses are closed.
**Why:** Each is what the chunk built and tested; the registry enumerated seven labels and two receiver labels and called the word set untested.
**Kept:** No classification is written for the readiness verb's read of bind records: it reads two more record kinds of a log family it already read. The exit-witness arm stays PROVISIONAL as it stood.
**Ref:** .andromeda/runs/2026-10-10T19-50-04Z-wrap/
