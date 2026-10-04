# Scope — Retry-storm interpretation names its cause

**Marker:** `2026-10-04-retry-storm-interpretation-names-its-cause` · **Version:** andromeda-pulse-0.3.0 · **Taken up:** 2026-10-04

## Intent (working entry, verbatim title + hint)
Retry-storm interpretation names its cause — an incident born of a retry storm names the retry, not only an
error-rate spike.

## What this chunk builds
Two halves, in order, per the founder ruling 2026-10-02 ("the fix follows the measurement"):

1. **Measure why.** Establish, from evidence this host can produce, why an incident whose triggering digest carried a
   `retry_storm` cue was interpreted as "Error Rate Spike" with no retry named (Conductor d3, 2026-10-01). The
   measurement must say WHERE the retry fact is lost, among these candidate layers (closed at P3 — research.md
   §Half 1):
   - **the digest** — VERIFIED it carries the retry: a Tier1 digest holds exactly the triggering cue, rendered as
     `[autonomous] retry_storm — retry_storm scope_id=S` under ATTENTION CUES, plus `OVERALL: anomalous`
     (`assembler.rs:261-272,652-686`). It ALSO carries CORPUS MATCHES selected on any current Q1 service scope, which
     can put a differently-kinded active incident's title in front of the model (`assembler.rs:286-287`).
   - **the prompt** — VERIFIED it presents the cue only as that one digest line: no role, convention, reminder or
     schema description ties `title` / `symptom` / the rank-1 hypothesis to the triggering cue (`prompt.rs:85-112`,
     `schema.json`).
   - **the model** — `[inferred]` (hypothesis, strongly supported, not measured): d3's Timeline
     `Error Rate Spike in <workspace-key>: 3m ago, active` has the corpus-match line shape
     (`retrieval.rs:144-147`), and only incident 7 (an active error_rate_spike incident opened 203.75 s earlier) fits;
     its title is not in the capture. The counterfactual needs the real model, absent on this host.
   - **the render** — VERIFIED it has no deterministic carrier: Title / Symptom / Timeline / Hypotheses are all
     model-authored, and `incident.kind` is rendered nowhere in the report (`markdown.rs:115-329`).
   The measurement is the chunk's first deliverable; its result decides half 2.
2. **Ship the remedy the measurement supports** — so an incident born of a retry storm names the retry. The shape
   (digest content, prompt framing, deterministic grounding of a field from the cue, or a combination) is a P4 fork
   decided from the half-1 result, never from a hypothesis.

## Boundaries
- The incident's identity tuple `(kind, scope, scope_id)` is decided (arch §Established Decisions [Fault Identity])
  and is not reopened; this chunk concerns what the incident's interpretation SAYS, not how incidents coalesce
  (verified: identity comes from `digest.attention_cues.first()` at `inference_runtime.rs:794-800`, untouched by any
  remedy branch in research.md).
- No new port, TauRPC procedure, capability JSON, corpus table or MCP tool is expected (verified for the branches
  research found: a render-side change in `assemble_report` reaches TauRPC and MCP through the existing projection).
  A new `Report` FIELD would change the TauRPC payload type and the generated bindings — if a remedy needs one, it is
  surfaced at P4, not absorbed silently.
- Not in scope: the L4 hardware probe on Arch hosts (its own route entry, next), the real-model availability on this
  host (a founder desk act), and the Conductor-side grading (`v3-09` is Conductor's matrix entry, not Pulse's).
- A finding that some OTHER cue kind loses its cause the same way is recorded and routed, not absorbed.

## Surfaces touched (closed against the code at P3)
- `crates/triage/src/digest/assembler.rs` — verified as the measurement's subject; it carries the retry, so a remedy
  need not change it (a corpus-match framing change would, and moves the `overall_line_*` /
  `unit_digest_runtime_scrub.rs` pins).
- `crates/interpretation/src/prompt.rs` (+ the embedded `schema.json`, `PROMPT_VERSION_*` in `schema.rs`) — verified:
  the triggering cue is not framed beyond its digest line. Template text stays ASCII (the argv-transport rule).
- `pulse-app/src/inference_runtime.rs::create_incident_from_l4_output` — verified: identity from the first cue
  (`:794-800`); `title` / `detail` from the model only (`:883-884`).
- [premise-corrected: `assemble_report` lives in `crates/interpretation/src/markdown.rs` since chunk #94, not in
  `incidents_router.rs`] the report render — `crates/interpretation/src/markdown.rs::assemble_report`, the single
  projection both `incidents_router.rs:479` (TauRPC `get_report`) and `crates/mcp-server/src/tools.rs:390` (MCP
  `retrieve_report`) call; Findings rows show `incident.title` (`incidents_router.rs:60`).

## Folded freight (from the working entry)
- **CONTEXT (verbatim):** "measure first — Conductor's d3 rank-1 interpretation (2026-10-01, relayed by the pc
  overseer) read "Error Rate Spike" with no retry named; why is unmeasured; the fix follows the measurement (founder
  ruling 2026-10-02)".
  - Coordinate re-verified at the artifact (Conductor repo, read-only):
    `conductor-0.3.0/chunks/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/report.md:162-167`
    grades d3 `NotIdentified`, rank 1 "Error Rate Spike in <workspace-key> indicates a potential issue with the
    Conductor service." (names `conductor`, no retry token). Its capture `evidence/rm-capture-d3.txt` holds the
    attributed report body (incident 10) and the Pulse log window.
  - Facts read from that capture — re-derived at P3 against the capture's own lines (verified, with the
    incident-10 ← tier-1 `retry_storm` digest link resting on timestamp adjacency plus HEAD's identity code):
    - the d3 window carries BOTH cue kinds: tier-1 digests with `cue_kind=retry_storm` (autonomous) and tier-2
      digests with `cue_kind=error_rate_spike` (suggested), interleaved (`digest.assemble.request` lines);
    - incident 10 was created at 20:04:57.902Z (`created=true severity=error priority_tier=autonomous`), whose parse
      (20:04:57.882Z) follows the prompt assembled at 20:04:53.604Z — the tier-1 `retry_storm` digest of
      20:04:53.596Z, by timestamp adjacency only;
    - the rendered report's Symptom and Timeline both read "Error Rate Spike in <workspace-key>"; its Previously Seen
      lists incident #2 as "retry_storm scope_id=conductor";
    - d1 (same series) was graded `Identified` with rank 1 `retry_storm scope_id=conductor is anomalous`, which
      Conductor notes only restates the cue line.
- **Causal claim (no marker on the entry; kept verbatim):** "why is unmeasured". No mechanism is asserted; the
  candidate layers above are this scope's own `[inferred]` enumeration.
- **Founder ruling 2026-10-02** (relayed by the pc overseer): measure first; the fix follows the measurement.
- **watch:** CI `boot smoke (ubuntu-22.04)` — the app reached `boot: ready`, then ended `exit 1` with no `app.exit`
  and no `app.panic.fatal` record before `agent-run.sh status` (run ci#37189514735 attempt 1 on a073722; the re-run
  of the failed job was green; cause not established; first red in 25 runs) (1/3; since
  2026-10-04-corpus-key-creation-is-race-free; 1 green so far: ci#37200709989 on 2099998). Observation only — no
  criterion, task or gate.

## Measurement constraints (operator facts, overseer phase note 2026-10-04)
- **The real L4 model is NOT on this Linux host**: no `Llama-3.2-3B*.gguf`, no `llama-cli` / `llama-server` under
  `~` (searched 6 levels deep); the AI-Model dir and the old `.exe` paths were Windows-only. If measuring the cause
  (or proving the remedy) needs the real model, that is said at P3 or P4 and the chunk STOPS there; restoring the
  model is a founder desk act.
- Available instead: the deterministic L4 path and the Conductor d3 evidence (the 2026-10-01 series).
- Any run that launches pulse-app needs a slot from the operator first (ports 4317/4318 are shared with
  conductor-builder).

## CI verdict read at Setup (5a)
- `e71dba5` (the last wrap's flip = HEAD): at Setup **verdict not yet available** — ci#37203184509 in progress (12
  of 13 checks running at read; oldest `lint / test (ubuntu-22.04)` 139 s), secret-scan#37203184546
  completed/success. Re-read at P3: **still in progress** (1 check running — the coverage gate, 696 s); no red to
  fold. The wrap reads the settled verdict.
