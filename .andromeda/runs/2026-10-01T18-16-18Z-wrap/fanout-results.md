# Fan-out results — 2026-10-01-real-model-incident-surfacing

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim (contracts line dropped for all seven:
security / design / layout carry no keyed-contract section; arch / tests / obs / a11y `registry.py contracts` exit 3
NOT MIGRATED). Returns were YAML with `#` commentary lines (stripped; no entities present — 0 `&lt;` / `&gt;` / `&amp;`).

## Verdicts
- architecture — 2 proposals (stripped: commentary on D-arch-decisions no-drift, obs target belongs to obs-plan, probe reads registered env only, re-exports not registry resources, :322 CI/CD line generic)
- security-plan — proposals: [] (stripped: per-detector no-drift basis — no new input surface; no auth touch; basic-ftp override not on the denylist, npm-policy unchanged; skip leaf has bounded fields, no scrub-coverage claim moved)
- design-system — proposals: [] (stripped: no UI rendered; no status surface touched)
- layout-templates — proposals: [] (stripped: no surface/region added; no status claim touched)
- test-plan — 5 proposals (stripped: none beyond the YAML)
- obs-plan — 1 proposal (stripped: D-obs-instrumentation / D-obs-stack / D-obs-defect-narrative no-drift basis)
- a11y-plan — proposals: [] (stripped: no interactive UI element; schemas unchanged)

## Proposals + dispositions

### architecture
1. D-arch-resources · warning · §Occupied Resources → xtask CLI surfaces — register `cargo xtask check:english-sources`
   (source_lint.rs; exit 0/1/2 + verdict + twin; ci.yml lint-test step + pre-push `source-lint` stage). basis `architecture.md:244`.
   **Disposition: apply** — check 1 playbook "Accurate this-chunk addition" (routine; the verb is in the report's Symbols +
   Harness bullets); check 5 carries the plan's arch xtask-surfaces entry.
2. D-arch-resources · warning · same section — `pre-push:linux` stage list 5 → 6, `source-lint` second. basis `:244`.
   **Disposition: apply** — check 1 "Accurate this-chunk addition"; check 5 (same plan entry). Check 4: :244 is a 14 231-char
   line — the stage-list site is located by offset at apply (splice summary + window), never from a clipped view.

### test-plan
3. D-tests-coverage · warning · §4 → interpretation crate — lineage v2.2 / v1.1-fallback / v1.1-reflection → v2.3 /
   v1.2-fallback / v1.2-reflection, and the schema-order pin. basis `test-plan.md:380`.
   **Disposition: apply** — check 1 "Accurate this-chunk addition"; check 5 (plan's §4 entry, A5 branch).
4. D-tests-coverage · warning · §4 → triage crate — the OVERALL render surface + 3 `overall_line_*` pins + the
   fixture-to-render pin. basis `:379`. **Disposition: apply** — check 1 "Accurate this-chunk addition"; check 5 (§4 entry).
5. D-tests-coverage · warning · §1 → Pending coverage triggers — new row for the untested `l4_decision_probe` input
   boundary. basis `:143` (the `inject-demo-arg-parse-unit-coverage` precedent). **Disposition: apply** — check 1
   "Accurate this-chunk addition" (the report's Coverage row reads `tests ✗ (dev-only by design …)` — a pending trigger
   records the gap, owes no work now). Re-derivation tell checked: the "`[[example]]` must declare `test = true`" clause is
   test-plan/testing-rule prose already in the corpus (testing.md 2026-08-29), not a source re-read — kept as a note.
6. D-tests-framework · warning · §9 → lint-test row — name the "No Cyrillic in sources" step, now `cargo xtask
   check:english-sources`. basis `:641`. **Disposition: apply** — check 1 "Accurate this-chunk addition"; check 5 (§9 entry).
7. D-tests-obs-harness · warning · §3 → `pre-push:linux` paragraph — six-stage list, keep the dated 5/5 reading, add the
   2026-10-01 6/6 ×2 reading. basis `:325`. **Disposition: apply** — check 1 "Accurate this-chunk addition"; check 5 (§3
   entry). The bind test-plan §3 ↔ obs-plan §3 holds: obs-plan §3 carries no pre-push stage list.

### obs-plan
8. D-obs-pii · escalate · §8 → default-deny allowlist — register exact leaf `interpretation.incident.skipped`
   {skip_reason, decision, severity, digest_kind}. basis `obs-plan.md:539`.
   **Disposition: apply (routine)** — check 1: playbook "Drift proposal (D-obs-pii, escalate severity) registering a NEW
   self-observation logging target the CURRENT chunk introduced" — both load-bearing conditions hold: (a) the leaf names
   ALL FOUR emitted fields (report Symbols + Schema bullets); (b) the guard is `pulse-app/tests/unit_observability_allowlist_incident_skip.rs`,
   where it runs. Its duplicate-occurrence half (a once-per-boot WARN registered in §6 too) does not apply — the target is an
   INFO record per non-creating generation, not a once-per-boot WARN; the §6 enumeration is checked in the cascade sweep.

## Validate — the six checks
1. Playbook — 8/8 routine (above); no rule collision.
2. Cross-contradiction — none: proposals 1/2 edit one arch line additively; 3/4/5/6/7 touch distinct test-plan sections.
3. Intent-consistency — the report's scope record carries one `widening` (`pulse-app/ui/package.json`) WITH the overseer's
   word → the justified branch; the `mechanical` lockfile line serves it. The A1 → A5 fall-through is the founder ruling
   (2026-10-01) applied through the pre-registered rule — the plan's own decision path, no intent divergence.
4. Absence needs evidence — the arch agent's "no other occurrence of the five-stage list" and the tests agent's "no other
   occurrence of the stage list or the v2.2 lineage" are re-checked by the cascade sweep (patterns over all seven masters,
   offsets on long lines), not taken from the agents.
5. Expected amendments — 8 plan entries: 5 carried by proposals (arch xtask surfaces → 1,2 · tests §3 → 7 · tests §9 → 6 ·
   tests §4 → 3,4 · obs §8 → 8); 3 not carried with the report's stated reason (arch [LLM Inference Runtime] and tests §1
   `l4-path-guard-callsite-wiring-coverage` — A1 not shipped; security L4 argv row — A3 not shipped). No orchestrator raise owed.
6. Disproved claims — (1) "the model dismissed" (scope + frozen route CONTEXT): chunk-artifact claim, recorded in the
   report, no amendment owed; the route entry is compacted at the flip. (2) research E5 / the seed question: chunk-artifact
   facts, recorded. (3) the plan's "no `pulse-app/ui/**` path touched": plan prose, recorded with the webview gates' green.
   All DISPOSED.

Escalations: 0.
