# Intent — append-chunk-68-corpus-sqlite-scaffold

_Captured by /andromeda-evolve at Phase 1c. Source: user CLI argument._

## User intent (verbatim)

```
/andromeda-evolve --allow-route-append ### #69 — Corpus SQLite scaffold + schema + encryption + PII scrubber
```

## Skill interpretation (no clarifying questions per session policy)

- **Flag:** `--allow-route-append`
- **Target plan:** `.andromeda/route.md`
- **Form:** 1 (chunk append to existing epoch)
- **Insertion epoch:** Epoch 9 — Foundation v0.2.0 (terminal epoch; the only epoch with active v0.2.0 work)
- **Route position:** #68 (next sequential; current `last_completed_chunk.route_index = 67`)
- **Source-doc reference:** `docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 5 line 329 (=v0.2.0-plan §69)

## Numbering divergence note

User wrote `#69` in CLI args. The `#69` matches the **source-doc** numbering
(v0.2.0-plan §69). The **route.md** target position is `#68` (next sequential).
The route.md and v0.2.0-plan numbering already diverge by 1: route#67 was
registered sourcing from v0.2.0-§68 (skipping v0.2.0-§67 "Drain Rust" which
is blocked on Pre-D2 spike per pulse-v0_2_0-route ordering note). Same
divergence pattern applies here — route#68 sources from v0.2.0-§69.

Cross-reference: chunk #67 marker
`.andromeda/runs/2026-05-17T23-51-55-spec-amendment-append-chunk-67-service-registry-lifecycle/amendment.md`
documents the same divergence in its Motivation field.

## Slug

`append-chunk-68-corpus-sqlite-scaffold` — matches the
`append-chunk-{N}-{kebab-title-keyword}` precedent used by chunks #58
through #67.
