# GREEN leg — `cargo xtask smoke:hue-shift` (plan step 10.4)

**Subject:** the release build from gate #12 (201 s). It carries the fix: `tier_effective_at_unix_nano` appears 2× in
`target/release/pulse-app.exe`, and the embedded bundle `index-fJdMGMJs.js` equals `pulse-app/ui/dist/assets/`. The leg
printed `binary … (built 5m ago)`. Precondition gate #14 was green: the binary is present and :4317/:4318 are refusing.

**Printed verdict (leg exit 0):**
```
smoke:hue-shift: rise duration_ms=9986 anchor_error_ms=44
smoke:hue-shift: fall duration_ms=510 anchor_error_ms=25
smoke:hue-shift: within the 2000 ms budget: no (context only — Conductor grades)
smoke:hue-shift: PASS
```
Against the RED at HEAD (`evidence/red-leg-at-head.md`), the rise anchor error went 9229 ms → **44 ms** and the fall
stayed at **25 ms**. The preserved log family is `target/hue-shift/2026-09-29T07-03-31Z/`. After the leg, no
`pulse-app` / `inject_demo` process was running and both ports were refusing.

**Finding surfaced (not graded here): the 9986 ms rise is a fresh-boot first-sighting latency.** Timeline:

| UTC | record |
|---|---|
| 07:04:02.422 | interpretation.incident.created (created=true, autonomous) |
| 07:04:02–11 | services.list_with_states.request item_count=0, every ~1 s |
| 07:04:11.843 | triage.lifecycle.tick — tracked_services_total=5 (first registry population) |
| 07:04:12.360 | services.list_with_states.request item_count=5 |
| 07:04:12.363 | metric.constellation.discovery_ms discovered_count=5 |
| 07:04:12.364 | metric.constellation.hue_update_ms duration_ms=9985.7, autonomous |

The service registry lists a service only after its first lifecycle heartbeat (`DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL`
= 15 s). So on a freshly booted app the dot, and therefore its hue, cannot exist until that tick. Paint followed the
first non-empty poll by 4 ms. A tier change on a service the registry already lists is bounded by the 1 s poll instead:
the fall measured 510 ms. The observable now reports this interval honestly. Before the fix it read ~0.5 s, anchored
on `last_seen`.

**Re-run after the wasmtime 46 → 48.0.3 bump (RUSTSEC-2026-0316), on a fresh release build (gate #12, 346 s):** exit 0,
`smoke:hue-shift: PASS`: rise duration_ms=9841 anchor_error_ms=36, fall duration_ms=578 anchor_error_ms=28. The preserved
log family is `target/hue-shift/2026-09-29T07-51-35Z/`. After the leg, no process remained and both ports were refusing.
