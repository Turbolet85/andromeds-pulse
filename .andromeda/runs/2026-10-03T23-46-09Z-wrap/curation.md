# Curation — 2026-10-02-incident-events-readable-through-mcp (resumed wrap)

Scope: this resumed window's conversation + the report's *Decisions & corrections* (the prior window's conversation
is gone — a correction only it held is not curated; stated in the resume offer).

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "an `npm ci` exit 0 printing `install-scripts … held back` is not proof the scripts did not run — check for a partial puppeteer cache before re-running a red npm stage" (confidence 0.8)
    Proof: report Deviations → Plan entry 18 + `evidence/operator-pass.md` — the native `npm` stage's first run went red on a partial `linux-148.0.7778.97` puppeteer cache left by a session-env `npm ci` that exited 0 with the held-back warning; green after the founder deleted exactly the two partial folders. Signals: +0.4 measured (a real gate failure) · +0.2 specific detail · +0.2 no-other-home (on no route annotation, in no master, no ledger note).
  Correction (cap-exempt): .claude/rules/observability.md Session Additions 2026-10-02 `[correction]` (app dual sink false at HEAD) — RETIRED: its own clause bound it until obs-plan §1/§3 was amended, and this wrap's P2 amended obs-plan (8 sites) and re-derived the rule body to the file-only sink; retirement directed by the route CARRY (overseer 2026-10-02: "fixed in the next chunk, nothing deferred").
    Proof: `pulse-app/src/observability.rs` init composes one JSON layer on the non-blocking file writer (re-read at HEAD this wrap); the body line now reads "Single sink (app)".
  Filters: 0 dup · 2 task-specific/pipeline (the hygiene host-path predicate on quoted plan paths; the cascade sweep regex missing a parenthesized count — both pipeline telemetry, recorded as friction, never curated) · 0 conflict · 0 deferred (cap)
  Rejected below threshold: `git check-ignore -v` prints nothing for a not-yet-existing directory while the file-path form `--no-index …/index.html` matches (one-off, 0.3).
  Recurrence: `recurrence-despite-learning: CLAUDE.md 2026-05-30 producer-existence entry` — research's writer census grepped the corpus SQL layer (`INSERT … incident_events` / `save_incident`) and missed the trait-level writer `IncidentPersistence::save_incident_event` one layer up; the entry's grep pattern names the persist path but not the layer. → handoff Deferred learnings (the remedy is a CHECK in phase research, the pipeline owners' call). The fact itself rides the architecture sidecar's Why (a writer census needs the trait-level grep).
  CLAUDE.md size: see the P7 console line (health.py check 1).
