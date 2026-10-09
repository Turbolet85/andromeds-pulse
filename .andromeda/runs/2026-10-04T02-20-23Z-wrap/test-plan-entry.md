
## 2026-10-04-supply-chain-advisories-on-wasmtime-resolved — §5 plugins row resolves wasmtime 48.0.5
**Section:** §5 Integration Test Strategy → plugins → runtime row
**Change:** Was `wasmtime` 48.x lockfile-resolved 48.0.3 as of 2026-09-29 (per the 2026-09-29-p-025-hue-shift-observable-made-gradable — webview runner vitest 4; wasmtime row 48.x entry, whose vitest claim stands); now 48.x lockfile-resolved 48.0.5 as of 2026-10-04. The 48.x family statement and the arch floor 25+ are unchanged.
**Why:** the chunk moved wasmtime off 48.0.3 to close RUSTSEC-2026-0325 / -0326 / -0327; the plugins tests pass 61/61 on 48.0.5 with no source change.
**Ref:** .andromeda/runs/2026-10-04T02-20-23Z-wrap/
