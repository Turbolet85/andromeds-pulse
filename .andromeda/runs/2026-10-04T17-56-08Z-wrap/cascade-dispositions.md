# Cascade dispositions — 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts

Amendments this pass (all ADDITIONS — no claim retired in a master):
1. architecture §Occupied Resources → Environment variables: `ANDROMEDA_PULSE_HARDWARE_PROFILE` registered.
2. security-plan §Input Validation → `CLI / env var inputs` row: the var listed + its `hardware_profile` validation.
3. test-plan §4 → `interpretation crate` bullet: the Linux CUDA probe candidate-set pins.

## The search
`cascade.py sweep` over `cascade-patterns.toml` (trail `cascade-sweep.txt`), baseline `03fb097f` (the pre-CI parent):
- `hw-profile` regex `hardware[ _-]?profile` (ci) — the amended subject's own naming;
- `gpu-tier` regex `GPU-(primary|fallback)` (ci) — the routing claim the probe feeds (what the probe's verdict decides);
- `env-registry` fixed `Environment variables (reserved` · `cli-env-row` fixed `CLI / env var inputs` ·
  `interp-crate` fixed `interpretation crate` — the three amended sections' anchors.
Every control fired (architecture.md:30 · :30 · :220 · security-plan.md:138 · test-plan.md:385).

By hand (no master control exists, so the tool refuses the pattern): `dlopen` — the disproved leaf claim's mechanism
word. `grep -rn dlopen .andromeda/*.md .claude/ CLAUDE.md` pre-pass → 1 hit, `.claude/docs/services/interpretation.md:26`;
0 in the seven masters, 0 in the curation homes and judgment bases.

## Rows (35 rows printed; every row dispositioned)
- `hw-profile` new — architecture.md:231 (amendment 1's own text) · security-plan.md:138 ×3 @c384,1748,1846 (all three are
  amendment 2's own text: the var name in the list, `hardware_profile`, `HardwareProfileDetector`) · test-plan.md:385 (amendment 3's own text) → amended, no change.
- `hw-profile` standing — architecture.md:30 · :71 ×2 · :367 (tier routing "per hardware-profile tier": TRUE, the routing
  rule is unchanged — the chunk changes which hosts read GPU-present, not the rule) · :190 · :194 · :229
  (`HardwareProfileSource` trait citations: TRUE, untouched) → no change.
- `gpu-tier` standing — architecture.md:30 ×2 · :71 · :229 (GPU tiers → CUDA build + `-ngl 99`: TRUE) → no change.
- `env-registry` / `cli-env-row` / `interp-crate` standing — the three amended anchors; `edited` = the amended lines
  themselves; re-read for an intra-line duplicate of a retired mechanism: none (additions only) → no change.
- `curation` — `.claude/rules/security.md:142` ×3 (the 2026-05-24 cross-crate serde-label Session Addition naming
  `triage::contract::HardwareProfile`): a true historical claim, not this pass's subject → no change (preserve-verbatim).
- `leaf` — re-derived (step 3 set):
  - `.claude/docs/services/interpretation.md` :1 · :9 ×2 · :10 · :34 · :40 (hw-profile) · :27 ×2 (gpu-tier) — RE-DERIVED:
    :26 "dlopen-shaped" → a per-platform file-presence check with the 8 Linux candidates (the report's disproved claim);
    the `interpretation.incident.created` bullet's "VIOLATED in code: a bare `interpretation` key exists at
    `observability.rs:1934`" → holds since 2026-08-30 (re-measured: the two-line registration window lists only dotted
    `interpretation.*` keys), guard location → `pulse-app/tests/observability_pins.rs`; Entry points → `hardware.rs`;
    Dependencies → the manifest's full set incl. `tempfile` (dev). The :9/:10/:27 rows stay true as written.
  - `.claude/docs/stack.md:27` ×2 (both patterns: tier routing via `HardwareProfileSource`) — TRUE → re-read, no change.
  - `CLAUDE.md:33` ×4 (Modules → `interpretation`: "cross-platform GPU probe: Metal on macOS, libcuda.so on Linux,
    nvcuda.dll on Windows") — still true (libcuda.so is among the Linux candidates) → re-read, no change.
  - `CLAUDE.md:9` (Overview: "`interpretation` crate hosts the trait surface") — TRUE → no change.
  - `.claude/docs/andromeda-after-mvp-playbook.md:182` — a historical route description, not a distillation of an
    amended section → no change.

## Leaf recompute set (step 3, provenance + table)
- architecture → CLAUDE.md `GENERATED:setup:*` (overview · modules · warnings · pointer-table · architecture) re-read:
  no block states the env-var registry or the probe path set beyond `CLAUDE.md:33` (true) → no change;
  `docs/services/interpretation.md` (the amended env var's module) → RE-DERIVED (above); provenance leaves
  `docs/stack.md` · `docs/conventions.md` (`Extracted from` architecture) re-read → no stale claim, no change.
- security-plan → `docs/security-summary.md` §Attack surface item 8 → RE-DERIVED (the closed-parse clause with the
  hardware-profile example); `.claude/rules/security.md` body (§Input validation covers `*_PATH`/`*_DIR` vars only; the
  var is not a path) → no change; CLAUDE.md warnings → no change.
- test-plan → `docs/tests-summary.md` (no per-crate unit-coverage list) → no change; `.claude/rules/testing.md` body (no
  per-crate list) → no change; CLAUDE.md warnings → no change.
- Lateral binds: test-plan §3 ↔ obs-plan §3 and a11y ↔ obs schema — neither side amended → hold.
