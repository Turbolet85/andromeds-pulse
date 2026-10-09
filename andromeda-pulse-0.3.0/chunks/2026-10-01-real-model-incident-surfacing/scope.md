# Scope — 2026-10-01-real-model-incident-surfacing

**Working entry (verbatim title + hint):** Real-model incident surfacing — the real model turns a storm digest into
an incident reliably, not in some storms only.

**Chunk base:** `09d0809` (W182) · version andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification.

## Folded freight (the entry's two blocks: CONTEXT 773 chars · PREREQ 727 chars — `route.py pins`)

### CONTEXT
- Founder 2026-09-30 (asked, answered) «Чинить в Pulse 0.3.0» (same relay §2) — the fix lands in this version.
- "measured by Conductor's series on clean input (Conductor `:56`, Pulse `fcc31b2`; that repo is read-only from
  here — re-verify at /phase): Llama 3.2 3B surfaced the canary in 4 of 9 storms and dismissed the scenario's storm
  digest both times it emitted (parse `ok`, no incident)" — a COMPANION-REPO measurement, cited not copied. The
  operator's take-up directive restates it as "canary 4 of 9 and scenario storm 0 of 2 (its 2026-09-30 series)".
  [premise-corrected: the counts are as the Conductor ledger states them (`attempt-ledger.md:56-66`, read-only), but
  "dismissed" there means "parse `ok`, no incident outcome" — an inference from SILENCE. At HEAD that silence is
  produced by FOUR distinct outcomes Pulse never logs apart: `decision=dismiss`, `severity=none`,
  `is_resolution_summary=true` (all model-authored; `pulse-app/src/inference_runtime.rs:762-767`) and a cue-less digest
  (`:786-787`). So "the model dismissed" is unproven; "the real model's output produced no incident" is what was
  measured. Measured at Pulse `fcc31b2`, before span-level redaction; re-measured here only in an operator slot]
- "measure WHY first — the prompt's surface/dismiss guidance, what the digest carries, the thresholds, the sampling"
  — the four named candidate causes. [VERIFIED as candidates, each grounded at HEAD (research.md §Mechanism
  re-derivations); their CAUSAL weight is unmeasured, and discriminating between them is the chunk's first deliverable.
  Offline facts behind each: (guidance) the prompt defines surface/dismiss/watch only as labels and states no criterion
  for when a signal warrants surfacing (`prompt.rs:96-103`); (digest) every Tier1 storm digest prints
  `OVERALL: nominal` by construction (`assembler.rs:225-236`, `:642-651`) and its cue line carries no magnitude,
  absolute value or persistence (`assembler.rs:266-270`); (thresholds) the decisive threshold is the creation predicate's
  three model-authored gates, not the storm-detection thresholds upstream — Conductor's digests DID reach the model
  (parse `ok`); (sampling) see the next bullet. A FIFTH candidate surfaced in research: the schema lists `decision`
  before every analysis field (`schema.json:36` vs `:46-87`), so under constrained generation the model may commit to a
  decision before writing any reasoning — unverified whether llama.cpp's grammar keeps that order (read live)]
- "`build_llama_cli_args` passes no `--seed` / `--temp`, so llama.cpp's own sampling defaults apply — measured at HEAD
  `pulse-app/src/llamacli_inference.rs:405`" — COORDINATE VERIFIED at take-up: `pub fn build_llama_cli_args` is at
  `:405` and its vector carries `-m` · `-ngl` · `-st` · `--simple-io` · `--no-display-prompt` · `--log-disable` · `-n` ·
  `--json-schema-file` · `-p`, with no seed or temperature argument. The defaults that therefore apply are llama.cpp
  b9305's `temp 0.80 · top_k 40 · top_p 0.95 · min_p 0.05 · seed 0xFFFFFFFF` (`common/common.h`, `include/llama.h`,
  fetched). [the CAUSAL part — that default sampling contributes to the dismissals — stays unmeasured; a 4-of-9 rate is
  what a high-variance sampler over a near-50/50 decision would produce, which is exactly why it must be measured, not
  assumed]
- "ordered after span-level redaction so it is measured on an honest input" — that chunk is `complete` (master), so
  the digest no longer collapses to one placeholder on a single true positive.
- "Conductor's third v3-09 series waits on this entry"; the operator's directive names Conductor `:71` as the waiting
  coordinate. [cited, not verified from here — a companion-repo coordinate with no bearing on this chunk's scope]

### PREREQ — the CI source-lint pair (from 2026-09-30-span-level-redaction, operator-directed at its wrap 2026-10-01)
- (1) "the ci.yml 'no Cyrillic in source' step (an inline python step over `.rs`/`.ts`/`.tsx` under crates ·
  pulse-app/src · pulse-app/tests · pulse-app/ui/src · xtask/src) crashes with `UnicodeEncodeError` under the Windows
  runner's cp1252 console whenever it has hits to print, so on windows-latest it fails as a bare `exit code 1` with no
  readable finding (measured on `ci#36842111417`, sha `9d14166`)". COORDINATE VERIFIED at take-up: the step is
  `.github/workflows/ci.yml:90-111` (`shell: python`; it prints each hit's line text, which carries the Cyrillic, to
  stdout). [VERIFIED against the recorded run — `ci#36842111417`, sha `9d14166`, job `lint / test (windows-latest)`
  110303404014: the step raised `UnicodeEncodeError: 'charmap' codec can't encode characters in position 109-114` at
  `print(f"::error::Cyrillic character in source - {h}")`, while the same step on `lint / test (ubuntu-22.04)` printed
  both hits readably (`crates/security/src/scrubber.rs:745` / `:746`). Mechanism per the Python docs (fetched): a pipe
  on Windows uses the ANSI codepage unless `PYTHONIOENCODING` / `PYTHONUTF8` is set — documented behaviour, not a
  runner-image defect]
- (2) "`cargo xtask pre-push:linux` does not run that step, so a Cyrillic source literal passed the local pre-push
  green and failed CI on all three `lint / test` jobs (same run)". COORDINATE VERIFIED: the verb lives at
  `xtask/src/pre_push.rs` (registered `xtask/src/main.rs:240`); `grep -i cyrillic xtask/src` finds nothing.

### Operator take-up directive (2026-10-01, this phase's invocation)
- PREREQ first, then the incident-surfacing work.
- "measure WHY the real model dismisses a storm digest before choosing a fix; no retune-until-pass."
- "Any pulse-app/model run or window is my slot: STOP and ask with its length." (Ports 4317/4318 are shared with
  conductor-builder — handoff Notes.)
- Disk 76 GB free; stop the session's rust-analyzer flycheck cargo tree by PID before heavy builds (never
  rust-analyzer itself — `host-win32.md` Session Additions 2026-10-01).

### CI read at Setup 5a (the last wrap's flip through HEAD — one sha)
- `09d0809` (the span-level-redaction wrap commit): **verdict not yet available** — `ci#36858849215` in progress
  (11 checks running; oldest `supply-chain` at 239 s), `secret-scan#36858849245` completed/success. Not a red; no
  disposition owed now. Wall-clock not yet available.

## Outcome
1. **PREREQ:** a Cyrillic literal in source is (a) reported READABLY by CI on every runner OS, Windows included,
   naming file:line, and (b) caught by `cargo xtask pre-push:linux` before a push, so it can no longer pass the local
   gate green and fail CI.
2. **Surfacing:** when the real L4 model reads a storm digest of the kind that should become an incident, it decides
   to surface it reliably — the cause of the dismissals is MEASURED and named first, and the fix addresses that
   measured cause, never a tuning loop run until a sample passes.

## What it builds
- **The source-lint pair.** The Cyrillic check prints its findings without depending on the console's code page
  (ASCII-safe output: escaped codepoints, or a UTF-8-reconfigured stream), and the same check is reachable from
  `pre-push:linux`. [VERIFIED as needed (both defects re-derived above); the SHAPE — one xtask verb shared by CI and a
  new pre-push stage, or the inline ci.yml python made UTF-8-safe with pre-push duplicating it — is a P4 fork]
- **The no-incident outcome becomes observable.** [added by research — premise correction above] Every L4 generation
  whose parse is `ok` but which creates no incident records WHICH of the four skip outcomes fired, as bounded labels only
  (`decision`, model severity, `is_resolution_summary`, `skip_reason`), never digest, prompt or model text. Without it
  neither this chunk's live half nor Conductor's next series can tell a dismissal from a `severity: none` or a stray
  `is_resolution_summary: true`.
- **A discriminating measurement of the dismissals** across the four named candidates. Its OFFLINE half needs no
  operator slot: what the composed prompt says about surface vs dismiss vs watch (`crates/interpretation/src/prompt.rs`
  — the decision guidance at `:88-150`), what a storm digest carries into the prompt, what thresholds gate a storm
  digest's creation, and what the incident-creation predicate rejects (`pulse-app/src/inference_runtime.rs:737-763` —
  `Decision::Dismiss` skips creation). Its LIVE half — running the real model over storm digests and recording each
  decision with its justification — runs only in an operator-granted slot. [VERIFIED feasible: the env-gated
  `pulse-app/tests/integration_real_llama_cli.rs` already drives `LlamaCliInference` directly against a composed prompt —
  no app boot, no ports, skip-clean without the three L4 env vars (unset in the agent shell; the operator supplies them in
  the slot). The prompts are SYNTHETIC storm digests in the exact render format, never captured telemetry (test-plan
  §11). The vehicle's shape — a measurement-only env-gated test or an xtask leg — is a P4 fork]
- **The fix, chosen from the measurement** — candidates the entry names: the prompt's surface/dismiss guidance, the
  digest's content, a creation threshold, or deterministic sampling arguments. [inferred — which one(s) is decided by
  the measurement; the plan states the decision rule in advance. The digest branch has a no-new-struct path:
  `assembler.rs:266-270` builds the cue summary from the full `AttentionCue`, so its quantities can enter the text without
  touching `DigestCueRef`]
- **The false `OVERALL: nominal` line is corrected regardless of the measurement** [added at P5 validation-1,
  intent-incomplete: the overseer's founder-delegated answer at P4 — "a status line the code knows is false must never
  stay in the model input, whatever its measured effect … nothing deferred"]. A0 is recorded on the untouched base
  first, arm A2 measures the corrected line alone, and the GREEN reading reports the combined change with A2's delta
  beside it.
- **A pinned regression witness** for the fix that does not depend on a live model where the cause is offline-testable
  (e.g. a prompt-composition or args-vector assertion), plus the live re-measurement as the outcome evidence.
  [VERIFIED shape: every candidate fix has an offline-pinnable artifact — the `render_payload` text, the prompt
  constants (`PROMPT_VERSION_*` bump), the `build_llama_cli_args` vector; a real-model generation cannot be a committed
  gate (test-plan §10 zero-flake)]

## Boundaries
- No retune-until-pass: the prompt, thresholds or sampling are changed only for a measured reason, and the
  re-measurement count is fixed in the plan before it runs.
- Every run that launches pulse-app, a window, or the model binary is an operator slot — the chunk stops and asks,
  stating the run's expected length. Nothing in plan or implement assumes a slot is free.
- The Conductor repo is read-only from here; its measurements are cited by coordinate, never copied, and never treated
  as a HEAD fact for this repo.
- Detection and redaction (`security::scrubber`) are untouched — the span-level chunk already shipped the honest input.
- The model, the llama.cpp build (b9305) and the GBNF/JSON-schema constraint path are not swapped by this chunk.
  [VERIFIED — arch §Established Decisions [LLM Inference Runtime] locks them; sampling args inside
  `build_llama_cli_args` are within that decision, a model or build swap is not]
- Incident identity stays coalesce-per-cue-identity `(kind, scope, scope_id)`; a predicate change alters only WHETHER an
  incident is created (arch §Fault Identity).
- A prompt-text change also reaches Investigate (`investigate_router.rs:248` is the second production caller of
  `build_primary_tier_prompt`), so its pins move with it.
- Prompt template text stays ASCII (`composed_prompt_templates_are_ascii_clean_for_argv_transport`, security.md
  2026-08-27).
- Conductor return (P-075, the next entry) is NOT absorbed here.

## Surfaces / contracts touched [VERIFIED at P3 — research.md §Files to modify is the authoritative list]
- `.github/workflows/ci.yml` (the Cyrillic step) · `xtask/src/pre_push.rs` + `xtask/src/main.rs` (+ a new xtask verb
  file if the shared-verb shape is chosen).
- `pulse-app/src/inference_runtime.rs` (the skip outcome emit) · `pulse-app/src/observability.rs` (its exact leaf — a
  boot-smoke trigger path).
- Fix candidates: `crates/interpretation/src/prompt.rs` · `schema.rs` / `schema.json` ·
  `pulse-app/src/llamacli_inference.rs` (args) · `crates/triage/src/digest/assembler.rs` (render). [premise-corrected:
  the creation predicate itself is NOT a planned fix surface — it implements the decided "dismiss/none create nothing"
  semantics; what changes is what the model is given and what the log records]
