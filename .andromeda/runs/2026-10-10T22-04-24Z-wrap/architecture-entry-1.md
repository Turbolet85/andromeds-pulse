
## 2026-10-10-shared-telemetry-test-data — a seventeenth workspace member, the dev-only `test-support`; the member count words follow
**Section:** §Occupied Resources → Cargo workspace crate names · §Design Philosophy (Single-process modular monolith) · §Stack and Technologies → Primary language / runtime
**Change:**
- Crate names: was sixteen reserved names; now seventeen, `test-support` added — the member `crates/test-support` (`publish = false`), a library crate with modules `otlp` (the telemetry test factories) and `ports` (the test port claim), taken as a dev-dependency only by `buffer`, `pulse-app` and `xtask`, depending on `ingest` alone, linked by no product target.
- Design Philosophy: was "sixteen workspace members total: fourteen library crates + the `pulse-app` binary crate + the `xtask` task-runner crate"; now seventeen: those fourteen + `pulse-app` + `xtask` + the dev-only test-support library crate `test-support`, which no product target links and the engine boot does not wire. "Fourteen library crates" stays, as the count of the crates the shared engine boot wires.
- Primary language row: was "all 16 members inherit it"; now 17.
**Why:** The chunk added the member and the registry enumerates crate names; the member was named and accepted by the operator at the chunk's plan dialog (the pc overseer, 2026-10-10).
**Kept:** The keyed contract Deployment model keeps "the Tauri process hosting all fourteen library crates": it is true of the fourteen this section names. `test-support` is a fifteenth library crate by form only; a later count word reads "fourteen" as the wired crates.
**Ref:** .andromeda/runs/2026-10-10T22-04-24Z-wrap/
