# Fan-out results — 2026-09-30-p-027-discovery-bound

Seven Explore doc-agents in one parallel batch, each over `chunks/2026-09-30-p-027-discovery-bound/report.md` and its
own master. No master carries migrated keyed contracts (`registry.py contracts`: architecture / test-plan /
obs-plan / a11y-plan `NOT MIGRATED`, the other three `n/a`), so no contracts line was sent. No return needed
stripping beyond its leading YAML; no entity survived decode; no raw twin is warranted.

| doc | verdict |
|---|---|
| architecture | 2 proposals |
| security-plan | 1 proposal |
| design-system | `proposals: []` — tokens n/a on all four surfaces; no status claim moved (Halo DEFERRED, dot hue signature unchanged) |
| layout-templates | `proposals: []` — no UI surface/region added; no status claim moved |
| test-plan | 2 proposals |
| obs-plan | 1 proposal |
| a11y-plan | `proposals: []` — no interactive UI element; no schema change; 0 a11y-plan hits on discover / P-027 / tick / scenario leg |

## Proposals and dispositions

1. **architecture · D-arch-resources · §Occupied Resources → xtask CLI surfaces (`:242`)** — register `cargo xtask
   smoke:discovery` (verb, module, runner, verdict, exit contract, artifact) and make the "two sibling scenario legs"
   wording count the new sibling. basis `architecture.md:242`.
   → **apply** — check 1: playbook "Accurate this-chunk addition" (routine). "Registry over-reach" does NOT
   govern: this registry enumerates individual xtask verbs, and each prior scenario leg registered its own
   there. Checks 2–4 pass. Check 5 matches expected amendment 1.
2. **architecture · D-arch-resources · §Occupied Resources → Tauri IPC routes, the
   `record_constellation_discovery_latency` delegated-timing entry (`:177`)** — state the P-027 anchor (registry
   `last_seen_unix_nano`, stamped at first sighting since this chunk; previously the 15 s tick stamp). basis
   `architecture.md:177`.
   → **apply** — check 1: "Accurate this-chunk addition" (routine). Check 5 matches expected amendment 2. Check 6
   disposes the report's measured anchor-premise fact.
3. **security-plan · D-security-logging · §Security Anti-Patterns → Logging (`:434`)** — keep the three
   `extract_service_name` consumers; add that the tap drives one `CompositeSpanObserver` reaching baseline ·
   restart · first-sighting, so the scrubbed tap name also keys the lifecycle `ServiceRegistry` at first sighting;
   the desync rationale covers the lifecycle registry too. basis `security-plan.md:434`. Detector sweep: `:436`,
   `:48`, `:166` unaffected; the other "tap" hits are the Homebrew tap.
   → **apply** — check 1: "Accurate this-chunk addition" (routine). "Boundary widening" (escalate) examined and
   NOT met: no read-only channel gains a write, and no validated surface admits a new input class. The registry
   already received these same scrubbed names from the baseline's map via `tick_all`; only the instant moved.
   This is recorded in plan §Provenance leans. Check 5 matches expected amendment 6, applied with the corrected
   premise (the consumer count is unchanged).
4. **test-plan · D-tests-coverage · §1 → Pending coverage triggers (`:111`)** — add open row
   `discovery-observer-wiring-coverage` (the `main.rs` composition order is pinned only by a self-built composite
   and the live leg). basis `test-plan.md:111-145`.
   → **apply** — check 1: "Accurate this-chunk addition" (routine). Check 5 matches expected amendment 4.
5. **test-plan · D-tests-framework · §3 → Per-chunk gate discipline (after `:318`)** — register `cargo xtask
   smoke:discovery` as the fourth SCENARIO leg. basis `test-plan.md:318`. The detector's absence claim ("no other
   test-plan site enumerates or counts the legs") is re-derived at the cascade sweep.
   → **apply** — check 1: "Accurate this-chunk addition" (routine). Check 5 matches expected amendment 3.
6. **obs-plan · D-obs-instrumentation · §8 → delegated timing leaves → `metric.constellation.discovery_ms` (`:543`)**
   — add the anchor sentence (paint instant minus registry `last_seen_unix_nano`, stamped at first sighting since
   this chunk; graded by `smoke:discovery`). basis `obs-plan.md:543`. The cited matrix notes sentence is carried by
   the report's Changes, so this is not a re-derivation.
   → **apply** — check 1: "Accurate this-chunk addition" (routine). Check 5 matches expected amendment 5. Check 6
   disposes the anchor-premise fact.

## Validate checks, pass-wide
- Check 2, cross-contradiction: none. Proposals 2 and 6 state the same anchor in two masters, consistently.
- Check 3, intent-consistency: the report's deviations are justified (the precondition guard, the clippy fix, the
  port-slot sequencing, the operator pass on the operator's word). Scope record: none — `gate.py scope` clean,
  0 recorded. No divergence from the working-route entry at `:138` or the plan acceptance.
- Check 5, expected amendments: 6 of 6 plan entries matched by a proposal; no orchestrator raise needed.
- Check 6, disproved claims: "none new". The measured premise is disposed by proposals 2 and 6 plus the matrix
  notes sentence written at /implement.
- Escalations: 0.
