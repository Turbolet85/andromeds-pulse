# Fan-out results — 2026-10-04-corpus-key-creation-is-race-free

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim (contracts line dropped for every
doc: `registry.py contracts` → exit 3 `NOT MIGRATED` for architecture; the plans carry no migrated keyed contracts).
Returns were plain YAML; no entity-escaped characters observed; no raw twin warranted (no `proposals: []` return
changed by stripping beyond trailing `#` commentary lines, no parse failure).

## Verdicts
- architecture — 8 proposals
- security-plan — 11 proposals (trailing note: §Security Anti-Patterns → Universal `rust-toolchain` ≥ 1.85.0 ban, out of its detectors' scope)
- design-system — `proposals: []` (commentary stripped: no UI, no status claim touched)
- layout-templates — `proposals: []` (commentary stripped: no surface, no status claim touched)
- test-plan — 2 proposals
- obs-plan — `proposals: []` (commentary stripped: no hot path, no new target, no §10 defect touched; CI attempt-1 red is a watched residual, not a §10 defect)
- a11y-plan — `proposals: []` (commentary stripped: no interactive element, no schema change)

## Proposals and dispositions

### architecture
| # | detector | section | disposition |
|---|---|---|---|
| A1 | D-arch-resources | §Occupied Resources → Filesystem locations — register the corpus-key lock file | **escalate** (check 1: playbook "Boundary widening" → escalate; a second product-written artifact outside data-dir confinement) |
| A2 | D-arch-resources (dependent-of A1) | §Occupied Resources → Out-of-data-dir egress sink — "the ONE deliberate exception" → one of TWO | **escalate** with A1 (dependent group) |
| A3 | D-arch-resources | §Occupied Resources → Corpus SQLite → At-rest posture — concurrent first creation converges (locked create-or-read with read-back) | **apply** (check 1: "Accurate this-chunk addition" → routine; expected amendment 2) |
| A4 | D-arch-decisions | §Stack and Technologies — "rustc 1.84+" → pinned 1.95.0, code floor ≥ 1.89, declared rust-version 1.85 stale → owner route entry | **apply** (check 1: "Drift proposal ACCURATELY correcting a doc claim that a MEASUREMENT disproved, where an impl half DOES exist but is too large to ride this chunk" → routine; the impl half is the founder-ruled entry; expected amendment 3; disproved claim 2) |
| A5 | D-arch-decisions (dependent-of A4) | §Established Decisions → [Primary Language] title floor | **apply** with A4 (not a reversal: Rust 2024 stands; only the floor qualifier moves) |
| A6 | D-arch-decisions (dependent-of A4) | §Infrastructure Patterns → directory tree `rust-toolchain.toml` comment "pin rustc 1.84+" → "pin rustc 1.95.0" | **apply** with A4 |
| A7 | D-arch-decisions (dependent-of A4) | §Inherited Defaults → Language / runtime floor | **apply** with A4 |
| A8 | D-arch-decisions (dependent-of A4) | §Standard Contracts → `app_info` example `"rust_version": "1.84.0"` → `"1.95.0"` | **reject** (pre-check: the report carries no fact about what `app_info` reports for `rust_version` — its source could be the declared floor, not the toolchain; the proposed value is a re-derivation the report does not support; an illustrative example, left standing; the Rust-floor entry owns the value question) |

### security-plan
| # | detector | section | disposition |
|---|---|---|---|
| S1 | D-security-input | §Input Validation → CLI / env var inputs — register `XDG_RUNTIME_DIR` (Linux; lock dir; canonicalize + absolute + is-dir; fail-closed; unconfined by ratified design) | **escalate** (check 1: "Boundary widening" → escalate; a product-binary `*_DIR` read admitted outside the confine-under-data-dir rule) |
| S2 | D-security-input (dependent-of S1) | §Security Anti-Patterns → Input — the `XDG_RUNTIME_DIR` lock-dir carve-out | **escalate** with S1 |
| S3 | D-security-input (dependent-of S1) | §Threat Model Summary → Attack surface → CLI input — entry point gains `XDG_RUNTIME_DIR` | **escalate** with S1 |
| S4 | D-security-auth | §Secret Management → Runtime — the locked create-or-read creation flow; lock failure degrades as store-unreachable | **apply** (check 1: "Accurate this-chunk addition" → routine; expected amendment 4) |
| S5 | D-security-auth (dependent-of S4) | §Secret Management → Production / dev separation — once-per-machine key also under concurrent first run | **apply** with S4 |
| S6 | D-security-auth (dependent-of S4) | §Secret Management → What counts as secret → Corpus encryption key | **apply** with S4 |
| S7 | D-security-auth (dependent-of S4) | §Data Protection → At rest → Persistent incident corpus — concurrent convergence | **apply** with S4 |
| S8 | D-security-auth (dependent-of S4) | §Threat Model Summary → Data classification → corpus — convergence + lock-dir-unusable routes to the fallback branch | **apply** with S4 |
| S9 | D-security-auth | §Data Protection → At rest — per medium — the content-free lock file bullet (+ the shared-`/tmp` residual) | **escalate** with S1 (registers the out-of-data-dir artifact; the residual is carried by the report's Changes → On-disk artifact → Residuals (2), added before validation) |
| S10 | D-security-logging | §Security Anti-Patterns → Logging — full-path NEVER-log coverage set gains the lock dir / lock path | **apply** (check 1: "Accurate this-chunk addition" → routine; the report: failures carry no path, print census 1 = the test skip helper) |
| S11 | D-security-logging (dependent-of S10) | §Logging & Monitoring → What NEVER to log — same enumeration | **apply** with S10 |

### test-plan
| # | detector | section | disposition |
|---|---|---|---|
| T1 | D-tests-coverage | §4 → What unit tests cover → corpus row — the concurrent re-exec witness + lock-dir / fail-closed pins (82 → 87), clean-skip including under `env -i` | **apply** (check 1: "Accurate this-chunk addition" → routine; expected amendment 5) |
| T2 | D-tests-framework | §4 → Framework (Rust crates) — "libtest bundled with rustc 1.85+" → pinned 1.95.0, code floor ≥ 1.89, stale declared rust-version → owner | **apply** (same rule as A4; disproved claim 2) |

## Validate — the six checks
1. **Playbook** — dispositions above. No two-rule collision.
2. **Cross-contradiction** — none: A4–A7 and T2 state one floor (pinned 1.95.0 · code ≥ 1.89 · declared 1.85 stale, owner the route entry); A1/S9 and S1–S3 describe one artifact and one input.
3. **Intent-consistency** — the report's one unplanned surface, the function-scoped `#[allow(clippy::incompatible_msrv)]`, is justified by the overseer ruling of 2026-10-04 (keep it; Cargo.toml out of scope); scope record empty, `gate.py scope` clean. No unjustified divergence.
4. **Absence needs evidence** — S10/S11 rest on the report's print census (`git diff 76d6cca -- keychain.rs | grep -cE '^\+.*(tracing::|println!|eprintln!|dbg!)'` → 1, the test skip helper). The agents' "only occurrence" claims: test-plan rustc floor (`grep -nE 'rustc|msrv|toolchain'` → only :343 states a version — re-checked at cascade); arch floor sites (:14 :47 :269 :352 — the report's grep). Re-verified by the cascade sweep.
5. **Expected amendments** — all five carried by proposals: arch Filesystem locations (A1, escalated), arch At-rest (A3), arch Inherited Defaults + Stack (A4–A7), security Runtime + restating sites + Data Protection lock-file medium (S4–S9), test-plan §4 corpus row (T1). None raised by the orchestrator.
6. **Disproved claims** — (1) plan's stage-5 `env -i` arm claim: a chunk-artifact claim → recorded in the report (and `evidence/operator-pass.md`), no amendment owed; the hazard routes to curation (P3). (2) stale Rust floor: arch A4–A7 + test-plan T2 bring the masters to current truth; CLAUDE.md's Stack line re-derives in the cascade; security-plan §Security Anti-Patterns → Universal "NEVER let the rust-toolchain drift below 1.85.0" stays — a true minimum-pin ban, not a floor-of-code claim — and is named for the founder-ruled entry "The declared Rust floor matches the code" (route-resolve), together with `.claude/rules/security.md` §Rust toolchain. DISPOSED.

## Escalation (one group)
Boundary widening — A1 · A2 · S1 · S2 · S3 · S9: the lock file is a second product-written artifact outside data-dir
confinement, and `XDG_RUNTIME_DIR` a product-read `*_DIR` input deliberately not confined. Recommended resolution:
apply, under the founder's live ratification of 2026-10-04 relayed by the overseer (plan §Provenance → Decisions; P5
approval). Resolution recorded below after the operator's answer.

## Escalation resolved
Boundary widening group (A1 · A2 · S1 · S2 · S3 · S9) — **apply as ratified**, the operator's answer at this wrap:
the founder ratified the widening live on 2026-10-04 (morning), choosing the per-user lock keyed on the credential
entry under `$XDG_RUNTIME_DIR` himself, which covers both the lock-file artifact and the `XDG_RUNTIME_DIR` read input.
All six applied; the sidecars name the ratification; the four residuals stated in the bodies.
