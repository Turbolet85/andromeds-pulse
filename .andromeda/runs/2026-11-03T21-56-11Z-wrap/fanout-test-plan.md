proposals:
  - detector: D-tests-framework
    severity: warning
    section: §4 Unit Test Strategy
    change: 'Framework (webview unit tests): `vitest` 4.x (4.1.12 with `@vitest/mocker` 4.1.12 since chunk 2026-11-03-advisories-since-2026-10-12-closed, a lockfile-only patch update with `package.json` unchanged; 4.x since chunk 2026-09-29-p-025-hue-shift-observable-made-gradable, the vitest 3 mocker being vulnerable, GHSA-82fw) — the rest of the paragraph unchanged'
    sidecar: '§4 webview Framework — vitest / @vitest/mocker resolved patch 4.1.11 → 4.1.12 (npm lockfile update, chunk 2026-11-03-advisories-since-2026-10-12-closed); framework and runner unchanged'
    rationale: 'The report Dependencies bullet lists vitest and @vitest/{expect, mocker, pretty-format, runner, snapshot, spy, utils} 4.1.11 → 4.1.12 in pulse-app/ui/package-lock.json with package.json unchanged; its Counts bullet (pattern npm-pkgs) reads this site as a stale patch qualifier; `npm run test --prefix pulse-app/ui` ran green on it (Outcome gates), so the runner is on-spec and only the stated version is stale'
    basis: 'test-plan fct-ahyhfjr6 · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-6y6bvfge'
  - detector: D-tests-framework
    severity: warning
    section: §1 Test Scope Summary
    change: 'Surfaces under test, the desktop-webview unit tests row Driver cell: `vitest` 4.x (4.1.12 since chunk 2026-11-03-advisories-since-2026-10-12-closed; 4.1.11 from chunk 2026-09-29-p-025-hue-shift-observable-made-gradable) with `jsdom` 26 environment + `@testing-library/react` 16 (in `pulse-app/ui/`) — the other cells unchanged'
    sidecar: '§1 Surfaces under test, webview unit row — vitest resolved patch 4.1.11 → 4.1.12 (chunk 2026-11-03-advisories-since-2026-10-12-closed)'
    rationale: 'Second occurrence of the vitest 4.1.11 claim retired above; the report Dependencies bullet moves vitest 4.1.11 → 4.1.12 and its Counts bullet (npm-pkgs) names this site stale beside the §4 one'
    basis: 'test-plan fct-ylz3qbha · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §4 Unit Test Strategy
    change: 'Framework (Rust crates), the rust-version parenthetical: the lockfile resolves wasmtime 48.0.8, cranelift 0.135.8 and pulley 48.0.8 since chunk 2026-11-03-advisories-since-2026-10-12-closed (requirement "48.0.8"); the count of 28 packages declaring 1.95.0 stands as read on the 48.0.5 / 0.135.5 / 48.0.5 graph, the pin stays 1.95.0 and `declared_floor_equals_the_pinned_channel` passes on the new graph — the rest of the paragraph unchanged'
    sidecar: '§4 Rust Framework — wasmtime / pulley 48.0.5 → 48.0.8 and cranelift 0.135.5 → 0.135.8 as resolved (chunk 2026-11-03-advisories-since-2026-10-12-closed); toolchain pin and MSRV pin unchanged'
    rationale: 'The report Dependencies bullet restates `wasmtime = "48.0.4"` → `"48.0.8"` and moves the lockfile wasmtime / pulley 48.0.5 → 48.0.8 and cranelift 0.135.5 → 0.135.8; its Counts bullet (wasmtime-ver, cranelift-ver) reads this site stale, its Expected amendments names test-plan §4 Framework, and its Outcome reads toolchain 1.95.0 with `declared_floor_equals_the_pinned_channel` passing; the 28-package rust-version count was not re-measured on the new graph, so it is kept as the 48.0.5 reading'
    basis: 'test-plan fct-uo3y5o44 · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-np6ugxqp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-aklotj3w · Cargo.toml:129-136'
  - detector: D-tests-framework
    severity: warning
    section: §5 Integration Test Strategy
    change: 'Boundary types covered, the plugins → runtime row Tools cell: `wasmtime` 48.x (lockfile-resolved 48.0.8 since chunk 2026-11-03-advisories-since-2026-10-12-closed, 48.0.5 before it; arch family floor 25+), `bindgen!` macro'
    sidecar: '§5 plugins row — wasmtime lockfile-resolved 48.0.5 → 48.0.8 (chunk 2026-11-03-advisories-since-2026-10-12-closed)'
    rationale: 'Second occurrence of the wasmtime 48.0.5 claim retired above; the report Dependencies bullet moves the lockfile wasmtime 48.0.5 → 48.0.8, its Counts bullet (wasmtime-ver) names this site stale, and its Expected amendments names the test-plan §5 plugins row'
    basis: 'test-plan fct-35qjdhiv · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-np6ugxqp'
    dependent-of: D-tests-framework
note: 'D-tests-coverage holds (the report adds no symbol, module or surface); D-tests-obs-harness holds (the report changes no harness, status shape or log format); Retired is none. The other version mentions the report names as history are left: test-plan fct-szwcsgq3 (libduckdb-sys 1.10502 / 1.10505), contracts/per-chunk-gate-discipline fct-gdjnsjph (the 1.10502 → 1.10505 bump) and contracts/5-command-implementation fct-paxx23uc (tao 0.35.0 as measured at chunk 2026-10-10-boot-smoke-s-self-end-closed) — each is a dated reading, not a current-version claim.'
