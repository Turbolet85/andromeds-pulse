# tests extract

## Relevance
Relevant — modifies service registry listing (backend) and constellation rendering (webview); both testable via unit/integration/E2E.

## Constraints
1. Standard tier (§1) applies — cross-platform, 7 critical paths, agent-driven discipline required.
2. 5-command harness discipline (§3) — boot/run/status/cleanup/logs for all integration tests.
3. Registry unit tests must verify liveness classification and state-filtering logic per §4 (pure functions, isolated).
4. Webview component tests use `vitest` 3.x + jsdom + @testing-library/react per §4.
5. Coverage threshold: ≥ 75% line / ≥ 70% branch for new code per §4 §10.
6. Self-bootstrapping fixtures — service-list tests seed via OTLP ingest, never hardcoded snapshots per §3 §7.
7. Chunk-gate baseline per §3 (amendment 2026-05-10) — mandatory: `cargo fmt --check`, `cargo clippy --workspace`, `cargo nextest run --workspace`, `npm run lint/typecheck/test --prefix pulse-app/ui`.

## Patterns to follow
1. Registry test fixture pattern (§4 §7): mock `InMemoryServiceRegistry` with lifecycle states (Active/Bootstrapping/Quiet tagged live; Silent/Dormant/Archived/Unknown inactive).
2. Integration pattern (§2 §5): Seed synthetic spans via OTLP gRPC `:4317` → buffer ingests → TauRPC `services.list_with_states` → assert state filtering + recency labels honest.
3. Webview pattern (§4): `vitest` renders constellation with mock `services.list_with_states` payload; asserts live services visible, inactive/persisted hidden or labeled historical.
4. Mock service builder (§7): `MockService::builder().name(…).state(…).last_seen_unix_nano(…).build()` for fixture data.

## Anti-patterns to avoid
1. No raw registry-struct dumps in assertions — assert specific fields only (state, name, label).
2. No pre-baked service-list JSON snapshots — seed via OTLP or builder factories per §7.
3. No manual UI verification (visual inspection) — constellation state verified via DOM queries only.

## Contract bindings
TauRPC `services.list_with_states` resolver binds to ui-bridge §3 (JSON response shape required); registry liveness classification binds to lifecycle state machine (state machine itself preserved per chunk boundaries); webview constellation a11y binds to a11y domain if interactive.

## Acceptance criteria contributions
1. "(tests) `cargo nextest run -p triage` passes" — unit tests for liveness classification in registry.
2. "(tests) `cargo nextest run` on services router integration passes" — `services.list_with_states` resolver filtering + recency.
3. "(tests) `npm run test --prefix pulse-app/ui` passes" — constellation component state-filtering + recency-label correctness.
4. "(tests) Coverage: new code in registry / resolver / constellation ≥ 75% line per §10".
5. "(tests) Fixtures via builder factories (§7)" — mock services, no literals where OTLP-seeding fits.
6. "(tests) Zero-telemetry scenario passes" — fresh buffer (no spans) asserts `services.list_with_states` shows no live services or only historical entries.

## Relevant amendment history
**2026-05-10 — Chunk-gate baseline coverage** (amendment `2026-05-10T12-41-03-...-mandate-standard-chunk-gates`): every chunk plan MUST list the standard gate set unconditionally — fmt / clippy / nextest / capability-drift + (webview touched) npm lint/typecheck/test. This chunk touches `pulse-app/ui/**`, so all gates apply. Rationale: without the unconditional baseline, gate-coverage drift accumulates (verified chunk #37).
