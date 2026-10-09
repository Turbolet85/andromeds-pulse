# Cascade dispositions — 2026-10-01-conductor-return

The search: `cascade.py sweep --patterns-file cascade-patterns.toml` over the seven masters (baseline `a2addb37`, the
pre-CI parent), the registry files, the three curation homes, the two judgment bases and the leaf bodies. Patterns
derived from every amendment of this pass (obs §1/§3/§6/§7/§8, security Logging/Universal, test §1/§3) — the claims they
touch: the init order and its last step (`spawn Tauri`), the guard's lifetime (`WorkerGuard` / `_guard`), the
error-class list (`Unhandled panics` / unlogged panics), signals (`SIGTERM|SIGINT|signal handler`), the 69f0b93 /
`exit 1` narrative. Three further patterns — `generate_context` (the `.run(ctx)` tail), `process::exit|ExitProcess|atexit`,
and a "no exit record / exits silently" phrasing — were REFUSED by the tool: 0 hits in the seven masters' pre-pass text,
so no master could carry a stale copy; their leaves were hand-grepped below. Sections read: obs-plan §1 Tracing init,
§3 Tracing init + Init body sketch, §6 `warn` row (5 110-char line read by its match window), §7, §8 leaf list;
security-plan §Security Anti-Patterns → Logging + Universal; test-plan §1 Pending coverage triggers + §3 Per-chunk gate
discipline.

## Rows (per pattern)
- **spawn-tauri** (standing 2 · leaf 2): `obs-plan.md:72`, `:175` — EDITED this pass (the init order now continues past
  step 4's spawn; the step-(2) stderr clause deliberately untouched — pre-existing, routed). Leaves
  `.claude/rules/observability.md:20`, `.claude/docs/obs-summary.md:16` — RE-DERIVED.
- **init-order** (standing 2 · leaf 2 · curation 2): the two obs sites — EDITED (as above); leaves rules/observability.md:20,
  obs-summary.md:12 — RE-DERIVED; curation `docs/session-learnings.md:633`, `:637` — a different init order (baseline-state
  bootstrap wiring in `main.rs`), true, NO CHANGE.
- **worker-guard** (new 5 · standing 4 · leaf 2): the 5 `new` rows are this pass's own text. `obs-plan.md:178` — the
  `Init body sketch`'s `let (file_writer, _guard) = …` is `init`'s internal body, which still builds and RETURNS the guard
  (signature unchanged) — TRUE, NO CHANGE; leaf copy `obs-summary.md:19` — NO CHANGE (same). `test-plan.md:325`
  (`platform_guard`), `obs-plan.md:541`, `:563`, `rules/observability.md:70` (`l4_path_guard`) — token collisions, NO CHANGE.
- **panic-only** (standing 3 · leaf 2): `obs-plan.md:482` "Unhandled panics (always via panic hook)" — still true (the
  process-end class is a SEPARATE new bullet beside it); `:621`, `:637` zero-unlogged-panics SLO — true; leaves
  `rules/observability.md:94`, `obs-summary.md:72` — NO CHANGE.
- **signal** (new 7 · standing 3 · leaf 1 · curation 3): `new` rows are this pass's text. `test-plan.md:188` (a cargo build
  killed by SIGTERM mid-rustc), `:333` (`tauri dev` stopped by SIGTERM), `:506` (the MCP sidecar's cleanup SIGTERM) — not
  pulse-app's signal handling, true, NO CHANGE. Leaf `.claude/docs/commands.md:89` (`agent-run.sh cleanup # SIGTERM +
  verify ports released`) — still true (TERM now records, then the app still ends by the signal), NO CHANGE. Curation
  `session-learnings.md:28`, `:30`, `:1189` — cargo/timeout SIGTERM lessons, unrelated, NO CHANGE.
- **exit-1** (new 1 · standing 13 · leaf 5 · curation 2): every standing row is a harness / gate exit code
  (`architecture.md:244` status arms; security `:217` npm gate; test `:56`, `:134`, `:185`, `:210`, `:216`, `:219`, `:315`,
  `:678`, `:693`; obs `:644`; a11y `:381`) — unrelated to process-end recording, NO CHANGE. Leaves
  (`rules/a11y.md:98`, `rules/verification-harness.md:22/25/26`, `docs/a11y-summary.md:94`) and curation
  (`rules/testing.md:296`, `rules/verification-harness.md:152`) — same class, NO CHANGE. The one `new` row is obs §6's
  69f0b93 reference (this pass).

## Refused patterns — leaves hand-grepped
`git grep -nE 'generate_context|process::exit|ExitProcess|atexit' a2addb3 -- CLAUDE.md .claude/rules .claude/docs` → 1 hit:
`.claude/rules/testing.md:250` (a `## Session Additions` curation entry: Tauri compile-time-embeds the frontend via
`generate_context!`) — a different claim, true, NO CHANGE. No leaf states the `.run(ctx)` tail or an exit/atexit claim;
after this pass the only other hits are this pass's own re-derived text.

## Leaf set re-derived (provenance: obs-plan · security-plan · test-plan)
- `.claude/docs/obs-summary.md` — init order (step after 3) + the `app.exit` process-end leaf paragraph.
- `.claude/rules/observability.md` — init-order line + the `app.exit` leaf bullet (body, beside `app.boot.window.navigation`;
  `## Session Additions` untouched).
- `.claude/rules/security.md` — the `app.exit` NO-SCRUB bullet + the `atexit`/signal-handler ban (body, §Logging & redaction;
  `## Session Additions` untouched).
- `.claude/docs/security-summary.md` — the ban added to Top anti-patterns (universal); the summary enumerates no NO-SCRUB
  boundaries (0 `ui.ipc.rejection` hits) — none added.
- `.claude/docs/tests-summary.md` — the process-end witness form (test-plan §3). The new §1 trigger is NOT added: neither
  test leaf enumerates the recent triggers (`grep -c 'l4-decision-probe-arg-parse|discovery-observer-wiring|damper-shared-instance'`
  → 0 in both), so the trigger list lives in the master alone.
- `.claude/rules/testing.md` — body unchanged (no amended claim restated); its `## Session Additions` 2026-08-15 re-exec
  entry ("VERIFY the child actually ran by running once with `--no-capture`") is a CURATION hit → routed to P3 as an
  in-place extension (in-arm child-ran proof when child stdout is discarded).
- `CLAUDE.md` `GENERATED:setup:warnings` — recomputed by read: no line restates an amended claim
  (`grep -nE 'Init order|spawn Tauri|panic hook|atexit|signal' CLAUDE.md` → 0); the block carries a selection of
  universal invariants, and the new Universal ban is not promoted into it — NO CHANGE.

## Kept (pre-existing, not this chunk's)
obs-plan §1:72 / §3:175 step (2) and `rules/observability.md:38` / `obs-summary.md:14` state a stderr + file dual sink;
HEAD's `init` builds the file layer only (research.md §Files inspected). Not amended here (the plan routes it to the wrap's
curation channel; playbook "Not this chunk's drift" → its owned channel): routed by route-resolve as a CARRY.
