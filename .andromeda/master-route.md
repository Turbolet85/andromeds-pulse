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
2026-06-28-investigate-actions-functional · complete · Investigate actions functional — 4 buttons run real L4-backed analysis with visible progress + result/error · → andromeda-pulse-0.3.0/chunks/2026-06-28-investigate-actions-functional/
2026-06-29-window-geometry-movable-shell · complete · Window geometry + movable shell — sane default size/position (centered or remembered) + a working custom-titlebar drag region · → andromeda-pulse-0.3.0/chunks/2026-06-29-window-geometry-movable-shell/
2026-06-29-predictable-close-self-verify · complete · Predictable close + honest tray (close→tray with a clear running indication, dashboard closable) + a minimal agent-headful self-verify harness (P-063 + P-078) · → andromeda-pulse-0.3.0/chunks/2026-06-29-predictable-close-self-verify/
2026-06-29-window-size-constraints · complete · Window size constraints — min inner-size + aspect-ratio constraint for the glance widget (P-062) · → andromeda-pulse-0.3.0/chunks/2026-06-29-window-size-constraints/
2026-06-30-browser-chrome-suppression · complete · Browser-chrome suppression — suppress default WebView2 context menu + canvas image-save app-wide in production (P-064/P-065) · → andromeda-pulse-0.3.0/chunks/2026-06-30-browser-chrome-suppression/
2026-06-30-widget-to-dashboard-navigation · complete · Widget-to-dashboard navigation — in-widget "Toggle dashboard" button + unified Cmd/Ctrl+Shift+P toggle the dashboard (widget stays); + P-061 geometry & P-063 close/toast corrections (P-066) · → andromeda-pulse-0.3.0/chunks/2026-06-30-widget-to-dashboard-navigation/
2026-07-01-live-only-service-truth · complete · Live-only service truth — constellation shows only currently-live services; persisted/stale registry entries hidden/marked historical + honest recency labels (P-067) · → andromeda-pulse-0.3.0/chunks/2026-07-01-live-only-service-truth/
2026-07-05-anomaly-surfacing · complete · Anomaly surfacing — errors/anomalies sorted/flagged to the top of Traces + semantic error tokens + filterable + viz/mcp completion-order fix so slow error spans reach the table (P-068) · → andromeda-pulse-0.3.0/chunks/2026-07-05-anomaly-surfacing/
2026-07-05-legible-labeled-constellation · complete · Legible labeled constellation — per-dot service names + health/severity via design-system color + Halo (P-069) · → andromeda-pulse-0.3.0/chunks/2026-07-05-legible-labeled-constellation/
2026-07-05-constellation-severity-live-wiring · complete · Constellation severity live-wiring — reconcile incident workspace key (resolver data_dir vs producer detected-root) so per-service severity + incidents panel light up under a live storm (P-079) · → andromeda-pulse-0.3.0/chunks/2026-07-05-constellation-severity-live-wiring/
2026-07-06-incidents-panel-dropdown-layout-bug · complete · Incidents (Findings) dropdown bounded-popover layout fix — no window stretch, opaque design-token bg, a11y preserved (frontend-only) · → andromeda-pulse-0.3.0/chunks/2026-07-06-incidents-panel-dropdown-layout-bug/
2026-07-07-traces-table-auto-refresh · complete · Traces table auto-refresh — periodic re-poll of viz.query.traces so the table reflects live spans (no more stale "No traces yet") · → andromeda-pulse-0.3.0/chunks/2026-07-07-traces-table-auto-refresh/
2026-07-07-plain-language-connection-status · complete · Plain-language connection status — worded connected-sources + spans/s + buffer-fill line (design-system typography) + honest ConnectionDot zero-span recency (P-070) · → andromeda-pulse-0.3.0/chunks/2026-07-07-plain-language-connection-status/
2026-07-08-self-explaining-empty-states · complete · Self-explaining empty states — Metrics/Logs empty surfaces explain themselves with an actionable hint + design-system iconography (P-071) · → andromeda-pulse-0.3.0/chunks/2026-07-08-self-explaining-empty-states/
2026-07-09-traces-table-layout-polish · complete · Traces table internal scroll (flex-fill: fixed constellation hero + Errors-only toolbar + sticky header, no outer page scroll) + Errors-only/internal-scroll wireframe doc + constellation label collision-avoidance + announce-in-updater fix (P-082) · → andromeda-pulse-0.3.0/chunks/2026-07-09-traces-table-layout-polish/
2026-07-10-incidents-floating-window-disclosure · complete · Incidents floating-window disclosure — separate borderless always-on-top Findings window docked below the compact widget (reuses P-080 FindingsDropdown content) + use-findings badge re-poll CARRY · → andromeda-pulse-0.3.0/chunks/2026-07-10-incidents-floating-window-disclosure/
2026-08-14-fingerprint-feed-capture-repair · complete · Fingerprint-feed capture and repair — the storm detector's feed made permanently observable (three tick-aggregated counters + a degraded-boot warn) and captured live: 936 span-events / 936 fingerprints / 936 observer invocations with storms_detected_total 0→2, so the dead-region-at-HEAD premise measured FALSE and no repair was needed; closes the Rust gate deferral open since 2026-07-06 · → andromeda-pulse-0.3.0/chunks/2026-08-14-fingerprint-feed-capture-repair/
2026-08-14-workspace-key-alignment · complete · Workspace-key alignment — one workspace-key derivation reached by both the app and the MCP sidecar (the sidecar's data_dir key diverged from the app's detected-root key since P-079), observed before it is fixed and normalization proven for canonicalized-vs-raw entry forms; carries the Conductor arm-zero classification leg · → andromeda-pulse-0.3.0/chunks/2026-08-14-workspace-key-alignment/
2026-08-15-corpus-key-persistence · complete · Corpus key persistence — the AES-256-GCM cell key survives a process boundary (a real platform credential-store backend linked, with the intended PassphraseFallback branch made reachable and warning where no store exists), and pre-fix content orphaned under ephemeral keys is retired by an operator-chosen disposition (recovery is cryptographically closed); carries the operator-gated arm-zero classification leg · → andromeda-pulse-0.3.0/chunks/2026-08-15-corpus-key-persistence/
2026-08-15-tier-1-incident-path-investigation · complete · Tier-1 incident-path investigation — a detected storm produces an incident again: premise-check the in-window regression claim, locate the break in the storm→cue→incident seam, fix it, and un-mute the diagnostics that forced an out-of-band sqlite3 read · → andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/
