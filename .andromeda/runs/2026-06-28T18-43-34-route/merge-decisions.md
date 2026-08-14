# Merge Decisions — andromeda-pulse-0.3.0 route (Phase 3)

One line per Phase-2 suggestion: `{validator} {type} · {action} · {reason / adjustment}`.

- security · No suggestions · n/a · v0.1/v0.2 bootstrap complete; no new requirements (loopback OTLP / MCP double-gate / corpus AES-GCM / uniform scrubber all pre-exist).
- design Rewrite (constellation) · adjusted · cite `design-system` color + Halo at category level (not inlined token names — route stays WHAT-not-HOW; specific tokens are phase-loop detail).
- design Rewrite (empty states) · adjusted · cite `design-system` iconography (category level).
- design Rewrite (status line) · adjusted · cite `design-system` typography (category level).
- design Rewrite (anomaly) · adjusted · "flagged with semantic error tokens" + `design-system` cite (category level).
- design Rewrite (window titlebar) · adjusted · "custom-titlebar drag region" + `layout-templates` cite.
- tests Rewrite (integration test) · adjusted · added `deterministic-L4` + `real-time push` (P6 critical path) to the path; trimmed to ≤25 words.
- obs Insert (heartbeat ticks for incident subsystems) · adjusted (folded) · folded into the P-074 Tier1 chunk ("elastic queue with heartbeat ticks · obs §Heartbeat") rather than a thin standalone chunk (sub-session granularity).
- a11y Insert ×7 (per-surface a11y verification) · adjusted (consolidated) · merged into ONE Epoch-4 "A11y verification — v0.3.0 interactive surfaces" chunk (focus/keyboard/contrast/SR + SC 2.3.3), matching the v2 Polish-audit pattern; 7 thin chunks would unbalance the epoch.

**Tally:** applied 8 (5 design + 1 tests + 1 obs-folded + 1 a11y-consolidated) · rejected 0 · deferred 0 · no-suggestion 1 (security).
**Net chunk count:** 16 → 17 (+1 a11y verification chunk in Epoch 4 → 3/5/5/4).
