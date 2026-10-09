
## 2026-09-29-p-025-hue-shift-observable-made-gradable — webview runner vitest 4; wasmtime row 48.x
**Section:** §1 Surfaces under test (desktop-webview unit row) · §4 Framework (webview unit tests) · §5 plugins → runtime row
**Change:** Was `vitest` 3.x; now `vitest` 4.x (4.1.11 with `@vitest/mocker` 4.1.11 — the vitest 3 mocker is vulnerable, GHSA-82fw). The §5 plugins row was `wasmtime` 46.x / 46.0.3; now 48.x / 48.0.3 as of 2026-09-29 (floor 25+ unchanged).
**Why:** both upgrades landed in-chunk to clear advisories on the first real CI run.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
