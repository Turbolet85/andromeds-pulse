
## 2026-10-09-supply-chain-job-same-on-push-and-pull-request — record_webgpu_adapter: the CI boot job is a witness on some runs
**Section:** §Occupied Resources → Tauri IPC routes → `telemetry.frontend.record_webgpu_adapter`
**Change:** Was "its only live witness is the dev-host `perf:frame-sample` leg (the CI boot job stops the app before the webview issues any IPC)". Now: the dev-host `perf:frame-sample` leg is the live witness for `obtained`; the CI boot job's log holds the record on some runs only — `outcome: no_navigator_gpu` in four of seven boot-job logs of 2026-10-09 and no webview-originated record in three, by how long the app lives before the smoke's `cleanup` stops it (0.57 s to 1.40 s; as measured).
**Why:** Measured false as a rule while reading the boot smoke's application logs; the same correction lands in obs-plan, test-plan and security-plan under this marker. The procedure, its input type and the procedure count are unchanged.
**Ref:** .andromeda/runs/2026-10-09T17-46-44Z-wrap/
