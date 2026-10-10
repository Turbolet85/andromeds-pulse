
## 2026-10-10-agent-harness-drives-the-console-engine — the harness reaches the console engine: three verbs, two changed verdicts, the `engine` word, the cycle's CI step
**Section:** §Occupied Resources (xtask CLI surfaces; Network ports; Process / service identity; Filesystem locations; Environment variables) · §Stack and Technologies (the exit-witness row; the injector's CLI contract) · §Infrastructure Patterns → CI/CD approach
**Change:**
- New entries `cargo xtask check:engine-log` (seven arms, exit 0 · 1 · 2), `cargo xtask harness:engine-settled` (seven members, five verdicts) and `cargo xtask harness:engine-cycle` (refusals, the cleared child environment and its pinned set, eight-member verdict, its CI caller).
- `harness:status` takes `--program window|console`: seven members (`program` added), five arms (`wrong-program`, exit 1). Was six members and "four arms". `harness:ready` the same: six members, five arms. Was five and four.
- `agent-run.sh` `boot` / `status` take one optional word, `engine`, which builds and spawns `target/release/andromeda-pulse-engine run`; the pair is in lockstep but for two sh-only parts. Was one sh-only part, and "the one the harness builds and spawns" was the window binary alone.
- The pid-file bullet no longer says the harness readers are "not aimed at the console program"; the spawn and exit records are written for the engine too, and `check:engine-log` reads the exit record.
- The exit witness is the window program's alone: bare `boot` reads the variable and preloads the window app's spawn; `boot engine` reads nothing and sets no preload; `harness:engine-cycle` is a second, pass-through reader of `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`. Was "read solely by `agent-run.sh boot`" and "the app's spawn line".
- Registered: harness-only ports 24317 / 24318; `target/engine-cycle/{stamp}/` and the `logs-engine-Linux` upload; `inject_demo` as a reader of the gRPC port variable; the five variables the cycle sets into its children; `HOME` · `PATH`, `_PIDFILE` and `_LOGFILE` readers.
- CI/CD approach: the `boot` job's chain continues past `ci-gates` with `Console engine cycle`, and has a second upload; four more workflow tests.
- `perf:budget`: `check:engine-log` named as an in-process caller of the grader (memory, required).
**Why:** The chunk aimed the agent harness at the console engine and graded its log in CI; the registry described a harness that builds, spawns and reads the window app alone.
**Kept:** The exit-witness arm's classification stays PROVISIONAL in security-plan and is not restated here. Which key path the engine of a cycle takes was not read from its log; the entry says so.
**Ref:** .andromeda/runs/2026-10-10T16-47-34Z-wrap/
