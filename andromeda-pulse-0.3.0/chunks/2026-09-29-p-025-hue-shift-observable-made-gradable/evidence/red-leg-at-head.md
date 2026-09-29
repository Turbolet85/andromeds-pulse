# RED leg at HEAD — `cargo xtask smoke:hue-shift` (measure-first, plan step 3)

**Subject:** the HEAD product: `pulse-app/ui` dist built, then `cargo build -p pulse-app --release` (3m57s, finished
2026-09-29 07:55:25 +0200). The binary was confirmed to be HEAD by content, not by timing:
`tier_effective_at_unix_nano` appears 0× in `target/release/pulse-app.exe` (the control string `list_with_states`
appears 8×), and the embedded bundle `index-BhmyDc09.js` matches `pulse-app/ui/dist/assets/`.

**Precondition:** the release binary was present, and :4317/:4318 were refusing before the leg (probe exit 0).
Census before the leg: no `pulse-app` / `inject_demo` process running.

**Printed verdict (leg exit 1):**
```
smoke:hue-shift: rise duration_ms=510 anchor_error_ms=9229
smoke:hue-shift: fall none observed
smoke:hue-shift: within the 2000 ms budget: yes (context only — Conductor grades)
smoke:hue-shift: FAIL — the rise anchor is 9229 ms from the incident's creation record (tolerance 1000 ms) — the sample measures a different interval
```

The expected RED held: a FAIL on the rise anchor, with 0 ERROR (the defect is anchor-shaped). HEAD reported
`duration_ms=510`, a "within budget" grade computed over the wrong interval, which is the contract's term-1 defect.

**Preserved log family:** `target/hue-shift/2026-09-29T05-56-19Z/agent-latest.jsonl.2026-09-29`. The timeline read from it:

| timestamp (UTC) | target | fields |
|---|---|---|
| 05:56:47.776 | interpretation.incident.created | created=true, tier=autonomous |
| 05:56:48.009 | interpretation.incident.created | created=true, tier=autonomous |
| 05:56:57.748 | metric.constellation.hue_update_ms | duration_ms=509.8, tier=autonomous |
| 05:59:12.251 | triage.incident.auto_resolve.tick | evaluated=2, resolved=2 |
| 05:59:12.746 | metric.constellation.hue_update_ms | duration_ms=519.9, tier=none |

- **Rise anchor:** 05:56:57.748 − 510 ms = 05:56:57.238. The nearest preceding creation is 05:56:48.009, so the
  error is **9229 ms**. HEAD's anchor was the service's `last_seen` instant, not the incident.
- **Fall:** the record existed. The leg printed "none observed" because the first-draft verdict returned on the rise
  failure before grading the fall. This was corrected in the same chunk, so the fall is now graded independently.
  Recomputed by the same formula: 05:59:12.746 − 520 ms = 05:59:12.226, against the resolving tick at 05:59:12.251,
  giving **25 ms**. At HEAD the fall anchors correctly only by coincidence: a healthy feed was refreshing
  `last_seen` about 0.5 s before the repaint. The rise is the discriminating half.
