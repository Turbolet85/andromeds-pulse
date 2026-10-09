# obs extract

## Relevance
partial. The chunk changes a security primitive (`scrub_attribute`'s `credit_card` arm). Obs is involved only because the scrubber feeds a counted metric, sits behind basename-only log rules for the workspace key, and every debug emit added while tuning it falls under the default-deny logging posture. The chunk adds no spans, targets or tick fields.

## Constraints
- obs-plan §5 (Metric Coverage, the `redactions_applied` counter row) requires the counter to count ONE increment per redacted cell or label pair, on PERSISTED cells only. A precision fix in the card arm changes what counts as a redaction at all five buffer scrub sites. The counter must keep its unit and its post-append fold site; only its input set should shrink, by exactly the removed false positives. Whether any site's tally depends on `credit_card` specifically is research's question.
- obs-plan §5 and §8 (`buffer` allowlist entry) require `redactions_applied` to stay an aggregate count. It must never carry the matched value, its category, or the attribute key. The chunk must not add a per-category breakdown (for example a `credit_card_redactions` field) to "prove" the fix. That would introduce a category label the allowlist forbids.
- obs-plan §8 (`app.boot.workspace_key` leaf) and §4 P7 (Required log fields) require basename plus `key_bytes` only on the INFO line. The full workspace root path, the key value and the `{data_dir}/run/workspace-key` path are never emitted. Tracing or debugging the key's route to the model's PROJECT line must not log the key or the basename of the data-dir run (`rm-…`) outside that leaf.
- obs-plan §8 PII Scrubbing (Default-deny posture) requires every new field on any target to be allowlisted by an EXACT leaf with the complete field set. The same section treats a bare-prefix fallback as a silent redactor. If research leads to any new or changed emit near the scrubber, drain or workspace-key paths, it needs its leaf. Otherwise it needs no emit at all.
- obs-plan §8 (UI-vocabulary exemption) requires secret-format heuristics to tell real secret formats from look-alike text, because over-broad patterns produce false positives. This is the obs-side rationale for adding precision to the card arm rather than exempting a field.
- obs-plan §11 Metrics (hot-path level-gating and per-field emission ban) and §5 (the `redactions_applied` row: "Per-field emission is barred by the §11 hot-path rule") forbid a per-scrub-decision log or event, including a "false positive avoided" record. Scrub decisions stay uncounted beyond the existing aggregate.

## Patterns to follow
- The §5 `redactions_applied` fold. Builders return a per-batch tally and `record_redactions` folds it at each table's post-append site in `consumer::dispatch_batch`. After the fix, this is the outside-the-process discriminator: a canary batch holding `rm-20260923-093840` in a scrubbed cell should add 0 where it used to add 1. A real card value in the same batch should still add 1.
- §8's canary discipline (the PII halves beside allowlist guards, e.g. `pulse-app/tests/unit_incident_producer.rs`). Pin both directions: known positives still reach the log as `[REDACTED:…]` or `[redacted: …]`, and the measured false positive reaches it as-is only where that surface is allowed to carry it.
- §8 per-leaf guard files under `pulse-app/tests/` (the `[lib] test = false` rule). Any obs assertion this chunk adds belongs there or in `crates/security`'s own tests. A src-level `mod tests` in pulse-app never runs.

## Anti-patterns to avoid
- obs-plan §11 Logs / §11 PII Scrubbing: never log the raw value being scrubbed, including while diagnosing why a digit run matched. A diagnostic that prints the input next to its `ScrubbedValue` verdict re-leaks exactly the class the scrubber exists to stop when the input really is a card number.
- obs-plan §11 Metrics: never add an unbounded or identity-bearing label, such as the matched category, the attribute key or `service_name`, to the redaction counter or to any tick in order to show the fix worked.
- obs-plan §11 PII Scrubbing ("NEVER hardcode scrubbing rules in a single location"): this is the obs-side reason the fix belongs in the shared `scrub_attribute`, which every layer inherits, rather than in a single consumer such as `project_context` or the L4 prompt assembly.

## Contract bindings
- obs ↔ security: §8 PII Scrubbing binds to security-plan §Logging & Monitoring and the P-047 catalog. The card arm's documented shape is owned by security. Obs owns only the count's unit and meaning (§5), so a changed card arm needs no obs amendment unless the counter's meaning changes.
- obs ↔ tests: the §3 Heartbeat ticks `buffer.tick` field `redactions_applied` is what harness smoke and legs read to grade P-047 recall from outside the process. A leg that relies on a date-time-shaped value being redacted would change verdict. Whether any existing leg or canary corpus depends on that is research's question.
- obs ↔ verification harness (folded watch): the Linux-boot WATCH reads `harness:status`'s `ended` record (`run/andromeda-pulse.exit`). That record is a harness artifact, not an obs-plan §3 surface. It is carried here only as a watch the chunk's CI runs advance (1/3). It is not an obs deliverable.

## Acceptance criteria contributions
- (obs) A canary run that ingests the measured false positive (`rm-20260923-093840`, in its workspace-key form) plus one known card positive through a scrubbed buffer cell shows `redactions_applied` on `buffer.tick` rising by exactly the positive's count. The false positive adds zero. (per obs-plan §5 Metric Coverage, `redactions_applied` row)
- (obs) No new or changed log or tracing emit carries a scrubber input value, a matched category, or a full workspace path or key. Every new field (if any) resolves to an EXACT allowlist leaf with its complete field set, asserted under `pulse-app/tests/`. (per obs-plan §8 PII Scrubbing, the Default-deny posture and the `app.boot.workspace_key` leaf)
- (obs) `redactions_applied` keeps its unit (one per redacted cell or label pair, persisted cells only) and gains no category or key label. (per obs-plan §5 and §11 Metrics)
