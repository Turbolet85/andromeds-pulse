# Fan-out results — 2026-08-17-incident-fingerprint-producer-repaired

7 doc-agents, one batch. **4 proposals (all arch) · 6 docs clean · 0 escalations.**

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | **4 proposals** (all D-arch-decisions, all landing in the single §Established Decisions [Fault Identity] bullet). D-arch-resources: no proposal — no new IPC / MCP tool / endpoint / port / socket / env var / crate / table; the two new symbols are struct fields on already-registered `triage::contract` payloads and `hex_lower` is `pub(crate)`, not public API. Raw twin: `.raw-fanout-arch.md`. |
| security-plan | D-security-input · D-security-auth · D-security-deps | clean. Input: no new external surface; `serde` derive is exactly what §Input Validation mandates (`garde` explicitly deferred until cross-field validation). Auth: no identity/session/token/key surface; keyring + passphrase flow untouched. Deps: Dependencies bullet is "none added, none bumped". **Surfaced but correctly did NOT propose:** `security-plan.md:218` audit-interval staleness — outside all three invariants. Orchestrator raised it at Validate check 5 (below). |
| design-system | D-design-tokens | clean. No Coverage row carries `hardcoded✗`; zero `.tsx`/`.css`/`@theme` edits; the one changed data path is `tokens n/a (no UI)`. |
| layout-templates | D-layout-surface | clean. No user-facing surface or region added; the two new fields are internal contract carriers, not rendered regions. Every §Wireframe entry remains an accurate baseline. |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness | clean. Coverage: all new paths tested at the §2 tier (7 new tests, 1803 green); the "workspace tally" flag does not bite — test-plan states no workspace tally. Framework: runner matches §3/§4 exactly; webview gates correctly excluded (zero `pulse-app/ui/**`). Harness: no change to the 5-command harness, status shape, or log format. |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-pii | clean, **and affirmatively measured** rather than absence-without-evidence. No new hot-path op; zero `tracing::` lines added; `observability.rs` untouched; the `interpretation.incident.created` leaf still carries exactly its 4 fields; 0 hex matches in the live-run log. §8 explicitly needs no amendment. |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | clean. No interactive UI element; violation-schema unchanged on both sides (a11y §1/§3 ↔ obs §6 bind intact). |

## Validation (orchestrator)

1. **Playbook** — all 5 amendments match the 2026-07-08 *routine-APPLY* rule (accurate · this-chunk · inside an already-documented structure). The audit re-pin additionally maintains requirement (d) of the 2026-08-15 *routine-BOUNDED-WAIT* rule (the probe interval must exist and stay current). No entry matched an escalate rule; nothing looked structural-and-surprising.
2. **Cross-contradiction** — none. All 5 move the same direction (deferral shipped / arm fed / interval advanced).
3. **Intent-consistency** — aligned. The plan's conditional expected-amendment named proposal #2 verbatim; proposals #1/#3/#4 are the same bullet's other clauses retired by the same shipped fact.
4. **Absence-needs-evidence** — two absence claims, both independently verified by the orchestrator rather than taken on trust: (a) the arch agent's "fingerprint occurs only on the [Fault Identity] line" — confirmed, exactly 1 hit in `architecture.md`; (b) the obs non-amendment — carries three measurements in the report (0 added `tracing::` lines · 4-field emit set live · 0 hex matches in the log).
5. **Expected-amendments reconciliation** — the plan's floor is met and exceeded. Its one conditional entry ("if step 8's comment correction changes what the 'Known producer defect' paragraph should say") is proposal #2. **Orchestrator-raised, not detector-proposed:** the `security-plan.md:218` audit-interval re-pin — routine, because the report's *Decisions & corrections* substantiates it with full provenance.
6. **Disproved-claims disposition** — the report's bullet reads "none"; nothing to dispose. Two arch passages were stale-by-shipping (a superseded deferral), which is an amendment, not a disproof.

**Escalations: 0.** No HALT.

## Applied

| # | doc | section | body | sidecar |
|---|---|---|---|---|
| 1–4 | architecture.md | §Established Decisions [Fault Identity] | one bullet, 4 clauses restated (deferral→SHIPPED · producer defect→repaired · blast radius→extended · L2 rationale→N-safe ground) | ✓ |
| 5 | security-plan.md | §Dependency Security → standing deferral | probe interval: session 28 DISCHARGED, next 31 | ✓ |

Body edits verified BEFORE each sidecar append (5/5 claims present, 4/4 retired phrasings gone).

## Cascade

- **Edge 2 (lateral binds)** — `test-plan §3 ↔ obs-plan §3` and `a11y schema ↔ obs schema`: neither side moved; no action.
- **Edge 3 (verbatim cross-master citations)** — swept all 7 masters + the 3 preserve-verbatim curation homes for every retired phrase. Two hits, both **leaf distillations**, neither in a preserve-verbatim home: `CLAUDE.md:51` (GENERATED warnings) and `.claude/rules/security.md:71`. Both re-derived. A third hit in `master-route.md` is the PRIOR chunk's record — immutable and accurate as history, correctly untouched.
- **Edge 3 leaves re-derived** — `CLAUDE.md` §warnings (fault-identity Tier-1 warning now says the arm is FED, and names the 32-char-vs-4-byte encoder trap) · `.claude/rules/security.md` §Supply chain + CI (interval discharged/reset).
- **Leaves recomputed and unchanged** — `.claude/docs/stack.md` (arch §Stack untouched) · `.claude/docs/security-summary.md` (carries no audit-interval statement).
- **Closure asserted:** every retired phrase returns zero hits across masters + leaves (excluding append-only sidecars and the immutable master-route). `CLAUDE.md` 153/200; all 20 section markers intact.
