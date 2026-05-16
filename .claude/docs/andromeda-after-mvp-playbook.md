# Andromeda Post-MVP Workflow Playbook

_Read by future agents (after `/clear` → `/andromeda-new-session`) to immediately understand the ongoing-work pattern in this project. Authored at session 66 (2026-05-16) when pulse moved from v0.1.0 MVP (chunks #1-#56 / Epochs 1-8 closed) to v0.2.0 evolution mode._

_Future setup-project enhancement should generate this file from a template (see `docs/andromeda-improvements.md` Proposal 2) so future Andromeda projects past MVP get this playbook automatically. For now this file is hand-authored in pulse._

---

## TL;DR — workflow shape

```
[Session N — chunk work]
new-session → evolve --allow-route-append → phase → implement → wrap

[Session N+1 — drift resolution, optional]
new-session → evolve (+ flags as needed) → setup --delta → wrap

[OR for structural arch changes (Pattern 4):]
[Session — arch prep] manual arch.md edit → setup (full) → wrap
[Session — chunk work] new-session → evolve --allow-route-append → phase → implement → wrap
```

Greenfield skills (`/andromeda-arch`, `/andromeda-route`, 6 specialist commands) are **write-once** — they produce documents, not amend them. Post-MVP evolution uses `/andromeda-evolve` (with narrow flags) + `/andromeda-setup-project --delta` for propagation through Tier 2/3 + CLAUDE.md ecosystem.

---

## Project state

- **Past v0.1.0 MVP:** route §2 56/56 chunks done; Epochs 1-8 closed (commit `de35e82`, 2026-05-14).
- **Targeting v0.2.0:** new chunks #57+ added via evolve flow (one at a time).
- **External v0.2.0 plan docs:** `docs/v0_2_0/` contains capability-spec / distillation-architecture / route v2 + vision (33 prospective chunks, 2 blocking decisions: Pre-D1 LLM runtime choice, Pre-D2 Drain Rust spike). These are PLANNING references — they don't directly drive the Andromeda pipeline until absorbed into specialist plans + arch + route via evolve amendments or manual edits.

---

## The 4 patterns

### Pattern 1 — Arch registry update (most common case)

**Use when:** new TauRPC procedure / broadcast topic / env var / capability identifier / reserved table / workspace crate name landed in code (or about to land via a chunk), and `arch.md` §Occupied Resources or §Workspace needs to acknowledge.

```
/andromeda-evolve --allow-arch-registry
  → Phase 1c dialog: which §section, what additions, code_evidence paths
  → Phase 3 Check 7: purely additive / registry section / code reality matches / no new architectural concept
  → Phase 6 atomic write:
      • marker file `.andromeda/runs/{ISO}-spec-amendment-{slug}/amendment.md` with Flag authorization block (flag_used + registry_section + registry_additions + code_evidence)
      • `arch.md §Architecture Registry Updates` — new Decisions Log entry
      • `state.yaml.spec_amendments.active` append with flag_used field

/andromeda-setup-project --delta
  → Phase 0 detects arch.md amendment, verifies flag_used + Trigger signature
  → Type 6 permit path: expected_propagation typically empty (registry addition doesn't cascade to Tier 2/3)
  → Phase 9 sets propagated_by_run = {ISO}-setup-project-delta/ run-dir
  → Commit (often just state.yaml + marker lifecycle checkbox)

/andromeda-wrap-session (next session)
  → Phase 8 lifecycle: archived_at + move to archive list
```

**Constraints (Check 7):** purely additive (no modification of existing entries) / registry section only (§Occupied Resources / §Workspace / §Capability Registry / similar list-style sections) / code evidence resolves to real files in repo / no new architectural concept (adding the FIRST entry to a section triggers a warning + user confirmation).

**Disallowed even with flag:** §Established Decisions / §Cross-cutting Patterns / §Stack / §Project Intent / §Design Philosophy — these require Pattern 4.

---

### Pattern 2 — Route chunk append (two forms)

**Use when:** new chunk needs to be added to `route.md` §2 — either to existing epoch (Form 1) or as first member of a new terminal-position epoch (Form 2). Form 2 added 2026-05-16 per `docs/andromeda-improvements.md` Proposal 4 (IMPLEMENTED).

**Form 1 — chunk append to existing epoch:**

```
/andromeda-evolve --allow-route-append
  → Phase 1c dialog: which Epoch K, new chunk text, motivation
  → Phase 3 Check 8: additive / existing epoch / position-stable / motivation grounded / format conforms / Decisions Log well-formed
  → Phase 6 atomic write: marker + route.md §2 insertion + §3 Decisions Log entry + state.yaml.spec_amendments.active

/andromeda-setup-project --delta → lifecycle progression only (route has no Tier 2/3 dependents)
/andromeda-wrap-session → archives
```

**Form 2 — terminal-position new epoch + first chunk(s):**

```
/andromeda-evolve --allow-route-append
  → Phase 1c dialog: new epoch name (K = max+1), epoch boundary rationale, first chunk(s) text, motivation
  → Phase 3 Check 8: additive / new terminal epoch / NON-EMPTY body (≥1 chunk) / terminal-position / position-stable / motivation grounded / format conforms / Decisions Log well-formed
  → Phase 6 atomic write:
      • marker (Type 7 Form 2 fields: new_epoch_created=true, new_epoch_title, new_epoch_position, epoch_boundary_rationale, scope_summary_updates)
      • route.md §1 mechanical update: "Total chunks: N→N+M", "Epochs: N→N+1"
      • route.md §2 appended `### Epoch K — {name}` heading + ≥1 chunk in body
      • route.md §3 Decisions Log entry citing epoch creation
      • state.yaml.spec_amendments.active

/andromeda-setup-project --delta → lifecycle progression only
/andromeda-wrap-session → archives
```

**Constraints (Check 8) — both forms:**
- Purely additive — no chunk text modification, no deletion, no reordering, no existing epoch heading rename.
- Insertion point > `state.yaml.last_completed_chunk.route_index` (no shifting completed chunks).
- Motivation grounded — concrete trigger (in-progress chunk ref OR amendment_id OR concrete trigger); abstract "future scope" refused.
- Chunk text ≤25 words single line.
- §3 Decisions Log entry has Decision / Rationale / Impact / By fields.

**Form 1 additional:** insertion target is EXISTING epoch heading.

**Form 2 additional:**
- New epoch position is terminal — K = max(existing K) + 1 (mid-route insertion refused).
- New epoch body non-empty — ≥1 chunk in same evolve invocation (empty placeholder refused).
- Motivation cites WHY existing epochs don't fit semantically.
- §1 mechanical updates ONLY to "Total chunks" + "Epochs" count lines; other §1 metrics (Drilldown depth / Hierarchy mode / Ordering principle) stay stale until next /andromeda-route re-generation.

**Use Form 2 when:** new chunks belong to a conceptually distinct phase that doesn't fit existing epoch semantics (e.g., post-MVP v0.2.0 feature work that doesn't belong in v0.1.0's "Polish & ship" Epoch 8).

---

### Pattern 3 — Specialist plan amendment (Type 1-5)

**Use when:** specialist plan (security / design / layout / tests / obs / a11y) needs textual update — clarification, cross-plan reconciliation, gap addition, cross-reference, or deprecation notice.

```
/andromeda-evolve   (no flags)
  → Phase 2 classify Type 1-5:
      • Type 1 — single-plan clarification (typo, wording, missing detail)
      • Type 2 — cross-plan reconciliation (≤3 plans! else Refuse 4)
      • Type 3 — documented gap addition (acknowledging missing coverage)
      • Type 4 — cross-reference clarification (two plans, different vocab, same concept)
      • Type 5 — deprecation notice (mark for future cleanup)
  → Phase 6 atomic write:
      • marker file
      • Decisions Log entry in each affected plan
      • state.yaml.spec_amendments.active

/andromeda-setup-project --delta
  → Per plan→file mapping table (in delta-rerun-protocol.md):
      - security-plan.md → security.md + security-summary.md + CLAUDE.md warnings
      - test-plan.md → testing.md + verification-harness.md + tests-summary.md + workflow
      - obs-plan.md → observability.md + obs-summary.md + workflow
      - a11y-plan.md → a11y.md + a11y-summary.md + warnings
      - design-system.md → design-summary.md + warnings
      - layout-templates.md → design-summary.md
  → Grep-expansion defense-in-depth (catches missed cascades by searching for old values across .claude/ + CLAUDE.md)
  → Phase 1-3 regenerate affected Tier 2/3 + CLAUDE.md GENERATED:setup:* sections
  → Phase 8 byte-identity check on preserved files
  → Phase 9 lifecycle + commit

/andromeda-wrap-session → archives
```

**Constraints:** ≤3 plans per evolve invocation (Refuse 4 — cascade danger); changes must not trigger Refuse 1 (technology shift) / Refuse 2 (principle violation) / Refuse 3 (architectural inversion) / Refuse 5 (impl code change).

**Splitting for >3 plan changes:** if a chunk needs to touch 4+ specialist plans, run multiple evolve invocations — each covers ≤3 plans. The split is the user's responsibility (skill won't auto-split).

---

### Pattern 4 — Arch structural change

**Use when:** new entry needed in `arch.md` §Established Decisions / §Stack / §Cross-cutting Patterns (NOT registry sections — those are Pattern 1).

**Status:** NO automated evolve path currently exists. Proposed `--allow-arch-decision` flag (see `docs/andromeda-improvements.md` Proposal 1) would address this gap; until implemented, manual edit is the path.

**Current workflow:**

```
1. MANUAL edit arch.md (add new §Established Decisions entry / §Stack row / §Cross-cutting bullet)
2. /andromeda-setup-project   (FULL mode — NOT --delta; --delta refuses arch.md changes without flag_used)
   → reads updated arch, regenerates CLAUDE.md GENERATED:setup:* sections + Tier 2/3 sections that derive from arch
3. /andromeda-wrap-session
```

**Discipline for manual edits** (to preserve audit-trail spirit):

- Keep edit narrow — single addition per session (don't bundle multiple structural changes).
- Make edit additive — never modify existing decisions in the same session as adding new ones.
- Add Decisions Log entry in `arch.md §Architecture Registry Updates` (or new `§Architecture Decisions Log` section if existing convention insufficient) — explain WHAT was added and WHY, similar to how evolve auto-generates Decisions Log entries.
- Reference manual edit in subsequent session-handoff "Key Decisions" — preserves chat context even though no formal marker file exists.

**Pulse v0.2.0 chunks requiring this pattern:**

- **Chunk #69 (corpus scaffold)** — adds encryption-at-rest + OS keychain + persistent SQLite as new architectural concepts. Touches §Established Decisions + §Stack (rusqlite + age/keychain libs) + possibly §Cross-cutting Patterns (encryption discipline).
- **Chunk #74 (LLM runtime + hardware profile)** — adds chosen LLM runtime per Pre-D1 to §Stack + §Established Decisions rationale entry.
- **Chunk #84 (MCP positioning)** — adds §Established Decisions entry formally stating MCP is one of three equal-tier output channels.

For each: split into pre-chunk session (manual arch edit + setup full mode + wrap) followed by chunk implementation session (new-session → evolve --allow-route-append → phase → implement → wrap).

---

## Drift resolution table

Wrap-session Phase 6 detects 6 dimensions D1-D6 per `~/.claude/skills/andromeda-wrap-session/references/integrity-protocol.md` Part C. New-session Phase 7 surfaces these in the start-of-session dashboard.

| Drift | Cause | Resolution pattern |
|---|---|---|
| **D1** | Living artifact stale (code mtime newer than `dep-tree.md` / `api-surface.md` reconciled_at) | Auto-resolved by wrap-session Phase 5 reconcile (re-runs `cargo tree` + `cargo public-api`) |
| **D2** | LIVING block content wrong despite reconcile (rare bug — diff between fresh tooling and current content) | Re-run wrap; manual paste if persists |
| **D3** | Plan-to-code drift — new procedure / topic / capability / env var in code that arch §Occupied Resources doesn't acknowledge | Pattern 1 (`/evolve --allow-arch-registry` + `setup --delta`) |
| **D4** | Plan-to-plan inconsistency — specialist plan A references state that specialist plan B has changed in its Decisions Log | Pattern 3 (Type 2 cross-plan reconciliation evolve + `setup --delta`) |
| **D5** | Plan mtime newer than CLAUDE.md — specialist plan was regenerated/amended since last setup-project run | If matched to active amendment with `propagated_by_run=null`: `setup --delta` clears. If no amendment match: `setup` full mode OR investigate edit source |
| **D6** | `state.yaml.last_completed_chunk.commit_sha` doesn't match git log's expected chunk-progression commit | Self-clears in next wrap-session Phase 10 post-commit SHA-fixup amend |

**Spec amendment lifecycle (related to D5):**

Amendments progress through 4 stages: `applied` → `noted` → `propagated` → `archived`. State.yaml tracks lifecycle; wrap-session and setup-project advance stages. New-session dashboard surfaces pending amendments distinctly from generic drift.

---

## Decision tree per chunk

```
This chunk modifies:
├── ONLY arch §Occupied Resources / §Workspace / §Capability Registry (registry sections)?
│   → Pattern 1 (/evolve --allow-arch-registry + setup --delta)
│
├── arch §Established Decisions / §Stack / §Cross-cutting Patterns?
│   → Pattern 4 (manual arch edit + setup full mode) — runs as pre-chunk session
│   → Future: Pattern 4 via /evolve --allow-arch-decision (proposal 1)
│
├── Specialist plan(s) — count ≤3?
│   → Pattern 3 (/evolve + setup --delta)
│
├── Specialist plan(s) — count 4+?
│   → Pattern 3 × N (split into multiple /evolve runs, each ≤3 plans)
│
├── Adds chunk to route.md §2 existing epoch?
│   → Pattern 2 Form 1 (/evolve --allow-route-append + setup --delta)
│
├── Adds NEW terminal-position epoch + ≥1 chunk to route.md?
│   → Pattern 2 Form 2 (/evolve --allow-route-append + setup --delta)
│     New epoch K = max(existing) + 1; non-empty body required.
│
└── Adds NEW mid-route epoch (between existing epochs)?
    → MANUAL route.md edit (insert new ### Epoch K — {name} heading + chunks)
       + manual §3 Decisions Log entry
       + setup full mode (route has no Tier 2/3 dependents but full setup re-checks consistency)
       + wrap
    (Note: /evolve --allow-route-append refuses new epoch creation even with flag. New epoch is /andromeda-route territory, but that skill is greenfield-only — manual edit is the practical path.)
```

---

## Key commands reference

| Skill | Purpose | When to invoke |
|---|---|---|
| `/andromeda-new-session` | Start-session dashboard | Every fresh session begin — reads `session-handoff.md`, surfaces drift, suggests next action |
| `/andromeda-evolve [flags]` | Plan-level evolution (Type 1-7 amendments) | Adding to specialist plan / arch registry / route.md additive — see flag matrix below |
| `/andromeda-phase` | Plan implementation phase for chunk | After chunk added to route.md, before `/andromeda-implement` |
| `/andromeda-implement` | Execute chunk plan; fix-loop until tests pass | After `/andromeda-phase` |
| `/andromeda-setup-project [--delta]` | Propagate amendments to Tier 2/3 + CLAUDE.md ecosystem | `--delta` after evolve amendments (scoped); without flag for full re-derive (handles manual upstream edits) |
| `/andromeda-wrap-session` | Session end maintenance | End of every working session — curation, reconcile, drift detection, commit |

**Evolve flag matrix:**

| Flag | What it permits | What it still refuses | Classification |
|---|---|---|---|
| (none) | Type 1-5 specialist plan amendments | arch.md body / route.md changes / impl code / >3 plan touches | Types 1-5 |
| `--allow-arch-registry` | Purely additive entries to arch.md §Occupied Resources / §Workspace / §Capability Registry | §Established Decisions / §Stack / §Cross-cutting / §Project Intent / §Design Philosophy / modifications | Type 6 |
| `--allow-route-append` | Form 1: new chunks in existing route.md §2 epochs. Form 2: terminal new epoch + ≥1 chunk in body. | Chunk modification / deletion / reordering / mid-route epoch creation / empty epoch creation / existing epoch heading rename / §1 modifications beyond Total chunks + Epochs auto-update | Type 7 |
| `--allow-arch-decision` (PROPOSED) | Purely additive entries to §Established Decisions / §Stack / §Cross-cutting Patterns | §Project Intent / §Design Philosophy / modifications / new sections | Type 8 (future) |
| `--dry-run` | Show proposed artifacts without writing | — | Combinable with any classification |

---

## NOT implemented as separate skill folders

These appear in `arch.md §Project Intent` + various SKILL.md descriptions as redirect targets, but **the skill folders do not exist** in `~/.claude/skills/`:

- `/andromeda-scope-arch` — referenced in `arch.md §Project Intent` ("Scopes will be added via /andromeda-scope-arch"). Intentionally not implemented (pipeline-simplicity decision per user 2026-05-16). Manual arch edit + setup full mode covers this case.
- `/andromeda-scope-route` — same. Manual route edit covers.
- `/andromeda-scope-{specialty}` — same. Use Pattern 3 (evolve) for specialist plan changes.

Don't invoke these — they'll fail. Use Pattern 1/2/3/4 mechanics instead.

---

## Trigger 4 — drift resolved inside `/andromeda-implement`

If a test or harness fires during `/andromeda-implement`'s fix-loop and surfaces spec ↔ reality drift, `/andromeda-implement` enters its Trigger 4 dialog (per `~/.claude/skills/andromeda-implement/references/spec-drift-protocol.md`):

- **Path A** — amend spec (creates spec-amendment marker right in the same session; subsequent setup-project --delta propagates).
- **Path A'** — fix impl to match spec (no amendment; just code change).
- **Path B** — defer (note in session-handoff "Deferred decisions" section; address in subsequent session).

This is an alternative to post-wrap drift resolution. Both paths are valid:

- **Trigger 4 in implement** — drift caught WHILE working on a chunk; resolved before chunk commits.
- **Wrap Phase 6 detection** — drift caught at session end (D3 capability-drift, D4 cross-plan inconsistency, D5 mtime drift); resolved in subsequent session via evolve + setup --delta.

---

## Cross-references to skill mechanics

Full skill details in `~/.claude/skills/andromeda-*/`:

- **Evolve mechanics:** `andromeda-evolve/references/`
  - `refuse-taxonomy.md` — 6 refuse categories + flag exceptions
  - `classification-taxonomy.md` — Types 1-7 (and proposed 8) classification rules
  - `validation-checks.md` — Checks 1-6 always run; Check 7 (arch-registry flag), Check 8 (route-append flag)
  - `output-templates.md` — marker file format + Decisions Log entry templates
  - `dialog-templates.md` — conversation flow templates
  - `example-runs.md` — annotated case studies (regression suite)

- **Setup-project --delta:** `andromeda-setup-project/references/delta-rerun-protocol.md`
  - Plan→file mapping table (which Tier 2/3 files depend on each specialist plan)
  - Type 6 permit path for arch registry amendments
  - Grep-expansion defense-in-depth

- **Spec amendment lifecycle:** `andromeda-setup-project/references/spec-amendment-protocol.md` (byte-identical copies in wrap-session/ and new-session/ references — enforced via setup-project Phase 8 cross-skill diff check)
  - Part A — marker file format
  - Part B — state.yaml.spec_amendments schema
  - Part C — D5 classification (amendment-aware)
  - Part D — lifecycle state machine

- **Drift detection (6 dimensions):** `andromeda-wrap-session/references/integrity-protocol.md` Part C

- **Andromeda states A-K:** `andromeda-new-session/references/session-state-contract.md` Part C

- **Health checks (14 checks):** `andromeda-{setup-project, wrap-session, new-session}/references/health-criteria.md`

---

## Pulse-specific context

(Drop this section in template — pulse-specific; future projects substitute their own context.)

Pulse v0.2.0 plan lives in `docs/v0_2_0/`. 33 chunks #57-#89 across 12 sub-phases. Reference details:

- `pulse-vision-and-backlog.md` — product soul, three pillars (application + skill + model), roadmap to v2.0
- `pulse-capability-spec.md` — 60 P-XXX capabilities formal contract (across 11 categories)
- `pulse-distillation-architecture.md` — 6-layer pipeline design (L0-L6) + self-observability
- `pulse-v0_2_0-route.md` — execution plan: 33 chunks with capability/distillation/specialist plan mapping + 2 blocking decisions (Pre-D1 LLM runtime choice, Pre-D2 Drain Rust spike)

Pulse is **Andromeda's first dogfood after-MVP experience.** Friction encountered during pulse v0.2.0 chunks is captured in:

- Session learnings (Tier 3 `.claude/docs/session-learnings.md`) — per-chunk observations.
- Improvement proposals (`docs/andromeda-improvements.md`) — when patterns suggest skill mechanic gaps that warrant Andromeda enhancement (e.g., Proposal 1 `--allow-arch-decision` flag).

Goal beyond shipping pulse v0.2.0: distill captured friction into a reusable playbook that future Andromeda projects can follow without rediscovering these patterns. This file is part of that distillation effort.

---

_End of playbook. Read alongside CLAUDE.md (always loaded) and `.claude/session-handoff.md` (start-of-session state)._
