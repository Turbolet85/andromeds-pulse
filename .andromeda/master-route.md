# Master Route — andromeda-pulse

<!--
Cross-version immutable index. APPEND-ONLY via promotion in /andromeda-phase — route never adds records.
One record per promoted chunk, grouped under its version section `## {project}-{version}`:
  {marker} · {status: pending|complete} · {super-laconic description} · → {chunk folder link}
marker = {date}-{slug} (e.g. 2026-06-28-otlp-batching), minted at promotion in /andromeda-phase.

Pre-v3 history (v0.1.0 + v0.2.0) was built under the v2 pipeline and is NOT recorded per-chunk here
(see the forensic note below). v3 chunk records accumulate forward under `## andromeda-pulse-0.3.0`
and later version sections. Only `## {project}-{version}` headings are version sections — the
version-cursor scan reads those exclusively; the forensic note is a blockquote, not a section.
-->

> **Pre-v3 history (forensic — NOT a version section):** v0.1.0 + v0.2.0 shipped 100 chunks across
> Epochs 1–9, route complete 100/100 (through session 185), all built under the v2 pipeline. Not
> retro-converted to v3 per-chunk records. Full history lives in `.andromeda/route.md` (flat route +
> amendment log) + `.andromeda/phases/` (per-chunk research + plan).

## andromeda-pulse-0.3.0
2026-06-28-deterministic-env-gated-l4-mode · complete · Deterministic env-gated L4 mode (canned L4Output, no GPU/3B) · → andromeda-pulse-0.3.0/chunks/2026-06-28-deterministic-env-gated-l4-mode/
2026-06-28-tier1-incident-path-reliability · complete · Tier1 incident-path reliability — thread triggering cue cadence→digest so a storm yields one reliable incident · → andromeda-pulse-0.3.0/chunks/2026-06-28-tier1-incident-path-reliability/
