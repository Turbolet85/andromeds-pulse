## 2026-11-03-advisories-since-2026-10-12-closed — vitest's resolved patch 4.1.12
**Section:** §1 Test Scope Summary (the desktop-webview unit tests row) · §4 Unit Test Strategy (Framework, webview unit tests)
**Change:**
- §1 row: was `vitest` 4.x (4.1.11 since chunk 2026-09-29-p-025-hue-shift-observable-made-gradable); now `vitest` 4.x (4.1.12, as resolved by `pulse-app/ui/package-lock.json`).
- §4 webview Framework: was 4.1.11 with `@vitest/mocker` 4.1.11 since chunk 2026-09-29-p-025-hue-shift-observable-made-gradable; now 4.1.12 with `@vitest/mocker` 4.1.12, as resolved by `pulse-app/ui/package-lock.json` (the vitest 3 mocker still named vulnerable, GHSA-82fw). Framework, environment and the rest of both texts unchanged.
**Why:** the chunk's npm lockfile update moved vitest and the `@vitest/*` packages 4.1.11 → 4.1.12 with `package.json` unchanged, and `npm run test` ran green on it; only the stated patch was stale.
**Ref:** .andromeda/runs/2026-11-03T21-56-11Z-wrap/

## 2026-11-03-advisories-since-2026-10-12-closed — wasmtime, cranelift and pulley as the lockfile resolves them
**Section:** §4 Unit Test Strategy (Framework, Rust crates) · §5 Integration Test Strategy (the plugins → runtime row)
**Change:**
- §4 Rust Framework: was 28 packages declare 1.95.0 (wasmtime 48.0.5, cranelift 0.135.5, pulley 48.0.5); now the count of 28 is a reading on the wasmtime 48.0.5 / cranelift 0.135.5 / pulley 48.0.5 graph, not re-read on the one `Cargo.lock` resolves — wasmtime 48.0.8, cranelift 0.135.8, pulley 48.0.8, requirement `"48.0.8"`.
- §5 plugins row: was `wasmtime` 48.x (lockfile-resolved 48.0.5 as of 2026-10-04); now lockfile-resolved 48.0.8 (arch family floor 25+ unchanged).
**Why:** the chunk restated the wasmtime requirement `"48.0.4"` → `"48.0.8"` and the lockfile moved wasmtime / pulley 48.0.5 → 48.0.8 and cranelift 0.135.5 → 0.135.8; the 28-package rust-version count was not re-measured on the new graph.
**Kept:** the pinned toolchain 1.95.0 and the workspace's `rust-version = "1.95"`, held equal by `declared_floor_equals_the_pinned_channel`, which passed on the new graph; the 28 stands as its 48.0.5 reading rather than a fresh count.
**Ref:** .andromeda/runs/2026-11-03T21-56-11Z-wrap/

## 2026-11-03-advisories-since-2026-10-12-closed — the door's item ratified
**Section:** §9 CI Integration (the `lint-test` job row) · §1 Test Scope Summary (the console engine door row) · contracts/per-chunk-gate-discipline (Spawned-program witness form)
**Change:**
- §9: was «The door is PROVISIONAL — a boundary widening, one item», its parts the operator's answers, the founder's word owed and nothing ratified; now «The door is a boundary widening, one item, ratified»: the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable, the stdio relay word `door`, and the `Door refuses another account` step with its second arm (the relay's); every other statement of the door in the plan rests on it.
- §1 door row: was (§9, which holds the door's PROVISIONAL sentence; the relay word and the step's second arm are PROVISIONAL with it); now (§9, which holds the door's item sentence).
- Per-chunk gate discipline: was (test-plan §9, which holds the door's PROVISIONAL sentence); now (test-plan §9, which holds the door's item sentence); the mark (PROVISIONAL with the door's item) after `unit_door_bridge.rs` dropped.
**Why:** the founder confirmed the door's own item and the stdio relay word (shaping-0.4.0 items 27 and 28) by dialog 2026-11-02, relayed by overseer1 and by the pc-overseer (founder-delegated) at this wrap; until then the door rested on the operator's answers (the pc overseer at the P4 dialog of 2026-10-10 and the operator's reading of 2026-10-11). Every door PROVISIONAL mark drops with it.
**Kept:** the other PROVISIONAL items stand — the pre-push sixth stage and the branch threshold; the founder's word reached the door's item alone.
**Ref:** .andromeda/runs/2026-11-03T21-56-11Z-wrap/
