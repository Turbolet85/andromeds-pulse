# Cascade dispositions — 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement

## The search
- Pattern set `cascade-patterns.toml`, run by `cascade.py sweep` (listing: `cascade-sweep.txt`, exit 0), baseline
  4e5b5959 (the pre-CI parent). Patterns: `model-repl` (the retired owner phrase, its verb-free noun) · `cpu-tier`,
  `ngl0`, `cpu-build`, `cpu-bin-var` (the retired "CPU route is a supported L4 tier" claim by tier label, flag,
  binary phrasing and its env var) · `pins16`, `every-flag` (the retired probe pin count and owed-flag scope) ·
  `schema-file` (the json-schema mechanism the grammar-prefill trap scopes) · `llama-3b` (the model-identity claim).
- Dropped before the run: `qwen` and `without-gpu` — their controls cannot fire on pre-pass text (the claims are new
  this pass, nothing standing to retire); the tool refused them (exit 3) and they were removed, not hand-controlled.
- NOT looked for: hardware-profile DETECTION labels as a retired claim (detection is unchanged; the labels still parse);
  the `-p` bound (unchanged by the report).
- Every control fired (header rows).

## Rows
- architecture.md:72 model-repl `new` — this pass's own replacement text; the retired owner phrase is gone. amended.
- architecture.md:30 / :71 / :230 / :367 (cpu-tier, ngl0, cpu-build, cpu-bin-var, schema-file `edited` / `new`) — the
  amended sites; each now carries the founder-retired status and its owner beside the still-true routing. amended.
- architecture.md:231 cpu-tier `standing` — `ANDROMEDA_PULSE_HARDWARE_PROFILE` override labels `cpu_primary` /
  `cpu_fallback`: a detection-side label set, still parsed by the code. no change (true claim sharing the token).
- security-plan.md:138 cpu-tier / cpu-bin-var `standing` (@c2007, @c5125, read by window) — the env-var input table:
  the labels and the CPU bin path are still read by the shipped binary until entry B. no change (true).
- security-plan.md:395 cpu-bin-var `standing` (@c511) — the L4 path guard covers the var; true. no change.
- test-plan.md:92 cpu-bin-var `standing` — the confinement-assertion row over the three L4 path vars; true. no change.
- test-plan.md:144 pins16 / every-flag / schema-file `edited` — the amended row; the "16 collected pins" phrase stands
  as the predecessor chunk's history clause, followed by this chunk's 29-pin clause; the owed-flag list is narrowed.
  amended.
- architecture.md:72 llama-3b `standing edited` ×2 — the predecessor series' model citation (history, true) and this
  pass's "shipped model stays Llama-3.2-3B". amended / no change.
- leaf .claude/docs/stack.md:24, :27, :28 — :27 re-derived (CPU route status); :25 re-derived (`-c 8192` / `-rea off`);
  :28 re-derived (the json-schema mechanism scoped to the shipped Llama template, the trap named); :24 the
  session-144 spike reading, true. re-derived.
- leaf .claude/docs/services/interpretation.md:27, :30, :31, :35 — :27 re-derived (CPU route status); :30 re-derived
  (model choice owned by the pattern-discrimination entry, the founder's pick shipped by the entry after it); :31 / :35
  the argv and the trap, written by /implement this chunk and still true. re-derived.
- leaf .claude/rules/security.md:17, .claude/docs/security-summary.md:43 cpu-bin-var — the path guard; true. no change.
- curation .claude/rules/testing.md:210 llama-3b — a Session Additions entry describing a past real-model test; true;
  preserve-verbatim. no change.

## Leaf set (step 3)
- architecture.md amended (§Stack, §Established Decisions, §Occupied Resources, §Inherited Defaults). Provenance
  enumeration (`Extracted from .andromeda/architecture.md` in the first lines): `conventions.md` · `stack.md`.
  - stack.md — re-derived (above).
  - conventions.md — recomputed against §Conventions (unamended): no row of the sweep, no change.
  - services/interpretation.md — reached by the sweep's leaf rows; re-derived (above).
  - CLAUDE.md `GENERATED:setup:*` — recomputed: the overview's llama.cpp clause (`:9`) and the `interpretation`
    module line state no tier routing, argv or model, and stay true at that grain; warnings / pointer-table carry no
    amended claim. no change.
- test-plan.md amended (§1 pending trigger row). Leaves: `.claude/docs/tests-summary.md` and `.claude/rules/testing.md`
  — neither carries the probe row (grep `l4-decision-probe` over `.claude/`: services/interpretation.md only); no
  change.
- Lateral binds: test-plan §3 ↔ obs-plan §3 and a11y ↔ obs schema untouched by this pass.
- Judgment bases (`playbook.md`, `drift-base.md`): no row.
