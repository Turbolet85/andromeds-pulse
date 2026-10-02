# Fan-out results — 2026-08-22-pii-scrubber-recall

7 doc-agents, one per spec source. Report is the single input.

| doc | verdict | proposals |
|---|---|---|
| arch | clean | 0 — no resource of any registered class landed; deps none; two near-misses correctly declined (`:4318` JSON prose already hedged "optional" in §Conventions; pre-existing "twelve vs fourteen library crates" prose, covered by arch's own count-words-defer-to-§Occupied-Resources disclaimer) |
| security-plan | clean (**MISS — see below**) | 0 returned, bare, no reasoning |
| design-system | clean | 0 — all three new surfaces carry `tokens n/a`; zero UI files touched |
| layout-templates | clean | 0 — no user-facing surface/region; counter rides a log event, not the footer/tray |
| test-plan | 5 proposals | 2 primaries + 3 dependents |
| obs-plan | 3 proposals | 1 primary + 2 dependents |
| a11y-plan | clean | 0 — no interactive element; violation schema untouched; the new field is inside `fields`, not the envelope |

## test-plan (5)
- **P1 · D-tests-coverage · §4 "What unit tests cover"** — add a `security` bullet (8-arm catalog, suite 14 → 23).
- D · §2 pyramid Unit row — add `security` to the per-crate list. `dependent-of: D-tests-coverage`
- D · §4 Conventions → Test file location → Rust — add `security` (co-located, not the pulse-app exception). `dependent-of: D-tests-coverage`
- **P2 · D-tests-coverage · §1 Pending coverage triggers** — NEW row `buffer-redaction-counter-unit-coverage`: three new public `buffer` items shipped with no in-crate test; proven only cross-crate (3 pulse-app guards + e2e counter-advance + one boot-smoke wire observation).
- D · §4 `buffer crate` bullet — qualify so the roster does not read as in-crate coverage. `dependent-of: D-tests-coverage`

Not fired: **D-tests-framework** clean (every runner matches §2/§4/§3; `-E` under `--workspace`, not `-p`; webview gates correctly omitted; direct-binary smoke variant is the documented form). **D-tests-obs-harness** clean (no harness/status/log-format contract change; the tick field is additive). Notably, the detector confirms test-plan §3 was RIGHT and the chunk plan was wrong about the boot-smoke trigger.

## obs-plan (3)
- **P · D-obs-instrumentation · §5 tick-aggregated Counter row** — record `redactions_applied` as a SECOND tick-aggregated counter, folded by the new `BufferState::record_redactions`, scoped to the OTLP persistence path.
- D · §1 Heartbeat ticks (`:101`) — extend the `buffer.tick` field enumeration. `dependent-of`
- D · §8 default-deny `buffer` whitelist (`:516`) — add the leaf, aggregate-count-only qualified. `dependent-of`

All three sites pre-verified by the orchestrator at `:101`, `:355`, `:516` — a §8-only apply would have left two stale.

Not fired: **D-obs-stack** clean (no dep, rides existing `tracing` heartbeat). **D-obs-pii** clean on its own invariant (aggregate only; canary literal 0 times in the obs log).

## Orchestrator self-raised (validate checks 5 + 6)

**security-plan under-ran the plan's expected-amendments floor.** The plan named `security-plan §Security Anti-Patterns → Logging` as the site of record for the catalog's shape; the agent proposed nothing and gave no reasoning. Raised here:

- **S1 (routine-APPLY)** — §Security Anti-Patterns → Logging, "Uniform scrubber coverage" paragraph (`:423`): the catalog is now EIGHT categories, the eighth being `provider_key` (bare-credential recall, anchored-prefix + length floor). No amendment has ever enumerated a scrubber pattern; the report's Counts bullet records the 7 → 8 move.
- **S2 (ESCALATE — blocked on owner)** — the SAME paragraph claims coverage of "OTLP appender DuckDB writes for `spans` / `log_records` / `span_events`". **Measurement disproves the `spans` half**: the actual scrub sites are the log body (`appender.rs:291`) and exception message/stacktrace (`:368`/`:369`); the `spans` table receives NO scrub, its only client-controlled string being `service_name` (`extract_service_name` `:45` → push `:58` → `StringArray` `:74` → column `:83`, no `scrub_otlp_field` on the path). Its stated invariant — "any attribute value crossing into persistent storage … passes through `scrub_attribute()`" — is falsified by five columns. Per the 2026-08-15 APPLY-AS-MEASURED rule this is applicable ONLY with the owning route entry named; that owner is the P5 trajectory decision, so this escalates.

**Check-6 disposition of the report's disproved-claims bullet:**
- #1 five unscrubbed columns → **S2, escalated** (owner needed).
- #2 the chunk plan's false boot-smoke claim → **DISPOSED, no spec amendment**: test-plan §3 was correct and the plan was wrong; the test-plan detector independently confirms this. Recorded in the report's Deviations; routed to P3 curation as a lesson.
- #3 `exception_type` deliberately raw → **DISPOSED, no action** (a documented decision, not a gap).
- **NEW, surfaced independently by BOTH arch and test-plan** — OTLP/HTTP returns **415** for `application/json`; this build is protobuf-only. Contradicts test-plan `:31`, `:58`, `:98`, `:379`, `:395` (JSON-body test recipes) and arch §Stack `:17` ("protobuf and JSON"), though arch §Conventions `:76` already hedges "HTTP/JSON encoding optional". Neither agent proposed it (correctly — the HTTP receiver is not in the report's Changes; pre-existing, not this chunk's drift). **Escalated** for disposition since it spans two masters.

**Expected-amendments reconciliation (the coverage floor — every entry disposed):**
| plan entry | disposition |
|---|---|
| test-plan §2 + §4 ×2 — add `security` | ✅ proposed by detector (3 sites) |
| obs-plan §5 + §8 — register the counter | ✅ proposed by detector, **plus §1** the plan did not anticipate |
| security-plan §Anti-Patterns Logging | ⚠️ **not proposed — orchestrator-raised as S1/S2** |
| architecture.md — "none expected" | ✅ **confirmed** by arch's `proposals: []` (a no-change disposition made by confirming, not skipping) |

**Plan-vs-shipped naming:** the plan and the operator-selected option preview both named the field `redactions_total`; the shipped name is `redactions_applied`. All detector proposals correctly use the SHIPPED name. Divergence recorded in the report's Deviations and raised to the operator.

## Cross-contradiction / intent-consistency / absence-evidence
- **Cross-contradiction:** none — no two proposals edit the same section in opposing directions.
- **Intent-consistency:** report matches the working-route entry + plan acceptance criteria; all 5 deviations carry justifications.
- **Absence-needs-evidence:** both absence claims independently re-derived by the orchestrator — test-plan's `grep -c 'crates/security'` = **0**, and obs-plan's three restating sites at `:101`/`:355`/`:516`.
