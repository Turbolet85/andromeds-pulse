# Intent (verbatim)

## Phase 1b brief intent

> я просто хочу обновить документацию до текущей ситуации, чтоб пайплайн был чистый

(Translation: "I just want to update the documentation to the current situation, so the pipeline is clean.")

## Phase 1c clarifying response

User was offered 4 candidates surfaced from the recent /andromeda-wrap-session report:

1. Coverage trigger for `cargo fmt --check` + `npx tsc --noEmit` "clean at commit time"
2. Boot smoke gate trigger expansion (overlaps with #1)
3. api-surface.md format-mismatch documentation
4. Something else entirely (placeholder)

User response: `все эти проблемы` ("all these problems").

After consolidation:
- Candidates #1 + #2 collapse to one Type 3 amendment (chunk-plan standard-gate-baseline coverage trigger)
- Candidate #3 becomes a second Type 3 amendment (wrap-session Phase 5 reconcile-skip discipline)
- Candidate #4 was a placeholder; no real fourth issue

User confirmed framing: `a` (option (a) — framing matches; proceed to classification).

## Final slug

`pipeline-discipline-doc-sync`

## Final classification

2 × Type 3 (documented gap addition), both targeting `.andromeda/test-plan.md`. Independent amendments per Type 3 multi-gap classification guidance (different concerns: chunk-plan authoring vs wrap-session protocol).
