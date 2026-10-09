# Mutation checks — 2026-10-04-retry-storm-interpretation-names-its-cause

One-shot controls from plan Step 9, run at /implement on 2026-10-04. Each was a workspace nextest run with
`--profile ci --no-fail-fast`. Logs are in `.andromeda/runs/2026-10-04T14-17-26Z-implement/`
(`red-before.log`, `green-after.log`, `mutation-title-line.log`).

## 1. Red before green (the untouched producer)

The Step 7 and Step 8 pins were written first and run against the producer at the chunk base, before Steps
1-5. The Step 2 label pin did not exist yet, because `cue_cause_label` did not exist.

Filter: `test(/names_its_cause|cause_label_names_retry/)`. Result: **8 tests run: 0 passed, 8 failed**.

Every failure is the expected one: the title equals the bare model title.

| pin | left (actual) | right (expected) |
|---|---|---|
| `incident_title_names_its_cause_for_a_retry_storm` | `Error Rate Spike in ws` | `Retry storm: Error Rate Spike in ws` |
| `incident_title_names_its_cause_without_retry_for_an_error_rate_spike` | `Errors climbing` | `Error-rate spike: Errors climbing` |
| `incident_title_names_its_cause_on_dedupe_refresh` | `second take` | `Retry storm: second take` |
| `incident_title_names_its_cause_in_the_resolution_summary` | `storm subsided` | `Retry storm: storm subsided` |
| `incident_title_names_its_cause_for_a_reflection_digest` | `slow drift` | `Reflection trend: slow drift` |
| `incident_title_names_its_cause_and_still_masks_secrets` | `rotate [redacted: provider_key] after [redacted: secret_kv]` (no prefix) | starts with `Retry storm: ` |
| `report_names_its_cause_for_a_retry_storm_incident` | `# Diagnostic Report: Error rate spike in payment-service` | `# Diagnostic Report: Retry storm: Error rate spike in payment-service` |
| `report_names_its_cause_under_the_deterministic_runner` | `# Diagnostic Report: Deterministic verification incident` | `# Diagnostic Report: Retry storm: Deterministic verification incident` |

The masking pin's red reading also shows both canaries were already masked at the base. The pin fails only on
the missing cause prefix.

## 2. Green after Steps 1-5 (Step 6 pins moved)

Filter: the line above plus `test(=distinct_fingerprint_same_service_still_coalesces_to_one_incident)` and
`test(=pii_canary_in_l4_text_is_scrubbed_before_persist)` (the two Step 6 pins). Result: **11 tests run: 11
passed**.

## 3. Mutation: Step 4's `title:` line reverted alone

`create_incident_from_l4_output`'s `title: scrub_text(&grounded.title)` was reverted to
`title: scrub_text(&parsed.title)`. The mutation was confirmed applied before the run: the reverted line was
present exactly once. Same 11-test filter. Result: **11 tests run: 5 passed, 6 failed**.

- RED (creation pins): `incident_title_names_its_cause_for_a_retry_storm`,
  `incident_title_names_its_cause_without_retry_for_an_error_rate_spike`,
  `incident_title_names_its_cause_and_still_masks_secrets`, `incident_title_names_its_cause_for_a_reflection_digest`,
  plus the two Step 6 pins `distinct_fingerprint_same_service_still_coalesces_to_one_incident` and
  `pii_canary_in_l4_text_is_scrubbed_before_persist`.
- GREEN (JSON paths, unaffected by the title line): `incident_title_names_its_cause_on_dedupe_refresh`,
  `incident_title_names_its_cause_in_the_resolution_summary`, both `report_names_its_cause_*` pins, and the triage
  label pin.

**Deviation from the plan's prediction:** Step 9 listed the Step 7 first, second and sixth bullets as the creation
pins that go red. The fifth bullet, `incident_title_names_its_cause_for_a_reflection_digest`, ALSO went red. It
asserts the creation `title` too, so it is a creation pin. The criterion "reverting Step 4's `title:` line reddens
the creation pins while the dedupe and resolution pins stay green" holds as stated.

The line was restored after the run. The final green reading is the plan's listed targeted entry, run through the
gate tool in this run.
