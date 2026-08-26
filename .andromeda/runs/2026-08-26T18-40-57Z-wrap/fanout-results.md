# Fan-out results — 2026-08-26-l4-runtime-security-residuals

Seven Explore doc-agents, one per spec source, one parallel batch. Report was the sole input
(report↔detector contract); no agent re-derived from git or the codebase.

## Verdicts

| doc | detectors | proposals | verdict |
|---|---|---|---|
| arch | D-arch-resources, D-arch-decisions | **3** | drift (D-arch-decisions clean) |
| security-plan | D-security-input, D-security-auth, D-security-deps, D-security-logging | **11** | drift (auth + deps clean) |
| design-system | D-design-tokens | 0 | clean |
| layout-templates | D-layout-surface | 0 | clean |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | **3** | drift (framework + obs-harness clean) |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-pii | **4** | drift (D-obs-stack clean) |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | 0 | clean |

**Total: 21 proposals.** Three docs returned `proposals: []` and are recorded here rather than as raw
twins (the sanctioned consolidated form). Raw twins kept for the four docs carrying proposals.

## Clean-return reasoning (recorded, not discarded)

- **design-system** — all three Coverage entries carry `tokens n/a`; Files lists only Rust sources/tests;
  no `hardcoded✗` anywhere.
- **layout-templates** — no `.tsx`/`ui/` file, no component, window or route; the one new item is an env
  var the report marks "Env-layer only", so it adds no rendered control.
- **a11y-plan** — no interactive UI element (all surfaces `a11y n/a`); the obs-side change is leaf-level
  under §8, while §6's envelope schema and the `a11y::assertion` field set are byte-unchanged.
- **D-arch-decisions** — Dependencies "none added, none bumped"; the guard is hand-rolled `std` on the
  existing `publish_workspace_key` precedent, which §Stack Validation ("None — serde + smart enum types")
  already allows.
- **D-security-auth** — no identity/session/token/key surface; `ANDROMEDA_PULSE_L4_ALLOW_ROOT` is a path
  root, not secret-class.
- **D-security-deps** — nothing added or bumped; `strict-path` appears only as evaluated-and-DECLINED,
  which is not an addition.
- **D-obs-stack** — all new emits ride existing `tracing` targets + the `observability.rs` allowlist Layer,
  matching §3's no-OTel-SDK harness.
- **D-tests-framework** — the gate list is byte-for-byte §3's standard set with webview gates correctly
  excluded; the new test file matches the `[lib] test = false` placement rule.
- **D-tests-obs-harness** — no harness verb, status shape or log format changed; the obs surface is
  leaf-level (§8), not §3.

## Validation (orchestrator)

1. **Playbook** — all 21 resolve ROUTINE; **zero escalations**.
   - The 12 `escalate`-severity proposals (D-security-input ×9, D-obs-pii ×3) match the **2026-08-23
     routine-APPLY-BY-ACTUAL-CLASS** rule. Both load-bearing conditions hold: (a) the escalate condition is
     *affirmatively absent* — the report marks validation ✓ on all three new surfaces with unit tests +
     mutation checks, and measures full paths 0× at the wire (model dir 0× / 11,251 lines; root 0×; no
     prompt text) — not merely unmentioned; (b) each finding's actual class has a governing rule:
     **2026-06-28** (new env var, code-validated + unit-tested ⇒ registry completeness), **2026-08-14**
     (doc-only correction where the impl is already correct — this chunk SHIPPED the impl, so there is no
     impl half left to fix), **2026-08-16** (obs allowlist-leaf registration; its conditions (a) leaf
     enumerates every emitted field and (b) guard lives where it runs BOTH hold —
     `pulse-app/tests/unit_observability_allowlist_l4_path_guard.rs`, asserting set equality in both
     directions).
   - The 9 `warning` proposals resolve under **2026-07-08 routine-APPLY** (accurate this-chunk addition
     inside an existing doc structure) and **2026-08-14 routine-APPLY**.
   - **Not** the 2026-08-25 rejected shape: the D-security-logging pair targets the basename-only coverage
     claim, not the `scrub_attribute` gate or the subscriber-layer sentence, and the boundary is new this
     chunk.
2. **Cross-contradiction** — none. Three proposals edit the §Anti-Patterns → Input MEASURED EXCEPTION
   paragraph (posture · primitive · argv clause) and two edit the §Input Validation CLI row (posture ·
   primitive); all complementary, none opposing.
3. **Intent-consistency** — the report matches the working-route entry and the plan's acceptance criteria;
   all four deviations carry justifications. No divergence.
4. **Absence needs evidence** — every absence claim cites its search: arch greps clean for `ALLOW_ROOT`
   and `strict-path`; security cites occurrence lines 133/138/249/393 and 137/339/444; test-plan cites
   289-301/315/325/163; the obs §8 non-registration was re-verified first-hand by the orchestrator using
   the two-line `by_target.insert(` window with a known-present control.
5. **Expected-amendments reconciliation** — the plan's coverage floor is FULLY met, none under-run:
   security-plan §Anti-Patterns → Input ✓ · security-plan §Input Validation ✓ · architecture.md §Occupied
   Resources → Environment variables ✓ · obs-plan §6 + §8 ✓ · test-plan §1 ✓. Two items land BEYOND the
   floor: the arch §Process/service identity binary-name split (operator directive 2) and the
   duplicate-occurrence sites the detectors' own sweeps found (§Threat Model, §Bootstrap phases,
   §Logging & Monitoring).
6. **Disproved-claims disposition** — all three report entries end DISPOSED:
   - #1 plan named a nonexistent binary path → chunk-artifact, recorded in the report; **additionally** the
     arch §Process/service identity amendment now makes the distinction explicit so it cannot recur.
   - #2 operator's "two consecutive plans" basis → **coordinate corrected**; the amendment proceeds on the
     corrected basis (one measured instance; the predecessor plan was correct). The arch sidecar carries
     the corrected count, not the directive's.
   - #3 `inference_mode` partial redaction → owner already named (obs-plan §8 backlog); the obs agent
     correctly declined to propose it. Corroborating evidence only.
