# Curation — 2026-09-30-dual-license

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "SWEEP HAZARD — grep the test trees for the OLD VALUE's literal, not the artifact's file name, when a chunk changes a value inside a data artifact" (confidence 0.8)
    Proof: implement run 1 — `cargo nextest run --workspace --profile ci` 2432/2433; `pulse-app/tests/distribution_manifests.rs::update_channels_workflow_scoop_manifest_required_keys` pinned the substring `"license": "MIT"` against `update-channels.yml`, which research's "tests that pin the changed artifact's data" sweep missed (report `Spec claims disproved by measurement`; scope-record companion line). Signals: gate-proven +0.4 · specific detail +0.2 · no-other-home +0.2.
  Correction (cap-exempt): testing.md 2026-08-15 bindings-regen entry — clauses (a) and (c) ("re-run the regen AFTER the light gate"; "re-check capability-drift after the LAST cargo op … running it LAST checks the index copy") tagged `[corrected 2026-09-30: …]`.
    Proof: the operator ratified the new order at this wrap's E1 (playbook :74 superseded, test-plan §3 amended); both implement full runs under it closed with `git diff --quiet 1dfca74… -- pulse-app/ui/src/bindings/index.ts` exit 0 (implement gate logs, entries 12–15).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup (the PreToolUse guard blocking a file-target cat heredoc — host-win32.md §Transports already states "documents: the Write tool") · 3 task-specific (the pre-push:linux wall time 107.6 s under the 20 GB WSL cap; the `cut -c` mid-UTF-8 display glyph; the P-072 matrix parse warning — a pipeline record, carried in friction + handoff) · 0 conflict · 0 deferred
  Not candidates (a home this wrap): the gate order (playbook rule appended + test-plan §3) · cargo-deny checks own crates (security-plan §Dependency Security) · Tauri `bundle.license` inherits the Cargo value (architecture [License]).
  No-other-home: "grep the test trees for the OLD VALUE's literal when a chunk changes a value inside a data artifact"
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B
