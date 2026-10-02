# Fan-out results — 2026-08-29-advisory-backlog wrap

7 doc-agents, one per spec source, all returned clean-parsing YAML (no stripping needed anywhere; raw twins saved only for the four proposal-carrying docs).

| doc | verdict | proposals |
|---|---|---|
| arch | 7 proposals (2 primaries + 5 dependent-of) | rmcp mechanism+caveat cluster ×4 ([MCP Server Surface] · §Stack MCP row · [API Style Surface 3] · §Inherited Defaults) + wasmtime requirement-plus-resolved restate ×3 (§Stack · [Plugin Runtime] · §Inherited Defaults). Negatives recorded: §Conventions + §Standard Contracts hold as wire-only; the "fourteen/twelve members" prose inconsistency is pre-existing and arch self-defers to §Occupied Resources. |
| security-plan | 10 proposals (3 primaries + 7 dependent-of) | strict-path retirement cluster ×4 (plugin-host row · env-var row · bootstrap install mandate · anti-patterns "intended primitive") + rmcp mechanism cluster ×3 (MCP row · threat-model vector · code-patterns naming) + deps cluster ×3 (Pinning item (b) · MCP-row pin sentence · standing-deferral session-58 discharge + 8→0 re-enumeration). Negatives: D-security-auth clean (no key/crypto delta); D-security-logging clean (no scrub-posture delta, restating sites verified). |
| obs-plan | 5 proposals (1 primary + 4 dependent-of) | the `rmcp 0.6` label sweep (§1 harness restate · §3 sink · §3 snapshot bullet · §4 table row) + the §4 P3 Cleanup "rmcp auto-closes" falsified mechanism claim. |
| test-plan | 2 proposals (1 primary + 1 dependent-of) | §9 Cranelift snippet recorded SHIPPED (ci.yml supply-chain `run:` step, this chunk) + §1 trigger's "(xtask check enforces)" attribution corrected (no xtask covers it). Negatives: D-tests-framework clean (standard gate set matches §3/§4; webview trio correctly excluded); D-tests-obs-harness clean. Adjacent flag: §5 `wasmtime 25.x` tooling cell — folded in the cascade's verbatim-citation pass. |
| design-system | proposals: [] | no rmcp occurrence in the doc; the status-narrative sites (Halo/signature) untouched by this chunk. |
| layout-templates | proposals: [] | MCP contacts are UI-toggle-level and AFFIRMED by the report; DEFERRED blocks are baseline. |
| a11y-plan | proposals: [] | no new surface; the wasmtime entity row carries no version claim; MCP refs are toggle-level. |

## Validation outcomes (the 6 checks)
1. Playbook: 20 proposals routine (2026-07-08 apply-accurate-this-chunk · 2026-08-14 apply-doc-only-disproof · 2026-08-23 APPLY-BY-ACTUAL-CLASS for the escalate-severity detectors firing on measured count/pin/retirement corrections · the strict-path set additionally riding the operator's P4 DROP ratification) · 4 proposals (the cross-master rmcp MECHANISM wording) staged to ESCALATE.
2. Cross-contradiction: none.
3. Intent-consistency: aligned (deviations justified in the report; the rmcp `"3"` target satisfies the entry's `≥1.4.0` with operator ratification).
4. Absence-needs-evidence: all absence claims cite greps (ci.yml cranelift measured-missing at research; detector negatives carry their search basis).
5. Expected-amendments floor: every plan-listed entry matched by a proposal (obs-plan exceeded its "none expected" with 5 real label corrections).
6. Disproved-claims disposition: #1 rmcp mechanism → escalation→applied across arch/security/obs + leaf cascade · #2 security 0.3.x pin → applied · #3 arch caveat → applied · #4 deny.toml provenance → fixed in-chunk (recorded).

## Escalation resolution
ONE escalation, resolved WITH the operator: the rmcp mechanism wording → **amend to measured reality** (hand-rolled `jsonrpc.rs` canonical; rmcp = linked feature-gated anchor dependency; wire-format ban restated as measured-conformant). All proposals then applied as returned.

## Apply + cascade tally
- Master body edits: arch 11 · security-plan 9 · obs-plan 11 · test-plan 4 = **35 edits** carrying the 24 proposals + the step-2 verbatim-citation folds (arch dir-tree comment + 3 "rmcp sidecar" namings · test-plan §1 row citation + §5 wasmtime cell · obs §11 ban naming + 4 naming leftovers).
- Sidecars: 4 entries appended (architecture / security-plan / obs-plan / test-plan amendments).
- Cascade leaves re-derived: CLAUDE.md (overview line · Stack paragraph · mcp-server module line · §Critical Warnings path-primitive line) · docs/gotchas.md ×2 · docs/stack.md ×2 · docs/security-summary.md ×3 · docs/obs-summary.md ×3 · docs/services/mcp-server.md ×5 · docs/services/{corpus,ingest,plugins,workspace-detector}.md ×8 · rules/security.md ×2 · rules/observability.md ×2 · agents/code-reviewer.md ×1. tests-summary §Cranelift line verified still-true (generic build-time phrasing, no enforcer misattribution). `.claude/backup/*` untouched by design.
