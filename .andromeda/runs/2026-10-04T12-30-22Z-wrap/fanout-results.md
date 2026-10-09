# Fan-out results — 2026-10-04-linux-launch-stays-up-on-nvidia-wayland

Seven Explore doc-agents, one batch; prompt per amendment-flow.md §Fan-out, `{contracts_line}` dropped (registry.py
contracts → exit 3 NOT MIGRATED for architecture · test-plan · obs-plan · a11y-plan). Detector counts per prompt:
arch 2 · security-plan 4 · design-system 2 · layout-templates 2 · test-plan 3 · obs-plan 4 · a11y-plan 2 = 19 (=
the drift-base `doc:` names). Returns carried no HTML entities; stripping removed only `#` commentary notes (no
proposal content), so no raw twin is warranted.

## Verdicts
- architecture — 2 proposals
- security-plan — 2 proposals
- design-system — `proposals: []` (notes: no UI; the founder ruling touches no design status claim)
- layout-templates — `proposals: []` (notes: no surface; no status claim touched)
- test-plan — 2 proposals
- obs-plan — 2 proposals
- a11y-plan — `proposals: []` (notes: no interactive element; violation schema untouched)

## Proposals + dispositions

### architecture
1. D-arch-resources (warning) · §Occupied Resources → Environment variables · register `__NV_DISABLE_EXPLICIT_SYNC`
   (system NVIDIA var; presence-read; product-set `1` on Linux when absent by `apply_linux_default()` as `main()`'s first
   statement; preset honoured, never parsed/logged; NVIDIA-scoped, no probe; WebKitGTK children inherit; name-only on
   `app.boot.render.posture`; founder-ratified widening 2026-10-04; residual preset `0` dies).
   → **apply** (check 1: records the Boundary widening — playbook `Boundary widening` → escalate; RESOLVED by the
   founder's live ratification of 2026-10-04, relayed by the pc overseer at P4 and confirmed by the overseer at this
   wrap; the registration itself is also an accurate this-chunk addition; check 5: plan expected amendment 1).
2. D-arch-resources (warning, dependent-of D-arch-resources) · same section, the `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`
   entry · reconcile its "must stay out of product config … GPU posture" clause with the product-set Linux default.
   → **apply** (check 5: plan expected amendment 1 names this reconcile explicitly; same ratification).

### security-plan
1. D-security-input (escalate) · §Input Validation → "CLI / env var inputs" row · add `__NV_DISABLE_EXPLICIT_SYNC` with
   its presence-only / product-set / preset-honoured posture and the residual.
   → **escalate → resolved, apply** (check 1: playbook `Boundary widening` → escalate, never routine; the
   new-env-var routine rule does NOT govern — its precondition "the report shows the input is code-validated (bounded
   enum / truthy / TryFrom parse)" fails: the value is never parsed; resolution = the founder's live ratification of
   2026-10-04, confirmed by the overseer at this wrap; check 5: plan expected amendment 2).
2. D-security-input (escalate, dependent-of) · §Threat Model Summary → Attack surface → "CLI input" vector ·
   entry point + trust boundary + residual.
   → **escalate → resolved, apply** (the group with 1; check 5: plan expected amendment 2).

### test-plan
1. D-tests-coverage (warning) · §1 Pending coverage triggers · new row `render-posture-main-placement-coverage`
   (the lib is pinned; the production placement in `main.rs` — apply first, before the runtime; emit after
   `emit_boot_spans` — is proven only by the plan's gate-time awk probe and the two operator legs).
   → **apply** (check 1: playbook `Accurate this-chunk addition` → routine; precedent rows
   `discovery-observer-wiring-coverage` / `exit-hook-main-composition-coverage` of the same class).
2. D-tests-coverage (warning) · §4 Conventions → Test file location (Rust) · "92 targets" → 101.
   → **apply** (check 1: `Accurate this-chunk addition` — the report's Counts bullet carries 99 → 101 with its basis).

### obs-plan
1. D-obs-instrumentation (warning) · §6 warn row · add `app.boot.render.posture`.
2. D-obs-instrumentation (warning, dependent-of) · §8 exact leaf.
   → **reject both** (Validate preamble: `basis` cites source coordinates the report does not carry —
   `pulse-app/src/render_posture.rs:19`, `pulse-app/src/observability.rs:1011` — the re-derivation tell; and proposal 2's
   change carries a collateral fact the report contradicts: it places the emit-site capture in
   `unit_render_posture.rs`, where the report's Coverage puts it in the allowlist guard file).
   → **orchestrator raises** the same dual-site registration from the report alone (check 5: plan expected amendment 3):
   §6 once-per-boot rows + §8 exact leaf `app.boot.render.posture {posture, lever}` — **apply** (check 1: playbook
   D-obs-pii new-target rule → routine; both load-bearing conditions hold per the report: (a) the leaf enumerates every
   field the emit site emits — `{posture, lever}` set-equal both ways, emit-site capture; (b) the guard lives in
   `pulse-app/tests/`).

## Validate checks 2-6
- 2 cross-contradiction: none (no two proposals edit one section in opposing directions).
- 3 intent-consistency: the report matches the working entry + plan acceptance; scope record none (gate.py scope clean).
- 4 absence-needs-evidence: arch's "absent from the registry" rests on the report's `grep -c … architecture.md` = 0;
  obs's on `grep -c 'app.boot.render.posture' obs-plan.md` = 0; security's on `grep -n 'CLI / env var'` (1 hit, the row
  itself, no `__NV_` token: `grep -c '__NV_DISABLE_EXPLICIT_SYNC' security-plan.md` = 0).
- 5 expected amendments: 1 arch → proposals arch 1+2 · 2 security → proposals sec 1+2 · 3 obs → orchestrator-raised.
- 6 disproved claims: none in the report.

## Escalations
- E1 (security 1+2, arch 1+2 — one class): Boundary widening, `__NV_DISABLE_EXPLICIT_SYNC` presence-read + product-set.
  RESOLVED: the founder's live ratification of 2026-10-04 (the P4 remedy fork, option 1, relayed by the pc overseer),
  confirmed by the overseer's wrap note ("Record the boundary widening as the founder's live ratification of
  2026-10-04"). No playbook rule is proposed for this class (never-routine).
