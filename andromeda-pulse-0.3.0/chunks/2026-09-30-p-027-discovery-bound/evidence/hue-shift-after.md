# P-025 after the fix — `cargo xtask smoke:hue-shift`

- **Run:** 2026-09-30T10:35:01Z → 10:37:44Z (UTC), in the operator-granted GREEN port slot, right after
  `smoke:discovery`; the fixed release binary (sha256 prefix `9e51d1d92e80fdc0`).
- **Exit:** 0.

## Printed lines (verbatim)

```
smoke:hue-shift: app=D:\dev\projects\andromeda-pulse\target\release\pulse-app.exe (built 8m ago)
smoke:external-resolve: building inject_demo (outside the timed section)
smoke:hue-shift: data dir D:\dev\projects\andromeda-pulse\target\external-resolve\run-28696
smoke:hue-shift: seeding the finite storm over real OTLP
smoke:hue-shift: rise painted — holding the services live with a healthy feed
smoke:hue-shift: log family preserved at D:\dev\projects\andromeda-pulse\target\hue-shift\2026-09-30T10-35-01Z
smoke:hue-shift: rise duration_ms=644 anchor_error_ms=327
smoke:hue-shift: fall duration_ms=468 anchor_error_ms=235
smoke:hue-shift: within the 2000 ms budget: yes (context only — Conductor grades)
smoke:hue-shift: PASS
```

## Beside the prior chunk

| sample | 2026-09-29-p-025 (before this chunk) | this chunk |
|---|---|---|
| rise `duration_ms` | 9 986 | 644 |
| fall `duration_ms` | 510 | 468 |
| both anchor errors ≤ 1000 ms | yes | yes (327 / 235) |

The rise shortened as forecast (≤ ~2 s predicted). The dot now exists before the incident opens, so the rise is a
witnessed none → tier change anchored on `tier_effective_at_unix_nano` rather than a first appearance that had to
wait for the 15 s tick. First `interpretation.incident.created` at 10:35:09.790; first
`metric.constellation.hue_update_ms` (`severity_tier: autonomous`, 644 ms) at 10:35:10.107. The hue observable's
anchor and emission rule are unchanged, and its ANCHOR verdict passes. 45 044 records, 0 `level: ERROR`,
0 `app.panic.fatal`.
