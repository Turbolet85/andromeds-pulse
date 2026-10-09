# Fan-out results — 2026-09-29-scrubber-path-false-positive (wrap 2026-09-30T06-23-56Z)

Seven Explore doc-agents ran in one parallel batch; the prompts are in `.prompt-{doc}.md`. No master is migrated, so every contracts line was dropped (`registry.py contracts` → `NOT MIGRATED` / n/a). Each return was parsed after stripping its trailing `#` commentary. Entity probe: 0 HTML entities in any return, and no return failed to parse, so no raw twin was saved.

| doc | verdict |
|---|---|
| architecture | `proposals: []` — D-arch-resources: no new resource; private items only. D-arch-decisions: no Rust dep; npm in-range bumps are not a Stack change |
| security-plan | 3 proposals — below |
| design-system | `proposals: []` |
| layout-templates | `proposals: []` |
| test-plan | 1 proposal — below |
| obs-plan | `proposals: []` — §5 `redactions_applied` states unit/semantics, not a count; §8/§10 carry no credit_card claim (0 hits) |
| a11y-plan | `proposals: []` |

## security-plan

**S1** · D-security-logging · warning · §Security Anti-Patterns → Logging → "Uniform scrubber coverage" (`security-plan.md:428`)
- Change: record the credit_card arm's Luhn-on-whole-group-windows predicate (candidate regex, 13–19 digit whole-group windows, replacing the checksum-free regex), the accepted recall trade, and closure of the harness-basename / 19-digit-timestamp false positive. P-048 whole-value replacement and `redactions_applied` are unchanged, and "recall over precision" is scoped to the other seven arms.
- Disposition: **apply, as an escalation resolved**.
  - Check 1: the Boundary-widening rule (escalate) GOVERNS. The validated scrub boundary now lets a new input class through unredacted: Luhn-invalid digit runs.
  - "Accurate this-chunk addition" (routine) also matches the subject. The escalate verdict wins because Boundary widening forbids a routine rule for its class.
  - Resolution: the FOUNDER's ratification, clicked live at /phase P4 on 2026-09-30 ("Luhn on windows"). It is restated in the operator relay `pc-overseer/relays/pulse-wrap-scrubber-2026-09-30.md` §1 as "the Boundary-widening ratification", and is recorded in the sidecar entry.
  - Checks 2–6 pass: no opposing proposal; intent-consistent with the scope.md P5 amendment; absence claims cite `grep -n 'recall.over.precision|recall-over-precision' .andromeda/*.md` (masters: `:428`, `:436` only); expected amendment 1 covered; no disproved claims.

**S2** · D-security-logging · warning · dependent-of D-security-logging · §Logging → "A THIRD scrub SHAPE" failure mode (`:436`)
- Change: "per the catalog's own recall-over-precision posture" is scoped to the key-anchored arms.
- Disposition: **apply** (atomic with S1). It is a same-master duplicate of the retired catalog-wide claim, found by the sweep above.

**S3** · D-security-logging · warning · dependent-of D-security-logging · §Logging → "Residual, unchanged by this chunk" (`:444`)
- Change: tie "The 8-category catalog is unchanged by this chunk: no arm added, reordered, or retuned" to its origin chunk, and note the later credit_card predicate retune.
- Disposition: **apply** (atomic with S1). The unscoped "this chunk" reads as current and contradicts the retune. The origin is `2026-08-23-ingestion-scrub-coverage`: the paragraph's own evidence, the `redactions_applied` 0 → 4 wire run and the closed route entry, is that chunk's (`security-plan-amendments-archive.md:184`, read).

**S4** (raised by the orchestrator) · §Dependency Security → npm channel (`pulse-app/ui`) → current state (`security-plan.md:221`, anchor-unique)
- Change: add this chunk's re-read (still `green-with-dispositions`, same exceptions) and the five closed GHSAs with their in-range bumps.
- Disposition: **apply, routine**.
  - Rule "Accurate this-chunk addition": the report's Dependencies bullet carries the bumps and the GHSA ids.
  - No detector binds to the npm current-state reading: D-security-deps checks bans only, and no drift-base detector reads this site. Its absence from the fan-out is therefore structural, not a finding.

## test-plan

**T1** · D-tests-coverage · warning · §4 Unit Test Strategy → security crate (`test-plan.md:366`)
- Change: re-sync the stale `(crate suite 14 → 23)` to the measured count (54, 39 → 54 this chunk), and name the card arm's two new corpora.
- Disposition: **apply, routine**. Rule "Accurate this-chunk addition": the count and corpora are this chunk's (report Counts bullet). It is expected amendment 2.
  - Check 4: the agent swept `crate suite|14 → 23|credit_card|Luhn` → `:366` only. `:126` is an audit-trail row, not a current count.

## Validate summary
- 4 fan-out proposals plus 1 raised by the orchestrator (S4): 5 apply, 0 reject.
  - S1: escalate, resolved by the founder's recorded ratification.
  - S2, S3: its dependents.
  - S4: routine.
  - T1: routine.
- Check 5, expected amendments: both covered, security-plan by S1 and test-plan by T1.
- Check 6, disproved claims: none listed in the report, so nothing to dispose.
- Check 3, scope record: one `widening`, `pulse-app/ui/package-lock.json`, carrying the founder's word via the overseer. The justified branch; `serves` holds (the CI read of the pushed head went green after it).
