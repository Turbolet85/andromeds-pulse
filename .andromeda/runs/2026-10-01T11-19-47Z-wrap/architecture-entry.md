
## 2026-09-30-span-level-redaction — span-masked identity and egress
**Section:** §Conventions → Primary key convention · §Occupied Resources → Out-of-data-dir egress sink (training export)
**Change:** Primary key convention — was: since the metric name is scrubbed at ingestion, two DISTINCT credential-shaped names redact to one placeholder and collide; now: the metric name is span-masked (`mask_secret_spans`), so two names collide only when they differ solely inside a masked span (a single-token credential-shaped name still masks whole; a multi-word name keeps its non-secret words); `seq` stays the disambiguator. Training-export egress — was: PII scrubbed via `scrub_attribute`; now: span-masked via `mask_secret_spans`, the `interpretation` field (a serialized `L4Output`) masked per JSON string leaf so it stays parseable, non-JSON text masked whole, and `count_redactions` counting every field CONTAINING a `[redacted:` placeholder.
**Why:** both sites named the retired whole-value mechanism. A Boundary widening ratified by the founder at P4 2026-10-01, «Ок давай по типу правила».
**Kept:** §Established Decisions [Fault Identity] — it states no derivation of `scope_id` from the scrubbed service name, so it carries nothing to amend; the L1 fingerprint stays computed from the raw stacktrace.
**Ref:** .andromeda/runs/2026-10-01T11-19-47Z-wrap/
