# Cascade dispositions — 2026-09-30-p-027-discovery-bound

**Search.**
- `cascade.py sweep --patterns-file cascade-patterns.toml` (baseline `71f33699`, the pre-CI parent). Ten patterns,
  each control fired on the pre-pass masters: `hue-leg` `smoke:hue-shift` · `scenario-leg` /scenario leg/i ·
  `sibling` /sibling scenario|two sibling|three sibling/i · `disc-ms` `discovery_ms` · `disc-latency`
  /discovery[ _]latency|discovery bound|auto-discovery/i · `span-arrival` /span.arrival/i · `tick-wait` /first (15
  ?s )?(lifecycle )?tick|registry tick|heartbeat tick/i · `extract-sn` `extract_service_name` · `tap` /SpanObserver`
  tap|SpanObserver tap|baseline tap|observer tap/i · `svc-registry` /ServiceRegistry/.
- Two patterns were refused by the tool (control unfired over the pre-pass masters) and hand-controlled instead:
  `triage.lifecycle.transition`, `last_seen_unix_nano`. `grep -rn` over `CLAUDE.md .claude/docs .claude/rules`
  outside `session-learnings.md` → 0 hits.
- Phrasings read beyond tokens: "the two sibling scenario legs" (arch :242); "THREE scenario legs" (tests-summary);
  the tap's consumer claim ("the baseline `SpanObserver` tap … baseline registry").
- Sections read: arch :177 / :242 whole entries; security-plan :434; test-plan §1 table (:111–:147) and §3
  scenario paragraphs (:315–:321); obs-plan §8 :542–:543.
- Leaf enumeration: the verb-listing leaves (`grep -rln 'smoke:gap-resume|smoke:external-resolve|smoke:hue-shift'`
  → obs-summary, tests-summary, rules/observability, rules/verification-harness) and the security leaves carrying
  the choke-point sentence.

## Rows

**hue-leg (3 standing, 1 leaf)**
- arch:242 edited, no change: the hue-shift entry stays true; the new discovery entry follows it.
- test-plan:319 · obs-plan:542, no change: true claims sharing the token.
- tests-summary:90, re-derived (see scenario-leg).

**scenario-leg (2 new, 4 standing, 3 leaf, 1 curation)**
- arch:242 ×3, amended this pass: the P-025 and new P-027 entries; "the two sibling scenario legs" →
  "the two remaining sibling scenario legs".
- test-plan:146 / :321, new text.
- test-plan:315 / :317 / :319, no change: each states its own ordinal (first / second / third), which stays true.
- rules/testing.md:250, curation: a historical Session Addition, no change.
- rules/verification-harness.md:89, re-derived: the §Scenario legs list gained `smoke:discovery`. It also gained
  `smoke:hue-shift`, which it had lacked since 2026-09-29 — recomputed from test-plan §3, not only from this pass.
- tests-summary.md:87 heading, no change.
- tests-summary.md:90, re-derived: "THREE" → "FOUR", with the fourth leg's verdict.

**sibling (2 standing, 1 curation)**
- arch:234, no change: `EvidenceRefs` fields (token share).
- arch:242, amended ("two remaining sibling").
- rules/testing.md:149, curation, no change (unrelated sibling tests).

**disc-ms (2 new, 3 standing, 2 leaf)**
- arch:177, amended.
- arch:242 / test-plan:321, new text.
- obs-plan:162, no change: it names the target in the metric-naming list and asserts nothing about its anchor.
- obs-plan:543, amended.
- rules/observability.md:72 and obs-summary.md:141, re-derived: the anchor sentence added.

**disc-latency (3 standing, 1 leaf, 1 curation)**
- arch:177, amended.
- obs-plan:41 / :113, no change: workspace "project discovery boundary", a token share.
- docs/services/triage.md:4, no change: "P-027 (Service Constellation Auto-Discovery)" is still true, and the doc
  states no registration timing.
- rules/testing.md:292, curation, no change (cargo auto-discovery).

**span-arrival (1 standing)**
- obs-plan:542, no change: it records that the P-025 hue sample "had measured span-arrival → paint before"
  chunk 2026-09-29, a true history of the hue leaf.

**tick-wait (2 new, 14 standing, 8 leaf, 2 curation)**
- arch:177 / obs-plan:543, new text.
- The 14 standing master rows (obs-plan :65 :101 :126 :127 :146 :253 :259×2 :372 :376 :378 :419 :620×2 :630
  :640), no change: each describes heartbeat-tick emission cadence or stall detection, and none states that a
  dot / registry listing waits for the lifecycle tick.
- The 8 leaf rows (rules/observability :79 :83; verification-harness :15 :45; obs-summary :58 :64 :73 :77), no
  change, same reason.
- rules/testing.md:290 and session-learnings.md:1937, curation, no change (historical).

**extract-sn (3 standing, 3 leaf, 3 curation)**
- security-plan:434, amended.
- security-plan:436, no change: "one fn feeding three consumers" stays true (the call sites are unchanged).
- obs-plan:356, no change: the non-storing callers (storm observer, baseline tap) pass `None`, still true.
- CLAUDE.md:32, no change: "the choke point for its three consumers" stays true.
- rules/observability.md:81, no change (`None` callers).
- rules/security.md:58, re-derived: the tap fan-out sentence added.
- session-learnings.md :1691 :2507 :2516, curation, no change (historical).

**tap (1 new, 3 standing, 3 leaf, 1 curation)**
- security-plan:434, amended.
- test-plan:146, new text.
- obs-plan:356, no change.
- obs-plan:650, no change: a historical probe narrative of defect 4.
- rules/observability.md:99 · obs-summary.md:86, no change: the same historical narrative.
- rules/security.md:58, re-derived.
- session-learnings.md:2509, curation, no change.

**svc-registry (3 new, 3 standing, 1 leaf, 3 curation)**
- arch:177 · security-plan:434 · obs-plan:543, new text.
- arch:183, no change: `services.list_with_states` → `InMemoryServiceRegistry::list`, still the dot's source.
- security-plan:166 / :432, no change: the corpus-persist scrub of the registry, unchanged.
- docs/services/triage.md:33, no change: `Arc<dyn ServiceRegistry>` injection pattern, still true.
- session-learnings.md :598 :669 :712, curation, no change.

## Leaves re-derived (step 3)
- `.claude/docs/tests-summary.md`, from test-plan §3.
- `.claude/rules/verification-harness.md` §Scenario legs, from test-plan §3.
- `.claude/docs/obs-summary.md` and `.claude/rules/observability.md`, from obs-plan §8.
- `.claude/rules/security.md`, from security-plan §Security Anti-Patterns → Logging.
- No change on recompute:
  - CLAUDE.md `GENERATED:setup:*`: the xtask description enumerates no scenario legs; the arch amendments are in
    §Occupied Resources, which the pointer table references by name only.
  - `.claude/docs/security-summary.md`: it does not restate the choke-point paragraph (0 hits).
  - `.claude/rules/testing.md` body: its pending-trigger list does not mirror the per-chunk wiring rows —
    `viz-read-connection-router-wiring-coverage`, `damper-shared-instance-wiring-coverage` and
    `snapshot-resolver-level-coverage` all read 0 in the body.
  - `.claude/docs/services/triage.md`.
- Lateral binds: test-plan §3 ↔ obs-plan §3 are unchanged; this pass adds a scenario leg, not a harness command or
  a status/log shape. a11y ↔ obs schema: untouched.
- Judgment bases (playbook / drift-base): 0 `base` rows.
