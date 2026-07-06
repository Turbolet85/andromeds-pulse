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
