# Tests validation — route draft

## Insert
- Between `Engine end-to-end gate reachable` and `Detection baseline through the console engine`: **"Scenario legs driven against the console engine — gap-and-resume and external-resolve keep their verdicts with the sidecar gone"** (epoch: `Epoch 1`)
  Reason: Per test-plan §3 key `per-chunk-gate-discipline`, `smoke:gap-resume` (arms A and B gate-grade) and `smoke:external-resolve` (verdict on `reconciled_count`, mutation-checked) grade engine behaviour by launching the window binary, the second through a real `andromeda-pulse-mcp` subprocess. The draft names only the hue-shift, discovery and frame legs as leaving and re-points neither of these, so one loses its driver at the door entry and both at the window's retirement.

- Between `Learned state survives a restart` and `Load profiles re-based on the engine`: **"Disk store opened across builds — a store written by an earlier build is carried forward or refused by a stated rule, pinned by a test"** (epoch: `Epoch 6`)
  Reason: Per test-plan §7 "Self-bootstrapping requirement", the `boot` migrations step is N/A only "for in-memory DuckDB"; it becomes live once the store survives a restart, and four Epoch 7 entries then change the stored shape with nothing saying what a restart-survival test or the harness `boot` expects of an older store. The corpus has such a rule in `Incident is the engine's own record` ("0.3.0 stores not read"); the telemetry store has none, and P-091 states none.

## Reorder
- Move `Agent harness drives the console engine` before `Door inside the engine's process`
  Reason: Per the harness-before-any-feature rule and test-plan §3 key `per-chunk-gate-discipline` (boot-smoke gate for every chunk touching a boot path), the door, the corpus-encryption retirement and the one-place move all change the boot of a process no harness verb can yet boot. The verbs depend only on `Console engine entry point` (key `pid-file` is data-dir-relative), and placed first they let `One place on a node` move pid and log under pinned verdict arms, at the cost of that entry re-rooting the verbs' log bases.

- Move `Shared telemetry test data` before `Door inside the engine's process`
  Reason: Per test-plan §7 "Self-bootstrapping requirement" and §3 key `bootstrap-phases` item 6 (all three factories owed, 0 workspace hits), fixtures precede their first consumer; since the door moved into Foundation, its span, metric and log answers — not the engine gate — are the first assertions over all three record kinds.

## Rewrite
- `Door inside the engine's process`: "stdio sidecar, its empty database, double gate leave" → "sidecar, its empty database, double gate, feature-named gates leave"
  Reason: Per test-plan §9 "Boot smoke (harness)" (the `boot` job builds `--features mcp-server`, `ci.yml:440`) and §3 key `per-chunk-gate-discipline` (the standing gate set's bindings-regen line and its `"mcp":` probe run under the same feature), two gates name the feature this entry removes yet are listed to leave only at `Window's gates retired`, so they cannot resolve it for the rest of Foundation. If the 25-word line cannot hold it, the entry's leave-list in `1a-tree.md` (which carries only the `mcp-test` job) can.
